# Task 002: Complete Quality And Doctor Policy

## 目的

quality slot、diagnostic capture rule、doctor tool/version policy を config/schema 起点へ揃え、diagnostic remap を v1 契約どおり維持する。

## 前提条件

- language / schema runtime が core で解釈できる
- `SPEC-core-quality-and-fix-pipeline` と `SPEC-cli-doctor-and-update` の契約が参照できる

## 作業内容

- `typecheck` `lint` `fix` `test` slot の command 解決を config、package scripts、schema から行う runtime を整える
- diagnostic capture rule を config/schema から読めるようにする
- doctor の required / optional tool と version floor 判定を config 起点へ寄せる
- source map 前提の diagnostic remap を新 runtime でも壊さない test を追加する

## 完了条件

- quality slot と doctor policy が hardcode 依存ではなく package policy で説明できる
- capture rule の違う fixture でも remap が同じ契約で動く
- doctor が required / optional / version floor を config 起点で返せる

## 検証方法

- `mds/core` の quality / doctor / diagnostics test を実行する
- success-path fixture と broken/remap fixture で remap 確認を行う

## 依存関係

- `task-001-complete-language-and-schema-resolution.md`
- `../../capability-schema-migration/phase-02-core-runtime-migration/task-002-replace-quality-slot-and-capture-resolution.md`
- `../../capability-schema-migration/phase-02-core-runtime-migration/task-003-preserve-diagnostic-remap-with-source-map.md`

## 成果物

- `mds/core/src/quality.rs`
- `mds/core/src/doctor.rs`
- `mds/core/src/runner.rs`
- `mds/core/src/diagnostics.rs`
- `mds/core/tests/`
