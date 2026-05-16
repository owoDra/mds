# Phase 05: Fixtures And Exit Validation

## 目的

`examples/`、live regression、validation、docs closure を揃え、v1 完成を受け入れ可能な状態まで持っていく。

## 前提条件

- core、CLI、LSP、VS Code の主要 spec 差分が解消されている
- `examples/` を横断確認資産として扱う前提が固まっている

## 完了条件

- `minimal-ts` が success-path fixture として build / package sync / representative quality を成功できる
- broken/remap fixture を使う live regression が追加される
- validation と docs 参照が v1 完成状態に追従する

## 検証方法

- Rust workspace test/build、VS Code compile、example command、package sync、diagnostic remap をまとめて確認する
- docs の参照整合と plan / spec / validation の閉じ方を確認する

## task 一覧

1. 以下は前 phase 完了後に並行着手できる
   - `task-001-align-minimal-ts-fixture.md`: `minimal-ts` を v1 success-path fixture へ揃える
   - `task-002-add-broken-remap-fixture-and-live-regressions.md`: broken/remap fixture と live regression を追加する
2. `task-003-run-v1-exit-validation-and-close-docs.md`: 最終 validation と docs closure を行う

## 依存関係

- `../phase-02-cli-authoring-entrypoints/index.md`
- `../phase-04-vscode-embedded-experience/index.md`
- `../../../specs/examples/SPEC-examples-v1-regression-fixtures.md`
- `../../../specs/examples/SPEC-examples-minimal-ts-fixture.md`
- `../../../validation.md`

## 参照

1. 以下は前 phase 完了後に並行着手できる
   - `task-001-align-minimal-ts-fixture.md`: `minimal-ts` success-path fixture の完成
   - `task-002-add-broken-remap-fixture-and-live-regressions.md`: broken/remap fixture と live regression の完成
2. `task-003-run-v1-exit-validation-and-close-docs.md`: 最終 validation と docs closure
