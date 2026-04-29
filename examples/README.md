# mds サンプルプロジェクト

このディレクトリには、mds の動作を確認するためのサンプルプロジェクトが含まれています。

## サンプル一覧

| ディレクトリ | 内容 |
| --- | --- |
| [minimal-ts](minimal-ts/) | TypeScript の最小構成 |
| [minimal-py](minimal-py/) | Python の最小構成 |
| [minimal-rs](minimal-rs/) | Rust の最小構成 |

## 使い方

```bash
cd crates

# 構造検査
cargo run -p mds-cli -- check --package ../examples/minimal-ts

# 生成プレビュー
cargo run -p mds-cli -- build --package ../examples/minimal-ts --dry-run

# 生成実行
cargo run -p mds-cli -- build --package ../examples/minimal-ts
```

## サンプルの構成

各サンプルプロジェクトは、mds の最小構成を示しています。

- `mds.config.toml` — mds の設定ファイル
- `package.md` — パッケージ情報
- `package.json` / `pyproject.toml` / `Cargo.toml` — 言語のパッケージ情報
- `src-md/index.md` — 実装 Markdown の入口
- `src-md/*.lang.md` — 実装 Markdown

生成後、`src/` と `tests/` に派生コードが作られます。
