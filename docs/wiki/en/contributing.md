# Contributing

> *This page was translated from [Japanese](../ja/contributing.md) by AI.*

This page explains the basic guidelines for reporting issues, making proposals, and improving documentation for mds.

The source code in this repository is not generated from mds's implementation Markdown. Edit the checked-in source and test trees under `mds/*` and `editors/vscode` directly rather than trying to regenerate this repository with mds.

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
cargo fmt --all --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

If you changed the VS Code extension, also run:

```bash
cd editors/vscode
npm install
npm run compile
```

If you have prepared sample packages that use mds, you can also perform the following smoke checks:

```bash
cargo run -p mds-cli -- check --package examples/minimal-ts
cargo run -p mds-cli -- build --package examples/minimal-ts --dry-run
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
