use std::collections::{HashMap};
use std::{fs};
use std::path::{Path};
use std::path::{PathBuf};
use crate::descriptor::{is_root_module_markdown_path};
use crate::diagnostics::{Diagnostic};
use crate::diagnostics::{RunState};
use crate::fs_utils::{collect_files};
use crate::fs_utils::{is_excluded};
use crate::model::{CodeFenceBlock};
use crate::model::{DocKind};
use crate::model::{DocProfile};
use crate::model::{ImplDoc};
use crate::model::{Lang};
use crate::model::{Package};
use crate::descriptor::{lang_for_markdown_path};
pub fn load_implementation_docs(
    package: &Package,
    state: &mut RunState,
) -> Result<Vec<ImplDoc>, String> {
    let markdown_root = source_markdown_root(package);
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
        if is_template_asset_markdown(&path) {
            continue;
        }
        let Some(lang) = lang_for_markdown_path(&path) else {
            continue;
        };
        if !package.config.adapters.get(&lang).copied().unwrap_or(true) {
            continue;
        }
        if let Some(doc) = parse_impl_doc(package, DocKind::Source, lang, &path, state) {
            docs.push(doc);
        }
    }
    docs.extend(load_test_docs(package, &docs, state)?);
    docs.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(docs)
}

fn load_test_docs(
    package: &Package,
    source_docs: &[ImplDoc],
    state: &mut RunState,
) -> Result<Vec<ImplDoc>, String> {
    let markdown_root = test_markdown_root(package);
    if !markdown_root.exists() {
        return Ok(Vec::new());
    }

    let mut docs = Vec::new();
    for path in collect_files(&markdown_root, false)? {
        if is_excluded(&package.root, &path, &package.config.excludes) || !is_test_doc(&path) {
            continue;
        }
        let Some(lang) = resolve_test_doc_lang(package, &path, source_docs, state) else {
            continue;
        };
        if let Some(doc) = parse_impl_doc(package, DocKind::Test, lang, &path, state) {
            docs.push(doc);
        }
    }
    Ok(docs)
}

fn is_template_asset_markdown(path: &Path) -> bool {
    path.extension().is_some_and(|ext| ext == "md")
        && path.components().any(|component| component.as_os_str() == "templates")
}

pub fn parse_impl_doc(
    package: &Package,
    doc_kind: DocKind,
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
    validate_impl_doc_structure(
        path,
        &text,
        &package.config.label_overrides,
        &package.config.check,
        state,
    );
    if package.config.check.markdown_links {
        validate_markdown_links(path, &text, state);
    }
    validate_code_block_boundaries(
        path,
        &lang,
        &text,
        &package.config.label_overrides,
        &package.config.check,
        state,
    );

    let sections = sections_with_labels(&text, &package.config.label_overrides);
    let section_blocks = code_fence_blocks_by_section(&text, &package.config.label_overrides);
    let source_blocks = section_blocks.get("Source").cloned().unwrap_or_default();
    let test_blocks = section_blocks.get("Test").cloned().unwrap_or_default();
    let implementation_code = code_from_section(sections.get("Source"), &source_blocks);
    let source_code = if matches!(doc_kind, DocKind::Source) {
        implementation_code.clone()
    } else {
        String::new()
    };
    let test_code = if matches!(doc_kind, DocKind::Test) {
        let code = if implementation_code.trim().is_empty() {
            code_from_section(sections.get("Test"), &test_blocks)
        } else {
            implementation_code
        };
        code
    } else {
        code_from_section(sections.get("Test"), &test_blocks)
    };
    let covers = covers_from_section(sections.get("Covers"));

    let code = extract_all_code_blocks(&text);
    let profile = doc_profile(doc_kind, path, &sections, &source_code);
    if package.config.check.documented_sections {
        validate_documented_sections(path, profile, &sections, state);
    }
    if package.config.check.documented_exports {
        validate_documented_exports(path, profile, &sections, &text, state);
        validate_documented_imports(path, &sections, state);
    }

    if package.config.check.code_blocks_required && code.trim().is_empty() && requires_code_block(profile) {
        state.diagnostics.push(Diagnostic::error(
            Some(path.to_path_buf()),
            "implementation md requires at least one code block",
        ));
    }

    let package_relative_path = match path.strip_prefix(&package.root) {
        Ok(path) => path.to_path_buf(),
        Err(_) => path.to_path_buf(),
    };
    let markdown_relative_path = match path.strip_prefix(markdown_root_for(package, doc_kind)) {
        Ok(path) => path.to_path_buf(),
        Err(_) => path.to_path_buf(),
    };
    let normalized_input = normalized_input(path, &text);

    Some(ImplDoc {
        doc_kind,
        lang,
        path: path.to_path_buf(),
        package_relative_path,
        markdown_relative_path,
        code,
        source_code,
        test_code,
        source_blocks,
        test_blocks,
        covers,
        normalized_input,
    })
}

fn is_module_root_metadata_doc(path: &Path) -> bool {
    is_root_module_markdown_path(path)
}

fn doc_profile(
    doc_kind: DocKind,
    path: &Path,
    sections: &HashMap<String, String>,
    source_code: &str,
) -> DocProfile {
    if matches!(path.file_name().and_then(|name| name.to_str()), Some("overview.md")) {
        return DocProfile::Overview;
    }
    match doc_kind {
        DocKind::Test => DocProfile::Test,
        DocKind::Source => {
            if is_module_root_metadata_doc(path) {
                DocProfile::Spec
            } else if !source_code.trim().is_empty()
                || sections.get("Source").is_some_and(|section| section.contains("```"))
            {
                DocProfile::Impl
            } else {
                DocProfile::Spec
            }
        }
    }
}

fn requires_code_block(profile: DocProfile) -> bool {
    matches!(profile, DocProfile::Impl | DocProfile::Test)
}

fn validate_documented_sections(
    path: &Path,
    profile: DocProfile,
    sections: &HashMap<String, String>,
    state: &mut RunState,
) {
    let required: &[&str] = match profile {
        DocProfile::Overview => &["Purpose", "Architecture", "Rules"],
        DocProfile::Spec => &["Contract"],
        DocProfile::Impl => &["Contract"],
        DocProfile::Test => &["Covers", "Cases"],
    };
    for section in required {
        if !sections.contains_key(*section) {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                format!("{:?} md requires ## {section}", profile).to_ascii_lowercase(),
            ));
        }
    }
    if profile == DocProfile::Overview {
        for forbidden in ["Imports", "Exports", "Expose", "Exposes"] {
            if sections.contains_key(forbidden) {
                state.diagnostics.push(Diagnostic::error(
                    Some(path.to_path_buf()),
                    format!("overview md must not contain ## {forbidden}"),
                ));
            }
        }
    }
}

fn validate_documented_exports(
    path: &Path,
    profile: DocProfile,
    sections: &HashMap<String, String>,
    text: &str,
    state: &mut RunState,
) {
    if profile == DocProfile::Overview {
        return;
    }
    let Some(section) = sections.get("Exports").or_else(|| sections.get("Expose")).or_else(|| sections.get("Exposes")) else {
        return;
    };
    let Some(rows) = table_rows(section) else {
        return;
    };
    let h5_sections = h5_sections(text);
    for row in rows {
        let Some(name) = row.get("name").map(String::as_str).map(str::trim) else {
            continue;
        };
        if is_blank_cell(name) {
            continue;
        }
        let summary = row.get("summary").map(String::as_str).unwrap_or_default();
        if is_blank_cell(summary) {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                format!("export `{name}` requires a non-empty Summary"),
            ));
        }
        if !matches!(profile, DocProfile::Spec | DocProfile::Impl) || is_module_root_metadata_doc(path) {
            continue;
        }
        let anchor = slugify_heading(name);
        match h5_sections.get(&anchor) {
            Some(section) if section.prose.trim().is_empty() => state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                format!("export `{name}` H5 shared definition requires explanatory prose"),
            )),
            Some(section) if profile == DocProfile::Impl && section.parent_section.as_deref() != Some("Source") => state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                format!("export `{name}` H5 shared definition must be in ## Source before its code block"),
            )),
            Some(section) if profile == DocProfile::Impl && !section.before_code_block => state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                format!("export `{name}` H5 shared definition must be followed by its Source code block"),
            )),
            Some(_) => {}
            None => state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                format!("export `{name}` requires a matching H5 shared definition"),
            )),
        }
    }
}

fn validate_documented_imports(
    path: &Path,
    sections: &HashMap<String, String>,
    state: &mut RunState,
) {
    let Some(section) = sections.get("Imports").or_else(|| sections.get("Uses")) else {
        return;
    };
    let Some(rows) = table_rows(section) else {
        return;
    };
    for row in rows {
        let from = row.get("from").map(String::as_str).unwrap_or_default().trim();
        let target = row.get("target").map(String::as_str).unwrap_or_default().trim();
        let reference = row.get("reference").map(String::as_str).unwrap_or_default();
        if is_blank_cell(from) && is_blank_cell(target) {
            continue;
        }
        let requires_reference = matches!(from, "internal" | "workspace" | "package");
        if requires_reference && is_blank_cell(reference) {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                format!("import `{target}` requires a Markdown Reference link"),
            ));
        }
    }
}

fn table_rows(section: &str) -> Option<Vec<HashMap<String, String>>> {
    let lines = section.lines().collect::<Vec<_>>();
    for index in 0..lines.len().saturating_sub(1) {
        if !lines[index].trim_start().starts_with('|') || !lines[index + 1].contains("---") {
            continue;
        }
        let headers = split_markdown_row(lines[index])
            .into_iter()
            .map(|header| header.trim().to_ascii_lowercase())
            .collect::<Vec<_>>();
        let mut rows = Vec::new();
        for row_line in lines.iter().skip(index + 2) {
            if !row_line.trim_start().starts_with('|') {
                break;
            }
            let cells = split_markdown_row(row_line);
            let mut row = HashMap::new();
            for (cell_index, header) in headers.iter().enumerate() {
                row.insert(
                    header.clone(),
                    cells.get(cell_index).cloned().unwrap_or_default(),
                );
            }
            rows.push(row);
        }
        return Some(rows);
    }
    None
}

struct H5Section {
    prose: String,
    parent_section: Option<String>,
    before_code_block: bool,
}

fn h5_sections(text: &str) -> HashMap<String, H5Section> {
    let mut sections = HashMap::new();
    let mut current: Option<String> = None;
    let mut current_parent: Option<String> = None;
    let mut body = String::new();
    let mut parent_section: Option<String> = None;
    for line in text.lines() {
        if current.is_none() {
            if let Some(title) = line.strip_prefix("## ") {
                parent_section = Some(title.trim().to_string());
            }
        }
        if let Some(title) = line.strip_prefix("##### ") {
            if let Some(name) = current.replace(slugify_heading(title.trim())) {
                sections.insert(name, h5_section(&body, current_parent.take()));
                body.clear();
            }
            current_parent = parent_section.clone();
        } else if line.starts_with("## ") || line.starts_with("### ") || line.starts_with("#### ") {
            if let Some(name) = current.take() {
                sections.insert(name, h5_section(&body, current_parent.take()));
                body.clear();
            }
            if let Some(title) = line.strip_prefix("## ") {
                parent_section = Some(title.trim().to_string());
            }
        } else if current.is_some() {
            body.push_str(line);
            body.push('\n');
        }
    }
    if let Some(name) = current {
        sections.insert(name, h5_section(&body, current_parent));
    }
    sections
}

fn h5_section(body: &str, parent_section: Option<String>) -> H5Section {
    H5Section {
        prose: prose_body(body),
        parent_section,
        before_code_block: body
            .lines()
            .map(str::trim_start)
            .any(|line| line.starts_with("```")),
    }
}

fn prose_body(body: &str) -> String {
    body.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('|') && !line.starts_with("```") && !line.starts_with("````"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn is_blank_cell(value: &str) -> bool {
    let trimmed = value.trim().trim_matches('`');
    trimmed.is_empty() || trimmed == "-"
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

pub fn source_markdown_root(package: &Package) -> PathBuf {
    let fixed = package.root.join(".mds/source");
    if fixed.exists()
        && (has_source_impl_docs(&fixed)
            || !package.root.join(&package.config.roots.markdown).exists())
    {
        fixed
    } else {
        package.root.join(&package.config.roots.markdown)
    }
}

pub fn test_markdown_root(package: &Package) -> PathBuf {
    package.root.join(".mds/test")
}

fn markdown_root_for(package: &Package, doc_kind: DocKind) -> PathBuf {
    match doc_kind {
        DocKind::Source => source_markdown_root(package),
        DocKind::Test => test_markdown_root(package),
    }
}

fn has_source_impl_docs(root: &Path) -> bool {
    let Ok(files) = collect_files(root, false) else {
        return false;
    };
    files.into_iter().any(|path| {
        matches!(path.extension().and_then(|ext| ext.to_str()), Some("md"))
            && !matches!(
                path.file_name().and_then(|name| name.to_str()),
                Some("overview.md")
            )
    })
}

fn is_test_doc(path: &Path) -> bool {
    matches!(path.extension().and_then(|ext| ext.to_str()), Some("md"))
        && !matches!(
            path.file_name().and_then(|name| name.to_str()),
            Some("overview.md")
        )
}

fn resolve_test_doc_lang(
    package: &Package,
    path: &Path,
    source_docs: &[ImplDoc],
    state: &mut RunState,
) -> Option<Lang> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) => {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                format!("failed to read test md: {error}"),
            ));
            return None;
        }
    };
    let sections = sections_with_labels(&text, &package.config.label_overrides);
    let covers = covers_from_section(sections.get("Covers"));
    if covers.is_empty() {
        state.diagnostics.push(Diagnostic::error(
            Some(path.to_path_buf()),
            "test md requires at least one Covers entry",
        ));
        return None;
    }

    let mut lang = None;
    for cover in covers {
        let matches = source_docs
            .iter()
            .filter(|doc| cover_matches(doc, &cover))
            .collect::<Vec<_>>();
        match matches.as_slice() {
            [] => {
                state.diagnostics.push(Diagnostic::error(
                    Some(path.to_path_buf()),
                    format!("test md Covers entry `{cover}` does not resolve to a source md"),
                ));
                return None;
            }
            [doc] => {
                if let Some(current) = &lang {
                    if current != &doc.lang {
                        state.diagnostics.push(Diagnostic::error(
                            Some(path.to_path_buf()),
                            "test md Covers entries must resolve to source docs of the same language",
                        ));
                        return None;
                    }
                } else {
                    lang = Some(doc.lang.clone());
                }
            }
            _ => {
                state.diagnostics.push(Diagnostic::error(
                    Some(path.to_path_buf()),
                    format!("test md Covers entry `{cover}` resolves ambiguously"),
                ));
                return None;
            }
        }
    }

    lang
}

fn cover_matches(doc: &ImplDoc, cover: &str) -> bool {
    let cover = wiki_link_target(cover.trim());
    let cover = cover.split('#').next().unwrap_or(&cover).trim();
    cover == logical_module_id(&doc.markdown_relative_path)
        || cover == legacy_slash_module_id(&doc.markdown_relative_path)
        || cover == doc.markdown_relative_path.to_string_lossy()
}

fn logical_module_id(path: &Path) -> String {
    let value = path.to_string_lossy().replace('\\', "/");
    let value = value.strip_suffix(".md").unwrap_or(&value);
    let without_lang = if let Some(index) = value.rfind('.') {
        &value[..index]
    } else {
        value
    };
    without_lang.replace('/', ".")
}

fn legacy_slash_module_id(path: &Path) -> String {
    logical_module_id(path).replace('.', "/")
}

fn validate_impl_doc_structure(
    path: &Path,
    text: &str,
    label_overrides: &HashMap<String, String>,
    check: &crate::model::CheckConfig,
    state: &mut RunState,
) {
    if check.code_fence_integrity {
        validate_code_fence_integrity(path, text, state);
    }
    if check.duplicate_h2_sections {
        validate_duplicate_h2_sections(path, text, label_overrides, state);
    }
}

fn validate_code_fence_integrity(path: &Path, text: &str, state: &mut RunState) {
    let mut fence_len: Option<usize> = None;
    let mut fence_start_line = 0usize;
    for (line_index, line) in text.lines().enumerate() {
        let Some((marker_len, suffix)) = backtick_fence(line) else {
            continue;
        };
        if let Some(open_len) = fence_len {
            if is_closing_fence(marker_len, suffix, open_len) {
                fence_len = None;
                fence_start_line = 0;
            } else if marker_len >= open_len {
                state.diagnostics.push(Diagnostic::error(
                    Some(path.to_path_buf()),
                    format!(
                        "code fence opened at line {} is not closed before a new fence opener at line {}",
                        fence_start_line,
                        line_index + 1
                    ),
                ));
                fence_len = Some(marker_len);
                fence_start_line = line_index + 1;
            }
        } else {
            fence_len = Some(marker_len);
            fence_start_line = line_index + 1;
        }
    }
    if fence_len.is_some() {
        state.diagnostics.push(Diagnostic::error(
            Some(path.to_path_buf()),
            format!("unterminated code fence opened at line {fence_start_line}"),
        ));
    }
}

fn validate_duplicate_h2_sections(
    path: &Path,
    text: &str,
    label_overrides: &HashMap<String, String>,
    state: &mut RunState,
) {
    let mut first_seen = HashMap::new();
    let mut fence_len: Option<usize> = None;
    for (line_index, line) in text.lines().enumerate() {
        if let Some((marker_len, suffix)) = backtick_fence(line) {
            if let Some(open_len) = fence_len {
                if is_closing_fence(marker_len, suffix, open_len) {
                    fence_len = None;
                }
            } else {
                fence_len = Some(marker_len);
            }
            continue;
        }
        if fence_len.is_some() {
            continue;
        }
        let Some(title) = line.strip_prefix("## ") else {
            continue;
        };
        let canonical = canonical_section_title(title.trim(), label_overrides);
        if let Some(first_line) = first_seen.insert(canonical.clone(), line_index + 1) {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                format!(
                    "duplicate H2 section `{canonical}`: first defined at line {first_line}, repeated at line {}",
                    line_index + 1
                ),
            ));
        }
    }
}

pub fn extract_all_code_blocks(text: &str) -> String {
    let mut fence_len: Option<usize> = None;
    let mut current = String::new();
    let mut blocks = Vec::new();
    for line in text.lines() {
        if let Some((marker_len, suffix)) = backtick_fence(line) {
            if let Some(open_len) = fence_len {
                if !is_closing_fence(marker_len, suffix, open_len) {
                    current.push_str(line);
                    current.push('\n');
                    continue;
                }
                blocks.push(current.trim_end_matches(['\r', '\n']).to_string());
                current.clear();
                fence_len = None;
            } else {
                fence_len = Some(marker_len);
            }
            continue;
        }
        if fence_len.is_some() {
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

fn code_fence_blocks_by_section(
    text: &str,
    label_overrides: &HashMap<String, String>,
) -> HashMap<String, Vec<CodeFenceBlock>> {
    let mut result = HashMap::new();
    let mut current_section: Option<String> = None;
    let mut fence_len: Option<usize> = None;
    let mut next_fence_index = 0usize;
    let mut current_fence_index = 0usize;
    let mut current_content = String::new();
    let mut current_content_start_line = 1usize;
    let mut current_content_end_line = 1usize;
    let mut line_number = 1usize;

    for line in text.lines() {
        if let Some((marker_len, suffix)) = backtick_fence(line) {
            if let Some(open_len) = fence_len {
                if is_closing_fence(marker_len, suffix, open_len) {
                    if let Some(section) = current_section.as_ref() {
                        result
                            .entry(section.clone())
                            .or_insert_with(Vec::new)
                            .push(CodeFenceBlock {
                                fence_index: current_fence_index,
                                content_start_line: current_content_start_line,
                                content_end_line: current_content_end_line,
                                content: current_content.trim_end_matches(['\r', '\n']).to_string(),
                            });
                    }
                    current_content.clear();
                    fence_len = None;
                } else {
                    current_content.push_str(line);
                    current_content.push('\n');
                    current_content_end_line = line_number;
                }
            } else {
                fence_len = Some(marker_len);
                current_fence_index = next_fence_index;
                next_fence_index += 1;
                current_content.clear();
                current_content_start_line = line_number + 1;
                current_content_end_line = current_content_start_line;
            }
            line_number += 1;
            continue;
        }

        if fence_len.is_none() && line.starts_with("## ") {
            let title = line.strip_prefix("## ").unwrap();
            current_section = Some(canonical_section_title(title.trim(), label_overrides));
        } else if fence_len.is_some() {
            current_content.push_str(line);
            current_content.push('\n');
            current_content_end_line = line_number;
        }
        line_number += 1;
    }

    result
}

fn code_from_fence_blocks(blocks: &[CodeFenceBlock]) -> String {
    if blocks.is_empty() {
        String::new()
    } else {
        blocks
            .iter()
            .map(|block| block.content.as_str())
            .collect::<Vec<_>>()
            .join("\n\n")
            + "\n"
    }
}

fn code_from_section(section: Option<&String>, blocks: &[CodeFenceBlock]) -> String {
    if !blocks.is_empty() {
        code_from_fence_blocks(blocks)
    } else {
        section.map(|section| code_from_table(section)).unwrap_or_default()
    }
}

fn code_from_table(section: &str) -> String {
    let lines: Vec<&str> = section.lines().collect();
    for index in 0..lines.len().saturating_sub(1) {
        if !lines[index].trim_start().starts_with('|') || !lines[index + 1].contains("---") {
            continue;
        }
        let headers = split_markdown_row(lines[index]);
        let Some(code_index) = headers
            .iter()
            .position(|header| header.trim().eq_ignore_ascii_case("statement"))
        else {
            continue;
        };
        let mut output = String::new();
        for row_line in lines.iter().skip(index + 2) {
            if !row_line.trim_start().starts_with('|') {
                break;
            }
            let cells = split_markdown_row(row_line);
            let value = cells
                .get(code_index)
                .map(String::as_str)
                .unwrap_or_default()
                .trim()
                .trim_matches('`');
            if value.is_empty() {
                continue;
            }
            output.push_str(value);
            output.push('\n');
        }
        return output;
    }
    String::new()
}

fn split_markdown_row(line: &str) -> Vec<String> {
    line.trim()
        .trim_matches('|')
        .split('|')
        .map(|cell| cell.trim().to_string())
        .collect()
}

fn covers_from_section(section: Option<&String>) -> Vec<String> {
    section
        .map(|section| {
            section
                .lines()
                .map(|line| line.trim().trim_start_matches(['-', '*']).trim())
                .filter(|line| !line.is_empty())
                .map(wiki_link_target)
                .collect()
        })
        .unwrap_or_default()
}

fn wiki_link_target(value: &str) -> String {
    let value = value.trim();
    if let Some(inner) = value.strip_prefix("[[").and_then(|value| value.strip_suffix("]]")) {
        inner.split('|').next().unwrap_or_default().trim().to_string()
    } else {
        value.to_string()
    }
}

fn validate_code_block_boundaries(
    path: &Path,
    lang: &Lang,
    text: &str,
    label_overrides: &HashMap<String, String>,
    _check: &crate::model::CheckConfig,
    state: &mut RunState,
) {
    let mut previous_block: Option<CodeBlock<'_>> = None;
    for block in code_blocks(text, label_overrides) {
        if let Some(previous) = previous_block {
            if is_unnecessary_code_block_split(lang, previous.content, block.content) {
                state.diagnostics.push(Diagnostic::error(
                    Some(path.to_path_buf()),
                    format!(
                        "code block starting at line {} appears to continue the previous code block; merge the blocks or split at a top-level logical boundary",
                        block.start_line
                    ),
                ));
            }
        }
        previous_block = Some(block);
    }
}

fn is_unnecessary_code_block_split(lang: &Lang, previous: &str, current: &str) -> bool {
    let previous_last = previous.lines().rev().find(|line| !line.trim().is_empty());
    let current_first = current.lines().find(|line| !line.trim().is_empty());
    let Some(previous_last) = previous_last.map(str::trim_end) else {
        return false;
    };
    let Some(current_first_raw) = current_first else {
        return false;
    };
    let current_first = current_first_raw.trim_start();
    let descriptor = crate::descriptor::builtin_descriptor(lang);
    previous_last.ends_with(['{', '(', '[', ',', '\\'])
        || current_first.starts_with(['}', ')', ']', ',', '.'])
        || (previous_last.ends_with(';') && descriptor.matches_code_block_merge_start(current_first))
        || current_first_raw.starts_with(' ')
        || current_first_raw.starts_with('\t')
}

#[derive(Debug)]
struct CodeBlock<'a> {
    content: &'a str,
    start_line: usize,
}

fn code_blocks<'a>(
    text: &'a str,
    _label_overrides: &HashMap<String, String>,
) -> Vec<CodeBlock<'a>> {
    let mut blocks = Vec::new();
    let mut fence_len: Option<usize> = None;
    let mut content_start = 0;
    let mut content_start_line = 1;
    let mut cursor = 0;
    let mut line_number = 1;
    for line in text.split_inclusive('\n') {
        let line_start = cursor;
        cursor += line.len();
        if let Some((marker_len, suffix)) = backtick_fence(line) {
            if let Some(open_len) = fence_len {
                if is_closing_fence(marker_len, suffix, open_len) {
                    blocks.push(CodeBlock {
                        content: &text[content_start..line_start],
                        start_line: content_start_line,
                    });
                    fence_len = None;
                }
            } else {
                fence_len = Some(marker_len);
                content_start = cursor;
                content_start_line = line_number + 1;
            }
        }
        line_number += 1;
    }
    blocks
}

fn backtick_fence(line: &str) -> Option<(usize, &str)> {
    let trimmed = line.trim_start();
    let count = trimmed.chars().take_while(|char| *char == '`').count();
    (count >= 3).then_some((count, &trimmed[count..]))
}

fn is_closing_fence(marker_len: usize, suffix: &str, open_len: usize) -> bool {
    marker_len >= open_len && suffix.trim().is_empty()
}

pub fn sections_with_labels(
    text: &str,
    label_overrides: &HashMap<String, String>,
) -> HashMap<String, String> {
    let mut result = HashMap::new();
    let mut current: Option<String> = None;
    let mut body = String::new();
    let mut fence_len: Option<usize> = None;
    for line in text.lines() {
        if let Some((marker_len, suffix)) = backtick_fence(line) {
            if let Some(open_len) = fence_len {
                if is_closing_fence(marker_len, suffix, open_len) {
                    fence_len = None;
                }
            } else {
                fence_len = Some(marker_len);
            }
        }
        if fence_len.is_none() && line.starts_with("## ") {
            let title = line.strip_prefix("## ").unwrap();
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
    for (canonical, aliases) in [
        ("Purpose", &["Purpose", "Overview", "概要", "目的"] as &[_]),
        ("Contract", &["Contract", "仕様", "契約"]),
        ("Exports", &["Exports", "API", "公開API", "Interface", "Expose", "Exposes"]),
        ("Imports", &["Imports", "Uses"]),
        ("Source", &["Source", "Implementation", "実装"]),
        ("Cases", &["Cases", "ケース"]),
        ("Test", &["Test", "Verification", "検証", "テスト"]),
        ("Covers", &["Covers", "対象"]),
        ("Architecture", &["Architecture"]),
        ("Rules", &["Rules"]),
    ] {
        if aliases.iter().any(|alias| title == *alias) {
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
    let link_text = text_without_code_blocks(text);
    for target in standard_markdown_links(&link_text)
        .into_iter()
        .chain(wikilinks(&link_text))
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

fn text_without_code_blocks(text: &str) -> String {
    let mut output = String::new();
    let mut fence_len: Option<usize> = None;
    for line in text.lines() {
        if let Some((marker_len, suffix)) = backtick_fence(line) {
            if let Some(open_len) = fence_len {
                if is_closing_fence(marker_len, suffix, open_len) {
                    fence_len = None;
                }
            } else {
                fence_len = Some(marker_len);
            }
            continue;
        }
        if fence_len.is_none() {
            output.push_str(line);
            output.push('\n');
        }
    }
    output
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
