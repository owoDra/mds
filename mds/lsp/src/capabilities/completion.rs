use std::path::{Path};
use mds_core::descriptor::{all_fence_label_completions};
use mds_core::descriptor::{fence_labels_for_lang};
use mds_core::descriptor::{lang_for_markdown_path};
use mds_core::model::{Config};
use tower_lsp::lsp_types::{*};
use crate::convert::{line_at};
use crate::labels::{resolve_label};
use crate::state::{WorkspaceState};

#[derive(Clone, Copy, PartialEq, Eq)]
enum MarkdownDocKind {
    Source,
    Test,
    Unknown,
}
pub fn provide_completions(
    text: &str,
    position: Position,
    path: Option<&Path>,
    config: &Config,
    state: Option<&WorkspaceState>,
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

    // Shared definition completion: `##### ` prefix
    if prefix.starts_with("##### ") || prefix == "#####" {
        items.push(CompletionItem {
            label: "mds: Shared Definition Section".to_string(),
            kind: Some(CompletionItemKind::SNIPPET),
            detail: Some("Add a H5 shared definition section".to_string()),
            insert_text: Some("${1:symbolName}\n\n${2:Describe the shared symbol.}\n".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            sort_text: Some("0_h5".to_string()),
            ..Default::default()
        });
        return items;
    }

    if let Some(wiki_prefix) = wiki_link_prefix(prefix) {
        items.extend(wiki_link_completions(wiki_prefix, state));
        return items;
    }

    // Table column completion: inside `| ... |` row
    if prefix.trim_start().starts_with('|') {
        items.extend(table_column_completions(config));
    }

    // Code block language completion: any opening backtick fence prefix
    if is_backtick_fence_prefix(prefix.trim_start()) {
        items.extend(code_block_language_completions(path));
    }

    // Snippet completions
    items.extend(snippet_completions(path, config));

    items
}

fn is_backtick_fence_prefix(prefix: &str) -> bool {
    let marker_len = prefix.chars().take_while(|character| *character == '`').count();
    marker_len >= 3 && prefix[marker_len..].trim().is_empty()
}

fn wiki_link_prefix(prefix: &str) -> Option<&str> {
    let start = prefix.rfind("[[")?;
    let candidate = &prefix[start + 2..];
    (!candidate.contains("]]")).then_some(candidate)
}

fn wiki_link_completions(prefix: &str, state: Option<&WorkspaceState>) -> Vec<CompletionItem> {
    let Some(state) = state else {
        return Vec::new();
    };
    let (module_prefix, symbol_prefix) = prefix.split_once('#').unwrap_or((prefix, ""));
    let mut items = Vec::new();
    for pkg in &state.packages {
        if prefix.contains('#') {
            for (module, symbol) in pkg.index.symbol_index.keys() {
                if module == module_prefix && symbol.starts_with(symbol_prefix) {
                    items.push(CompletionItem {
                        label: symbol.clone(),
                        kind: Some(CompletionItemKind::FUNCTION),
                        detail: Some(format!("Exported symbol from {module}")),
                        insert_text: Some(format!("{symbol}]]")),
                        sort_text: Some(format!("0_{symbol}")),
                        ..Default::default()
                    });
                }
            }
        } else {
            let mut modules = pkg.index.module_index.keys().cloned().collect::<Vec<_>>();
            modules.sort();
            modules.dedup();
            for module in modules {
                if module.starts_with(module_prefix) {
                    items.push(CompletionItem {
                        label: module.clone(),
                        kind: Some(CompletionItemKind::MODULE),
                        detail: Some("mds module".to_string()),
                        insert_text: Some(format!("{module}]]")),
                        sort_text: Some(format!("0_{module}")),
                        ..Default::default()
                    });
                }
            }
        }
    }
    items
}

fn section_completions(config: &Config) -> Vec<CompletionItem> {
    let canonical_sections = [
        ("仕様", "Readable contract and invariants"),
        ("API", "Public API prose"),
        ("実装", "Implementation code"),
        ("検証", "Verification notes"),
        ("対象", "Covered source modules"),
        ("ケース", "Verification cases"),
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

fn table_column_completions(config: &Config) -> Vec<CompletionItem> {
    let imports_columns = [
        (
            "From",
            "Import source: builtin, external, package, workspace, internal",
        ),
        ("Target", "Import target module or package"),
        ("Symbols", "Imported symbols from the target"),
        ("Via", "Import mechanism or adapter-specific path"),
        ("Summary", "Brief description of the import"),
        ("Reference", "Markdown reference for navigation"),
    ];

    let export_columns = [
        ("Name", "Exported symbol name"),
        ("Visibility", "Public or internal visibility"),
        ("Summary", "Brief description of the exported symbol"),
    ];

    let mut items: Vec<CompletionItem> = imports_columns
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

    items.extend(export_columns.iter().map(|(name, detail)| {
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

fn code_block_language_completions(path: Option<&Path>) -> Vec<CompletionItem> {
    let detected = path.and_then(lang_for_markdown_path);
    let recommended = detected
        .as_ref()
        .map(fence_labels_for_lang)
        .unwrap_or_default();

    all_fence_label_completions()
        .into_iter()
        .map(|(label, detail)| {
            let is_recommended = recommended.contains(&label);

            CompletionItem {
                label: label.to_string(),
                kind: Some(CompletionItemKind::ENUM_MEMBER),
                detail: Some(detail),
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

fn markdown_doc_kind(path: Option<&Path>) -> MarkdownDocKind {
    let Some(path) = path else {
        return MarkdownDocKind::Unknown;
    };

    let normalized = path.to_string_lossy().replace('\\', "/");
    if normalized.contains("/.mds/test/") {
        MarkdownDocKind::Test
    } else if normalized.contains("/.mds/source/") {
        MarkdownDocKind::Source
    } else {
        MarkdownDocKind::Unknown
    }
}

fn snippet_completions(path: Option<&Path>, config: &Config) -> Vec<CompletionItem> {
    let mut items = Vec::new();
    let lang = path.and_then(lang_for_markdown_path);
    let doc_kind = markdown_doc_kind(path);

    let lang_label = lang
        .as_ref()
        .and_then(|l| fence_labels_for_lang(l).into_iter().next())
        .or_else(|| all_fence_label_completions().into_iter().map(|(label, _)| label).next())
        .unwrap_or_else(|| "text".to_string());

    items.push(CompletionItem {
        label: "mds: New Source Document".to_string(),
        kind: Some(CompletionItemKind::SNIPPET),
        detail: Some("Tableless authoring-v2 source document".to_string()),
        insert_text: Some(format!(
            "## {purpose}\n\n${{1:Describe the module purpose}}\n\n\
             ## {contract}\n\n${{2:Define behavior, constraints, and boundaries}}\n\n\
             ## {api}\n\n${{3:Describe the public API in prose. Keep import/export details in the source block.}}\n\n\
             ## {source}\n\n\
             ```{lang_label}\n${{4:// implementation}}\n```\n\n\
             ## {cases}\n\n- ${{5:Describe expected behavior}}\n",
            purpose = resolve_label("purpose", config),
            contract = resolve_label("contract", config),
            api = resolve_label("api", config),
            source = resolve_label("source", config),
            cases = resolve_label("cases", config),
        )),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        sort_text: Some(
            match doc_kind {
                MarkdownDocKind::Source => "8_source_template",
                _ => "9_source_template",
            }
            .to_string(),
        ),
        ..Default::default()
    });

    items.push(CompletionItem {
        label: "mds: New Test Document".to_string(),
        kind: Some(CompletionItemKind::SNIPPET),
        detail: Some("Tableless authoring-v2 test document".to_string()),
        insert_text: Some(format!(
            "## {purpose}\n\n${{1:Describe the verification goal}}\n\n\
             ## {covers}\n\n- ${{2:module.id}}\n\n\
             ## {cases}\n\n- ${{3:Describe the test case}}\n\n\
             ## {test}\n\n\
             ```{lang_label}\n${{4:// test code}}\n```\n",
            purpose = resolve_label("purpose", config),
            covers = resolve_label("covers", config),
            cases = resolve_label("cases", config),
            test = resolve_label("test", config),
        )),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        sort_text: Some(
            match doc_kind {
                MarkdownDocKind::Test => "8_test_template",
                _ => "9_test_template",
            }
            .to_string(),
        ),
        ..Default::default()
    });

    items.push(CompletionItem {
        label: "mds: New Spec Document".to_string(),
        kind: Some(CompletionItemKind::SNIPPET),
        detail: Some("Tableless source spec without generated code".to_string()),
        insert_text: Some(format!(
            "## {purpose}\n\n${{1:Describe the feature purpose}}\n\n\
             ## {contract}\n\n${{2:Define behavior, constraints, and boundaries}}\n\n\
             ## {api}\n\n${{3:Describe the public API in prose.}}\n\n\
             ## {cases}\n\n- ${{4:Describe expected behavior}}\n",
            purpose = resolve_label("purpose", config),
            contract = resolve_label("contract", config),
            api = resolve_label("api", config),
            cases = resolve_label("cases", config),
        )),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        sort_text: Some("9_spec_template".to_string()),
        ..Default::default()
    });

    items.push(CompletionItem {
        label: "mds: Code Block".to_string(),
        kind: Some(CompletionItemKind::SNIPPET),
        detail: Some("Add a new code block".to_string()),
        insert_text: Some(format!("```{lang_label}\n${{1:// code}}\n```\n")),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        sort_text: Some("9_code_block".to_string()),
        ..Default::default()
    });

    let section_choices = match doc_kind {
        MarkdownDocKind::Source => format!(
            "{purpose},{contract},{api},{source},{cases}",
            purpose = resolve_label("purpose", config),
            contract = resolve_label("contract", config),
            api = resolve_label("api", config),
            source = resolve_label("source", config),
            cases = resolve_label("cases", config),
        ),
        MarkdownDocKind::Test => format!(
            "{purpose},{covers},{cases},{test}",
            purpose = resolve_label("purpose", config),
            covers = resolve_label("covers", config),
            cases = resolve_label("cases", config),
            test = resolve_label("test", config),
        ),
        MarkdownDocKind::Unknown => format!(
            "{purpose},{contract},{api},{source},{covers},{cases},{test}",
            purpose = resolve_label("purpose", config),
            contract = resolve_label("contract", config),
            api = resolve_label("api", config),
            source = resolve_label("source", config),
            covers = resolve_label("covers", config),
            cases = resolve_label("cases", config),
            test = resolve_label("test", config),
        ),
    };

    items.push(CompletionItem {
        label: "mds: New Section".to_string(),
        kind: Some(CompletionItemKind::SNIPPET),
        detail: Some("Add a new authoring-v2 section".to_string()),
        insert_text: Some(format!("## ${{1|{section_choices}|}}\n\n${{2:Content}}\n")),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        sort_text: Some("9_section".to_string()),
        ..Default::default()
    });

    items
}
