# Task 001: Freeze Capability Schema Surface

## 目的

language identity、output rule、special file rule、quality slot、diagnostic capture rule を表現する最小 schema surface を確定する。

## 前提条件

- proposal が active に存在する
- shared/core spec の更新方針が合意済み

## 作業内容

- capability schema で表現する項目を列挙する
- package config に残す項目と外部 schema に出す項目を分ける
- shared/core spec を必要に応じて更新する

## 完了条件

- schema の最小項目一覧が正本に反映されている
- language/tool 固有の built-in 前提が削減されている

## 検証方法

- shared/core spec を読み、language identity、output、capture の責務が一貫していることを確認する

## 依存関係

- `../index.md`
- `../../../proposals/active/proposal-capability-schema-runtime.md`

## 成果物

- `docs/project/specs/shared/SPEC-language-extension-contract.md`
- `docs/project/specs/mds-core/SPEC-core-config-and-authoring-policy.md`
