# Task 003: Remap Diagnostics And Add Extension Coverage

## 目的

generated / embedded diagnostics を Markdown 正本へ戻す挙動を固め、VS Code extension の回帰確認手段を追加する。

## 前提条件

- status bar と embedded provider bridge の主要機能が揃っている
- broken/remap fixture を使う validation 方針が参照できる

## 作業内容

- generated file diagnostics だけでなく embedded/shadow language diagnostics の mirror 方針を整える
- remap 不能時の表示抑止と fallback message を整える
- extension 側の compile 以外の回帰確認手段を追加する

## 完了条件

- diagnostic mirror が generated / embedded の両系統で Markdown 正本へ戻る
- remap 不能 case で誤った Markdown diagnostic を表示しない
- extension の主要 bridge UX を継続確認できる coverage がある

## 検証方法

- extension compile を実行する
- success-path fixture と broken/remap fixture で diagnostic mirror を確認する

## 依存関係

- `task-001-add-status-bar-and-active-context.md`
- `task-002-expand-embedded-provider-delegation.md`
- `../phase-03-lsp-workspace-and-navigation/task-003-expand-authoring-and-bridge-command-surface.md`

## 成果物

- `editors/vscode/src/extension.ts`
- `editors/vscode/package.json`
- `editors/vscode/README.md`
- extension test or regression assets
