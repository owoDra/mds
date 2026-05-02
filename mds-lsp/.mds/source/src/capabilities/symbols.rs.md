# src/capabilities/symbols.rs

## Purpose

Migrated implementation source for `src/capabilities/symbols.rs`.

## Contract

- Preserve the behavior of the pre-migration Rust source.
- This file is synchronized into `.build/rust/mds-lsp/src/capabilities/symbols.rs`.

## Source

````rs
use tower_lsp::lsp_types::*;

use crate::state::WorkspaceState;
````

````rs
/// Extract document symbols (section headings) from mds Markdown.
/// Returns SymbolInformation with placeholder URIs; callers should replace
/// URIs with the actual document URI.
#[allow(deprecated)]

pub fn document_symbols(text: &str) -> Vec<SymbolInformation> {
    let mut symbols = Vec::new();

    for (idx, line) in text.lines().enumerate() {
        if let Some(title) = line.strip_prefix("## ") {
            let title = title.trim();
            if !title.is_empty() {
                #[allow(deprecated)]
                symbols.push(SymbolInformation {
                    name: title.to_string(),
                    kind: SymbolKind::NAMESPACE,
                    tags: None,
                    deprecated: None,
                    location: Location {
                        uri: Url::parse("file:///").unwrap_or_else(|_| {
                            Url::parse("untitled:symbol").expect("fallback URI")
                        }),
                        range: Range {
                            start: Position {
                                line: idx as u32,
                                character: 0,
                            },
                            end: Position {
                                line: idx as u32,
                                character: line.len() as u32,
                            },
                        },
                    },
                    container_name: None,
                });
            }
        } else if let Some(title) = line.strip_prefix("### ") {
            let title = title.trim();
            if !title.is_empty() {
                #[allow(deprecated)]
                symbols.push(SymbolInformation {
                    name: title.to_string(),
                    kind: SymbolKind::FIELD,
                    tags: None,
                    deprecated: None,
                    location: Location {
                        uri: Url::parse("file:///").unwrap_or_else(|_| {
                            Url::parse("untitled:symbol").expect("fallback URI")
                        }),
                        range: Range {
                            start: Position {
                                line: idx as u32,
                                character: 0,
                            },
                            end: Position {
                                line: idx as u32,
                                character: line.len() as u32,
                            },
                        },
                    },
                    container_name: None,
                });
            }
        }
    }

    symbols
}

/// Search workspace symbols (expose names) matching a query.
#[allow(deprecated)]
````

````rs
pub fn workspace_symbols(query: &str, state: &WorkspaceState) -> Vec<SymbolInformation> {
    let mut symbols = Vec::new();

    for pkg in &state.packages {
        for (name, paths) in &pkg.index.expose_index {
            if query.is_empty() || name.to_lowercase().contains(query) {
                for path in paths {
                    if let Ok(uri) = Url::from_file_path(path) {
                        #[allow(deprecated)]
                        symbols.push(SymbolInformation {
                            name: name.clone(),
                            kind: SymbolKind::MODULE,
                            tags: None,
                            deprecated: None,
                            location: Location {
                                uri,
                                range: Range::default(),
                            },
                            container_name: Some(
                                pkg.package
                                    .root
                                    .file_name()
                                    .map(|n| n.to_string_lossy().to_string())
                                    .unwrap_or_default(),
                            ),
                        });
                    }
                }
            }
        }
    }

    symbols
}
````