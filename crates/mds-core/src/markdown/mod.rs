use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Component, Path};

use crate::diagnostics::{Diagnostic, RunState};
use crate::fs_utils::collect_files;
use crate::model::{ImplDoc, Lang, OutputKind, Package, UseExpose, UseFrom, UseRow};
use crate::table::parse_table;

pub(crate) fn load_implementation_docs(
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

pub(crate) fn parse_impl_doc(
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

    let sections = sections(&text);
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
            uses.insert(kind, parse_uses(section, path, state));
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

pub(crate) fn sections(text: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();
    let mut current: Option<String> = None;
    let mut body = String::new();
    for line in text.lines() {
        if let Some(title) = line.strip_prefix("## ") {
            if let Some(name) = current.replace(title.trim().to_string()) {
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

pub(crate) fn parse_uses(section: &str, path: &Path, state: &mut RunState) -> Vec<UseRow> {
    let Some(rows) = parse_table(
        section,
        &["From", "Target", "Expose", "Summary"],
        path,
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

pub(crate) fn parse_use_exposes(value: &str, path: &Path, state: &mut RunState) -> Vec<UseExpose> {
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

pub(crate) fn validate_target(from: UseFrom, target: &str, path: &Path, state: &mut RunState) {
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

pub(crate) fn code_blocks(section: &str, path: &Path, state: &mut RunState) -> String {
    let mut in_block = false;
    let mut current = String::new();
    let mut blocks = Vec::new();
    for (idx, line) in section.lines().enumerate() {
        if line.trim_start().starts_with("```") {
            if in_block {
                blocks.push(
                    current
                        .trim_end_matches(|ch| ch == '\r' || ch == '\n')
                        .to_string(),
                );
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

pub(crate) fn normalized_input(path: &Path, text: &str) -> String {
    let mut normalized = path.display().to_string();
    normalized.push('\n');
    normalized.push_str(text.replace("\r\n", "\n").trim_end());
    normalized.push('\n');
    normalized
}
