# tests/capabilities.rs

## Purpose

Migrated implementation source for `tests/capabilities.rs`.

## Contract

- Preserve the behavior of the pre-migration Rust source.
- This file is synchronized into `.build/rust/mds/lsp/tests/capabilities.rs`.

## Covers

- capabilities/code_action
- capabilities/completion
- capabilities/symbols
- convert

## Imports

| From | Target | Symbols | Via | Summary | Reference |
| --- | --- | --- | --- | --- | --- |
| external | tower_lsp::lsp_types | * | - | - | - |
| external | mds_lsp::capabilities::code_action | provide_code_actions | - | - | [../source/capabilities/code_action.rs.md#source](../source/capabilities/code_action.rs.md#source) |
| external | mds_lsp::capabilities::completion | provide_completions | - | - | [../source/capabilities/completion.rs.md#source](../source/capabilities/completion.rs.md#source) |
| external | mds_lsp::capabilities::symbols | document_symbols | - | - | [../source/capabilities/symbols.rs.md#source](../source/capabilities/symbols.rs.md#source) |
| external | mds_lsp::convert | line_at | - | - | [../source/convert.rs.md#source](../source/convert.rs.md#source) |
| external | mds_lsp::convert | table_cell_at_position | - | - | [../source/convert.rs.md#source](../source/convert.rs.md#source) |
| external | mds_lsp::convert | word_at_position | - | - | [../source/convert.rs.md#source](../source/convert.rs.md#source) |


## Test


````rs
#[test]
fn test_document_symbols_extracts_headings() {
    let text = r#"## Purpose

A module.

## Contract

Contract.

## Types

Types content.

## Source

Source content.

### Uses

Uses content.
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
    assert!(names.contains(&"Types"), "should contain Types: {names:?}");
    assert!(
        names.contains(&"Source"),
        "should contain Source: {names:?}"
    );
    assert!(names.contains(&"Uses"), "should contain Uses: {names:?}");
}
````

````rs
#[test]
fn test_section_completion_on_heading_prefix() {
    let text = "## ";
    let position = Position {
        line: 0,
        character: 3,
    };
    let config = mds_core::Config::default();
    let items = provide_completions(text, position, None, &config);
    let labels: Vec<&str> = items.iter().map(|i| i.label.as_str()).collect();
    assert!(
        labels.contains(&"Purpose"),
        "should offer Purpose: {labels:?}"
    );
    assert!(
        labels.contains(&"Contract"),
        "should offer Contract: {labels:?}"
    );
    assert!(labels.contains(&"Types"), "should offer Types: {labels:?}");
    assert!(
        labels.contains(&"Source"),
        "should offer Source: {labels:?}"
    );
}
````

````rs
#[test]
fn test_code_block_language_completion() {
    let text = "```";
    let position = Position {
        line: 0,
        character: 3,
    };
    let config = mds_core::Config::default();
    let path = std::path::Path::new("/test/example.ts.md");
    let items = provide_completions(text, position, Some(path), &config);
    let labels: Vec<&str> = items.iter().map(|i| i.label.as_str()).collect();
    assert!(
        labels.contains(&"typescript"),
        "should offer typescript: {labels:?}"
    );
}
````

````rs
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
````

````rs
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
````

````rs
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
````

````rs
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
````

````rs
#[test]
fn test_line_at() {
    let text = "line 0\nline 1\nline 2";
    assert_eq!(line_at(text, 0), Some("line 0"));
    assert_eq!(line_at(text, 1), Some("line 1"));
    assert_eq!(line_at(text, 2), Some("line 2"));
    assert_eq!(line_at(text, 3), None);
}
````

````rs
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
````

````rs
#[test]
fn test_document_symbols_empty() {
    let symbols = document_symbols("");
    assert!(symbols.is_empty(), "empty document should have no symbols");
}
````

````rs
#[test]
fn test_document_symbols_with_h4() {
    let text = "## Purpose\n\n#### Detail\n\nContent.";
    let symbols = document_symbols(text);
    let names: Vec<&str> = symbols.iter().map(|s| s.name.as_str()).collect();
    assert!(names.contains(&"Purpose"), "should have Purpose");
    // H4 headings should NOT appear in symbols (only ## and ###)
    assert!(!names.contains(&"Detail"), "should not have H4 heading");
}
````

````rs
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
````

````rs
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
````



````rs
#[test]
fn test_completion_snippet_provided() {
    let text = "## ";
    let position = Position {
        line: 0,
        character: 3,
    };
    let config = mds_core::Config::default();
    let items = provide_completions(text, position, None, &config);
    let labels: Vec<&str> = items.iter().map(|i| i.label.as_str()).collect();
    assert!(
        labels.contains(&"Purpose"),
        "should provide section completions: {labels:?}"
    );
}
````
