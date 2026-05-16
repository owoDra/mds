# Phase 01: Core Runtime And Authoring Policy

## 目的

shared / `mds-core` spec が要求する config/schema runtime、authoring policy、overview special file 契約を `mds-core` で成立させる。

## 前提条件

- `docs/project/specs/shared/` と `docs/project/specs/mds-core/` の v1 個票が揃っている
- `../../capability-schema-migration/index.md` の差分整理が参照できる

## 完了条件

- language identity、output rule、quality slot、capture rule、doctor policy が config/schema 起点で説明できる
- link policy 3 mode と `lint --fix` 正規化が package policy として成立する
- `overview.md` special file と `package sync` managed region 契約が spec どおり成立する

## 検証方法

- `mds/core` の unit / integration test で config、quality、package sync、remap を確認する
- `examples/minimal-ts` を使う build / lint / package sync の代表確認で policy 反映を確認する

## task 一覧

1. `task-001-complete-language-and-schema-resolution.md`: language、output、special file、doc profile 解決を config/schema runtime へ揃える
2. `task-002-complete-quality-and-doctor-policy.md`: quality slot、capture rule、doctor policy、remap 契約を config/schema 起点へ揃える
3. `task-003-enforce-link-policy-and-overview-contract.md`: link policy 正規化と `overview.md` / `package sync` 契約を完成させる

## 依存関係

- `../index.md`
- `../../capability-schema-migration/index.md`
- `../../../specs/shared/SPEC-model-package-layout.md`
- `../../../specs/shared/SPEC-authoring-markdown-format.md`
- `../../../specs/shared/SPEC-language-extension-contract.md`
- `../../../specs/mds-core/SPEC-core-config-and-authoring-policy.md`
- `../../../specs/mds-core/SPEC-core-overview-and-package-sync.md`
- `../../../specs/mds-core/SPEC-core-quality-and-fix-pipeline.md`

## 参照

1. `task-001-complete-language-and-schema-resolution.md`: language / output / special file rule の runtime 置換
2. `task-002-complete-quality-and-doctor-policy.md`: quality / doctor / remap policy の runtime 置換
3. `task-003-enforce-link-policy-and-overview-contract.md`: link policy と overview special file 契約の完成
