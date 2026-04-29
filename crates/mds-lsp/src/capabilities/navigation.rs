use std::path::Path;

use mds_core::model::Lang;
use tower_lsp::lsp_types::*;

use crate::convert::{line_at, table_cell_at_position, word_at_position};
use crate::state::WorkspaceState;

/// Go to Definition: resolve Uses table targets to their source markdown files.
pub fn goto_definition(
    text: &str,
    position: Position,
    path: &Path,
    state: &WorkspaceState,
) -> Option<GotoDefinitionResponse> {
    let line_text = line_at(text, position.line)?;

    // Check if cursor is in a Uses table row
    if !line_text.trim_start().starts_with('|') {
        return None;
    }

    // Try to resolve the target cell
    let cell = table_cell_at_position(text, position)?;
    if cell.is_empty() {
        return None;
    }

    // Reject path traversal attempts
    if cell.contains("..") || cell.starts_with('/') || cell.contains('\\') {
        return None;
    }

    let pkg_state = state.package_for_path(path)?;
    let package = &pkg_state.package;
    let markdown_root = package.root.join(&package.config.roots.markdown);

    // Try as internal target: resolve relative to markdown root
    let lang = Lang::from_path(path)?;
    let ext = match lang {
        Lang::TypeScript => ".ts.md",
        Lang::Python => ".py.md",
        Lang::Rust => ".rs.md",
    };

    let target_path = markdown_root.join(format!("{cell}{ext}"));
    // Verify resolved path is within markdown root (path traversal prevention)
    if let (Ok(canonical_root), Ok(canonical_target)) =
        (markdown_root.canonicalize(), target_path.canonicalize())
    {
        if !canonical_target.starts_with(&canonical_root) {
            return None;
        }
    }

    if target_path.exists() {
        let uri = Url::from_file_path(&target_path).ok()?;
        return Some(GotoDefinitionResponse::Scalar(Location {
            uri,
            range: Range::default(),
        }));
    }

    // Try looking up in expose index
    let locations = state.find_expose_locations(&cell);
    if !locations.is_empty() {
        let locs: Vec<Location> = locations
            .iter()
            .filter_map(|p| {
                Url::from_file_path(p).ok().map(|uri| Location {
                    uri,
                    range: Range::default(),
                })
            })
            .collect();
        if locs.len() == 1 {
            return Some(GotoDefinitionResponse::Scalar(locs.into_iter().next()?));
        }
        return Some(GotoDefinitionResponse::Array(locs));
    }

    None
}

/// Find References: find all Uses table entries that reference the expose at cursor.
pub fn find_references(
    text: &str,
    position: Position,
    path: &Path,
    state: &WorkspaceState,
) -> Option<Vec<Location>> {
    let word = word_at_position(text, position)?;
    if word.is_empty() {
        return None;
    }

    let pkg_state = state.package_for_path(path)?;

    // Get this file's module name (its expose stem)
    let doc = pkg_state.index.docs.get(path)?;
    let stem = doc
        .markdown_relative_path
        .to_string_lossy()
        .replace(".ts.md", "")
        .replace(".py.md", "")
        .replace(".rs.md", "");

    // Search all docs in this package for Uses rows targeting this stem
    let mut locations = Vec::new();
    let mut seen_files = std::collections::HashSet::new();

    for (doc_path, doc) in &pkg_state.index.docs {
        for uses in doc.uses.values() {
            for use_row in uses {
                if use_row.target == stem || use_row.target == word {
                    // Only read each file once
                    if !seen_files.insert(doc_path.clone()) {
                        continue;
                    }
                    let file_text = match std::fs::read_to_string(doc_path) {
                        Ok(t) => t,
                        Err(_) => continue,
                    };
                    for (idx, line) in file_text.lines().enumerate() {
                        if line.trim_start().starts_with('|') && line.contains(&use_row.target) {
                            if let Ok(uri) = Url::from_file_path(doc_path) {
                                let char_end = u32::try_from(line.len()).unwrap_or(u32::MAX);
                                locations.push(Location {
                                    uri,
                                    range: Range {
                                        start: Position {
                                            line: idx as u32,
                                            character: 0,
                                        },
                                        end: Position {
                                            line: idx as u32,
                                            character: char_end,
                                        },
                                    },
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    if locations.is_empty() {
        None
    } else {
        Some(locations)
    }
}
