# Phase 03: LSP Workspace And Navigation

## 目的

`mds-lsp` の workspace index、diagnostics、navigation、editor bridge command surface を v1 spec へ揃える。

## 前提条件

- `phase-01-core-runtime-and-authoring-policy` が完了している
- core が source/test/overview、link policy、source map を安定して返せる

## 完了条件

- `overview.md` と test doc を含む workspace index が安定して更新される
- definition / references / symbols / generated remap が structured-first で説明できる
- editor 実装が再利用する bridge command surface が v1 仕様に足る

## 検証方法

- `mds/lsp` test で diagnostics、navigation、remap、refresh を確認する
- example fixture と broken/remap fixture を使う手動確認で index と remap を確認する

## task 一覧

1. `task-001-index-overview-test-and-refresh-events.md`: source/test/overview index と refresh trigger を完成させる
2. 以下は `task-001` 完了後に並行着手できる
   - `task-002-prioritize-structured-navigation-and-references.md`: structured-first navigation / references を固める
   - `task-003-expand-authoring-and-bridge-command-surface.md`: guided authoring と editor bridge command surface を拡張する

## 依存関係

- `../phase-01-core-runtime-and-authoring-policy/index.md`
- `../../../specs/mds-lsp/SPEC-lsp-authoring-navigation-remap.md`
- `../../../specs/shared/SPEC-ux-navigation-and-traceability.md`
- `../../../specs/shared/SPEC-ux-embedded-language-bridge.md`

## 参照

1. `task-001-index-overview-test-and-refresh-events.md`: workspace index と refresh trigger の完成
2. 以下は `task-001` 完了後に並行着手できる
   - `task-002-prioritize-structured-navigation-and-references.md`: structured-first navigation の完成
   - `task-003-expand-authoring-and-bridge-command-surface.md`: authoring / bridge command surface の完成
