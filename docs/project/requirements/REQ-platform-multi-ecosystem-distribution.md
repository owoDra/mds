---
id: REQ-platform-multi-ecosystem-distribution
status: 採用
related:
  - README.md
  - docs/project/architecture.md
---

# 配布

## 目標

mds は GitHub Releases の native binary archive と VS Code 拡張を通じて導入して利用できること。

## 根拠

Markdown 正本から多言語コード、型、テストを生成するため、CLI 本体は native binary として提供し、editor integration は VS Code 拡張として提供する必要があるため。

## 対象範囲

- GitHub Releases の native binary archive から `mds` と `mds-lsp` を導入できること
- VS Code 拡張を導入でき、通常は同封済み `mds-lsp` で LSP 機能を利用できること
- Rust core、native CLI、LSP、language adapter 規則の責務を分離すること

## 対象外

- npm package として配布すること
- `mds-core` を standalone crate として配布すること
- `mds-cli` / `mds-lsp` を crates.io package として配布すること
- Python package または uv / uvx entrypoint として配布すること
- GUI application として配布すること

## 成功指標

- GitHub Releases binary / install script / VS Code 拡張の導入口が文書化されている
- CLI が native binary として動作する
- VS Code 拡張が同封済み `mds-lsp` で起動できる
- TypeScript、Python、Rust の代表プロジェクトで同じ mds 概念を利用できる

## 制約 / 品質条件

- language 固有の差分は adapter 境界に閉じ込める
- 言語横断の中核契約は Rust core 側で一貫させる

## 関連資料

- `../../README.md`
- `../architecture.md`
