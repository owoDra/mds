---
id: SPEC-release-prepublish-quality
status: 採用
related:
  - docs/project/requirements/REQ-release-prepublish-quality.md
  - docs/project/requirements/REQ-platform-multi-ecosystem-distribution.md
---

# 公開前品質

## 概要

`mds release check` は現行配布経路について、publish 前に artifact、checksum、署名、SBOM、provenance、install smoke test を確認する。

## 関連要求

- `../../requirements/REQ-release-prepublish-quality.md`
- `../../requirements/REQ-platform-multi-ecosystem-distribution.md`

## 入力

- native binary artifact
- Cargo crate package
- VS Code extension package
- release metadata
- release quality manifest

## 出力

- checksum
- signature
- SBOM
- provenance / attestations
- install smoke test result
- release checklist result

## 挙動

- Cargo crate、native binary、VS Code extension package の配布経路を公開前品質の対象にする。
- 各 artifact は checksum、署名、SBOM、provenance / attestations を持つ。
- 各配布経路は install 後に `mds --version` 相当と代表 command smoke test を実行する。
- OS / architecture 別 artifact は release metadata と対応付ける。
- publish は明示承認された release flow でのみ実行する。
- `mds release check` は既定で `release.mds.toml` を読み、`--manifest <path>` で manifest path を指定できる。
- release quality manifest は `[[artifacts]]` を持ち、各 artifact は `name`、`channel`、`path`、`checksum`、`signature`、`sbom`、`provenance`、`smoke` を持つ。
- `checksum` は artifact 本体の SHA-256 と一致する digest を含む。
- `signature` は空でない署名 artifact を指す。
- `sbom` は SPDX または CycloneDX 形状の JSON を指す。
- `provenance` は JSON または JSONL の provenance / attestation を指す。

## 状態遷移 / 不変条件

- supply-chain 成果物が欠ける artifact は公開前 gate を通過できない。
- release quality は distribution と version 仕様の上位 gate として扱う。
- publish 実行はこの spec の検証対象だが、暗黙実行しない。
- `mds release check` は publish を実行しない。

## エラー / 例外

- checksum、署名、SBOM、provenance のいずれかが欠ける場合は公開前 gate を失敗させる。
- checksum が artifact 本体と一致しない場合は公開前 gate を失敗させる。
- SBOM または provenance が parse できない場合は公開前 gate を失敗させる。
- install smoke test が失敗する場合は公開前 gate を失敗させる。
- artifact と release metadata の version / compatibility mismatch は公開前 gate を失敗させる。
- registry publish に必要な credential 不足は environment 不足として扱う。
- release quality manifest が読めない、または `[[artifacts]]` がない場合は診断にする。

## 横断ルール

- `SPEC-distribution-and-versions.md` の package name と runtime version を正とする。
- release checklist は手動承認項目と自動検証項目を分ける。
- release automation を追加する場合も publish は明示承認を必要とする。
- `mds release check` の終了コードは CLI 共通の 0 / 1 / 2 / 3 / 4 体系に従う。

## 検証観点

- 全配布経路で checksum、署名、SBOM、provenance が生成・参照できることを確認する。
- checksum が artifact 本体の SHA-256 と一致することを確認する。
- SBOM が SPDX または CycloneDX JSON として読めることを確認する。
- provenance が JSON または JSONL として読めることを確認する。
- Cargo crate、native binary、VS Code extension package の install smoke test を確認する。
- OS / architecture 別 artifact と release metadata の対応を確認する。
- supply-chain 成果物欠落時に gate が失敗することを確認する。

## 関連資料

- `../../requirements/REQ-release-prepublish-quality.md`
- `../../requirements/REQ-platform-multi-ecosystem-distribution.md`
- `SPEC-distribution-and-versions.md`
- `../../validation.md`
