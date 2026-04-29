use std::path::PathBuf;

use mds_core::Config;
use mds_lsp::capabilities::diagnostics;

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(format!("/tmp/mds-test/{name}"))
}

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
    assert!(
        diags.is_empty(),
        "expected no diagnostics, got: {diags:?}"
    );
}

#[test]
fn test_missing_sections() {
    let text = r#"## Purpose

A module with missing sections.
"#;

    let path = fixture_path("missing.ts.md");
    let config = Config::default();
    let diags = diagnostics::validate_impl_md_text(&path, text, &config);

    let messages: Vec<&str> = diags.iter().map(|d| d.message.as_str()).collect();
    assert!(
        messages.iter().any(|m| m.contains("Contract")),
        "should report missing Contract: {messages:?}"
    );
    assert!(
        messages.iter().any(|m| m.contains("Types")),
        "should report missing Types: {messages:?}"
    );
    assert!(
        messages.iter().any(|m| m.contains("Source")),
        "should report missing Source: {messages:?}"
    );
    assert!(
        messages.iter().any(|m| m.contains("Test")),
        "should report missing Test: {messages:?}"
    );
}

#[test]
fn test_heading_depth_violation() {
    let text = r#"## Purpose

A module.

## Contract

Contract.

## Types

```typescript
type A = string;
```

## Source

##### Invalid deep heading

```typescript
function main() {}
```

## Cases

Cases.

## Test

```typescript
test("x", () => {});
```
"#;

    let path = fixture_path("deep-heading.ts.md");
    let config = Config::default();
    let diags = diagnostics::validate_impl_md_text(&path, text, &config);

    let messages: Vec<&str> = diags.iter().map(|d| d.message.as_str()).collect();
    assert!(
        messages.iter().any(|m| m.contains("H3-H4")),
        "should report deep heading: {messages:?}"
    );
}

#[test]
fn test_config_validation_invalid_toml() {
    let text = "this is not valid toml [[[";
    let path = fixture_path("mds.config.toml");
    let diags = diagnostics::validate_config_text(&path, text);
    assert!(
        !diags.is_empty(),
        "should report TOML parse error"
    );
}

#[test]
fn test_package_md_missing_sections() {
    let text = r#"## Package

| Name | Version |
| --- | --- |
| test-pkg | 1.0.0 |
"#;

    let path = fixture_path("package.md");
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
        warnings.iter().any(|m| m.contains("python") && m.contains("ts")),
        "should warn about language mismatch: {warnings:?}"
    );
}

#[test]
fn test_empty_document_diagnostics() {
    let text = "";
    let path = fixture_path("empty.ts.md");
    let config = Config::default();
    let diags = diagnostics::validate_impl_md_text(&path, text, &config);
    // Should report missing sections
    assert!(
        diags.len() >= 5,
        "empty doc should report many missing sections: {diags:?}"
    );
}

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
    // May have diagnostics about missing package.md etc, but no TOML errors
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

#[test]
fn test_package_md_full_valid() {
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

    let path = fixture_path("package.md");
    let config = Config::default();
    let diags = diagnostics::validate_package_md_text(&path, text, &config);
    assert!(
        diags.is_empty(),
        "valid package.md should have no errors: {diags:?}"
    );
}
