---
id: REQ-platform-multi-ecosystem-distribution
status: 採用
related:
  - README.md
  - docs/project/architecture.md
---

# マルチエコシステム配布

## 目標

mds は npm 専用ではなく、Node、Rust、Python の各エコシステムから導入して利用できること。

## 根拠

Markdown 正本から多言語コード、型、テストを生成するため、利用環境を Node に閉じない必要があるため。

## 対象範囲

- npm から CLI と language adapter を導入できること
- Cargo から native CLI を導入できること
- uv または uvx から Python 利用者が CLI を導入できること
- Rust core、native CLI、language adapter の責務を分離すること

## 対象外

- npm だけで全機能を完結させる前提
- 各エコシステムの配布名を完全に同一にすること
- GUI または IDE 拡張としての配布

## 成功指標

- npm、Cargo、uv の導入口が文書化されている
- CLI が native binary として動作する
- TypeScript、Python、Rust の代表プロジェクトで同じ mds 概念を利用できる

## 制約 / 品質条件

- エコシステム固有の差分は adapter または配布パッケージに閉じ込める
- 言語横断の中核契約は Rust core 側で一貫させる

## 関連資料

- `../../README.md`
- `../architecture.md`
