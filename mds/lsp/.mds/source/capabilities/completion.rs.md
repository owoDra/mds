# src/capabilities/completion.rs

## Purpose

Migrated implementation source for `src/capabilities/completion.rs`.

## Contract

- Preserve the behavior of the pre-migration Rust source.
- This file is synchronized into `.build/rust/mds/lsp/src/capabilities/completion.rs`.

## Imports

| From | Target | Symbols | Via | Summary | Reference |
| --- | --- | --- | --- | --- | --- |
| builtin | std::path | Path | - | - | - |
| external | mds_core::model | Config | - | - | [../../../../core/.mds/source/model.rs.md#source](../../../../core/.mds/source/model.rs.md#source) |
| external | mds_core::model | Lang | - | - | [../../../../core/.mds/source/model.rs.md#source](../../../../core/.mds/source/model.rs.md#source) |
| external | tower_lsp::lsp_types | * | - | - | - |
| internal | crate::convert | line_at | - | - | [../convert.rs.md#source](../convert.rs.md#source) |
| internal | crate::labels | resolve_label | - | - | [../labels.rs.md#source](../labels.rs.md#source) |


## Source


Provide completion items based on cursor position.

````rs
pub fn provide_completions(
    text: &str,
    position: Position,
    path: Option<&Path>,
    config: &Config,
) -> Vec<CompletionItem> {
    let mut items = Vec::new();

    let Some(line_text) = line_at(text, position.line) else {
        return items;
    };

    let col = position.character as usize;
    let prefix = if col <= line_text.len() {
        &line_text[..col]
    } else {
        line_text
    };

    // Section name completion: `## ` prefix
    if prefix.starts_with("## ") || prefix == "##" {
        items.extend(section_completions(config));
        return items;
    }

    // Table column completion: inside `| ... |` row
    if prefix.trim_start().starts_with('|') {
        items.extend(table_column_completions(config));
    }

    // Code block language completion: ``` prefix
    if prefix.trim_start().starts_with("```") && prefix.trim_start().len() <= 3 {
        items.extend(code_block_language_completions(path));
    }

    // Snippet completions
    items.extend(snippet_completions(path, config));

    items
}

````

Section heading completions.

````rs
fn section_completions(config: &Config) -> Vec<CompletionItem> {
    let canonical_sections = [
        ("Purpose", "Module purpose and responsibility"),
        ("Contract", "Public API contract and invariants"),
        ("Types", "Type definitions"),
        ("Source", "Implementation source code"),
        ("Cases", "Use cases and examples"),
        ("Test", "Test code"),
        ("Expose", "Public exports"),
    ];

    canonical_sections
        .iter()
        .map(|(name, detail)| {
            let label = resolve_label(name, config);
            CompletionItem {
                label: label.clone(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some(detail.to_string()),
                insert_text: Some(format!("{label}\n\n")),
                insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                sort_text: Some(format!("0_{name}")),
                ..Default::default()
            }
        })
        .collect()
}

````

Table column name completions.

````rs
fn table_column_completions(config: &Config) -> Vec<CompletionItem> {
    let uses_columns = [
        (
            "From",
            "Import source: builtin, package, workspace, internal",
        ),
        ("Target", "Import target module or package"),
        ("Expose", "Exposed names from the target"),
        ("Summary", "Brief description of the import"),
    ];

    let package_columns = [("Name", "Package name"), ("Version", "Package version")];

    let mut items: Vec<CompletionItem> = uses_columns
        .iter()
        .map(|(name, detail)| {
            let label = resolve_label(&name.to_lowercase(), config);
            CompletionItem {
                label,
                kind: Some(CompletionItemKind::FIELD),
                detail: Some(detail.to_string()),
                ..Default::default()
            }
        })
        .collect();

    items.extend(package_columns.iter().map(|(name, detail)| {
        let label = resolve_label(&name.to_lowercase(), config);
        CompletionItem {
            label,
            kind: Some(CompletionItemKind::FIELD),
            detail: Some(detail.to_string()),
            ..Default::default()
        }
    }));

    items
}

````

Code block language label completions.

````rs
fn code_block_language_completions(path: Option<&Path>) -> Vec<CompletionItem> {
    let detected = path.and_then(Lang::from_path);

    let languages = [
        ("typescript", "TypeScript"),
        ("python", "Python"),
        ("rust", "Rust"),
    ];

    languages
        .iter()
        .map(|(label, detail)| {
            let is_recommended = detected
                .as_ref()
                .map(|lang| match lang {
                    Lang::TypeScript => *label == "typescript",
                    Lang::Python => *label == "python",
                    Lang::Rust => *label == "rust",
                    Lang::Other(ext) => *label == ext.as_str(),
                })
                .unwrap_or(false);

            CompletionItem {
                label: label.to_string(),
                kind: Some(CompletionItemKind::ENUM_MEMBER),
                detail: Some(detail.to_string()),
                sort_text: Some(if is_recommended {
                    format!("0_{label}")
                } else {
                    format!("1_{label}")
                }),
                preselect: Some(is_recommended),
                ..Default::default()
            }
        })
        .collect()
}

````



Snippet completions for common mds patterns.

````rs
fn snippet_completions(path: Option<&Path>, config: &Config) -> Vec<CompletionItem> {
    let mut items = Vec::new();
    let lang = path.and_then(Lang::from_path);

    let lang_label = lang
        .as_ref()
        .map(|l| match l {
            Lang::TypeScript => "typescript",
            Lang::Python => "python",
            Lang::Rust => "rust",
            Lang::Other(ext) => ext.as_str(),
        })
        .unwrap_or("typescript");

    // Full implementation document template
    items.push(CompletionItem {
        label: "mds: New Implementation Document".to_string(),
        kind: Some(CompletionItemKind::SNIPPET),
        detail: Some("Full mds implementation document template".to_string()),
        insert_text: Some(format!(
            "## {purpose}\n\n${{1:Describe the module purpose}}\n\n\
             ## {contract}\n\n${{2:Define the public contract}}\n\n\
             ## {types}\n\n### Uses\n\n| {from} | {target} | {expose} | {summary} |\n\
             | --- | --- | --- | --- |\n\
             | ${{3:builtin}} | ${{4:target}} | ${{5:Name}} | ${{6:description}} |\n\n\
             ```{lang_label}\n${{7:// type definitions}}\n```\n\n\
             ## {source}\n\n### Uses\n\n| {from} | {target} | {expose} | {summary} |\n\
             | --- | --- | --- | --- |\n\
             | ${{8:internal}} | ${{9:target}} | ${{10:Name}} | ${{11:description}} |\n\n\
             ```{lang_label}\n${{12:// implementation}}\n```\n\n\
             ## {cases}\n\n${{13:Describe use cases}}\n\n\
             ## {test}\n\n### Uses\n\n| {from} | {target} | {expose} | {summary} |\n\
             | --- | --- | --- | --- |\n\
             | ${{14:internal}} | ${{15:target}} | ${{16:Name}} | ${{17:description}} |\n\n\
             ```{lang_label}\n${{18:// test code}}\n```\n",
            purpose = resolve_label("purpose", config),
            contract = resolve_label("contract", config),
            types = resolve_label("types", config),
            source = resolve_label("source", config),
            cases = resolve_label("cases", config),
            test = resolve_label("test", config),
            from = resolve_label("from", config),
            target = resolve_label("target", config),
            expose = resolve_label("expose", config),
            summary = resolve_label("summary", config),
        )),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        sort_text: Some("9_template".to_string()),
        ..Default::default()
    });

    // Uses table row snippet
    items.push(CompletionItem {
        label: "mds: Uses Table Row".to_string(),
        kind: Some(CompletionItemKind::SNIPPET),
        detail: Some("Add a new Uses table row".to_string()),
        insert_text: Some(
            "| ${1|builtin,package,workspace,internal|} | ${2:target} | ${3:Name} | ${4:description} |".to_string(),
        ),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        sort_text: Some("9_uses_row".to_string()),
        ..Default::default()
    });

    // Code block snippet
    items.push(CompletionItem {
        label: "mds: Code Block".to_string(),
        kind: Some(CompletionItemKind::SNIPPET),
        detail: Some("Add a new code block".to_string()),
        insert_text: Some(format!("```{lang_label}\n${{1:// code}}\n```\n")),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        sort_text: Some("9_code_block".to_string()),
        ..Default::default()
    });

    // Section snippet
    items.push(CompletionItem {
        label: "mds: New Section".to_string(),
        kind: Some(CompletionItemKind::SNIPPET),
        detail: Some("Add a new section with Uses table and code block".to_string()),
        insert_text: Some(format!(
            "## ${{1|{purpose},{contract},{types},{source},{cases},{test}|}}\n\n\
             ### Uses\n\n\
             | {from} | {target} | {expose} | {summary} |\n\
             | --- | --- | --- | --- |\n\
             | ${{2:builtin}} | ${{3:target}} | ${{4:Name}} | ${{5:description}} |\n\n\
             ```{lang_label}\n${{6:// code}}\n```\n",
            purpose = resolve_label("purpose", config),
            contract = resolve_label("contract", config),
            types = resolve_label("types", config),
            source = resolve_label("source", config),
            cases = resolve_label("cases", config),
            test = resolve_label("test", config),
            from = resolve_label("from", config),
            target = resolve_label("target", config),
            expose = resolve_label("expose", config),
            summary = resolve_label("summary", config),
        )),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        sort_text: Some("9_section".to_string()),
        ..Default::default()
    });

    items
}
````
