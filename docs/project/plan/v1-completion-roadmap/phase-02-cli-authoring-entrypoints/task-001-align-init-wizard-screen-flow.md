# Task 001: Align Init Wizard Screen Flow

## 目的

`mds init` wizard を v1 spec の Welcome / policy / optional branch / confirm 構造へ揃える。

## 前提条件

- core が section profile、link policy、quality slot summary を返せる
- `SPEC-cli-init-wizard-screen-flow` の画面順と入力責務が確定している

## 作業内容

- Welcome、Section Profile Preset、Custom Section Labels、Link Policy、Quality Summary、Quality Advanced、AI Kit 系、Confirm 画面を実装または再構成する
- quality setup を tool 名入力中心から slot semantic summary 中心へ置き換える
- Confirm に差分要約、link policy、quality summary、AI kit 有無を集約する

## 完了条件

- wizard が spec の画面順と条件分岐を満たす
- 通常フローで raw tool / raw section title 入力を強制しない
- 生成される plan / config / template が wizard の選択結果と整合する

## 検証方法

- wizard screen 遷移 test または snapshot を追加する
- 手動確認で cancel、default flow、custom label、advanced quality flow を確認する

## 依存関係

- `../phase-01-core-runtime-and-authoring-policy/task-001-complete-language-and-schema-resolution.md`
- `../phase-01-core-runtime-and-authoring-policy/task-002-complete-quality-and-doctor-policy.md`
- `../phase-01-core-runtime-and-authoring-policy/task-003-enforce-link-policy-and-overview-contract.md`

## 成果物

- `mds/cli/src/wizard.rs`
- `mds/cli/src/main.rs`
- `mds/cli/tests/wizard_test.rs`
