use pxp_ast::{
    DocBlock, DocBlockComment, DocBlockDeprecatedTag, DocBlockExtendsTag, DocBlockGenericTag,
    DocBlockImplementsTag, DocBlockMethodTag, DocBlockNode, DocBlockParamClosureThisTag,
    DocBlockParamTag, DocBlockPropertyTag, DocBlockReturnTag, DocBlockTag, DocBlockTagNode,
    DocBlockTemplateTag, DocBlockTemplateTagValue, DocBlockTextNode, DocBlockUsesTag,
    DocBlockVarTag, SimpleVariable,
};
use pxp_bytestring::ByteString;
use pxp_diagnostics::Severity;
use pxp_span::{IsSpanned, Span};
use pxp_token::TokenKind;

use crate::{Parser, ParserDiagnostic};

impl<'a> Parser<'a> {
    pub(crate) const fn is_in_docblock(&self) -> bool {
        self.in_docblock
    }

    fn enter_docblock(&mut self) {
        self.in_docblock = true;
    }

    fn exit_docblock(&mut self) {
        self.in_docblock = false;
    }

    pub(crate) fn skip_doc_eol(&mut self) {
        if self.current_kind() == TokenKind::PhpDocEol {
            self.next();
        }

        while !self.is_eof() && self.current_kind() == TokenKind::PhpDocHorizontalWhitespace {
            self.next();
        }
    }

    pub(crate) fn parse_docblock(&mut self) -> DocBlockComment {
        if self.current_kind() != TokenKind::OpenPhpDoc {
            unreachable!();
        }

        self.enter_docblock();

        let start = self.next();
        let mut nodes = Vec::new();

        while !self.is_eof() && self.current_kind() != TokenKind::ClosePhpDoc {
            match self.current_kind() {
                TokenKind::PhpDocEol => {
                    self.next();
                }
                TokenKind::PhpDocTag => {
                    let tag = self.parse_docblock_tag();

                    nodes.push(DocBlockNode::Tag(tag))
                }
                _ => {
                    if let Some(text) = self.parse_docblock_text() {
                        nodes.push(DocBlockNode::Text(text))
                    }
                }
            };
        }

        let close_phpdoc = self.skip(TokenKind::ClosePhpDoc);
        let span = start.join(close_phpdoc);

        self.exit_docblock();

        DocBlockComment {
            id: self.id(),
            span,
            doc: DocBlock {
                id: self.id(),
                span,
                nodes,
            },
        }
    }

    fn parse_docblock_tag(&mut self) -> DocBlockTagNode {
        let tag = match self.current_symbol().as_ref() {
            b"@param-closure-this" | b"@phpstan-param-closure-this" => {
                self.param_closure_this_tag()
            }
            b"@param" | b"@phpstan-param" | b"@psalm-param" | b"@phan-param" => self.param_tag(),
            b"@var" | b"@phpstan-var" | b"@psalm-var" | b"@phan-var" => self.var_tag(),
            b"@return" | b"@phpstan-return" | b"@psalm-return" | b"@phan-return"
            | b"@phan-real-return" => self.return_tag(),
            b"@property"
            | b"@property-read"
            | b"@property-write"
            | b"@phpstan-property"
            | b"@phpstan-property-read"
            | b"@phpstan-property-write"
            | b"@psalm-property"
            | b"@psalm-property-read"
            | b"@psalm-property-write"
            | b"@phan-property"
            | b"@phan-property-read"
            | b"@phan-property-write" => self.property_tag(),
            b"@method" | b"@phpstan-method" | b"@psalm-method" | b"@phan-method" => {
                self.method_tag()
            }
            b"@template"
            | b"@phpstan-template"
            | b"@psalm-template"
            | b"@phan-template"
            | b"@template-covariant"
            | b"@phpstan-template-covariant"
            | b"@psalm-template-covariant"
            | b"@template-contravariant"
            | b"@phpstan-template-contravariant"
            | b"@psalm-template-contravariant" => self.template_tag(),
            b"@extends" | b"@phpstan-extends" | b"@phan-extends" | b"@phan-inherits"
            | b"@template-extends" => self.extends_tag(),
            b"@implements" | b"@phpstan-implements" | b"@template-implements" => {
                self.implements_tag()
            }
            b"@use" | b"@phpstan-use" | b"@template-use" => self.use_tag(),
            b"@deprecated" => self.deprecated_tag(),
            _ => self.generic_tag(),
        };

        DocBlockTagNode {
            id: self.id(),
            span: tag.span(),
            tag,
        }
    }

    fn deprecated_tag(&mut self) -> DocBlockTag {
        let tag = self.current().to_owned();

        self.next();

        let (text, text_span) = self.read_text_until_eol_or_close();

        let span = if let Some(text_span) = text_span {
            tag.span.join(text_span)
        } else {
            tag.span
        };

        DocBlockTag::Deprecated(DocBlockDeprecatedTag {
            id: self.id(),
            span,
            tag,
            text,
        })
    }

    fn use_tag(&mut self) -> DocBlockTag {
        let tag = self.current().to_owned();

        self.next();

        let data_type = self.parse_data_type();
        let (text, text_span) = self.read_text_until_eol_or_close();

        DocBlockTag::Uses(DocBlockUsesTag {
            id: self.id(),
            span: if let Some(text_span) = text_span {
                tag.span.join(text_span)
            } else {
                tag.span.join(data_type.span)
            },
            tag,
            data_type,
            text,
        })
    }

    fn implements_tag(&mut self) -> DocBlockTag {
        let tag = self.current().to_owned();

        self.next();

        let data_type = self.parse_data_type();
        let (text, text_span) = self.read_text_until_eol_or_close();

        DocBlockTag::Implements(DocBlockImplementsTag {
            id: self.id(),
            span: if let Some(text_span) = text_span {
                tag.span.join(text_span)
            } else {
                tag.span.join(data_type.span())
            },
            tag,
            data_type,
            text,
        })
    }

    fn extends_tag(&mut self) -> DocBlockTag {
        let tag = self.current().to_owned();

        self.next();

        let data_type = self.parse_data_type();
        let (text, text_span) = self.read_text_until_eol_or_close();

        DocBlockTag::Extends(DocBlockExtendsTag {
            id: self.id(),
            span: if let Some(text_span) = text_span {
                tag.span.join(text_span)
            } else {
                tag.span.join(data_type.span())
            },
            tag,
            data_type,
            text,
        })
    }

    fn template_tag(&mut self) -> DocBlockTag {
        let tag = self.current().to_owned();

        self.next();

        let value = self.template_tag_value(true);

        self.read_text_until_eol_or_close();

        let span = tag.span.join(value.span);

        DocBlockTag::Template(DocBlockTemplateTag {
            id: self.id(),
            span,
            tag,
            value,
        })
    }

    fn method_tag(&mut self) -> DocBlockTag {
        let tag = self.current().to_owned();

        self.next();

        let r#static = self.optional(TokenKind::Static);

        let (return_type, name) = if self.is_identifier_maybe_reserved(self.current_kind())
            && self.peek_kind() == TokenKind::LeftParen
        {
            (None, self.parse_identifier_maybe_reserved())
        } else {
            (
                Some(self.parse_data_type()),
                self.parse_identifier_maybe_reserved(),
            )
        };

        let mut templates = Vec::new();

        if self.current_kind() == TokenKind::LessThan {
            self.next();

            loop {
                templates.push(self.template_tag_value(false));

                if self.current_kind() != TokenKind::Comma {
                    break;
                } else {
                    self.next();
                    self.skip_doc_eol();
                }
            }

            self.expect(TokenKind::GreaterThan);
        }

        let parameters = self.parse_function_parameter_list();
        let (text, text_span) = self.read_text_until_eol_or_close();
        let span = if let Some(text_span) = text_span {
            tag.span.join(text_span)
        } else {
            tag.span.join(parameters.span)
        };

        DocBlockTag::Method(DocBlockMethodTag {
            id: self.id(),
            span,
            tag,
            r#static,
            return_type,
            templates,
            name,
            parameters,
            text,
        })
    }

    pub(crate) fn template_tag_value(
        &mut self,
        parse_description: bool,
    ) -> DocBlockTemplateTagValue {
        let template = self.parse_type_identifier();
        let (mut bound, mut lower_bound) = (None, None);

        if self.current_kind() == TokenKind::PhpDocOf || self.current_kind() == TokenKind::As {
            self.next();
            bound = Some(self.parse_data_type());
        }

        if self.current_kind() == TokenKind::PhpDocSuper {
            self.next();
            lower_bound = Some(self.parse_data_type());
        }

        let default = if self.current_kind() == TokenKind::Equals {
            self.next();
            Some(self.parse_data_type())
        } else {
            None
        };

        let description = if parse_description {
            self.parse_docblock_text()
        } else {
            None
        };

        DocBlockTemplateTagValue {
            id: self.id(),
            span: if let Some(description) = &description {
                template.span.join(description.span)
            } else if let Some(default) = &default {
                template.span.join(default.span())
            } else if let Some(lower_bound) = &lower_bound {
                template.span.join(lower_bound.span())
            } else if let Some(upper_bound) = &bound {
                template.span.join(upper_bound.span())
            } else {
                template.span
            },
            template,
            bound,
            default,
            lower_bound,
            description,
        }
    }

    fn property_tag(&mut self) -> DocBlockTag {
        let tag = self.current().to_owned();

        self.next();

        let data_type = self.parse_optional_data_type();
        let variable = self.parse_simple_variable();
        let (text, text_span) = self.read_text_until_eol_or_close();

        let span = if let Some(text_span) = text_span {
            tag.span.join(text_span)
        } else if variable.is_missing() {
            tag.span.join(data_type.span())
        } else {
            tag.span.join(variable.span())
        };

        DocBlockTag::Property(DocBlockPropertyTag {
            id: self.id(),
            span,
            tag,
            data_type,
            variable,
            text,
        })
    }

    fn param_closure_this_tag(&mut self) -> DocBlockTag {
        let tag = self.current().to_owned();

        self.next();

        let r#type = self.parse_data_type();

        let variable = match self.current_kind() {
            TokenKind::Variable => self.parse_simple_variable(),
            _ => {
                self.diagnostic(
                    ParserDiagnostic::ExpectedToken {
                        expected: vec![TokenKind::Variable],
                        found: self.current().to_owned(),
                    },
                    Severity::Warning,
                    self.current_span(),
                );

                SimpleVariable::missing(self.id(), self.current_span())
            }
        };

        let (text, text_span) = self.read_text_until_eol_or_close();

        let span = if let Some(text_span) = text_span {
            tag.span.join(text_span)
        } else if !variable.is_missing() {
            tag.span.join(variable.span())
        } else {
            tag.span.join(r#type.span())
        };

        DocBlockTag::ParamClosureThis(DocBlockParamClosureThisTag {
            id: self.id(),
            span,
            tag,
            r#type,
            variable,
            text,
        })
    }

    fn param_tag(&mut self) -> DocBlockTag {
        let tag = self.current().to_owned();

        self.next();

        let ampersand = self.optional(TokenKind::Ampersand);
        let ellipsis = self.optional(TokenKind::Ellipsis);
        let data_type = self.parse_optional_data_type();
        let variable = self.parse_optional_simple_variable();
        let (text, text_span) = self.read_text_until_eol_or_close();

        let span = if let Some(text_span) = text_span {
            tag.span.join(text_span)
        } else if variable.is_some() {
            tag.span.join(variable.span())
        } else if data_type.is_some() {
            tag.span.join(data_type.span())
        } else {
            tag.span
        };

        DocBlockTag::Param(DocBlockParamTag {
            id: self.id(),
            span,
            tag,
            ampersand,
            ellipsis,
            data_type,
            variable,
            text,
        })
    }

    fn var_tag(&mut self) -> DocBlockTag {
        let tag = self.current().to_owned();

        self.next();

        let data_type = self.parse_optional_data_type();
        let variable = self.parse_optional_simple_variable();
        let (text, text_span) = self.read_text_until_eol_or_close();

        let span = if let Some(text_span) = text_span {
            tag.span.join(text_span)
        } else if variable.is_some() {
            tag.span.join(variable.span())
        } else if data_type.is_some() {
            tag.span.join(data_type.span())
        } else {
            tag.span
        };

        DocBlockTag::Var(DocBlockVarTag {
            id: self.id(),
            span,
            tag,
            data_type,
            variable,
            text,
        })
    }

    fn return_tag(&mut self) -> DocBlockTag {
        let tag = self.current().to_owned();

        self.next();

        let data_type = self.parse_optional_data_type();
        let (text, text_span) = self.read_text_until_eol_or_close();

        DocBlockTag::Return(DocBlockReturnTag {
            id: self.id(),
            span: if let Some(text_span) = text_span {
                tag.span.join(text_span)
            } else if data_type.is_some() {
                tag.span.join(data_type.span())
            } else {
                tag.span
            },
            tag,
            data_type,
            text,
        })
    }

    fn generic_tag(&mut self) -> DocBlockTag {
        let tag = self.current().to_owned();

        self.next();

        let (text, text_span) = self.read_text_until_eol_or_close();

        let span = if let Some(text_span) = text_span {
            Span::combine(tag.span, text_span)
        } else {
            tag.span
        };

        DocBlockTag::Generic(DocBlockGenericTag {
            id: self.id(),
            span,
            tag,
            text,
        })
    }

    fn parse_docblock_text(&mut self) -> Option<DocBlockTextNode> {
        let (content, span) = self.read_text_until_eol_or_close();

        content.as_ref()?;

        Some(DocBlockTextNode {
            id: self.id(),
            span: span.unwrap(),
            content: content.unwrap(),
        })
    }

    fn read_text_until_eol_or_close(&mut self) -> (Option<ByteString>, Option<Span>) {
        let mut text = ByteString::empty();
        let start_span = self.current_span();

        loop {
            if self.is_eof()
                || matches!(
                    self.current_kind(),
                    TokenKind::PhpDocEol | TokenKind::ClosePhpDoc
                )
            {
                break;
            }

            text.extend_with_bytes(self.current_symbol());

            self.next_without_skipping_whitespace();
        }

        if text.is_empty() {
            return (None, None);
        }

        let end_span = self.current_span();
        let span = Span::combine(start_span, end_span);

        (Some(text), Some(span))
    }
}
