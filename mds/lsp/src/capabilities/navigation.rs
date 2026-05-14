use std::path::{Path};
use mds_core::descriptor::{lang_for_markdown_path};
use mds_core::descriptor::{markdown_suffix_for_lang};
use mds_core::markdown::{source_markdown_root};
use tower_lsp::lsp_types::{*};
use crate::convert::{line_at};
use crate::convert::{table_cell_at_position};
use crate::convert::{word_at_position};
use crate::state::{WorkspaceState};
pub fn goto_definition(
    text: &str,
    position: Position,
    path: &Path,
    state: &WorkspaceState,
) -> Option<GotoDefinitionResponse> {
    let line_text = line_at(text, position.line)?;

    if let Some(location) = wiki_link_location_at(line_text, position, state) {
        return Some(GotoDefinitionResponse::Scalar(location));
    }

    if !line_text.trim_start().starts_with('|') {
        return None;
    }

    // Try to resolve the target cell
    let cell = table_cell_at_position(text, position)?;
    if cell.is_empty() {
        return None;
    }

    if let Some(location) = markdown_link_location(path, &cell) {
        return Some(GotoDefinitionResponse::Scalar(location));
    }

    if cell.contains("..") || cell.starts_with('/') || cell.contains('\\') {
        return None;
    }

    let pkg_state = state.package_for_path(path)?;
    let package = &pkg_state.package;
    let markdown_root = source_markdown_root(package);

    // Try as internal target: resolve relative to markdown root
    let lang = lang_for_markdown_path(path)?;
    let ext = markdown_suffix_for_lang(&lang)?;

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
            range: h5_range_for_name(&target_path, &cell).unwrap_or_default(),
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
                    range: h5_range_for_name(p, &cell).unwrap_or_default(),
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

fn wiki_link_location_at(line_text: &str, position: Position, state: &WorkspaceState) -> Option<Location> {
    let col = position.character as usize;
    let upto_cursor = &line_text[..line_text.len().min(col)];
    let start = upto_cursor.rfind("[[")?;
    let end = line_text[start + 2..].find("]]")? + start + 2;
    if col > end + 2 {
        return None;
    }
    let target = line_text[start + 2..end].split('|').next().unwrap_or_default().trim();
    let (module, symbol) = target.split_once('#').unwrap_or((target, ""));
    let paths = if symbol.is_empty() {
        state.find_module_locations(module)
    } else {
        state.find_symbol_locations(module, symbol)
    };
    let path = paths.into_iter().next()?;
    let range = if symbol.is_empty() {
        Range::default()
    } else {
        h5_range_for_name(&path, symbol)
            .or_else(|| heading_range_for_anchor(&path, symbol))
            .unwrap_or_default()
    };
    Some(Location { uri: Url::from_file_path(path).ok()?, range })
}

fn markdown_link_location(source_path: &Path, cell: &str) -> Option<Location> {
    let (label, target) = markdown_link_parts(cell)?;
    let (target_file, anchor) = target.split_once('#').unwrap_or((target, ""));
    if target_file.contains("..") || target_file.starts_with('/') || target_file.contains('\\') {
        return None;
    }
    let target_path = source_path.parent()?.join(target_file);
    if !target_path.exists() {
        return None;
    }
    let range = if anchor.is_empty() {
        h5_range_for_name(&target_path, label).unwrap_or_default()
    } else {
        heading_range_for_anchor(&target_path, anchor).unwrap_or_default()
    };
    Some(Location {
        uri: Url::from_file_path(target_path).ok()?,
        range,
    })
}

fn markdown_link_parts(value: &str) -> Option<(&str, &str)> {
    let value = value.trim();
    if !value.starts_with('[') || !value.ends_with(')') {
        return None;
    }
    let middle = value.find("](")?;
    Some((&value[1..middle], &value[middle + 2..value.len() - 1]))
}

fn h5_range_for_name(path: &Path, name: &str) -> Option<Range> {
    heading_range_for_anchor(path, &slugify_heading(name))
}

fn heading_range_for_anchor(path: &Path, anchor: &str) -> Option<Range> {
    let text = std::fs::read_to_string(path).ok()?;
    for (idx, line) in text.lines().enumerate() {
        let Some(title) = line.trim_start().strip_prefix('#') else {
            continue;
        };
        let title = title.trim_start_matches('#').trim();
        if slugify_heading(title) == anchor.trim_start_matches('#') {
            let char_end = u32::try_from(line.len()).unwrap_or(u32::MAX);
            return Some(Range {
                start: Position {
                    line: idx as u32,
                    character: 0,
                },
                end: Position {
                    line: idx as u32,
                    character: char_end,
                },
            });
        }
    }
    None
}

fn slugify_heading(value: &str) -> String {
    value
        .trim()
        .to_ascii_lowercase()
        .chars()
        .map(|character| if character.is_ascii_alphanumeric() { character } else { '-' })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

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

    // Get this file's module stem
    let doc = pkg_state.index.docs.get(path)?;
    let stem = doc
        .markdown_relative_path
        .to_string_lossy()
        .replace(".md", "");

    // Search all docs for textual references to this stem or the word
    let mut locations = Vec::new();
    let mut seen_files = std::collections::HashSet::new();

    for doc_path in pkg_state.index.docs.keys() {
        if doc_path == path {
            continue;
        }
        let file_text = match std::fs::read_to_string(doc_path) {
            Ok(t) => t,
            Err(_) => continue,
        };
        for (idx, line) in file_text.lines().enumerate() {
            if line.contains(&stem) || line.contains(&word) {
                if !seen_files.insert((doc_path.clone(), idx)) {
                    continue;
                }
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

    if locations.is_empty() {
        None
    } else {
        Some(locations)
    }
}
