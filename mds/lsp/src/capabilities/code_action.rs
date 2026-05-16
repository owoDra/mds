use std::path::Path;

use crate::capabilities::authoring;
use crate::labels::{resolve_label};
use crate::state::WorkspaceState;
use mds_core::descriptor::lang_for_markdown_path;
use mds_core::model::{Config, DocKind};
use tower_lsp::lsp_types::{*};

pub fn provide_code_actions(uri: &Url, text: &str, config: &Config) -> CodeActionResponse {
    provide_code_actions_with_state(uri, text, config, None)
}

pub fn provide_code_actions_with_state(
    uri: &Url,
    text: &str,
    config: &Config,
    workspace_state: Option<&WorkspaceState>,
) -> CodeActionResponse {
    let mut actions = Vec::new();
    let path = uri.to_file_path().ok();
    let doc_kind = authoring::doc_kind_for_path(path.as_deref(), config, workspace_state);

    let sections = authoring::sections_with_labels_for_doc(text, &config.label_overrides, doc_kind);
    let required_sections = required_sections(doc_kind);
    let missing: Vec<&str> = required_sections
        .iter()
        .copied()
        .filter(|section| !sections.contains_key(*section))
        .collect();

    if !missing.is_empty() {
        let line_count = text.lines().count() as u32;

        let mut new_text = String::new();
        if !text.ends_with('\n') {
            new_text.push('\n');
        }
        new_text.push('\n');

        for section in &missing {
            append_section(&mut new_text, section, config, path.as_deref());
        }

        actions.push(insert_action(
            uri,
            line_count,
            format!("Add missing sections: {}", missing.join(", ")),
            new_text,
        ));

        for section in &missing {
            let mut section_text = String::new();
            if !text.ends_with('\n') {
                section_text.push('\n');
            }
            section_text.push('\n');
            append_section(&mut section_text, section, config, path.as_deref());
            actions.push(insert_action(
                uri,
                line_count,
                format!("Add missing ## {section} section"),
                section_text,
            ));
        }
    }

    actions
}

fn required_sections(doc_kind: DocKind) -> &'static [&'static str] {
    match doc_kind {
        DocKind::Source => &["Purpose", "Contract", "Source"],
        DocKind::Test => &["Purpose", "Covers", "Cases", "Test"],
    }
}

fn append_section(buffer: &mut String, section: &str, config: &Config, path: Option<&Path>) {
    let label = resolve_label(&section.to_lowercase(), config);
    buffer.push_str(&format!("## {label}\n\n"));

    match section {
        "Purpose" | "Contract" | "Cases" => {
            buffer.push_str("<!-- TODO: fill in -->\n\n");
        }
        "Covers" => {
            buffer.push_str("<!-- TODO: add covered module wiki links -->\n\n");
        }
        "Source" | "Test" => {
            append_code_block(buffer, path);
        }
        _ => {
            buffer.push_str("<!-- TODO: fill in -->\n\n");
        }
    }
}

fn append_code_block(buffer: &mut String, path: Option<&Path>) {
    if let Some(lang) = path.and_then(lang_for_markdown_path) {
        buffer.push_str(&format!("```{}\n\n```\n\n", lang.key()));
    } else {
        buffer.push_str("```\n\n```\n\n");
    }
}

fn insert_action(uri: &Url, line_count: u32, title: String, new_text: String) -> CodeActionOrCommand {
    let edit = TextEdit {
        range: Range {
            start: Position {
                line: line_count,
                character: 0,
            },
            end: Position {
                line: line_count,
                character: 0,
            },
        },
        new_text,
    };

    let mut changes = std::collections::HashMap::new();
    changes.insert(uri.clone(), vec![edit]);

    CodeActionOrCommand::CodeAction(CodeAction {
        title,
        kind: Some(CodeActionKind::QUICKFIX),
        diagnostics: None,
        edit: Some(WorkspaceEdit {
            changes: Some(changes),
            ..Default::default()
        }),
        ..Default::default()
    })
}
