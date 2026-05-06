# src/capabilities/hover.rs

## Purpose

Migrated implementation source for `src/capabilities/hover.rs`.

## Contract

- Preserve the behavior of the pre-migration Rust source.
- This file is synchronized into `.build/rust/mds/lsp/src/capabilities/hover.rs`.

## Imports

| From | Target | Symbols | Via | Summary | Reference |
| --- | --- | --- | --- | --- | --- |
| builtin | std::path | Path | - | - | - |
| external | mds_core::descriptor | lang_for_markdown_path | - | - | [../../../../core/.mds/source/descriptor.rs.md#source](../../../../core/.mds/source/descriptor.rs.md#source) |
| external | mds_core::descriptor | markdown_suffix_for_lang | - | - | [../../../../core/.mds/source/descriptor.rs.md#source](../../../../core/.mds/source/descriptor.rs.md#source) |
| external | mds_core::markdown | sections_with_labels | - | - | [../../../../core/.mds/source/markdown.rs.md#source](../../../../core/.mds/source/markdown.rs.md#source) |
| external | mds_core::markdown | source_markdown_root | - | - | [../../../../core/.mds/source/markdown.rs.md#source](../../../../core/.mds/source/markdown.rs.md#source) |
| external | tower_lsp::lsp_types | * | - | - | - |
| internal | crate::convert | line_at | - | - | [../convert.rs.md#source](../convert.rs.md#source) |
| internal | crate::convert | word_at_position | - | - | [../convert.rs.md#source](../convert.rs.md#source) |
| internal | crate::state | WorkspaceState | - | - | [../state.rs.md#source](../state.rs.md#source) |


## Source


Provide hover information for mds Markdown files.

````rs
pub fn provide_hover(
    text: &str,
    position: Position,
    path: &Path,
    state: &WorkspaceState,
) -> Option<Hover> {
    let line_text = line_at(text, position.line)?;

    // Hover on section headings: show section description
    if let Some(title) = line_text.strip_prefix("## ") {
        return hover_section(title.trim());
    }

    if let Some(title) = line_text.strip_prefix("##### ") {
        return Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: format!(
                    "**Shared definition**: `{}`\n\nH5 sections document reusable exported symbols referenced by `Exports`, `Imports`, and tests.",
                    title.trim()
                ),
            }),
            range: None,
        });
    }

    // Hover on Uses table rows: show target module's Purpose
    if line_text.trim_start().starts_with('|') {
        return hover_uses_target(text, position, path, state);
    }

    None
}

````

Hover information for section headings.

````rs
fn hover_section(title: &str) -> Option<Hover> {
    let description = match title {
        "Purpose" | "目的" => "**Purpose**: Module purpose and responsibility.\n\nDescribe what this module does and why it exists.",
        "Contract" | "契約" => "**Contract**: Public API contract and invariants.\n\nDefine the guarantees this module provides.",
        "Imports" | "依存" => "**Imports**: Dependency metadata.\n\nRecord imports outside code blocks using From, Target, Symbols, Via, Summary, and Reference columns.",
        "Exports" | "公開" | "Expose" | "Exposes" | "公開面" => "**Exports**: Public exports.\n\nDefines the module's public surface and shared definitions.",
        "Types" | "型定義" => "**Types**: Type definitions section.\n\nContains code blocks with type/interface definitions.",
        "Source" | "実装" => "**Source**: Implementation section.\n\nContains code blocks with the main implementation.",
        "Cases" | "ケース" => "**Cases**: Use cases and examples.\n\nDescribe how this module is used.",
        "Test" | "テスト" => "**Test**: Test section.\n\nContains code blocks with test code.",
        _ => return None,
    };

    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: description.to_string(),
        }),
        range: None,
    })
}

````



Hover information for a Uses table target. Show the target module's Purpose section.

````rs
fn hover_uses_target(
    text: &str,
    position: Position,
    path: &Path,
    state: &WorkspaceState,
) -> Option<Hover> {
    let word = word_at_position(text, position)?;
    if word.is_empty() {
        return None;
    }

    let pkg_state = state.package_for_path(path)?;
    let package = &pkg_state.package;
    let markdown_root = source_markdown_root(package);

    // Try to find the target file
    let lang = lang_for_markdown_path(path)?;
    let ext = markdown_suffix_for_lang(&lang)?;

    let target_path = markdown_root.join(format!("{word}{ext}"));
    let file_text = std::fs::read_to_string(&target_path).ok()?;
    let sections = sections_with_labels(&file_text, &package.config.label_overrides);

    let purpose = sections.get("Purpose")?;
    let module_name = target_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(&word);

    let hover_text = format!("### {module_name}\n\n{purpose}");

    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: hover_text,
        }),
        range: None,
    })
}
````
