use std::path::Path;

use mds_core::config::merge_config_file;
use mds_core::diagnostics::RunState;
use mds_core::markdown::{code_blocks, parse_uses, sections_with_labels, validate_markdown_links};
use mds_core::model::{Config, Lang, OutputKind};
use mds_core::table::parse_table_with_labels;
use tower_lsp::lsp_types;

use crate::convert::to_lsp_diagnostic;

/// Validate an implementation Markdown file and return LSP diagnostics.
pub fn validate_impl_md_text(
    path: &Path,
    text: &str,
    config: &Config,
) -> Vec<lsp_types::Diagnostic> {
    let mut state = RunState::default();

    // Validate markdown links
    validate_markdown_links(path, text, &mut state);

    // Check heading depth
    for (idx, line) in text.lines().enumerate() {
        if line.starts_with("#####") {
            state.diagnostics.push(
                mds_core::Diagnostic::error(
                    Some(path.to_path_buf()),
                    "implementation md only allows H3-H4 helper headings",
                )
                .at_line(idx + 1),
            );
        }
    }

    // Section structure validation
    let sections = sections_with_labels(text, &config.label_overrides);
    for required in ["Purpose", "Contract", "Types", "Source", "Cases", "Test"] {
        if !sections.contains_key(required) {
            state.diagnostics.push(mds_core::Diagnostic::error(
                Some(path.to_path_buf()),
                format!("implementation md requires ## {required}"),
            ));
        }
    }

    // Validate code sections
    for kind in [OutputKind::Types, OutputKind::Source, OutputKind::Test] {
        if let Some(section) = sections.get(kind.section()) {
            parse_uses(section, path, &config.label_overrides, &mut state);
            let joined = code_blocks(section, path, &mut state);
            if joined.trim().is_empty() {
                state.diagnostics.push(mds_core::Diagnostic::error(
                    Some(path.to_path_buf()),
                    format!(
                        "{} section requires at least one code block",
                        kind.section()
                    ),
                ));
            }
        }
    }

    // Check language matching with file extension
    if let Some(lang) = Lang::from_path(path) {
        validate_code_block_languages(text, lang, path, &mut state);
    }

    state.diagnostics.iter().map(to_lsp_diagnostic).collect()
}

/// Validate mds.config.toml and return LSP diagnostics.
pub fn validate_config_text(path: &Path, text: &str) -> Vec<lsp_types::Diagnostic> {
    let mut state = RunState::default();
    let mut config = Config::default();

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

/// Validate package.md and return LSP diagnostics.
pub fn validate_package_md_text(
    path: &Path,
    text: &str,
    config: &Config,
) -> Vec<lsp_types::Diagnostic> {
    let mut state = RunState::default();

    validate_markdown_links(path, text, &mut state);

    let sections = sections_with_labels(text, &config.label_overrides);
    for required in ["Package", "Dependencies", "Dev Dependencies", "Rules"] {
        if !sections.contains_key(required) {
            state.diagnostics.push(mds_core::Diagnostic::error(
                Some(path.to_path_buf()),
                format!("package.md requires ## {required}"),
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
                "package.md Package section requires Name and Version table columns",
            ));
        }
    }

    state.diagnostics.iter().map(to_lsp_diagnostic).collect()
}

/// Validate that code block language labels match the file's language.
fn validate_code_block_languages(
    text: &str,
    expected_lang: Lang,
    path: &Path,
    state: &mut RunState,
) {
    let expected_labels: &[&str] = match expected_lang {
        Lang::TypeScript => &["typescript", "ts", "tsx"],
        Lang::Python => &["python", "py"],
        Lang::Rust => &["rust", "rs"],
    };

    for (idx, line) in text.lines().enumerate() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") && trimmed.len() > 3 {
            let label = trimmed[3..].trim().to_lowercase();
            if !label.is_empty()
                && !expected_labels.contains(&label.as_str())
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
