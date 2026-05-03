# tests/diagnostics.rs

## Purpose

Migrated implementation source for `tests/diagnostics.rs`.

## Contract

- Preserve the behavior of the pre-migration Rust source.
- This file is synchronized into `.build/rust/mds/lsp/tests/diagnostics.rs`.

## Covers

- capabilities/diagnostics
- convert

## Test

````rs
use std::path::PathBuf;

use mds_core::Config;
use mds_lsp::capabilities::diagnostics;
````

````rs
fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(format!("/tmp/mds-test/{name}"))
}
````

````rs
#[test]
fn test_valid_impl_md_no_diagnostics_for_minimal() {
    let text = r#"## Purpose

A minimal test module.

## Contract

Public contract.

## Types

### Uses

| From | Target | Expose | Summary |
| --- | --- | --- | --- |
| builtin | node:path | join | Path join utility |

```typescript
export type MyType = string;
```

## Source

### Uses

| From | Target | Expose | Summary |
| --- | --- | --- | --- |
| internal | utils/helper | helper | Helper function |

```typescript
export function main(): void {}
```

## Cases

Basic use case.

## Test

### Uses

| From | Target | Expose | Summary |
| --- | --- | --- | --- |
| internal | utils/helper | helper | Helper function |

```typescript
test("it works", () => {});
```
"#;

    let path = fixture_path("valid.ts.md");
    let config = Config::default();
    let diags = diagnostics::validate_impl_md_text(&path, text, &config);
    assert!(diags.is_empty(), "expected no diagnostics, got: {diags:?}");
}
````

````rs
#[test]
fn test_missing_sections() {
    // With the new format, the only requirement is at least one code block
    let text = r#"## Purpose

A module with no code blocks.
"#;

    let path = fixture_path("missing.ts.md");
    let config = Config::default();
    let diags = diagnostics::validate_impl_md_text(&path, text, &config);

    let messages: Vec<&str> = diags.iter().map(|d| d.message.as_str()).collect();
    assert!(
        messages.iter().any(|m| m.contains("code block")),
        "should report missing code block: {messages:?}"
    );
}
````

````rs
#[test]
fn test_heading_depth_violation() {
    // H5+ is no longer an error in the new format
    let text = r#"## Purpose

A module.

##### Deep heading is allowed now

## Source

```typescript
function main() {}
```
"#;

    let path = fixture_path("deep-heading.ts.md");
    let config = Config::default();
    let diags = diagnostics::validate_impl_md_text(&path, text, &config);

    // No heading-depth errors anymore
    let messages: Vec<&str> = diags.iter().map(|d| d.message.as_str()).collect();
    assert!(
        !messages.iter().any(|m| m.contains("H3-H4")),
        "should not report deep heading: {messages:?}"
    );
}
````

````rs
#[test]
fn test_config_validation_invalid_toml() {
    let text = "this is not valid toml [[[";
    let path = fixture_path("mds.config.toml");
    let diags = diagnostics::validate_config_text(&path, text);
    assert!(!diags.is_empty(), "should report TOML parse error");
}
````

````rs
#[test]
fn test_package_index_missing_sections() {
    let text = r#"## Package

| Name | Version |
| --- | --- |
| test-pkg | 1.0.0 |
"#;

    let path = fixture_path("index.md");
    let config = Config::default();
    let diags = diagnostics::validate_package_md_text(&path, text, &config);

    let messages: Vec<&str> = diags.iter().map(|d| d.message.as_str()).collect();
    assert!(
        messages.iter().any(|m| m.contains("Dependencies")),
        "should report missing Dependencies: {messages:?}"
    );
    assert!(
        messages.iter().any(|m| m.contains("Rules")),
        "should report missing Rules: {messages:?}"
    );
}
````

````rs
#[test]
fn test_code_block_language_mismatch_warning() {
    let text = r#"## Purpose

A module.

## Contract

Contract.

## Types

```python
x = 1
```

## Source

```python
def main(): pass
```

## Cases

Cases.

## Test

```python
def test_it(): assert True
```
"#;

    // File is .ts.md but code blocks use python
    let path = fixture_path("mismatch.ts.md");
    let config = Config::default();
    let diags = diagnostics::validate_impl_md_text(&path, text, &config);

    let warnings: Vec<&str> = diags
        .iter()
        .filter(|d| d.severity == Some(tower_lsp::lsp_types::DiagnosticSeverity::WARNING))
        .map(|d| d.message.as_str())
        .collect();
    assert!(
        warnings
            .iter()
            .any(|m| m.contains("python") && m.contains("ts")),
        "should warn about language mismatch: {warnings:?}"
    );
}
````

````rs
#[test]
fn test_empty_document_diagnostics() {
    let text = "";
    let path = fixture_path("empty.ts.md");
    let config = Config::default();
    let diags = diagnostics::validate_impl_md_text(&path, text, &config);
    // Should report missing code block
    assert!(
        !diags.is_empty(),
        "empty doc should report missing code block: {diags:?}"
    );
    assert!(
        diags.iter().any(|d| d.message.contains("code block")),
        "should mention code block requirement: {diags:?}"
    );
}
````

````rs
#[test]
fn test_valid_config_no_diagnostics() {
    // Write a valid config to a temp file
    let dir = tempfile::tempdir().unwrap();
    let config_path = dir.path().join("mds.config.toml");
    std::fs::write(
        &config_path,
        "[package]\nenabled = true\n\n[roots]\nmarkdown = \"src-md\"\n",
    )
    .unwrap();

    let diags = diagnostics::validate_config_text(
        &config_path,
        "[package]\nenabled = true\n\n[roots]\nmarkdown = \"src-md\"\n",
    );
    // May have diagnostics about missing index.md etc, but no TOML errors
    let toml_errors: Vec<&str> = diags
        .iter()
        .filter(|d| d.message.contains("TOML"))
        .map(|d| d.message.as_str())
        .collect();
    assert!(
        toml_errors.is_empty(),
        "valid TOML should not have parse errors: {toml_errors:?}"
    );
}
````

````rs
#[test]
fn test_package_index_full_valid() {
    let text = r#"## Package

| Name | Version |
| --- | --- |
| test-pkg | 1.0.0 |

## Dependencies

No dependencies.

## Dev Dependencies

No dev dependencies.

## Rules

No special rules.
"#;

    let path = fixture_path("index.md");
    let config = Config::default();
    let diags = diagnostics::validate_package_md_text(&path, text, &config);
    assert!(
        diags.is_empty(),
        "valid index.md should have no errors: {diags:?}"
    );
}
````