# Task 003: Add CLI Integration Coverage

## 目的

CLI command surface、wizard、init/new、usage error、exit code 契約を回帰確認できる test 群を揃える。

## 前提条件

- wizard と `new` 契約が v1 仕様に揃っている
- representative fixture または temp package で end-to-end 確認できる

## 作業内容

- `build` `lint` `typecheck` `test` `doctor` `package sync` `init` `new` `update` の主要 command path を integration test 化する
- invalid option combination、usage error、quality error、environment error、internal error の exit code 差分を確認する
- wizard の主要分岐と init/new 出力の整合を回帰化する

## 完了条件

- CLI の主要 contract が test で固定される
- 既存の薄い test stub が実運用レベルのケースへ置き換わる
- 仕様変更時に CLI 振る舞い差分を検知できる

## 検証方法

- `mds/cli` test を実行する
- `examples/minimal-ts` または temp fixture を使う end-to-end 実行を確認する

## 依存関係

- `task-001-align-init-wizard-screen-flow.md`
- `task-002-align-new-command-and-templates.md`

## 成果物

- `mds/cli/tests/main_test.rs`
- `mds/cli/tests/wizard_test.rs`
- `mds/cli/tests/args_test.rs`
- `mds/core/tests/`
