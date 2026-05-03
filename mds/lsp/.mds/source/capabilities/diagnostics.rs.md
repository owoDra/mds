# src/capabilities/diagnostics.rs

## Purpose

Migrated implementation source for `src/capabilities/diagnostics.rs`.

## Contract

- Preserve the behavior of the pre-migration Rust source.
- This file is synchronized into `.build/rust/mds/lsp/src/capabilities/diagnostics.rs`.

## Imports

| Kind | From | Target | Symbols | Via | Summary | Code |
| --- | --- | --- | --- | --- | --- | --- |
| rust-use | builtin | std::path | Path | std |  | `use std::path::Path;` |
| rust-use | external | mds_core::config | merge_config_file | mds_core |  | `use mds_core::config::merge_config_file;` |
| rust-use | external | mds_core::descriptor | set_workspace_descriptor_root | mds_core |  | `use mds_core::descriptor::set_workspace_descriptor_root;` |
| rust-use | external | mds_core::diagnostics | RunState | mds_core |  | `use mds_core::diagnostics::RunState;` |
| rust-use | external | mds_core::markdown | extract_all_code_blocks, sections_with_labels, validate_markdown_links | mds_core |  | `use mds_core::markdown::{extract_all_code_blocks, sections_with_labels, validate_markdown_links};` |
| rust-use | external | mds_core::model | Config, Lang | mds_core |  | `use mds_core::model::{Config, Lang};` |
| rust-use | external | mds_core::table | parse_table_with_labels | mds_core |  | `use mds_core::table::parse_table_with_labels;` |
| rust-use | external | tower_lsp | lsp_types | tower_lsp |  | `use tower_lsp::lsp_types;` |
| rust-use | internal | crate::convert | to_lsp_diagnostic | crate |  | `use crate::convert::to_lsp_diagnostic;` |


## Source


Validate an implementation Markdown file and return LSP diagnostics.

````rs
pub fn validate_impl_md_text(
    path: &Path,
    text: &str,
    _config: &Config,
) -> Vec<lsp_types::Diagnostic> {
    let mut state = RunState::default();
    set_workspace_descriptor_root(path.parent());

    // Validate markdown links
    validate_markdown_links(path, text, &mut state);

    // Validate that there is at least one code block
    let code = extract_all_code_blocks(text);
    if code.trim().is_empty() {
        state.diagnostics.push(mds_core::Diagnostic::error(
            Some(path.to_path_buf()),
            "implementation md requires at least one code block",
        ));
    }

    // Check language matching with file extension
    if let Some(ref lang) = Lang::from_path(path) {
        validate_code_block_languages(text, lang, path, &mut state);
    }

    state.diagnostics.iter().map(to_lsp_diagnostic).collect()
}

````

Validate `mds.config.toml` and return LSP diagnostics.

````rs
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

````

Validate legacy package index text and return LSP diagnostics.

````rs
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
                format!("index.md requires ## {required}"),
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
                "index.md Package section requires Name and Version table columns",
            ));
        }
    }

    state.diagnostics.iter().map(to_lsp_diagnostic).collect()
}

````

Validate that code block language labels match the file's language.

````rs
fn validate_code_block_languages(
    text: &str,
    expected_lang: &Lang,
    path: &Path,
    state: &mut RunState,
) {
    let expected_labels: Vec<String> = match expected_lang {
        Lang::TypeScript => vec![
            "typescript".to_string(),
            "ts".to_string(),
            "tsx".to_string(),
        ],
        Lang::Python => vec!["python".to_string(), "py".to_string()],
        Lang::Rust => vec!["rust".to_string(), "rs".to_string()],
        Lang::Other(ext) => vec![ext.clone()],
    };

    for (idx, line) in text.lines().enumerate() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") && trimmed.len() > 3 {
            let label = trimmed[3..].trim().to_lowercase();
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
````


