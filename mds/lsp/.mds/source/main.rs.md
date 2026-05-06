# src/main.rs

## Purpose

Migrated implementation source for `src/main.rs`.

## Contract

- Preserve the behavior of the pre-migration Rust source.
- This file is synchronized into `.build/rust/mds/lsp/src/main.rs`.

## Exports

| Name | Visibility | Summary |
| --- | --- | --- |
| main | internal | mds-lsp process entrypoint. |

## Imports

| From | Target | Symbols | Via | Summary | Reference |
| --- | --- | --- | --- | --- | --- |
| external | tower_lsp | LspService | - | - | - |
| external | tower_lsp | Server | - | - | - |
| external | tracing_subscriber | EnvFilter | - | - | - |


## Source


##### main

Starts the mds language server over standard input and output.


````rs
mod capabilities;
mod convert;
mod labels;
mod server;
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
