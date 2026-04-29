use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Component, Path};

use crate::diagnostics::{Diagnostic, RunState};
use crate::fs_utils::{collect_files, is_excluded};
use crate::model::{ImplDoc, Lang, OutputKind, Package, UseExpose, UseFrom, UseRow};
use crate::table::parse_table_with_labels;

pub fn load_implementation_docs(
    package: &Package,
    state: &mut RunState,
) -> Result<Vec<ImplDoc>, String> {
    let markdown_root = package.root.join(&package.config.roots.markdown);
    if !markdown_root.exists() {
        state.diagnostics.push(Diagnostic::error(
            Some(markdown_root),
            "markdown root does not exist",
        ));
        return Ok(Vec::new());
    }

    let mut docs = Vec::new();
    for path in collect_files(&markdown_root, false)? {
        if is_excluded(&package.root, &path, &package.config.excludes) {
            continue;
        }
        let Some(lang) = Lang::from_path(&path) else {
            continue;
        };
        if !package.config.adapters.get(&lang).copied().unwrap_or(true) {
            continue;
        }
        if let Some(doc) = parse_impl_doc(package, lang, &path, state) {
            docs.push(doc);
        }
    }
    docs.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(docs)
}

pub fn parse_impl_doc(
    package: &Package,
    lang: Lang,
    path: &Path,
    state: &mut RunState,
) -> Option<ImplDoc> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) => {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                format!("failed to read implementation md: {error}"),
            ));
            return None;
        }
    };
    validate_markdown_links(path, &text, state);
    for (idx, line) in text.lines().enumerate() {
        if line.starts_with("#####") {
            state.diagnostics.push(
                Diagnostic::error(
                    Some(path.to_path_buf()),
                    "implementation md only allows H3-H4 helper headings",
                )
                .at_line(idx + 1),
            );
        }
    }

    let sections = sections_with_labels(&text, &package.config.label_overrides);
    for required in ["Purpose", "Contract", "Types", "Source", "Cases", "Test"] {
        if !sections.contains_key(required) {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                format!("implementation md requires ## {required}"),
            ));
        }
    }

    let mut uses = HashMap::new();
    let mut code = HashMap::new();
    for kind in [OutputKind::Types, OutputKind::Source, OutputKind::Test] {
        if let Some(section) = sections.get(kind.section()) {
            uses.insert(
                kind,
                parse_uses(section, path, &package.config.label_overrides, state),
            );
            let joined = code_blocks(section, path, state);
            if joined.trim().is_empty() {
                state.diagnostics.push(Diagnostic::error(
                    Some(path.to_path_buf()),
                    format!(
                        "{} section requires at least one code block",
                        kind.section()
                    ),
                ));
            }
            code.insert(kind, joined);
        }
    }

    let package_relative_path = match path.strip_prefix(&package.root) {
        Ok(path) => path.to_path_buf(),
        Err(_) => path.to_path_buf(),
    };
    let markdown_relative_path =
        match path.strip_prefix(package.root.join(&package.config.roots.markdown)) {
            Ok(path) => path.to_path_buf(),
            Err(_) => path.to_path_buf(),
        };
    let normalized_input = normalized_input(path, &text);

    Some(ImplDoc {
        lang,
        path: path.to_path_buf(),
        package_relative_path,
        markdown_relative_path,
        uses,
        code,
        normalized_input,
    })
}

pub fn sections_with_labels(
    text: &str,
    label_overrides: &HashMap<String, String>,
) -> HashMap<String, String> {
    let mut result = HashMap::new();
    let mut current: Option<String> = None;
    let mut body = String::new();
    for line in text.lines() {
        if let Some(title) = line.strip_prefix("## ") {
            let title = canonical_section_title(title.trim(), label_overrides);
            if let Some(name) = current.replace(title) {
                result.insert(name, body.trim_matches('\n').to_string());
                body.clear();
            }
        } else if current.is_some() {
            body.push_str(line);
            body.push('\n');
        }
    }
    if let Some(name) = current {
        result.insert(name, body.trim_matches('\n').to_string());
    }
    result
}

fn canonical_section_title(title: &str, label_overrides: &HashMap<String, String>) -> String {
    for canonical in [
        "Purpose", "Contract", "Types", "Source", "Cases", "Test", "Expose", "Exposes",
    ] {
        if title == canonical {
            return canonical.to_string();
        }
        let key = canonical.to_ascii_lowercase();
        if label_overrides
            .get(&key)
            .is_some_and(|override_label| override_label.trim() == title)
        {
            return canonical.to_string();
        }
    }
    title.to_string()
}

pub fn parse_uses(
    section: &str,
    path: &Path,
    label_overrides: &HashMap<String, String>,
    state: &mut RunState,
) -> Vec<UseRow> {
    let Some(rows) = parse_table_with_labels(
        section,
        &["From", "Target", "Expose", "Summary"],
        path,
        label_overrides,
        state,
    ) else {
        state.diagnostics.push(Diagnostic::error(
            Some(path.to_path_buf()),
            "Uses table requires From, Target, Expose, and Summary columns",
        ));
        return Vec::new();
    };
    let mut seen = HashSet::new();
    let mut uses = Vec::new();
    for row in rows {
        let from_text = row
            .get("from")
            .map(String::as_str)
            .unwrap_or_default()
            .trim();
        let Some(from) = UseFrom::parse(from_text) else {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                format!(
                    "Uses.From must be one of builtin, package, workspace, internal: `{from_text}`"
                ),
            ));
            continue;
        };
        let target = row
            .get("target")
            .map(String::as_str)
            .unwrap_or_default()
            .trim()
            .to_string();
        validate_target(from, &target, path, state);
        let exposes = parse_use_exposes(
            row.get("expose").map(String::as_str).unwrap_or_default(),
            path,
            state,
        );
        let key = (
            from,
            target.clone(),
            exposes
                .iter()
                .map(UseExpose::render_key)
                .collect::<Vec<_>>()
                .join(","),
        );
        if !seen.insert(key) {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                "duplicate Uses row with the same From, Target, and Expose",
            ));
        }
        uses.push(UseRow {
            from,
            target,
            exposes,
        });
    }
    uses
}

pub fn parse_use_exposes(value: &str, path: &Path, state: &mut RunState) -> Vec<UseExpose> {
    let mut exposes = Vec::new();
    let mut has_default = false;
    let mut has_namespace = false;
    for token in value
        .split(',')
        .map(str::trim)
        .filter(|token| !token.is_empty())
    {
        if let Some(local) = token.strip_prefix("default:") {
            let local = local.trim();
            if local.is_empty() {
                state.diagnostics.push(Diagnostic::error(
                    Some(path.to_path_buf()),
                    "Uses.Expose default token requires a local name",
                ));
                continue;
            }
            has_default = true;
            exposes.push(UseExpose::Default {
                local: local.to_string(),
            });
        } else if let Some(local) = token.strip_prefix("* as ") {
            let local = local.trim();
            if local.is_empty() {
                state.diagnostics.push(Diagnostic::error(
                    Some(path.to_path_buf()),
                    "Uses.Expose namespace token requires a local name",
                ));
                continue;
            }
            has_namespace = true;
            exposes.push(UseExpose::Namespace {
                local: local.to_string(),
            });
        } else if let Some((name, alias)) = token.split_once(" as ") {
            let name = name.trim();
            let alias = alias.trim();
            if name.is_empty() || alias.is_empty() {
                state.diagnostics.push(Diagnostic::error(
                    Some(path.to_path_buf()),
                    "Uses.Expose alias token requires both source and local names",
                ));
                continue;
            }
            exposes.push(UseExpose::Named {
                name: name.to_string(),
                alias: Some(alias.to_string()),
            });
        } else {
            exposes.push(UseExpose::Named {
                name: token.to_string(),
                alias: None,
            });
        }
    }
    if has_default && has_namespace {
        state.diagnostics.push(Diagnostic::error(
            Some(path.to_path_buf()),
            "Uses.Expose does not allow default and namespace imports in the same cell",
        ));
    }
    if has_namespace && exposes.len() > 1 {
        state.diagnostics.push(Diagnostic::error(
            Some(path.to_path_buf()),
            "Uses.Expose namespace import must be the only token in the cell",
        ));
    }
    exposes
}

pub fn validate_target(from: UseFrom, target: &str, path: &Path, state: &mut RunState) {
    if target.is_empty() {
        state.diagnostics.push(Diagnostic::error(
            Some(path.to_path_buf()),
            "Uses.Target is required",
        ));
        return;
    }
    if target.contains(".md") || target.contains('\\') {
        state.diagnostics.push(Diagnostic::error(
            Some(path.to_path_buf()),
            "Uses.Target must not contain .md or backslash separators",
        ));
    }
    if from == UseFrom::Internal {
        let invalid = target.starts_with("./")
            || target.starts_with("../")
            || target.starts_with('/')
            || target.ends_with('/')
            || Path::new(target)
                .components()
                .any(|component| matches!(component, Component::ParentDir | Component::RootDir));
        if invalid
            || target
                .rsplit('/')
                .next()
                .is_some_and(|leaf| leaf.contains('.'))
        {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                "internal Uses.Target must be a markdown_root relative module path without ./, ../, extension, trailing slash, or absolute path",
            ));
        }
    }
}

pub fn code_blocks(section: &str, path: &Path, state: &mut RunState) -> String {
    let mut in_block = false;
    let mut current = String::new();
    let mut blocks = Vec::new();
    for (idx, line) in section.lines().enumerate() {
        if line.trim_start().starts_with("```") {
            if in_block {
                blocks.push(current.trim_end_matches(['\r', '\n']).to_string());
                current.clear();
                in_block = false;
            } else {
                in_block = true;
            }
            continue;
        }
        if in_block {
            let trimmed = line.trim_start();
            if trimmed.starts_with("import ")
                || trimmed.starts_with("from ")
                || trimmed.starts_with("use ")
                || trimmed.starts_with("require(")
                || trimmed.starts_with("const ") && trimmed.contains("require(")
            {
                state.diagnostics.push(
                    Diagnostic::error(
                        Some(path.to_path_buf()),
                        "code blocks must not contain import/use/require; use the Uses table",
                    )
                    .at_line(idx + 1),
                );
            }
            current.push_str(line);
            current.push('\n');
        }
    }
    blocks.join("\n\n") + if blocks.is_empty() { "" } else { "\n" }
}

pub fn normalized_input(path: &Path, text: &str) -> String {
    let mut normalized = path.display().to_string();
    normalized.push('\n');
    normalized.push_str(text.replace("\r\n", "\n").trim_end());
    normalized.push('\n');
    normalized
}

pub fn validate_markdown_links(path: &Path, text: &str, state: &mut RunState) {
    for target in standard_markdown_links(text)
        .into_iter()
        .chain(wikilinks(text))
    {
        if !should_validate_local_link(&target) {
            continue;
        }
        let clean_target = target
            .split('#')
            .next()
            .unwrap_or_default()
            .trim()
            .trim_matches('<')
            .trim_matches('>');
        if clean_target.is_empty() {
            continue;
        }
        let target_path = path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join(clean_target);
        if !target_path.exists() {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                format!("Markdown link target does not exist: `{target}`"),
            ));
        }
    }
}

fn standard_markdown_links(text: &str) -> Vec<String> {
    let mut links = Vec::new();
    let bytes = text.as_bytes();
    let mut idx = 0;
    while idx < bytes.len() {
        if bytes[idx] != b'[' {
            idx += 1;
            continue;
        }
        let Some(close_text) = text[idx + 1..].find(']') else {
            break;
        };
        let open_target = idx + close_text + 2;
        if bytes.get(open_target) != Some(&b'(') {
            idx += 1;
            continue;
        }
        let Some(close_target) = text[open_target + 1..].find(')') else {
            break;
        };
        links.push(text[open_target + 1..open_target + 1 + close_target].to_string());
        idx = open_target + close_target + 2;
    }
    links
}

fn wikilinks(text: &str) -> Vec<String> {
    let mut links = Vec::new();
    let mut rest = text;
    while let Some(start) = rest.find("[[") {
        rest = &rest[start + 2..];
        let Some(end) = rest.find("]]") else {
            break;
        };
        let target = rest[..end]
            .split('|')
            .next()
            .unwrap_or_default()
            .trim()
            .to_string();
        links.push(target);
        rest = &rest[end + 2..];
    }
    links
}

fn should_validate_local_link(target: &str) -> bool {
    let target = target.trim();
    if target.is_empty()
        || target.starts_with('#')
        || target.starts_with("http://")
        || target.starts_with("https://")
        || target.starts_with("mailto:")
    {
        return false;
    }
    let clean = target.split('#').next().unwrap_or_default();
    clean.ends_with(".md") || clean.contains('/')
}
