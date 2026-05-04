# Task

## 目的

release.mds.toml に基づく公開前品質 gate の artifact 生成・検証パイプラインを実装する。

## 状態

not_started

## 依頼内容

alpha release scaffold として `release.mds.toml` は作成済み。以下を後続で実装する:

1. checksum 生成 (SHA-256) と artifact 本体との照合
2. 署名 artifact の生成（GPG or sigstore）
3. SBOM 生成（SPDX or CycloneDX JSON）
4. provenance / attestations 生成
5. install smoke test の自動実行
6. `mds release check` の結果が上記を検証して gate 判定する

## 確定前提

- `release.mds.toml` の `[[artifacts]]` 構造は alpha scaffold に従う。
- `mds release check` コマンドの基本構造は既に存在する。
- publish 実行はこの task の対象外。

## 未確定事項

- 署名方式（GPG vs sigstore）は未決定。
- CI/CD pipeline の具体的な実装先は未指定。

## 対象範囲

- `crates/mds/core/src/release_quality/`
- `release.mds.toml`
- CI/CD pipeline (未作成)

## 対象外

- 実 publish。
- registry credential の管理。

## 守るべき不変条件

- 公開前品質では全配布経路について checksum、署名、SBOM、provenance、install smoke test を release gate として扱う。
- publish は明示承認された release flow でのみ実行する。

## 参照する正本

- `docs/project/requirements/REQ-release-prepublish-quality.md`
- `docs/project/specs/shared/SPEC-release-prepublish-quality.md`
- `docs/project/specs/shared/SPEC-distribution-and-versions.md`

## 今回読まなくてよい資料

- `docs/project/adr/archive/`

## 実施方針

- 作業モードは自走。

## 実施手順

1. `mds release check` の現行実装を確認する。
2. checksum 検証ロジックを追加する。
3. SBOM / provenance の parse と検証を追加する。
4. install smoke test の自動実行を追加する。
5. 全体を統合テストで確認する。

## 検証項目

- 全 artifact に checksum が生成・照合される。
- supply-chain 成果物欠落時に gate が失敗する。
- install smoke test が自動実行される。

## 完了条件

- 全 artifact に checksum、署名、SBOM、provenance が生成される。
- `mds release check` がこれらを検証し、欠落時に gate 失敗を返す。
- install smoke test が自動実行される。

## 進捗記録

- (未着手)

## 次に読むもの

- `docs/project/specs/shared/SPEC-release-prepublish-quality.md`
