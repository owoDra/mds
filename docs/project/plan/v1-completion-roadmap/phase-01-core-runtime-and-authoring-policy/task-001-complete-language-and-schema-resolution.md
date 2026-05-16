# Task 001: Complete Language And Schema Resolution

## 目的

`mds-core` が language identity、output rule、special file rule、doc profile 判定を built-in 前提ではなく config/schema runtime から決定できる状態にする。

## 前提条件

- `SPEC-language-extension-contract` と `SPEC-core-config-and-authoring-policy` の契約が参照できる
- `capability-schema-migration` の architecture / runtime 差分が整理されている

## 作業内容

- capability schema 参照方法と config loader の責務境界を `mds-core` で実装する
- file suffix、fence label、package config / schema から language identity を決定する runtime を整える
- output rule、special file rule、root module / prose-only doc profile 判定を config/schema へ寄せる
- LSP / CLI が再利用する core API を built-in 依存から切り離す

## 完了条件

- language / output / special file / doc profile 解決が config/schema 由来で決定できる
- 既存 TS fixture が新 runtime でも build / parse できる
- 将来の language 追加に built-in 追加が必須でない責務境界が code 上で明確である

## 検証方法

- `mds/core` の config / package / markdown / generation 系 test を実行する
- `examples/minimal-ts` で dry-run build と通常 build を確認する

## 依存関係

- `../../capability-schema-migration/phase-01-architecture-and-schema/task-001-freeze-capability-schema-surface.md`
- `../../capability-schema-migration/phase-02-core-runtime-migration/task-001-replace-language-and-file-rule-resolution.md`

## 成果物

- `mds/core/src/config.rs`
- `mds/core/src/model.rs`
- `mds/core/src/descriptor.rs`
- `mds/core/src/markdown.rs`
- `mds/core/src/generation.rs`
- `mds/core/tests/`
