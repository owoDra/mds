# Contributing

> *This page was translated from [Japanese](../ja/contributing.md) by AI.*

This page explains the basic guidelines for reporting issues, making proposals, and improving documentation for mds.

The source code in this repository is not generated from mds's implementation Markdown. Therefore, this page does not assume "using mds to fix mds itself."

## Welcome Contributions

mds welcomes the following types of contributions:

- Bug reports
- Adding reproduction steps
- Documentation improvements
- Pointing out unclear parts of the specification
- Usage example proposals
- Improvement suggestions for error messages and diagnostics
- Requests for supported languages or distribution methods

## What to Check When Making Proposals or Reports

When making proposals or reports, check the following:

| Type | What to Check |
| --- | --- |
| Bug report | Verify reproduction steps, executed commands, expected results, and actual results. |
| Specification proposal | Verify what you want to solve and in which use case it is needed. |
| Documentation improvement | Verify that the explanation makes it clear what the user should do next. |
| Usage example addition | Verify the files, commands, and expected results needed for the example. |

## Recommended Checks

When modifying source code, run checks as you would for a normal Rust, JavaScript, or Python project.

For Rust changes, run the following checks where possible:

```bash
cd crates
cargo test
```

If you have prepared sample packages that use mds, you can also perform the following checks:

```bash
mds lint --package path/to/package
mds build --package path/to/package --dry-run
```

## Notes When Writing Documentation

Do not over-abbreviate explanations so that readers can understand without prior knowledge.

When using abbreviations or technical terms, explain their meaning first.

Public documentation covers mds tool usage, design, specifications, and operational notes. Do not include work notes specific to a particular development environment or information unrelated to public users.

## Information to Include in Bug Reports

When reporting bugs, the following information helps investigate the cause:

- Executed command
- Expected result
- Actual result
- Exit code
- Target `mds.config.toml`
- Minimal example of the target implementation Markdown
- Versions of Rust, Node.js, Python being used

## Language Policy

The primary language of this project is Japanese. Issues, merge requests, and documentation are maintained in Japanese.

**If you are not comfortable writing in Japanese**, please write in English — we welcome contributions in any language. However, we kindly ask that you also include a Japanese translation alongside your English text. Machine translation (Google Translate, DeepL, ChatGPT, etc.) is perfectly acceptable. This helps maintainers process contributions more efficiently.

Example format:

```
## Title (English)
English description here.

---
## タイトル（日本語訳）
日本語の説明（機械翻訳可）
```
