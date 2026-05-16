use tower_lsp::lsp_types::{*};
use mds_lsp::capabilities::code_action::{provide_code_actions};
use mds_lsp::capabilities::completion::{provide_completions};
use mds_lsp::capabilities::diagnostics::{validate_impl_md_text};
use mds_lsp::capabilities::hover::{provide_hover};
use mds_lsp::capabilities::navigation::{goto_definition};
use mds_lsp::capabilities::symbols::{document_symbols};
use mds_lsp::convert::{line_at};
use mds_lsp::convert::{table_cell_at_position};
use mds_lsp::convert::{word_at_position};
use mds_lsp::state::{PackageState};
use mds_lsp::state::{WorkspaceIndex};
use mds_lsp::state::{WorkspaceState};
use mds_core::{Config};
use mds_core::{Package};
use std::collections::{HashMap};
use std::{fs};

fn action_texts(actions: &CodeActionResponse) -> Vec<String> {
    actions
        .iter()
        .filter_map(|action| match action {
            CodeActionOrCommand::CodeAction(action) => action
                .edit
                .as_ref()
                .and_then(|edit| edit.changes.as_ref())
                .and_then(|changes| changes.values().next())
                .map(|edits| edits.iter().map(|edit| edit.new_text.clone()).collect::<String>()),
            CodeActionOrCommand::Command(_) => None,
        })
        .collect()
}
#[test]
fn test_document_symbols_extracts_headings() {
    let text = r#"## Purpose

A module.

## Contract

Contract.

## Source

Source content.

### Uses

Uses content.

##### SharedName

Shared definition.
"#;

    let symbols = document_symbols(text);
    let names: Vec<&str> = symbols.iter().map(|s| s.name.as_str()).collect();
    assert!(
        names.contains(&"Purpose"),
        "should contain Purpose: {names:?}"
    );
    assert!(
        names.contains(&"Contract"),
        "should contain Contract: {names:?}"
    );
    assert!(
        names.contains(&"Source"),
        "should contain Source: {names:?}"
    );
    assert!(names.contains(&"Uses"), "should contain Uses: {names:?}");
    assert!(
        names.contains(&"SharedName"),
        "should contain H5 shared definition: {names:?}"
    );
}

#[test]
fn test_section_completion_on_heading_prefix() {
    let text = "## ";
    let position = Position {
        line: 0,
        character: 3,
    };
    let config = mds_core::Config::default();
    let items = provide_completions(text, position, None, &config, None);
    let labels: Vec<&str> = items.iter().map(|i| i.label.as_str()).collect();
    assert!(
        labels.contains(&"仕様"),
        "should offer 仕様: {labels:?}"
    );
    assert!(
        labels.contains(&"API"),
        "should offer API: {labels:?}"
    );
    assert!(
        labels.contains(&"実装"),
        "should offer 実装: {labels:?}"
    );
}

#[test]
fn test_h5_completion_on_heading_prefix() {
    let text = "##### ";
    let position = Position {
        line: 0,
        character: 6,
    };
    let config = mds_core::Config::default();
    let items = provide_completions(text, position, None, &config, None);
    let labels: Vec<&str> = items.iter().map(|i| i.label.as_str()).collect();
    assert!(
        labels.contains(&"mds: Shared Definition Section"),
        "should offer H5 shared definition snippet: {labels:?}"
    );
}

#[test]
fn test_types_heading_has_no_special_hover_affordance() {
    let hover = provide_hover(
        "## Types",
        Position {
            line: 0,
            character: 3,
        },
        std::path::Path::new("/test/example.ts.md"),
        &WorkspaceState::default(),
    );
    assert!(hover.is_none());
}

#[test]
fn test_impl_diagnostics_do_not_emit_legacy_types_message() {
    let text = r#"## Purpose

A module.

## Contract

Stable contract.

## Types

```ts
export type Greeting = string;
```

## Source

```ts
export const greet = (): Greeting => 'hi';
```
"#;

    let diagnostics = validate_impl_md_text(
        std::path::Path::new("/test/example.ts.md"),
        text,
        &Config::default(),
    );
    assert!(
        diagnostics
            .iter()
            .all(|diagnostic| !diagnostic.message.contains("legacy section")),
        "unexpected diagnostics: {diagnostics:?}"
    );
}

#[test]
fn test_code_block_language_completion() {
    let text = "```";
    let position = Position {
        line: 0,
        character: 3,
    };
    let config = mds_core::Config::default();
    let path = std::path::Path::new("/test/example.ts.md");
    let items = provide_completions(text, position, Some(path), &config, None);
    let labels: Vec<&str> = items.iter().map(|i| i.label.as_str()).collect();
    assert!(
        labels.contains(&"typescript"),
        "should offer typescript: {labels:?}"
    );
}

#[test]
fn test_code_block_language_completion_with_long_fence() {
    let text = "````";
    let position = Position {
        line: 0,
        character: 4,
    };
    let config = mds_core::Config::default();
    let path = std::path::Path::new("/test/example.rs.md");
    let items = provide_completions(text, position, Some(path), &config, None);
    let labels: Vec<&str> = items.iter().map(|i| i.label.as_str()).collect();
    assert!(labels.contains(&"rust"), "should offer rust: {labels:?}");
}

#[test]
fn test_wiki_link_definition_resolves_module_symbol() {
    let root = std::env::temp_dir().join(format!("mds-lsp-wiki-{}", std::process::id()));
    let source = root.join(".mds/source/app/greet.ts.md");
    fs::create_dir_all(source.parent().unwrap()).unwrap();
    fs::write(&source, "# app.greet\n\n## 実装\n\n### greet\n").unwrap();

    let mut module_index = HashMap::new();
    module_index.insert("app.greet".to_string(), vec![source.clone()]);
    let mut symbol_index = HashMap::new();
    symbol_index.insert(("app.greet".to_string(), "greet".to_string()), vec![source.clone()]);
    let state = WorkspaceState {
        packages: vec![PackageState {
            package: Package {
                root: root.clone(),
                config: Config::default(),
                package_manager_id: "npm".to_string(),
            },
            index: WorkspaceIndex {
                module_index,
                symbol_index,
                ..WorkspaceIndex::default()
            },
        }],
        ..WorkspaceState::default()
    };

    let text = "See [[app.greet#greet]]";
    let response = goto_definition(
        text,
        Position { line: 0, character: 12 },
        &root.join(".mds/source/app/caller.ts.md"),
        &state,
    );
    assert!(matches!(response, Some(GotoDefinitionResponse::Scalar(_))));
}

#[test]
fn test_wiki_link_completion_offers_modules_and_symbols() {
    let mut module_index = HashMap::new();
    module_index.insert("app.greet".to_string(), Vec::new());
    let mut symbol_index = HashMap::new();
    symbol_index.insert(("app.greet".to_string(), "greet".to_string()), Vec::new());
    let state = WorkspaceState {
        packages: vec![PackageState {
            package: Package {
                root: std::env::temp_dir(),
                config: Config::default(),
                package_manager_id: "npm".to_string(),
            },
            index: WorkspaceIndex {
                module_index,
                symbol_index,
                ..WorkspaceIndex::default()
            },
        }],
        ..WorkspaceState::default()
    };

    let config = Config::default();
    let modules = provide_completions("[[app", Position { line: 0, character: 5 }, None, &config, Some(&state));
    assert!(modules.iter().any(|item| item.label == "app.greet"));
    let symbols = provide_completions("[[app.greet#g", Position { line: 0, character: 13 }, None, &config, Some(&state));
    assert!(symbols.iter().any(|item| item.label == "greet"));
}

#[test]
fn test_code_action_missing_sections() {
    let text = r#"## Purpose

A module.
"#;
    let uri = Url::parse("file:///test/example.ts.md").unwrap();
    let config = mds_core::Config::default();
    let actions = provide_code_actions(&uri, text, &config);
    assert!(
        !actions.is_empty(),
        "should provide code actions for missing sections"
    );
}

#[test]
fn test_code_action_legacy_sections_keep_authoring_actions_without_migration_fix() {
    let text = r#"## Uses

| From | Target | Expose | Summary |
| --- | --- | --- | --- |
"#;
    let uri = Url::parse("file:///test/example.ts.md").unwrap();
    let config = mds_core::Config::default();
    let actions = provide_code_actions(&uri, text, &config);
    let titles: Vec<String> = actions
        .iter()
        .filter_map(|action| match action {
            CodeActionOrCommand::CodeAction(action) => Some(action.title.clone()),
            CodeActionOrCommand::Command(command) => Some(command.title.clone()),
        })
        .collect();
    assert!(
        titles.iter().any(|title| title.contains("Add missing")),
        "should still provide useful authoring actions: {titles:?}"
    );
    assert!(
        !titles.iter().any(|title| title.contains("Rename ##")),
        "should not provide legacy migration quick fixes: {titles:?}"
    );
}

#[test]
fn test_word_at_position() {
    let text = "hello world foo-bar";
    let word = word_at_position(
        text,
        Position {
            line: 0,
            character: 7,
        },
    );
    assert_eq!(word, Some("world".to_string()));
}

#[test]
fn test_word_at_position_with_path() {
    let text = "| internal | utils/helper | Name | desc |";
    let word = word_at_position(
        text,
        Position {
            line: 0,
            character: 14,
        },
    );
    assert_eq!(word, Some("utils/helper".to_string()));
}

#[test]
fn test_table_cell_at_position() {
    let text = "| internal | utils/helper | Name | desc |";
    let cell = table_cell_at_position(
        text,
        Position {
            line: 0,
            character: 14,
        },
    );
    assert_eq!(cell, Some("utils/helper".to_string()));
}

#[test]
fn test_line_at() {
    let text = "line 0\nline 1\nline 2";
    assert_eq!(line_at(text, 0), Some("line 0"));
    assert_eq!(line_at(text, 1), Some("line 1"));
    assert_eq!(line_at(text, 2), Some("line 2"));
    assert_eq!(line_at(text, 3), None);
}

#[test]
fn test_code_action_empty_document() {
    let text = "";
    let uri = Url::parse("file:///test/empty.ts.md").unwrap();
    let config = mds_core::Config::default();
    let actions = provide_code_actions(&uri, text, &config);
    assert!(
        !actions.is_empty(),
        "should offer to add all missing sections"
    );
}

#[test]
fn test_code_action_source_doc_prefers_tableless_authoring_sections() {
    let text = r#"## Purpose

Source module.
"#;
    let uri = Url::parse("file:///workspace/pkg/.mds/source/example.ts.md").unwrap();
    let config = mds_core::Config::default();
    let actions = provide_code_actions(&uri, text, &config);
    let inserted = action_texts(&actions).join("\n");

    assert!(inserted.contains("## Contract"), "source quick fix should add Contract: {inserted:?}");
    assert!(inserted.contains("## Source"), "source quick fix should add Source: {inserted:?}");
    assert!(!inserted.contains("## Imports"), "source quick fix should not add Imports: {inserted:?}");
    assert!(!inserted.contains("## Exports"), "source quick fix should not add Exports: {inserted:?}");
    assert!(!inserted.contains("| From |"), "source quick fix should stay tableless: {inserted:?}");
}

#[test]
fn test_code_action_test_doc_prefers_tableless_test_sections() {
    let text = r#"## Purpose

Verification module.
"#;
    let uri = Url::parse("file:///workspace/pkg/.mds/test/example.md").unwrap();
    let config = mds_core::Config::default();
    let actions = provide_code_actions(&uri, text, &config);
    let inserted = action_texts(&actions).join("\n");

    assert!(inserted.contains("## Covers"), "test quick fix should add Covers: {inserted:?}");
    assert!(inserted.contains("## Cases"), "test quick fix should add Cases: {inserted:?}");
    assert!(inserted.contains("## Test"), "test quick fix should add Test: {inserted:?}");
    assert!(!inserted.contains("## Contract"), "test quick fix should not add Contract: {inserted:?}");
    assert!(!inserted.contains("## Source"), "test quick fix should not add Source: {inserted:?}");
    assert!(!inserted.contains("## Imports"), "test quick fix should not add Imports: {inserted:?}");
}

#[test]
fn test_document_symbols_empty() {
    let symbols = document_symbols("");
    assert!(symbols.is_empty(), "empty document should have no symbols");
}

#[test]
fn test_document_symbols_with_h4() {
    let text = "## Purpose\n\n#### Detail\n\nContent.";
    let symbols = document_symbols(text);
    let names: Vec<&str> = symbols.iter().map(|s| s.name.as_str()).collect();
    assert!(names.contains(&"Purpose"), "should have Purpose");
    // H4 headings should NOT appear in symbols (only ## and ###)
    assert!(!names.contains(&"Detail"), "should not have H4 heading");
}

#[test]
fn test_document_symbols_with_h5_shared_definition() {
    let text = "## Exports\n\n##### greet\n\nShared entrypoint.";
    let symbols = document_symbols(text);
    let names: Vec<&str> = symbols.iter().map(|s| s.name.as_str()).collect();
    assert!(names.contains(&"Exports"), "should have Exports");
    assert!(names.contains(&"greet"), "should have H5 shared definition");
}

#[test]
fn test_word_at_position_boundary() {
    let text = "hello";
    // At beginning
    assert_eq!(
        word_at_position(
            text,
            Position {
                line: 0,
                character: 0
            }
        ),
        Some("hello".to_string())
    );
    // Beyond end
    assert_eq!(
        word_at_position(
            text,
            Position {
                line: 0,
                character: 100
            }
        ),
        None
    );
}

#[test]
fn test_table_cell_not_a_table() {
    let text = "This is not a table line";
    let cell = table_cell_at_position(
        text,
        Position {
            line: 0,
            character: 5,
        },
    );
    assert_eq!(cell, None);
}

#[test]
fn test_completion_snippet_provided() {
    let text = "## ";
    let position = Position {
        line: 0,
        character: 3,
    };
    let config = mds_core::Config::default();
    let items = provide_completions(text, position, None, &config, None);
    let labels: Vec<&str> = items.iter().map(|i| i.label.as_str()).collect();
    assert!(
        labels.contains(&"仕様"),
        "should provide section completions: {labels:?}"
    );
}
