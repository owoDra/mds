use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use mds_core::config::{merge_config_file};
use mds_core::descriptor::{set_workspace_descriptor_root};
use mds_core::descriptor::{fence_labels_for_lang};
use mds_core::descriptor::{is_root_module_markdown_path};
use mds_core::descriptor::{lang_for_markdown_path};
use mds_core::diagnostics::{RunState};
use mds_core::markdown::{extract_all_code_blocks};
use mds_core::markdown::{sections_with_labels};
use mds_core::markdown::{validate_markdown_links};
use mds_core::model::{CheckDiagnosticPolicy, Config, DocKind, Lang};
use mds_core::table::{parse_table_with_labels};
use tower_lsp::{lsp_types};

use crate::capabilities::authoring;
use crate::convert::{to_lsp_diagnostic};

use crate::state::WorkspaceState;

pub fn validate_impl_md_text(
    path: &Path,
    text: &str,
    config: &Config,
) -> Vec<lsp_types::Diagnostic> {
    validate_impl_md_text_with_state(path, text, config, None)
}

pub fn validate_impl_md_text_with_state(
    path: &Path,
    text: &str,
    config: &Config,
    workspace_state: Option<&WorkspaceState>,
) -> Vec<lsp_types::Diagnostic> {
    let mut state = RunState::default();
    let doc_kind = authoring::doc_kind_for_path(Some(path), config, workspace_state);

    set_workspace_descriptor_root(path.parent());

    if config.check.markdown_links {
        validate_markdown_links(path, text, &mut state);
    }

    let sections = authoring::sections_with_labels_for_doc(text, &config.label_overrides, doc_kind);

    if config.check.documented_sections {
        validate_documented_sections(path, text, doc_kind, &sections, &mut state);
    }
    if config.check.documented_exports {
        validate_documented_exports(path, text, &sections, &mut state);
        validate_import_documentation(path, &sections, &mut state);
    }

    validate_legacy_table_sections(path, &sections, &config.check, &mut state);
    validate_split_source_and_test(
        path,
        doc_kind,
        sections.get("Source"),
        sections.get("Test"),
        &config.check,
        &mut state,
    );
    validate_wiki_link_targets(path, text, config, workspace_state, &mut state);

    if let Some(ref lang) = lang_for_markdown_path(path) {
        validate_code_block_languages(text, lang, path, &mut state);
    }

    state.diagnostics.iter().map(to_lsp_diagnostic).collect()
}

fn validate_documented_sections(
    path: &Path,
    text: &str,
    doc_kind: DocKind,
    sections: &HashMap<String, String>,
    state: &mut RunState,
) {
    if !sections.contains_key("Purpose") {
        state.diagnostics.push(mds_core::Diagnostic::error(
            Some(path.to_path_buf()),
            format!("{} md requires ## Purpose", doc_kind.key()),
        ));
    }

    match doc_kind {
        DocKind::Source if has_generated_code(sections.get("Source")) && !sections.contains_key("Contract") => {
            state.diagnostics.push(mds_core::Diagnostic::error(
                Some(path.to_path_buf()),
                "source md requires ## Contract",
            ));
        }
        DocKind::Test if has_generated_code(sections.get("Test")) => {
            for section in ["Covers", "Cases"] {
                if !sections.contains_key(section) {
                    state.diagnostics.push(mds_core::Diagnostic::error(
                        Some(path.to_path_buf()),
                        format!("test md requires ## {section}"),
                    ));
                }
            }
        }
        _ => {}
    }
}

fn has_generated_code(section: Option<&String>) -> bool {
    section.is_some_and(|section| !extract_all_code_blocks(section).trim().is_empty())
}

fn has_generated_source(sections: &HashMap<String, String>) -> bool {
    has_generated_code(sections.get("Source"))
}

fn validate_documented_exports(
    path: &Path,
    text: &str,
    sections: &HashMap<String, String>,
    state: &mut RunState,
) {
    let Some(section) = sections.get("Exports").or_else(|| sections.get("Expose")).or_else(|| sections.get("Exposes")) else {
        return;
    };
    let Some(rows) = table_rows(section) else {
        return;
    };
    let h5 = h5_sections(text);
    for row in rows {
        let name = row.get("name").map(String::as_str).unwrap_or_default().trim();
        if is_blank_cell(name) {
            continue;
        }
        let summary = row.get("summary").map(String::as_str).unwrap_or_default();
        if is_blank_cell(summary) {
            state.diagnostics.push(mds_core::Diagnostic::error(
                Some(path.to_path_buf()),
                format!("export `{name}` requires a non-empty Summary"),
            ));
        }
        if !is_root_module_markdown_path(path) {
            match h5.get(&slugify_heading(name)) {
                Some(section) if is_blank_cell(&section.prose) => state.diagnostics.push(mds_core::Diagnostic::error(
                    Some(path.to_path_buf()),
                    format!("export `{name}` H5 shared definition requires explanatory prose"),
                )),
                Some(section) if has_generated_source(sections) && section.parent_section.as_deref() != Some("Source") => state.diagnostics.push(mds_core::Diagnostic::error(
                    Some(path.to_path_buf()),
                    format!("export `{name}` H5 shared definition must be in ## Source before its code block"),
                )),
                Some(section) if has_generated_source(sections) && !section.before_code_block => state.diagnostics.push(mds_core::Diagnostic::error(
                    Some(path.to_path_buf()),
                    format!("export `{name}` H5 shared definition must be followed by its Source code block"),
                )),
                Some(_) => {}
                None => state.diagnostics.push(mds_core::Diagnostic::error(
                    Some(path.to_path_buf()),
                    format!("export `{name}` requires a matching H5 shared definition"),
                )),
            }
        }
    }
}

fn validate_import_documentation(
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
            state.diagnostics.push(mds_core::Diagnostic::error(
                Some(path.to_path_buf()),
                format!("import `{target}` requires a Markdown Reference link"),
            ));
        }
    }
}

fn validate_legacy_table_sections(
    path: &Path,
    sections: &HashMap<String, String>,
    check: &mds_core::model::CheckConfig,
    state: &mut RunState,
) {
    for section_name in ["Imports", "Exports"] {
        let Some(section) = sections.get(section_name) else {
            continue;
        };
        if !authoring::contains_markdown_table(section) {
            continue;
        }
        push_policy_diagnostic(
            check.legacy_tables,
            path,
            format!("legacy table metadata in ## {section_name} is deprecated"),
            state,
        );
    }
}

fn validate_split_source_and_test(
    path: &Path,
    doc_kind: DocKind,
    source_section: Option<&String>,
    test_section: Option<&String>,
    check: &mds_core::model::CheckConfig,
    state: &mut RunState,
) {
    if !check.split_source_and_test {
        return;
    }

    match doc_kind {
        DocKind::Source if has_generated_code(test_section) => state.diagnostics.push(
            mds_core::Diagnostic::error(
                Some(path.to_path_buf()),
                "source md must not contain generated test code in ## Test",
            ),
        ),
        DocKind::Test if has_generated_code(source_section) => state.diagnostics.push(
            mds_core::Diagnostic::error(
                Some(path.to_path_buf()),
                "test md must not contain generated source code in ## Source",
            ),
        ),
        _ => {}
    }
}

fn validate_wiki_link_targets(
    path: &Path,
    text: &str,
    config: &Config,
    workspace_state: Option<&WorkspaceState>,
    state: &mut RunState,
) {
    let Some(workspace_state) = workspace_state else {
        return;
    };
    let Some(package_state) = workspace_state.package_for_path(path) else {
        return;
    };

    let link_text = authoring::text_without_code_blocks(text);
    for target in authoring::wikilinks(&link_text) {
        if !should_validate_module_wikilink(&target) {
            continue;
        }
        validate_wiki_link_target(
            path,
            &target,
            &package_state.index.module_index,
            &package_state.index.symbol_index,
            config.check.unresolved_module_symbols,
            state,
        );
    }
}

fn validate_wiki_link_target(
    path: &Path,
    target: &str,
    module_index: &HashMap<String, Vec<PathBuf>>,
    symbol_index: &HashMap<(String, String), Vec<PathBuf>>,
    symbol_policy: CheckDiagnosticPolicy,
    state: &mut RunState,
) {
    let Some((module_id, symbol)) = target.split_once('#') else {
        match unique_path_count(module_index.get(target)) {
            1 => {}
            0 => state.diagnostics.push(mds_core::Diagnostic::error(
                Some(path.to_path_buf()),
                format!("wiki link target `[[{target}]]` does not resolve to a module"),
            )),
            _ => state.diagnostics.push(mds_core::Diagnostic::error(
                Some(path.to_path_buf()),
                format!("wiki link target `[[{target}]]` resolves ambiguously"),
            )),
        }
        return;
    };

    let module_id = module_id.trim();
    let symbol = symbol.trim();
    if symbol_resolves(symbol_index, module_id, symbol) {
        return;
    }

    push_policy_diagnostic(
        symbol_policy,
        path,
        format!(
            "wiki link target `[[{module_id}#{symbol}]]` does not resolve to a documented symbol"
        ),
        state,
    );
}

fn unique_path_count(paths: Option<&Vec<PathBuf>>) -> usize {
    paths.map_or(0, |paths| paths.iter().collect::<HashSet<_>>().len())
}

fn symbol_resolves(
    symbol_index: &HashMap<(String, String), Vec<PathBuf>>,
    module_id: &str,
    symbol: &str,
) -> bool {
    let symbol_slug = slugify_heading(symbol);
    symbol_index.iter().any(|((candidate_module, candidate_symbol), paths)| {
        candidate_module == module_id
            && !paths.is_empty()
            && (candidate_symbol == symbol || slugify_heading(candidate_symbol) == symbol_slug)
    })
}

fn should_validate_module_wikilink(target: &str) -> bool {
    let target = target.trim();
    !target.is_empty() && !target.starts_with('#') && !should_validate_local_link(target)
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

fn table_rows(section: &str) -> Option<Vec<HashMap<String, String>>> {
    let lines = section.lines().collect::<Vec<_>>();
    for index in 0..lines.len().saturating_sub(1) {
        if !lines[index].trim_start().starts_with('|') || !lines[index + 1].contains("---") {
            continue;
        }
        let headers = split_row(lines[index])
            .into_iter()
            .map(|header| header.trim().to_ascii_lowercase())
            .collect::<Vec<_>>();
        let mut rows = Vec::new();
        for row_line in lines.iter().skip(index + 2) {
            if !row_line.trim_start().starts_with('|') {
                break;
            }
            let cells = split_row(row_line);
            let mut row = HashMap::new();
            for (cell_index, header) in headers.iter().enumerate() {
                row.insert(header.clone(), cells.get(cell_index).cloned().unwrap_or_default());
            }
            rows.push(row);
        }
        return Some(rows);
    }
    None
}

fn split_row(line: &str) -> Vec<String> {
    line.trim()
        .trim_matches('|')
        .split('|')
        .map(|cell| cell.trim().to_string())
        .collect()
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
            if let Some(anchor) = current.replace(slugify_heading(title.trim())) {
                sections.insert(anchor, h5_section(&body, current_parent.take()));
                body.clear();
            }
            current_parent = parent_section.clone();
        } else if line.starts_with("## ") || line.starts_with("### ") || line.starts_with("#### ") {
            if let Some(anchor) = current.take() {
                sections.insert(anchor, h5_section(&body, current_parent.take()));
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
    if let Some(anchor) = current {
        sections.insert(anchor, h5_section(&body, current_parent));
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
        .filter(|line| !line.is_empty() && !line.starts_with('|') && !line.starts_with("```"))
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

fn push_policy_diagnostic(
    policy: CheckDiagnosticPolicy,
    path: &Path,
    message: impl Into<String>,
    state: &mut RunState,
) {
    let message = message.into();
    match policy {
        CheckDiagnosticPolicy::Warn => state
            .diagnostics
            .push(mds_core::Diagnostic::warning(Some(path.to_path_buf()), message)),
        CheckDiagnosticPolicy::Error => state
            .diagnostics
            .push(mds_core::Diagnostic::error(Some(path.to_path_buf()), message)),
        CheckDiagnosticPolicy::Allow => {}
    }
}

pub fn validate_config_text(path: &Path, text: &str) -> Vec<lsp_types::Diagnostic> {
    let mut state = RunState::default();
    let mut config = Config::default();
    set_workspace_descriptor_root(path.parent());

    // Try to parse and merge the config
    match text.parse::<toml::Value>() {
        Ok(_value) => {
            // Use the existing config merge logic by writing to temp and reading
            // For now, validate through the existing merge path
            merge_config_file(&mut config, path, &mut state);
        }
        Err(error) => {
            // Extract line information from TOML parse error
            let msg = format!("TOML parse error: {error}");
            state
                .diagnostics
                .push(mds_core::Diagnostic::error(Some(path.to_path_buf()), msg));
        }
    }

    state.diagnostics.iter().map(to_lsp_diagnostic).collect()
}

#[allow(dead_code)]
pub fn validate_package_md_text(
    path: &Path,
    text: &str,
    config: &Config,
) -> Vec<lsp_types::Diagnostic> {
    let mut state = RunState::default();
    set_workspace_descriptor_root(path.parent());

    validate_markdown_links(path, text, &mut state);

    let sections = sections_with_labels(text, &config.label_overrides);
    for required in ["Package", "Dependencies", "Dev Dependencies", "Rules"] {
        if !sections.contains_key(required) {
            state.diagnostics.push(mds_core::Diagnostic::error(
                Some(path.to_path_buf()),
                format!("package overview requires ## {required}"),
            ));
        }
    }

    if let Some(package_section) = sections.get("Package") {
        if parse_table_with_labels(
            package_section,
            &["Name", "Version"],
            path,
            &config.label_overrides,
            &mut state,
        )
        .is_none()
        {
            state.diagnostics.push(mds_core::Diagnostic::error(
                Some(path.to_path_buf()),
                "package overview Package section requires Name and Version table columns",
            ));
        }
    }

    state.diagnostics.iter().map(to_lsp_diagnostic).collect()
}

fn validate_code_block_languages(
    text: &str,
    expected_lang: &Lang,
    path: &Path,
    state: &mut RunState,
) {
    let expected_labels = fence_labels_for_lang(expected_lang);

    for (idx, line) in text.lines().enumerate() {
        let trimmed = line.trim_start();
        if let Some(label) = opening_fence_label(trimmed) {
            if !label.is_empty()
                && !expected_labels.contains(&label)
                && label != "text"
                && label != "txt"
                && label != "markdown"
                && label != "md"
                && label != "toml"
                && label != "json"
                && label != "yaml"
                && label != "yml"
                && label != "sh"
                && label != "bash"
                && label != "shell"
            {
                state.diagnostics.push(
                    mds_core::Diagnostic::warning(
                        Some(path.to_path_buf()),
                        format!(
                            "code block language `{label}` may not match file language `{}`",
                            expected_lang.key()
                        ),
                    )
                    .at_line(idx + 1),
                );
            }
        }
    }
}

fn opening_fence_label(trimmed: &str) -> Option<String> {
    let marker_len = trimmed.chars().take_while(|character| *character == '`').count();
    if marker_len < 3 {
        return None;
    }
    let label = trimmed[marker_len..]
        .split_whitespace()
        .next()
        .unwrap_or_default()
        .to_ascii_lowercase();
    (!label.is_empty()).then_some(label)
}
