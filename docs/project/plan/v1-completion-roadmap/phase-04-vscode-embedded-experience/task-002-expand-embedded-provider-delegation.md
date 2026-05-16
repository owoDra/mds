# Task 002: Expand Embedded Provider Delegation

## 目的

embedded code block から references、rename、code action、formatting まで host editor provider を再利用できる状態にする。

## 前提条件

- VS Code extension が virtual / shadow surface を安定して作れる
- LSP の bridge command surface が editor 連携に必要な情報を返せる

## 作業内容

- references、rename、code action、formatting provider を登録し、既存 completion / hover / definition bridge と揃える
- text edit、workspace edit、location list を Markdown 正本位置へ再対応付けする
- provider 不在時の degrade 動作を completion / hover と同じ方針へ揃える

## 完了条件

- provider がある言語で references / rename / code action / formatting が `mds file` 内から再利用できる
- edit / location remap が Markdown 正本に戻る
- provider 不在や remap 不能時に誤編集や誤位置表示を行わない

## 検証方法

- extension compile と bridge scenario 確認を行う
- sample block 上で各 provider の request / remap を確認する

## 依存関係

- `../phase-03-lsp-workspace-and-navigation/task-003-expand-authoring-and-bridge-command-surface.md`

## 成果物

- `editors/vscode/src/extension.ts`
- `editors/vscode/package.json`
