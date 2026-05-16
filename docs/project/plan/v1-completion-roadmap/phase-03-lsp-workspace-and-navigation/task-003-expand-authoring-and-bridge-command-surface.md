# Task 003: Expand Authoring And Bridge Command Surface

## 目的

guided authoring 最低保証を保ったまま、editor 実装が再利用する remap / bridge command surface を v1 仕様に足る水準へ広げる。

## 前提条件

- workspace index と core remap 契約が安定している
- VS Code 側が必要とする bridge 需要が整理されている

## 作業内容

- section / fence / snippet / missing-section quick fix の guided authoring contract を不足分まで揃える
- generated-to-Markdown と Markdown-to-generated の command surface を見直す
- VS Code が references、rename、code action、formatting などを橋渡しするための command / data surface を追加または整理する

## 完了条件

- editor 側が再利用する command surface が spec どおり安定している
- guided authoring の最低保証が tests と実装で一致している
- remap 不能ケースが誤った Markdown 位置を返さない

## 検証方法

- authoring / code action / command surface test を追加する
- VS Code 側の handoff シナリオを想定した request / response を確認する

## 依存関係

- `task-001-index-overview-test-and-refresh-events.md`
- `../phase-01-core-runtime-and-authoring-policy/task-002-complete-quality-and-doctor-policy.md`

## 成果物

- `mds/lsp/src/capabilities/completion.rs`
- `mds/lsp/src/capabilities/code_action.rs`
- `mds/lsp/src/server.rs`
- `mds/lsp/tests/`
