---
id: REQ-release-prepublish-quality
status: 採用
related:
  - docs/project/requirements/REQ-platform-multi-ecosystem-distribution.md
  - mds/core/.mds/source/release_quality.rs.md
---

# 公開前品質

## 目標

mds は Cargo crate、native binary、VS Code extension の配布経路について、公開前に artifact、checksum、署名、SBOM、provenance、install smoke test を確認できること。

## 根拠

mds は native CLI と editor extension として配布されるため、利用者が取得した binary / package の完全性、互換性、実行可能性を確認できる必要があるため。

## 対象範囲

- Cargo crate、native binary、VS Code extension package の公開前検証
- OS / architecture 別 artifact の一覧、checksum、署名、SBOM、provenance / attestations
- install 後 smoke test と CLI の exit code / stdout / stderr 確認
- release checklist と publish 前 gate

## 対象外

- この要件だけで registry publish を実行すること
- 手動承認なしに release automation を本番実行すること
- mds と無関係な第三者 artifact の検証

## 成功指標

- 現行配布経路で公開前 smoke test が実行できる
- artifact ごとに checksum、署名、SBOM、provenance が紐づく
- artifact と release metadata の version / compatibility mismatch を公開前に検出できる

## 制約 / 品質条件

- publish は明示承認された release flow でのみ実行する
- supply-chain 成果物が欠ける場合は公開前 gate を失敗させる
- 公開前品質の仕様は distribution 仕様と矛盾しない

## 関連資料

- `REQ-platform-multi-ecosystem-distribution.md`
- `../../../mds/core/.mds/source/release_quality.rs.md`
- `../tech-stack.md`
- `../adr/active/ADR-006-ai-agent-init-and-dev-setup.md`
