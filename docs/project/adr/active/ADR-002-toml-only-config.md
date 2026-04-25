---
id: ADR-002-toml-only-config
status: 採用
related:
  - docs/project/requirements/REQ-config-toml-fixed-config.md
  - docs/project/specs/shared/SPEC-config-toml-resolution.md
---

# 設定ファイルを mds.config.toml に固定する

## 背景

mds は Node、Rust、Python のエコシステムを横断して利用される。

## 判断

設定ファイルは `mds.config.toml` に固定する。`mds.config.ts` や `mds.config.json` は採用しない。

## 代替案

- TypeScript config: Node 利用者には便利だが、Cargo / uv 環境で自然に読めない。
- JSON config: 広く読めるが、コメントや階層設定の表現で TOML より扱いにくい。
- 複数形式対応: 利便性は上がるが、解決順と実装が複雑になる。

## 結果

設定形式の分岐を避け、すべての runtime で同じ設定解決を目指す。

## 関連資料

- `../../requirements/REQ-config-toml-fixed-config.md`
- `../../specs/shared/SPEC-config-toml-resolution.md`
- `../../tech-stack.md`
