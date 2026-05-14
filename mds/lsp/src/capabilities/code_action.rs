use crate::labels::{resolve_label};
use mds_core::markdown::{sections_with_labels};
use mds_core::model::{Config};
use tower_lsp::lsp_types::{*};
pub fn provide_code_actions(uri: &Url, text: &str, config: &Config) -> CodeActionResponse {
    let mut actions = Vec::new();

    let sections = sections_with_labels(text, &config.label_overrides);
    let required_sections = [
        "Purpose", "Contract", "Exports", "Imports", "Source", "Cases", "Test",
    ];
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
            append_section(&mut new_text, section, config);
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
            append_section(&mut section_text, section, config);
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

fn append_section(buffer: &mut String, section: &str, config: &Config) {
    let label = resolve_label(&section.to_lowercase(), config);
    buffer.push_str(&format!("## {label}\n\n"));

    match section {
        "Purpose" | "Contract" | "Cases" => {
            buffer.push_str("<!-- TODO: fill in -->\n\n");
        }
        "Exports" => {
            let name_label = resolve_label("name", config);
            let visibility_label = resolve_label("visibility", config);
            let summary_label = resolve_label("summary", config);
            buffer.push_str(&format!(
                "| {name_label} | {visibility_label} | {summary_label} |\n\
                 | --- | --- | --- |\n\n\
                 ##### symbolName\n\n<!-- TODO: describe shared symbol -->\n\n"
            ));
        }
        "Imports" => {
            let from_label = resolve_label("from", config);
            let target_label = resolve_label("target", config);
            let symbols_label = resolve_label("symbols", config);
            let via_label = resolve_label("via", config);
            let summary_label = resolve_label("summary", config);
            let reference_label = resolve_label("reference", config);
            buffer.push_str(&format!(
                "| {from_label} | {target_label} | {symbols_label} | {via_label} | {summary_label} | {reference_label} |\n\
                 | --- | --- | --- | --- | --- | --- |\n\n"
            ));
        }
        "Source" | "Test" => {
            buffer.push_str("```\n// TODO: implementation\n```\n\n");
        }
        _ => {
            buffer.push_str("<!-- TODO: fill in -->\n\n");
        }
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
