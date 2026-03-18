use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use clap::Args;
use dashmap::DashMap;
use pxp_ast::{Name, NameKind, Node};
use pxp_index::{FileId, Index};
use pxp_lexer::Lexer;
use pxp_node_finder::NodeFinder;
use pxp_parser::Parser;
use pxp_span::{byte_offset_to_line_and_column, ByteOffset, IsSpanned};
use tokio::sync::RwLock;
use tower_lsp::{
    jsonrpc::Result,
    lsp_types::{
        CompletionItem, CompletionItemKind, CompletionOptions, CompletionParams,
        CompletionResponse, DidChangeTextDocumentParams, DidCloseTextDocumentParams,
        DidOpenTextDocumentParams, DidSaveTextDocumentParams, GotoDefinitionParams,
        GotoDefinitionResponse, Hover, HoverContents, HoverParams, HoverProviderCapability,
        InitializeParams, InitializeResult, InitializedParams, Location, MarkupContent,
        MarkupKind, MessageType, OneOf, Position, Range, ServerCapabilities, ServerInfo,
        TextDocumentSyncCapability, TextDocumentSyncKind, Url, WorkDoneProgressOptions,
    },
    Client, LanguageServer,
};
use tracing::{error, info, warn};

/// Symbol information extracted from AST nodes
#[derive(Debug, Clone)]
struct SymbolInfo {
    name: String,
    kind: SymbolKind,
    location: pxp_span::Span,
    hover_info: String,
}

/// Types of symbols we can identify
#[derive(Debug, Clone, PartialEq)]
enum SymbolKind {
    Function,
    Class,
    Method,
    Variable,
}

#[derive(Args, Debug)]
pub struct Lsp {
    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,

    /// Log file path (defaults to stderr)
    #[arg(long)]
    log_file: Option<PathBuf>,
}

pub fn lsp(args: Lsp) -> anyhow::Result<()> {
    // Initialize logging
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(if args.verbose {
            tracing::Level::DEBUG
        } else {
            tracing::Level::INFO
        });

    if let Some(log_file) = args.log_file {
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file)?;
        subscriber.with_writer(file).init();
    } else {
        subscriber.with_writer(std::io::stderr).init();
    }

    info!("Starting PXP Language Server");

    // Start the LSP server
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(async {
            let stdin = tokio::io::stdin();
            let stdout = tokio::io::stdout();

            let (service, socket) = tower_lsp::LspService::new(|client| PxpLanguageServer::new(client));

            tower_lsp::Server::new(stdin, stdout, socket).serve(service).await;
        });

    Ok(())
}

/// The main Language Server implementation
pub struct PxpLanguageServer {
    client: Client,
    index: Arc<RwLock<Index>>,
    open_files: Arc<DashMap<Url, String>>,
    file_id_map: Arc<DashMap<Url, FileId>>,
    workspace_root: Arc<RwLock<Option<PathBuf>>>,
}

impl PxpLanguageServer {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            index: Arc::new(RwLock::new(Index::new())),
            open_files: Arc::new(DashMap::new()),
            file_id_map: Arc::new(DashMap::new()),
            workspace_root: Arc::new(RwLock::new(None)),
        }
    }

    /// Index a single PHP file
    async fn index_file(&self, uri: &Url, content: &str) -> anyhow::Result<()> {
        let path = uri.to_file_path().map_err(|_| {
            anyhow::anyhow!("Invalid file URI: {}", uri)
        })?;

        // Parse the file
        let parse_result = Parser::parse(Lexer::new(content.as_bytes()));
        if !parse_result.diagnostics.is_empty() {
            warn!("Parse errors in {}: {:?}", path.display(), parse_result.diagnostics);
        }

        // Get or create file ID
        let file_id = {
            let mut index = self.index.write().await;
            let file_id = FileId::new(self.file_id_map.len());
            self.file_id_map.insert(uri.clone(), file_id);
            
            // Index the AST
            index.index(file_id, &parse_result.ast);
            file_id
        };

        info!("Indexed file: {} (ID: {:?})", path.display(), file_id);
        Ok(())
    }

    /// Index all PHP files in the workspace
    async fn index_workspace(&self) -> anyhow::Result<()> {
        let workspace_root = self.workspace_root.read().await;
        let Some(root) = workspace_root.as_ref() else {
            warn!("No workspace root set, skipping workspace indexing");
            return Ok(());
        };

        info!("Starting workspace indexing from: {}", root.display());

        // Find all PHP files
        let php_files = self.find_php_files(root).await?;
        info!("Found {} PHP files to index", php_files.len());

        // Index each file
        for file_path in php_files {
            if let Ok(content) = tokio::fs::read_to_string(&file_path).await {
                let uri = Url::from_file_path(&file_path).map_err(|_| {
                    anyhow::anyhow!("Failed to create URI from path: {}", file_path.display())
                })?;
                
                if let Err(e) = self.index_file(&uri, &content).await {
                    error!("Failed to index {}: {}", file_path.display(), e);
                }
            } else {
                warn!("Could not read file: {}", file_path.display());
            }
        }

        let index = self.index.read().await;
        info!(
            "Workspace indexing complete. Files: {}, Functions: {}, Classes: {}",
            index.number_of_files(),
            index.number_of_functions(),
            index.number_of_classes()
        );

        Ok(())
    }

    /// Find all PHP files in a directory recursively
    async fn find_php_files(&self, dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
        let mut php_files = Vec::new();
        let mut stack = vec![dir.to_path_buf()];

        while let Some(current_dir) = stack.pop() {
            if let Ok(mut entries) = tokio::fs::read_dir(&current_dir).await {
                while let Ok(Some(entry)) = entries.next_entry().await {
                    let path = entry.path();
                    
                    if path.is_dir() {
                        // Skip common directories that shouldn't be indexed
                        if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                            if !matches!(dir_name, "node_modules" | ".git" | "vendor" | ".vscode" | ".idea") {
                                stack.push(path);
                            }
                        }
                    } else if let Some(ext) = path.extension() {
                        if ext == "php" {
                            php_files.push(path);
                        }
                    }
                }
            }
        }

        Ok(php_files)
    }

    /// Get completion items for a position
    async fn get_completions(&self, _uri: &Url, _position: Position) -> Result<Vec<CompletionItem>> {
        let _index = self.index.read().await;
        let mut completions = Vec::new();

        // Add function completions
        // Note: This is a simplified implementation. In a real LSP, you'd need to:
        // 1. Parse the current position to understand context
        // 2. Determine if we're in a function call, method call, etc.
        // 3. Provide context-appropriate completions
        
        // For now, we'll just provide all known functions
        // This would need to be implemented when the reflection API is enhanced
        // to provide iteration over all functions
        
        // Add some basic PHP built-in completions as an example
        completions.push(CompletionItem {
            label: "echo".to_string(),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: Some("PHP built-in function".to_string()),
            documentation: Some(tower_lsp::lsp_types::Documentation::MarkupContent(
                MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: "Outputs one or more strings".to_string(),
                }
            )),
            ..Default::default()
        });

        completions.push(CompletionItem {
            label: "print".to_string(),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: Some("PHP built-in function".to_string()),
            documentation: Some(tower_lsp::lsp_types::Documentation::MarkupContent(
                MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: "Outputs a string".to_string(),
                }
            )),
            ..Default::default()
        });

        info!("Providing {} completions", completions.len());
        Ok(completions)
    }

    /// Convert LSP position to byte offset
    async fn position_to_byte_offset(&self, uri: &Url, position: Position) -> Result<ByteOffset> {
        let content = self.open_files.get(uri)
            .ok_or_else(|| tower_lsp::jsonrpc::Error::invalid_params("File not found in open files"))?;
        
        let lines: Vec<&str> = content.lines().collect();
        let mut offset = 0;
        
        // Add bytes for all lines before the target line
        for (line_idx, line) in lines.iter().enumerate() {
            if line_idx as u32 >= position.line {
                break;
            }
            offset += line.len() + 1; // +1 for newline character
        }
        
        // Add bytes for characters within the target line
        if let Some(target_line) = lines.get(position.line as usize) {
            let char_offset = std::cmp::min(position.character as usize, target_line.len());
            offset += char_offset;
        }
        
        Ok(offset)
    }

    /// Convert byte offset to LSP position
    fn byte_offset_to_position(&self, source: &str, offset: ByteOffset) -> Position {
        let (line, column) = byte_offset_to_line_and_column(source.as_bytes(), offset);
        Position {
            line: line as u32,
            character: column as u32,
        }
    }

    /// Find symbol at position and get symbol information
    async fn find_symbol_at_position(&self, uri: &Url, position: Position) -> Result<Option<SymbolInfo>> {
        let content = self.open_files.get(uri)
            .ok_or_else(|| tower_lsp::jsonrpc::Error::invalid_params("File not found in open files"))?;
        
        let offset = self.position_to_byte_offset(uri, position).await?;
        
        // Parse the document
        let parse_result = Parser::parse(pxp_lexer::Lexer::new(content.as_bytes()));
        
        // Find the node at the given position
        if let Some((node, _ancestors)) = NodeFinder::find_at_byte_offset(&parse_result.ast, offset) {
            let symbol_info = self.extract_symbol_info(&node, &content).await;
            return Ok(symbol_info);
        }
        
        Ok(None)
    }

    /// Extract symbol information from AST node - simplified version
    async fn extract_symbol_info(&self, node: &Node<'_>, _content: &str) -> Option<SymbolInfo> {
        // For now, return basic placeholder information based on node type
        // This is a simplified implementation to get the LSP working
        Some(SymbolInfo {
            name: format!("Symbol at position"),
            kind: SymbolKind::Variable,
            location: node.span,
            hover_info: format!("**Symbol information**\n\nType: {}", node.name()),
        })
    }

    /// Extract name string from Name AST node
    fn extract_name_string(&self, name: &Name) -> String {
        match &name.kind {
            NameKind::Resolved(resolved) => String::from_utf8_lossy(&resolved.resolved).to_string(),
            NameKind::Unresolved(unresolved) => String::from_utf8_lossy(&unresolved.symbol).to_string(),
            NameKind::Special(_special) => "special".to_string(),
        }
    }

    /// Get hover information for a position
    async fn get_hover(&self, uri: &Url, position: Position) -> Result<Option<Hover>> {
        info!("Hover request at {}:{} in {}", position.line, position.character, uri);
        
        if let Some(symbol_info) = self.find_symbol_at_position(uri, position).await? {
            let content = self.open_files.get(uri).unwrap();
            let range = Some(Range {
                start: self.byte_offset_to_position(&content, symbol_info.location.start),
                end: self.byte_offset_to_position(&content, symbol_info.location.end),
            });
            
            return Ok(Some(Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: symbol_info.hover_info,
                }),
                range,
            }));
        }
        
        Ok(None)
    }

    /// Get definition location for a symbol
    async fn get_definition(&self, uri: &Url, position: Position) -> Result<Option<GotoDefinitionResponse>> {
        info!("Go to definition request at {}:{} in {}", position.line, position.character, uri);
        
        if let Some(symbol_info) = self.find_symbol_at_position(uri, position).await? {
            let index = self.index.read().await;
            
            match symbol_info.kind {
                SymbolKind::Function => {
                    if let Some(function) = index.get_function(symbol_info.name.as_str()) {
                        if let Some(location) = self.create_location_from_function(&function).await {
                            return Ok(Some(GotoDefinitionResponse::Scalar(location)));
                        }
                    }
                },
                SymbolKind::Class => {
                    if let Some(class) = index.get_class(symbol_info.name.as_str()) {
                        if let Some(location) = self.create_location_from_class(&class).await {
                            return Ok(Some(GotoDefinitionResponse::Scalar(location)));
                        }
                    }
                },
                SymbolKind::Method => {
                    // For now, methods will just show "Not implemented" 
                    // TODO: Implement proper method goto definition
                },
                SymbolKind::Variable => {
                    // Variables don't have global definitions in PHP
                    // Could implement local variable definition finding in the future
                }
            }
        }
        
        Ok(None)
    }

    /// Create LSP Location from function entity - simplified for now
    async fn create_location_from_function(&self, _function: &pxp_index::ReflectionFunction<'_>) -> Option<Location> {
        // TODO: Implement once file path mapping is available
        None
    }

    /// Create LSP Location from class entity - simplified for now  
    async fn create_location_from_class(&self, _class: &pxp_index::ReflectionClass<'_>) -> Option<Location> {
        // TODO: Implement once file path mapping is available
        None
    }

    // For now, we'll skip method-specific goto definition until we have proper method resolution
    // async fn create_location_from_method(&self, method: &pxp_index::ReflectionMethod<'_>) -> Option<Location> {
    //     if let Some(path) = self.index.read().await.get_file_path(method.location()) {
    //         let uri = Url::from_file_path(path).ok()?;
    //         let content = tokio::fs::read_to_string(path).await.ok()?;
    //         
    //         let location = method.location();
    //         let range = Range {
    //             start: self.byte_offset_to_position(&content, location.span().start),
    //             end: self.byte_offset_to_position(&content, location.span().end),
    //         };
    //         
    //         return Some(Location { uri, range });
    //     }
    //     None
    // }
}

#[tower_lsp::async_trait]
impl LanguageServer for PxpLanguageServer {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        info!("LSP Initialize request received");

        // Store workspace root
        if let Some(workspace_folders) = params.workspace_folders {
            if let Some(folder) = workspace_folders.first() {
                if let Ok(path) = folder.uri.to_file_path() {
                    *self.workspace_root.write().await = Some(path);
                }
            }
        } else if let Some(root_uri) = params.root_uri {
            if let Ok(path) = root_uri.to_file_path() {
                *self.workspace_root.write().await = Some(path);
            }
        }

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![
                        "$".to_string(),   // Variable completion
                        "->".to_string(),  // Method completion
                        "::".to_string(),  // Static method completion
                        "\\".to_string(),  // Namespace completion
                    ]),
                    work_done_progress_options: WorkDoneProgressOptions::default(),
                    all_commit_characters: None,
                    ..Default::default()
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "pxp-lsp".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        info!("LSP Server initialized");
        
        // Start workspace indexing in the background
        let server = self.clone();
        tokio::spawn(async move {
            if let Err(e) = server.index_workspace().await {
                error!("Workspace indexing failed: {}", e);
            }
        });

        self.client
            .log_message(MessageType::INFO, "PXP Language Server is ready!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        info!("LSP Server shutting down");
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        info!("Document opened: {}", params.text_document.uri);
        
        // Store the file content
        self.open_files.insert(
            params.text_document.uri.clone(),
            params.text_document.text.clone(),
        );

        // Index the file
        if let Err(e) = self.index_file(&params.text_document.uri, &params.text_document.text).await {
            error!("Failed to index opened file: {}", e);
        }
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        info!("Document changed: {}", params.text_document.uri);
        
        // Update the stored content with the full document (since we use FULL sync)
        if let Some(change) = params.content_changes.into_iter().next() {
            self.open_files.insert(
                params.text_document.uri.clone(),
                change.text.clone(),
            );

            // Re-index the file with new content
            if let Err(e) = self.index_file(&params.text_document.uri, &change.text).await {
                error!("Failed to re-index changed file: {}", e);
            }
        }
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        info!("Document saved: {}", params.text_document.uri);
        
        // If the save includes text, update our content and re-index
        if let Some(text) = params.text {
            self.open_files.insert(params.text_document.uri.clone(), text.clone());
            
            if let Err(e) = self.index_file(&params.text_document.uri, &text).await {
                error!("Failed to re-index saved file: {}", e);
            }
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        info!("Document closed: {}", params.text_document.uri);
        self.open_files.remove(&params.text_document.uri);
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = &params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;

        info!("Completion request at {}:{} in {}", position.line, position.character, uri);

        let completions = self.get_completions(uri, position).await?;
        Ok(Some(CompletionResponse::Array(completions)))
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        self.get_hover(uri, position).await
    }

    async fn goto_definition(&self, params: GotoDefinitionParams) -> Result<Option<GotoDefinitionResponse>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        self.get_definition(uri, position).await
    }
}

// Implement Clone for PxpLanguageServer to allow spawning tasks
impl Clone for PxpLanguageServer {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            index: self.index.clone(),
            open_files: self.open_files.clone(),
            file_id_map: self.file_id_map.clone(),
            workspace_root: self.workspace_root.clone(),
        }
    }
}