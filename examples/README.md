# mds サンプルプロジェクト

このディレクトリには、mds の動作を確認するためのサンプルプロジェクトが含まれています。

## サンプル一覧

| ディレクトリ | 内容 | v1 位置づけ |
| --- | --- | --- |
| `minimal-ts` | TypeScript の最小構成 | 必須回帰 fixture |

## 使い方

```bash
cd <repo-root>

# 構造検査
cargo run --manifest-path Cargo.toml -p mds-cli -- lint --package examples/minimal-ts

# 生成プレビュー
cargo run --manifest-path Cargo.toml -p mds-cli -- build --package examples/minimal-ts --dry-run

# 生成実行
cargo run --manifest-path Cargo.toml -p mds-cli -- build --package examples/minimal-ts
```

## サンプルの構成

v1 では `minimal-ts` を必須回帰 fixture として扱います。

- `mds.config.toml` — mds の設定ファイル
- `package.json` / `pyproject.toml` / `Cargo.toml` — 言語のパッケージ情報
- `.mds/source/overview.md` — source root の overview
- `.mds/source/*.lang.md` — tableless source Markdown
- `.mds/test/*.lang.md` — tableless test Markdown

生成後、`src/` と `tests/` に `[output]` / `[[output.override]]` に従った派生コードが作られます。

Python / Rust examples は v1 spec の対象外で、今後整理予定です。
