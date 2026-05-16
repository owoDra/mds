# Task 001: Align Minimal Ts Fixture

## 目的

`examples/minimal-ts` を v1 の正式 success-path fixture として、quality scripts、managed region、generated output 比較が成立する状態にする。

## 前提条件

- core / CLI の policy と template 契約が固まっている
- `SPEC-examples-minimal-ts-fixture` の要求が参照できる

## 作業内容

- `package.json` に `typecheck` `lint` `format` `test` scripts と必要 dependency snapshot を追加する
- `mds.config.toml` の quality slot を scripts と整合する最小値へ揃える
- `overview.md` に `Package Summary` `Dependencies` `Dev Dependencies` の managed region を正式配置する
- generated source/test output と manifest の期待比較が安定するよう fixture を整える

## 完了条件

- `minimal-ts` が build / package sync / representative quality command を成功できる
- source/test Markdown、generated output、manifest、overview managed region の対応が説明できる
- fixture が success-path 以外の検証軸で肥大化していない

## 検証方法

- `mds build --package examples/minimal-ts`
- `mds package sync --package examples/minimal-ts --check`
- representative quality command 実行

## 依存関係

- `../phase-01-core-runtime-and-authoring-policy/task-003-enforce-link-policy-and-overview-contract.md`
- `../phase-02-cli-authoring-entrypoints/task-002-align-new-command-and-templates.md`

## 成果物

- `examples/minimal-ts/package.json`
- `examples/minimal-ts/mds.config.toml`
- `examples/minimal-ts/.mds/source/overview.md`
- `examples/minimal-ts/src/`
- `examples/minimal-ts/tests/`
- `examples/minimal-ts/.mds/manifest.toml`
