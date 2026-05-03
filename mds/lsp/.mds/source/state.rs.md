# src/state.rs

## Purpose

Migrated implementation source for `src/state.rs`.

## Contract

- Preserve the behavior of the pre-migration Rust source.
- This file is synchronized into `.build/rust/mds/lsp/src/state.rs`.

## Imports

| Kind | From | Target | Symbols | Via | Summary | Code |
| --- | --- | --- | --- | --- | --- | --- |
| rust-use | builtin | std::collections | HashMap | std |  | `use std::collections::HashMap;` |
| rust-use | builtin | std::path | Path, PathBuf | std |  | `use std::path::{Path, PathBuf};` |
| rust-use | builtin | std::sync | Arc | std |  | `use std::sync::Arc;` |
| rust-use | external | mds_core | Config, ImplDoc, Lang, Package | mds_core |  | `use mds_core::{Config, ImplDoc, Lang, Package};` |
| rust-use | external | tokio::sync | RwLock | tokio |  | `use tokio::sync::RwLock;` |

## Source

````rs
#![allow(dead_code)]
````

Open file content tracked by the LSP.

````rs
#[derive(Debug, Clone)]
pub struct OpenFile {
    pub uri: String,
    pub path: PathBuf,
    pub text: String,
    pub version: i32,
    pub lang: Option<Lang>,
}
````

Parsed workspace index built from mds authoring roots.

Workspace index field meanings:

- `docs`: absolute path to parsed `ImplDoc`
- `expose_index`: expose name to file path list
- `file_exposes`: file path to expose name list

````rs
#[derive(Debug, Clone, Default)]
pub struct WorkspaceIndex {
    pub docs: HashMap<PathBuf, ImplDoc>,
    pub expose_index: HashMap<String, Vec<PathBuf>>,
    pub file_exposes: HashMap<PathBuf, Vec<String>>,
}
````

Per-package state.

````rs
#[derive(Debug, Clone)]
pub struct PackageState {
    pub package: Package,
    pub index: WorkspaceIndex,
}
````

Global workspace state shared across handlers.

Workspace state field meanings:

- `workspace_folders`: root folders in the workspace
- `open_files`: open documents tracked by URI
- `packages`: discovered mds packages
- `configs`: config file path to `Config`

````rs
#[derive(Debug, Default)]
pub struct WorkspaceState {
    pub workspace_folders: Vec<PathBuf>,
    pub open_files: HashMap<String, OpenFile>,
    pub packages: Vec<PackageState>,
    pub configs: HashMap<PathBuf, Config>,
}
````

````rs
pub type SharedState = Arc<RwLock<WorkspaceState>>;
````

Find the package that owns a given file path.

````rs
impl WorkspaceState {
    pub fn package_for_path(&self, path: &Path) -> Option<&PackageState> {
        self.packages
            .iter()
            .find(|pkg| path.starts_with(&pkg.package.root))
    }
}
````

Find an `ImplDoc` by its absolute path across all packages.

````rs
impl WorkspaceState {
    pub fn find_doc(&self, path: &Path) -> Option<&ImplDoc> {
        for pkg in &self.packages {
            if let Some(doc) = pkg.index.docs.get(path) {
                return Some(doc);
            }
        }
        None
    }
}
````

Find all file paths that expose a given name.

````rs
impl WorkspaceState {
    pub fn find_expose_locations(&self, name: &str) -> Vec<PathBuf> {
        let mut results = Vec::new();
        for pkg in &self.packages {
            if let Some(paths) = pkg.index.expose_index.get(name) {
                results.extend(paths.iter().cloned());
            }
        }
        results
    }
}
````