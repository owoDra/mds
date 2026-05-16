# Phase 02: CLI Authoring Entrypoints

## 目的

`mds init` と `mds new` を v1 guided authoring の正式入口として成立させ、CLI command surface を実運用レベルで検証する。

## 前提条件

- `phase-01-core-runtime-and-authoring-policy` が完了している
- section profile、link policy、quality slot runtime を CLI から参照できる

## 完了条件

- wizard が spec で定義した画面責務と分岐を満たす
- `mds new <path> <kind> [options]` 契約と template 起票が spec どおり成立する
- CLI command surface と exit code 契約に対する実運用テストが揃う

## 検証方法

- CLI integration test で wizard / init / new / usage error / exit code を確認する
- `examples/minimal-ts` 相当の package 初期化と新規文書起票を確認する

## task 一覧

1. 以下は `phase-01` 完了後に並行着手できる
   - `task-001-align-init-wizard-screen-flow.md`: wizard を v1 screen flow に揃える
   - `task-002-align-new-command-and-templates.md`: `mds new` 契約と template 群を v1 に揃える
2. `task-003-add-cli-integration-coverage.md`: command surface、wizard、init/new の回帰確認を固める

## 依存関係

- `../phase-01-core-runtime-and-authoring-policy/index.md`
- `../../../specs/mds-cli/SPEC-cli-command-surface-and-execution.md`
- `../../../specs/mds-cli/SPEC-cli-init-and-new-workflows.md`
- `../../../specs/mds-cli/SPEC-cli-init-wizard-screen-flow.md`
- `../../../specs/mds-cli/SPEC-cli-doctor-and-update.md`

## 参照

1. 以下は `phase-01` 完了後に並行着手できる
   - `task-001-align-init-wizard-screen-flow.md`: wizard flow の完成
   - `task-002-align-new-command-and-templates.md`: `mds new` と template の完成
2. `task-003-add-cli-integration-coverage.md`: CLI 回帰確認の固定化
