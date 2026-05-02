# src/state.rs

## Purpose

Migrated implementation source for `src/state.rs`.

## Contract

- Preserve the behavior of the pre-migration Rust source.
- This file is synchronized into `.build/rust/mds-lsp/src/state.rs`.

## Source

````rs
#![allow(dead_code)]
````

````rs
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use mds_core::{Config, ImplDoc, Lang, Package};
use tokio::sync::RwLock;
````

````rs
/// Open file content tracked by the LSP.
#[derive(Debug, Clone)]

pub struct OpenFile {
    pub uri: String,
    pub path: PathBuf,
    pub text: String,
    pub version: i32,
    pub lang: Option<Lang>,
}

/// Parsed workspace index built from `src-md/` directories.
#[derive(Debug, Clone, Default)]
````

````rs
pub struct WorkspaceIndex {
    /// Map from absolute path → parsed ImplDoc.
    pub docs: HashMap<PathBuf, ImplDoc>,
    /// Map from expose name → list of file paths that expose it.
    pub expose_index: HashMap<String, Vec<PathBuf>>,
    /// Map from file path → list of expose names.
    pub file_exposes: HashMap<PathBuf, Vec<String>>,
}

/// Per-package state.
#[derive(Debug, Clone)]
````

````rs
pub struct PackageState {
    pub package: Package,
    pub index: WorkspaceIndex,
}

/// Global workspace state shared across handlers.
#[derive(Debug, Default)]
````

````rs
pub struct WorkspaceState {
    /// Root folders of the workspace.
    pub workspace_folders: Vec<PathBuf>,
    /// Open documents tracked by URI.
    pub open_files: HashMap<String, OpenFile>,
    /// Discovered mds packages.
    pub packages: Vec<PackageState>,
    /// Config file path → Config.
    pub configs: HashMap<PathBuf, Config>,
}
````

````rs
pub type SharedState = Arc<RwLock<WorkspaceState>>;
````

````rs
impl WorkspaceState {
    /// Find the package that owns a given file path.
    pub fn package_for_path(&self, path: &Path) -> Option<&PackageState> {
        self.packages
            .iter()
            .find(|pkg| path.starts_with(&pkg.package.root))
    }

    /// Find an ImplDoc by its absolute path across all packages.
    pub fn find_doc(&self, path: &Path) -> Option<&ImplDoc> {
        for pkg in &self.packages {
            if let Some(doc) = pkg.index.docs.get(path) {
                return Some(doc);
            }
        }
        None
    }

    /// Find all file paths that expose a given name.
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