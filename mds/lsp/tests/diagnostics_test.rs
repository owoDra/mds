use std::path::{PathBuf};
use mds_core::model::CheckDiagnosticPolicy;
use mds_core::{Config, Package};
use mds_lsp::capabilities::{diagnostics};
use mds_lsp::state::{PackageState, WorkspaceIndex, WorkspaceState};
use std::collections::HashMap;
use tower_lsp::lsp_types::DiagnosticSeverity;

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(format!("/tmp/mds-test/{name}"))
}

fn sample_markdown(text: &str) -> String {
    text.replace("{h2}", "##")
        .replace("{h5}", "#####")
        .replace("{fence}", "```")
}

#[test]
fn test_valid_impl_md_no_diagnostics_for_minimal() {
    let text = sample_markdown(r#"{h2} Purpose

A minimal test module.

{h2} Contract

Public contract.

{h2} Source
{fence}typescript
export function main(): void {}
{fence}
"#);

    let path = fixture_path("valid.ts.md");
    let config = Config::default();
    let diags = diagnostics::validate_impl_md_text(&path, &text, &config);
    assert!(diags.is_empty(), "expected no diagnostics, got: {diags:?}");
}

#[test]
fn test_missing_sections() {
    let text = sample_markdown(r#"{h2} Purpose

A module with implementation code but no contract.

{h2} Source

{fence}typescript
export function main(): void {}
{fence}
"#);

    let path = fixture_path("missing.ts.md");
    let config = Config::default();
    let diags = diagnostics::validate_impl_md_text(&path, &text, &config);

    let messages: Vec<&str> = diags.iter().map(|d| d.message.as_str()).collect();
    assert!(
        messages.iter().any(|m| m.contains("Contract")),
        "should report missing Contract: {messages:?}"
    );
}

#[test]
fn test_heading_depth_violation() {
    // H5+ is no longer an error in the new format
    let text = sample_markdown(r#"{h2} Purpose

A module.

{h5} Deep heading is allowed now

{h2} Source

{fence}typescript
function main() {}
{fence}
"#);

    let path = fixture_path("deep-heading.ts.md");
    let config = Config::default();
    let diags = diagnostics::validate_impl_md_text(&path, &text, &config);

    // No heading-depth errors anymore
    let messages: Vec<&str> = diags.iter().map(|d| d.message.as_str()).collect();
    assert!(
        !messages.iter().any(|m| m.contains("H3-H4")),
        "should not report deep heading: {messages:?}"
    );
}

#[test]
fn test_config_validation_invalid_toml() {
    let text = "this is not valid toml [[[";
    let path = fixture_path("mds.config.toml");
    let diags = diagnostics::validate_config_text(&path, text);
    assert!(!diags.is_empty(), "should report TOML parse error");
}

#[test]
fn test_package_index_missing_sections() {
    let text = sample_markdown(r#"{h2} Package

| Name | Version |
| --- | --- |
| test-pkg | 1.0.0 |
"#);

    let path = fixture_path("index.md");
    let config = Config::default();
    let diags = diagnostics::validate_package_md_text(&path, &text, &config);

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
    let text = sample_markdown(r#"{h2} Purpose

A module.

{h2} Contract

Contract.

{h2} Source

{fence}python
x = 1
{fence}

{h2} Source

{fence}python
def main(): pass
{fence}

{h2} Cases

Cases.

{h2} Test

{fence}python
def test_it(): assert True
{fence}
"#);

    // File is .ts.md but code blocks use python
    let path = fixture_path("mismatch.ts.md");
    let config = Config::default();
    let diags = diagnostics::validate_impl_md_text(&path, &text, &config);

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

#[test]
fn test_code_block_language_mismatch_warning_with_long_fence() {
    let text = r#"{h2} Purpose

A module.

{h2} Contract

Contract.

{h2} Source

{fence4}python
def main(): pass
{fence4}
"#.replace("{h2}", "##").replace("{fence4}", "````");

    let path = fixture_path("mismatch.ts.md");
    let config = Config::default();
    let diags = diagnostics::validate_impl_md_text(&path, &text, &config);
    assert!(
        diags.iter().any(|d| d.message.contains("python") && d.message.contains("ts")),
        "long fence label should be detected: {diags:?}"
    );
}

#[test]
fn test_import_reference_required_for_internal_imports() {
    let text = sample_markdown(r#"{h2} Purpose

A module.

{h2} Imports

| From | Target | Symbols | Via | Summary | Reference |
| --- | --- | --- | --- | --- | --- |
| internal | utils/helper | helper | - | Helper function | - |
"#);
    let path = fixture_path("imports.ts.md");
    let config = Config::default();
    let diags = diagnostics::validate_impl_md_text(&path, &text, &config);
    assert!(
        diags.iter().any(|d| d.message.contains("requires a Markdown Reference")),
        "internal imports should require Reference links: {diags:?}"
    );
}

#[test]
fn test_empty_document_diagnostics() {
    let text = "";
    let path = fixture_path("empty.ts.md");
    let config = Config::default();
    let diags = diagnostics::validate_impl_md_text(&path, text, &config);
    assert!(
        !diags.is_empty(),
        "empty doc should report missing documentation: {diags:?}"
    );
    assert!(
        diags.iter().any(|d| d.message.contains("Purpose")),
        "should mention Purpose requirement: {diags:?}"
    );
}

#[test]
fn test_valid_config_with_canonical_roots_has_no_toml_errors() {
    let config_text =
        "[package]\nenabled = true\n\n[roots]\nsource_md = \".mds/source\"\ntest_md = \".mds/test\"\n";

    let dir = tempfile::tempdir().unwrap();
    let config_path = dir.path().join("mds.config.toml");
    std::fs::write(&config_path, config_text).unwrap();

    let diags = diagnostics::validate_config_text(&config_path, config_text);
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
fn test_legacy_roots_markdown_is_reported_as_unsupported() {
    let config_text =
        "[package]\nenabled = true\n\n[roots]\nmarkdown = \"src-md\"\n";

    let dir = tempfile::tempdir().unwrap();
    let config_path = dir.path().join("mds.config.toml");
    std::fs::write(&config_path, config_text).unwrap();

    let diags = diagnostics::validate_config_text(&config_path, config_text);
    assert!(
        diags
            .iter()
            .any(|d| d.message.contains("ignoring unsupported roots config `markdown`")),
        "legacy roots.markdown should be reported as unsupported: {diags:?}"
    );
}

#[test]
fn test_package_index_full_valid() {
    let text = sample_markdown(r#"{h2} Package

| Name | Version |
| --- | --- |
| test-pkg | 1.0.0 |

{h2} Dependencies

No dependencies.

{h2} Dev Dependencies

No dev dependencies.

{h2} Rules

No special rules.
"#);

    let path = fixture_path("index.md");
    let config = Config::default();
    let diags = diagnostics::validate_package_md_text(&path, &text, &config);
    assert!(
        diags.is_empty(),
        "valid index.md should have no errors: {diags:?}"
    );
}

#[test]
fn test_legacy_table_policy_controls_severity() {
    let text = sample_markdown(r#"{h2} Purpose

Legacy metadata fixture.

{h2} Imports

| From | Target | Symbols | Via | Summary | Reference |
| --- | --- | --- | --- | --- | --- |
| builtin | node:path | join | - | Path join utility | - |
"#);
    let path = fixture_path("legacy.ts.md");

    for (policy, expected) in [
        (CheckDiagnosticPolicy::Warn, Some(DiagnosticSeverity::WARNING)),
        (CheckDiagnosticPolicy::Error, Some(DiagnosticSeverity::ERROR)),
        (CheckDiagnosticPolicy::Allow, None),
    ] {
        let mut config = Config::default();
        config.check.legacy_tables = policy;
        let diags = diagnostics::validate_impl_md_text(&path, &text, &config);
        let legacy = diags
            .iter()
            .find(|diagnostic| diagnostic.message.contains("legacy table metadata"));

        match expected {
            Some(severity) => assert_eq!(legacy.and_then(|diagnostic| diagnostic.severity), Some(severity)),
            None => assert!(legacy.is_none(), "allow should suppress legacy table diagnostics: {diags:?}"),
        }
    }
}

#[test]
fn test_split_source_and_test_reports_mixing_for_both_doc_kinds() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("pkg");
    let config = Config::default();
    let state = workspace_state(&root, config.clone(), HashMap::new(), HashMap::new());

    let source_path = root.join(".mds/source/mixed.ts.md");
    let source_text = sample_markdown(r#"{h2} Purpose

Source doc.

{h2} Contract

Contract.

{h2} Test

{fence}typescript
test("it works", () => {});
{fence}
"#);
    let source_diags = diagnostics::validate_impl_md_text_with_state(
        &source_path,
        &source_text,
        &config,
        Some(&state),
    );
    assert!(
        source_diags.iter().any(|diagnostic| diagnostic.message.contains("source md must not contain generated test code")),
        "source doc should report test mixing: {source_diags:?}"
    );

    let test_path = root.join(".mds/test/mixed.md");
    let test_text = sample_markdown(r#"{h2} Purpose

Test doc.

{h2} Covers

<!-- TODO -->

{h2} Cases

Case.

{h2} Source

{fence}typescript
export const value = 1;
{fence}
"#);
    let test_diags = diagnostics::validate_impl_md_text_with_state(
        &test_path,
        &test_text,
        &config,
        Some(&state),
    );
    assert!(
        test_diags.iter().any(|diagnostic| diagnostic.message.contains("test md must not contain generated source code")),
        "test doc should report source mixing: {test_diags:?}"
    );
}

#[test]
fn test_wiki_link_unresolved_uses_workspace_index_policies() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("pkg");
    let source_path = root.join(".mds/source/pkg/greet.ts.md");
    let test_path = root.join(".mds/test/greet.md");
    let mut module_index = HashMap::new();
    module_index.insert("pkg.greet".to_string(), vec![source_path.clone()]);
    let mut symbol_index = HashMap::new();
    symbol_index.insert(
        ("pkg.greet".to_string(), "greet".to_string()),
        vec![source_path.clone()],
    );
    let state = workspace_state(&root, Config::default(), module_index, symbol_index);

    let text = sample_markdown(r#"{h2} Purpose

Wiki link validation.

{h2} Covers

- [[pkg.greet]]
- [[pkg.missing]]
- [[pkg.greet#missing]]

{h2} Cases

Ensure unresolved links surface in the editor.
"#);

    let diags = diagnostics::validate_impl_md_text_with_state(
        &test_path,
        &text,
        &Config::default(),
        Some(&state),
    );

    assert!(
        diags.iter().any(|diagnostic| {
            diagnostic.message.contains("wiki link target `[[pkg.missing]]` does not resolve to a module")
                && diagnostic.severity == Some(DiagnosticSeverity::ERROR)
        }),
        "missing module should be an error: {diags:?}"
    );
    assert!(
        diags.iter().any(|diagnostic| {
            diagnostic.message.contains("wiki link target `[[pkg.greet#missing]]` does not resolve to a documented symbol")
                && diagnostic.severity == Some(DiagnosticSeverity::WARNING)
        }),
        "missing symbol should follow warn policy: {diags:?}"
    );
}

fn workspace_state(
    root: &std::path::Path,
    config: Config,
    module_index: HashMap<String, Vec<PathBuf>>,
    symbol_index: HashMap<(String, String), Vec<PathBuf>>,
) -> WorkspaceState {
    WorkspaceState {
        packages: vec![PackageState {
            package: Package {
                root: root.to_path_buf(),
                config,
                package_manager_id: "npm".to_string(),
            },
            index: WorkspaceIndex {
                module_index,
                symbol_index,
                ..WorkspaceIndex::default()
            },
        }],
        ..WorkspaceState::default()
    }
}
