# src/main.rs

## Purpose

Migrated implementation source for `src/main.rs`.

## Contract

- Preserve the behavior of the pre-migration Rust source.
- This file is synchronized into `.build/rust/mds/lsp/src/main.rs`.

## Imports

| Kind | From | Target | Symbols | Via | Summary | Code |
| --- | --- | --- | --- | --- | --- | --- |
| rust-use | external | tower_lsp | LspService, Server | tower_lsp |  | `use tower_lsp::{LspService, Server};` |
| rust-use | external | tracing_subscriber | EnvFilter | tracing_subscriber |  | `use tracing_subscriber::EnvFilter;` |


## Source


````rs
mod capabilities;
````

````rs
mod convert;
````

````rs
mod labels;
````

````rs
mod server;
````



````rs
mod state;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_writer(std::io::stderr)
        .init();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(server::MdsLanguageServer::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}
````