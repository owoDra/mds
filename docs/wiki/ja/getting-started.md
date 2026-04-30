# はじめに

このページでは、mds を試すために必要な前提と、基本的な実行手順を説明します。

## インストール

インストールスクリプトでインストールします（推奨）:

```bash
curl -fsSL https://raw.githubusercontent.com/owo-x-project/owox-mds/main/install.sh | sh
```

`mds` と `mds-lsp` が `~/.local/bin` にインストールされます。

### VSCode 拡張

Marketplace で **"mds"** を検索するか、以下のコマンドでインストールできます。

```bash
code --install-extension owo-x-project.mds
```

## 前提

mds は開発中のツールです。現在アルファ版として公開されています。

## 必要な実行環境

ランタイム依存なし — mds は単一の静的バイナリです。

| 用途 | 必要なもの |
| --- | --- |
| mds コマンドの実行 | なし（ビルド済みバイナリ） |
| TypeScript の検査、修正、テスト | Node.js 24 以上と、選択した ESLint、Prettier、Biome、Vitest、Jest など |
| Python の検査、修正、テスト | Python 3.13 以上と、選択した Ruff、Black、Pytest、unittest など |
| Rust の検査、修正、テスト | Rust 1.86 以上、Cargo と、選択した rustfmt、Clippy、cargo-nextest など |

`mds check` と `mds build` は、Markdown の構造と生成を扱います。`mds lint` や `mds test` は、対象言語ごとに選択した検査ツールやテスト実行ツールを利用します。未選択のツールは暗黙には必須になりません。

## 最小構成

mds の対象パッケージには、次のファイルを用意します。

| ファイル | 役割 |
| --- | --- |
| `mds.config.toml` | mds の有効化、入力元、出力先、言語アダプターを設定します。 |
| `package.md` | パッケージ名、依存関係、パッケージ単位のルールを説明します。 |
| `src-md/**/*.ts.md` | TypeScript の実装 Markdown です。 |
| `src-md/**/*.py.md` | Python の実装 Markdown です。 |
| `src-md/**/*.rs.md` | Rust の実装 Markdown です。 |
| `package.json`、`pyproject.toml`、`Cargo.toml` | 対象言語のパッケージ情報です。 |

すべての言語を同時に使う必要はありません。対象にする言語だけを有効にします。

## 基本的な流れ

まず、対象パッケージの構造を検査します。

```bash
mds check --package ./path/to/package
```

次に、生成予定と差分を確認します。

```bash
mds build --package ./path/to/package --dry-run
```

問題がなければ、派生コードを書き込みます。

```bash
mds build --package ./path/to/package
```

## 生成されるもの

実装 Markdown の `Types`、`Source`、`Test` に書いたコードブロックから、対象言語のファイルが生成されます。

たとえば `src-md/foo/bar.ts.md` は、既定では次のようなファイルに対応します。

| 種別 | 生成先の例 |
| --- | --- |
| `Source` | `src/foo/bar.ts` |
| `Types` | `src/foo/bar.types.ts` |
| `Test` | `tests/foo/bar.test.ts` |

生成先の詳細は、[生成の仕組み](generation.md)を参照してください。

## 次に読むページ

- [基本概念](concepts.md)
- [Markdown 正本](markdown-source.md)
- [コマンド](commands.md)
- [設定](configuration.md)
