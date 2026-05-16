use std::collections::{HashMap, HashSet};
use std::path::{Path};
use std::path::{PathBuf};
use std::sync::{Arc};
use mds_core::{Config};
use mds_core::{ImplDoc};
use mds_core::{Lang};
use mds_core::{Package};
use mds_core::{SourceMap, SourceSpan};
use tower_lsp::lsp_types::{Location, Position, Range, Url};
use tokio::sync::{RwLock};
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OpenFile {
    pub uri: String,
    pub path: PathBuf,
    pub text: String,
    pub version: i32,
    pub lang: Option<Lang>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct WorkspaceIndex {
    pub docs: HashMap<PathBuf, ImplDoc>,
    pub expose_index: HashMap<String, Vec<PathBuf>>,
    pub file_exposes: HashMap<PathBuf, Vec<String>>,
    pub module_index: HashMap<String, Vec<PathBuf>>,
    pub symbol_index: HashMap<(String, String), Vec<PathBuf>>,
    pub source_map: SourceMap,
    pub generated_files: HashSet<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct PackageState {
    pub package: Package,
    pub index: WorkspaceIndex,
}

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct WorkspaceState {
    pub workspace_folders: Vec<PathBuf>,
    pub open_files: HashMap<String, OpenFile>,
    pub packages: Vec<PackageState>,
    pub configs: HashMap<PathBuf, Config>,
}

pub type SharedState = Arc<RwLock<WorkspaceState>>;

#[allow(dead_code)]
impl WorkspaceState {
    pub fn package_for_path(&self, path: &Path) -> Option<&PackageState> {
        self.packages
            .iter()
            .find(|pkg| path.starts_with(&pkg.package.root))
    }

    pub fn package_for_generated_path(&self, path: &Path) -> Option<&PackageState> {
        self.packages
            .iter()
            .find(|pkg| pkg.index.generated_files.contains(path))
    }
}

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

#[allow(dead_code)]
impl WorkspaceState {
    pub fn find_module_locations(&self, module: &str) -> Vec<PathBuf> {
        let mut results = Vec::new();
        for pkg in &self.packages {
            if let Some(paths) = pkg.index.module_index.get(module) {
                results.extend(paths.iter().cloned());
            }
        }
        results
    }
}

#[allow(dead_code)]
impl WorkspaceState {
    pub fn find_symbol_locations(&self, module: &str, symbol: &str) -> Vec<PathBuf> {
        let mut results = Vec::new();
        for pkg in &self.packages {
            if let Some(paths) = pkg.index.symbol_index.get(&(module.to_string(), symbol.to_string())) {
                results.extend(paths.iter().cloned());
            }
        }
        results
    }
}

fn source_map_line(line: u32) -> usize {
    line as usize + 1
}

fn lsp_line(line: usize) -> u32 {
    line.saturating_sub(1) as u32
}

fn collapsed_range(position: Position) -> Range {
    Range {
        start: position,
        end: position,
    }
}

fn file_location(path: &Path, range: Range) -> Option<Location> {
    Some(Location {
        uri: Url::from_file_path(path).ok()?,
        range,
    })
}

fn generated_line_for_markdown(span: &SourceSpan, line: usize) -> Option<usize> {
    span.contains_markdown_line(line)
        .then_some(span.generated_start_line + (line - span.markdown_start_line))
}

#[allow(dead_code)]
impl PackageState {
    pub fn contains_generated_path(&self, path: &Path) -> bool {
        self.index.generated_files.contains(path)
    }

    pub fn remap_generated_position(&self, path: &Path, position: Position) -> Option<Location> {
        let line = source_map_line(position.line);
        let span = self.index.source_map.find_generated(path, line)?;
        let markdown_line = span.markdown_line_for_generated(line)?;
        file_location(
            &span.markdown_path,
            collapsed_range(Position {
                line: lsp_line(markdown_line),
                character: position.character,
            }),
        )
    }

    pub fn remap_generated_range(&self, path: &Path, range: Range) -> Option<Location> {
        let start_line = source_map_line(range.start.line);
        let end_line = source_map_line(range.end.line);
        let start_span = self.index.source_map.find_generated(path, start_line)?;
        let end_span = self.index.source_map.find_generated(path, end_line)?;
        if start_span != end_span {
            return None;
        }

        file_location(
            &start_span.markdown_path,
            Range {
                start: Position {
                    line: lsp_line(start_span.markdown_line_for_generated(start_line)?),
                    character: range.start.character,
                },
                end: Position {
                    line: lsp_line(end_span.markdown_line_for_generated(end_line)?),
                    character: range.end.character,
                },
            },
        )
    }

    pub fn remap_generated_location(&self, location: &Location) -> Option<Location> {
        let path = location.uri.to_file_path().ok()?;
        self.remap_generated_range(&path, location.range)
    }

    pub fn resolve_generated_position(&self, path: &Path, position: Position) -> Option<Location> {
        let line = source_map_line(position.line);
        let span = self.index.source_map.find_markdown(path, line)?;
        let generated_line = generated_line_for_markdown(span, line)?;
        file_location(
            &span.generated_path,
            collapsed_range(Position {
                line: lsp_line(generated_line),
                character: position.character,
            }),
        )
    }
}

#[allow(dead_code)]
impl WorkspaceState {
    pub fn remap_generated_position(&self, uri: &Url, position: Position) -> Option<Location> {
        let path = uri.to_file_path().ok()?;
        self.package_for_generated_path(&path)?
            .remap_generated_position(&path, position)
    }

    pub fn remap_generated_range(&self, uri: &Url, range: Range) -> Option<Location> {
        let path = uri.to_file_path().ok()?;
        self.package_for_generated_path(&path)?
            .remap_generated_range(&path, range)
    }

    pub fn remap_generated_location(&self, location: &Location) -> Option<Location> {
        let path = location.uri.to_file_path().ok()?;
        self.package_for_generated_path(&path)?
            .remap_generated_location(location)
    }

    pub fn remap_generated_locations(&self, locations: &[Location]) -> Vec<Option<Location>> {
        locations
            .iter()
            .map(|location| self.remap_generated_location(location))
            .collect()
    }

    pub fn resolve_generated_position(&self, markdown_uri: &Url, position: Position) -> Option<Location> {
        let path = markdown_uri.to_file_path().ok()?;
        self.package_for_path(&path)?
            .resolve_generated_position(&path, position)
    }
}
