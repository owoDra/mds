use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::diagnostics::{Diagnostic, RunState};
use crate::fs_utils::{collect_files, is_excluded};
use crate::model::{ImplDoc, Lang, Package};

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

    let code = extract_all_code_blocks(&text);
    if code.trim().is_empty() {
        state.diagnostics.push(Diagnostic::error(
            Some(path.to_path_buf()),
            "implementation md requires at least one code block",
        ));
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
        code,
        normalized_input,
    })
}

/// Extract all code blocks from the entire markdown content.
/// Code blocks are separated by `\n\n` in the output.
pub fn extract_all_code_blocks(text: &str) -> String {
    let mut in_block = false;
    let mut current = String::new();
    let mut blocks = Vec::new();
    for line in text.lines() {
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
            current.push_str(line);
            current.push('\n');
        }
    }
    if blocks.is_empty() {
        String::new()
    } else {
        blocks.join("\n\n") + "\n"
    }
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
