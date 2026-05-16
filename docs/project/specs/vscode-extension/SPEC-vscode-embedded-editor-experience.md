---
id: SPEC-vscode-embedded-editor-experience
status: 提案中
related:
  - ../shared/SPEC-ux-embedded-language-bridge.md
  - ../shared/SPEC-authoring-markdown-format.md
  - ../shared/SPEC-ux-navigation-and-traceability.md
  - ../../requirements/v1/REQ-ux-guided-editor-authoring.md
  - ../../requirements/v1/REQ-ux-language-aware-embedded-lsp-bridge.md
  - ../../requirements/v1/REQ-ux-navigable-mds-knowledge-graph.md
subproject: vscode-extension
---

# VS Code Embedded Editor Experience

## 概要

VS Code 拡張が `mds file` に対して提供する language discovery、status bar、embedded language delegation、preview、diagnostic mirror の契約を定義する。

## 関連要求

- `REQ-ux-guided-editor-authoring`
- `REQ-ux-language-aware-embedded-lsp-bridge`
- `REQ-ux-navigable-mds-knowledge-graph`

## 入力

- workspace files
- package config / schema language definitions
- `mds.config.toml`
- active editor / cursor position
- `mds-lsp` command surface
- host editor language providers

## 出力

- active language registry
- status bar indicator
- embedded completion / hover / definition / references / rename / code action / formatting delegation
- mirrored diagnostics on Markdown 正本
- preview command

## 挙動

- 拡張は package config / schema、authoring root、file suffix、fence から active language を発見する。
- `mds-markdown` file では、カーソル位置の active code block 言語と doc kind を判定できる。
- status bar には少なくとも `mds <active-language> | <doc-kind>` を表示し、利用者が `mds file` であることと現在の言語文脈を把握できる。
- embedded code に対しては virtual / shadow surface を使い、host editor の既存言語機能へ橋渡しする。
- v1 では completion、hover、definition、references、rename、code action、formatting など、host editor が提供する機能を可能な限り `mds file` 内で再利用できることを目標にする。
- generated file 上の diagnostics や location は `mds-lsp` remap を使って Markdown 正本へ戻して表示できる。

## 状態遷移 / 不変条件

- active language indicator はカーソル移動に追従して更新されること。
- embedded result の text edit や location は `mds file` 上の位置に再対応付けされること。
- bridge failure があっても Markdown 編集自体は継続できること。

## エラー / 例外

- host editor provider 不在時は、その機能だけ unavailable として degrade する。
- 言語不明 block は status bar と bridge の両方で unknown 扱いにできる。
- remap 不能 diagnostic や location は誤った Markdown 位置へ表示しない。

## 横断ルール

- VS Code 実装は v1 の最初の editor 実装だが、shared bridge contract を逸脱しないこと。
- VS Code 固有 UI は shared contract を可視化する手段であり、shared semantics 自体を上書きしないこと。
- 将来他 editor を追加する場合も、language discovery と remap 契約を再利用できる構造にすること。

## 検証観点

- active language / doc kind が status bar で視認できる。
- embedded completion / hover / definition / references / rename / code action / formatting が provider のある言語で動作する。
- diagnostics mirror が generated file から Markdown 正本へ戻る。
- bridge unavailable 時も editor UX が破綻しない。

## 関連資料

- `../shared/SPEC-ux-embedded-language-bridge.md`
- `../shared/SPEC-authoring-markdown-format.md`
- `../shared/SPEC-ux-navigation-and-traceability.md`
- `../../requirements/v1/REQ-ux-guided-editor-authoring.md`
- `../../requirements/v1/REQ-ux-language-aware-embedded-lsp-bridge.md`
- `../../requirements/v1/REQ-ux-navigable-mds-knowledge-graph.md`
