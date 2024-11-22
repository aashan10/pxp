test:
    cargo test --lib --bins --tests

tokenise +args:
    RUSTFLAGS=-Awarnings cargo build -q --package pxp-internal --bin tokenise --release
    ./target/release/tokenise {{args}}

bench-tokenise +args:
    RUSTFLAGS=-Awarnings cargo build -q --package pxp-internal --bin tokenise --release
    hyperfine --warmup=1 --runs=3 \
        --command-name="PXP (Release)" "./target/release/tokenise {{args}}"

parse +args:
    RUSTFLAGS=-Awarnings cargo run -q --package pxp-internal --bin parse -- {{args}}

multi-thread-parse +args:
    RUSTFLAGS=-Awarnings cargo run -q --package pxp-internal --bin multi-thread-parse -- {{args}}

node-finder +args:
    RUSTFLAGS=-Awarnings cargo run -q --package pxp-internal --bin node-finder -- {{args}}

infer +args:
    RUSTFLAGS=-Awarnings cargo run -q --package pxp-internal --bin infer -- {{args}}

pls:
    cargo build --package pls
    just --working-directory ./crates/pls/editors/vscode --justfile=./crates/pls/editors/vscode/Justfile package-install

generate-ast:
    php ./meta/scripts/generate-ast.php
    cargo fmt --package pxp-ast

generate-visitor:
    php ./meta/scripts/generate-visitor.php
    cargo fmt --package pxp-ast

generate-node-finder:
    php ./meta/scripts/generate-node-finder.php
    cargo fmt --package pxp-node-finder

bench-parser:
    cargo build --release --bin parse
    hyperfine --warmup=1 --runs=1 \
        --command-name="PHP (AST Ext)" "php ./benches/parsing/core.php ./playground/framework" \
        --command-name="PXP (Release)" "./target/release/parse ./playground/framework --no-output"

bench-lexer:
    cargo build --release --bin tokenise
    hyperfine --warmup=1 --runs=1 \
        --command-name="PHP (Core)" "php ./benches/lexing/php.php ./playground/framework" \
        --command-name="PHP (Core + Opcache)" "php -d opcache.enable_cli=1 ./benches/lexing/php.php ./playground/framework" \
        --command-name="PXP (Release)" "./target/release/tokenise ./playground/framework --no-output"

bench-indexer:
    cargo build --release --bin index
    hyperfine --warmup=1 --runs=1 \
        --command-name="Indexer (Release)" "./target/release/index ./playground/framework --no-output"

bench-visitor:
    cargo build --release --bin visit
    hyperfine --warmup=1 --runs=1 \
        --command-name="Visitor (Release)" "./target/release/visit ./playground/framework --no-output"

bench-node-visitor:
    cargo build --release --bin node-visit
    hyperfine --warmup=1 --runs=1 \
        --command-name="Node Visitor (Release)" "./target/release/node-visit ./playground/framework --no-output"