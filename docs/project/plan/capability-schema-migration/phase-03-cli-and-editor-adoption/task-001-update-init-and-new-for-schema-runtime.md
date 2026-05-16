# Task 001: Update Init And New For Schema Runtime

## 目的

`mds init` wizard と `mds new` を config/schema runtime 前提へ更新する。

## 前提条件

- core が schema surface を解釈できる

## 作業内容

- wizard の quality summary / advanced を schema runtime 前提へ寄せる
- section semantic profile、link policy、schema 参照設定を init 出力へ反映する
- `new` template 起票が新 policy を使うようにする

## 完了条件

- wizard / init / new が新 runtime と矛盾しない

## 検証方法

- init plan、generated config、new template を確認する

## 依存関係

- `../../phase-02-core-runtime-migration/task-001-replace-language-and-file-rule-resolution.md`

## 成果物

- `mds/cli/src/wizard.rs`
- `mds/core/src/init/mod.rs`
- `mds/core/src/new.rs`
