---
id: SPEC-release-prepublish-quality
status: 採用
related:
  - docs/project/requirements/REQ-release-prepublish-quality.md
  - docs/project/requirements/REQ-platform-multi-ecosystem-distribution.md
---

# 公開前品質

## 概要

mds は全配布経路について、publish 前に artifact、checksum、署名、SBOM、provenance、install smoke test を確認する。

## 関連要求

- `../../requirements/REQ-release-prepublish-quality.md`
- `../../requirements/REQ-platform-multi-ecosystem-distribution.md`

## 入力

- native binary artifact
- npm package
- Cargo crate package
- Python package / uvx entrypoint
- wrapper package
- release metadata

## 出力

- checksum
- signature
- SBOM
- provenance / attestations
- install smoke test result
- release checklist result

## 挙動

- npm、Cargo、Python / uvx、native binary の全配布経路を公開前品質の対象にする。
- 各 artifact は checksum、署名、SBOM、provenance / attestations を持つ。
- 各配布経路は install 後に `mds --version` 相当と代表 command smoke test を実行する。
- npm / Python wrapper は同梱 binary または解決された native CLI を起動し、exit code / stdout / stderr 規則が native CLI と一致することを確認する。
- OS / architecture 別 artifact は release metadata と対応付ける。
- publish は明示承認された release flow でのみ実行する。

## 状態遷移 / 不変条件

- supply-chain 成果物が欠ける artifact は公開前 gate を通過できない。
- wrapper は core の意味体系を変更しない。
- release quality は distribution と version 仕様の上位 gate として扱う。
- publish 実行はこの spec の検証対象だが、暗黙実行しない。

## エラー / 例外

- checksum、署名、SBOM、provenance のいずれかが欠ける場合は公開前 gate を失敗させる。
- install smoke test が失敗する場合は公開前 gate を失敗させる。
- wrapper と native CLI の version / compatibility mismatch は公開前 gate を失敗させる。
- registry publish に必要な credential 不足は environment 不足として扱う。

## 横断ルール

- `SPEC-distribution-and-versions.md` の package name、runtime version、wrapper 契約を正とする。
- release checklist は手動承認項目と自動検証項目を分ける。
- release automation を追加する場合も publish は明示承認を必要とする。

## 検証観点

- 全配布経路で checksum、署名、SBOM、provenance が生成・参照できることを確認する。
- npm、Cargo、Python / uvx、native binary の install smoke test を確認する。
- OS / architecture 別 artifact と release metadata の対応を確認する。
- supply-chain 成果物欠落時に gate が失敗することを確認する。

## 関連資料

- `../../requirements/REQ-release-prepublish-quality.md`
- `../../requirements/REQ-platform-multi-ecosystem-distribution.md`
- `SPEC-distribution-and-versions.md`
- `../../validation.md`
