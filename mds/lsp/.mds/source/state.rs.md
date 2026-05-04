# src/state.rs

## Purpose

Migrated implementation source for `src/state.rs`.

## Contract

- Preserve the behavior of the pre-migration Rust source.
- This file is synchronized into `.build/rust/mds/lsp/src/state.rs`.

## Imports

| From | Target | Symbols | Via | Summary | Reference |
| --- | --- | --- | --- | --- | --- |
| builtin | std::collections | HashMap | - | - | - |
| builtin | std::path | Path | - | - | - |
| builtin | std::path | PathBuf | - | - | - |
| builtin | std::sync | Arc | - | - | - |
| external | mds_core | Config | - | - | [../../../core/.mds/source/lib.rs.md#source](../../../core/.mds/source/lib.rs.md#source) |
| external | mds_core | ImplDoc | - | - | [../../../core/.mds/source/lib.rs.md#source](../../../core/.mds/source/lib.rs.md#source) |
| external | mds_core | Lang | - | - | [../../../core/.mds/source/lib.rs.md#source](../../../core/.mds/source/lib.rs.md#source) |
| external | mds_core | Package | - | - | [../../../core/.mds/source/lib.rs.md#source](../../../core/.mds/source/lib.rs.md#source) |
| external | tokio::sync | RwLock | - | - | - |

## Source

Open file content tracked by the LSP.

````rs
#[allow(dead_code)]
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
#[allow(dead_code)]
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
#[allow(dead_code)]
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
#[allow(dead_code)]
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
#[allow(dead_code)]
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
#[allow(dead_code)]
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
