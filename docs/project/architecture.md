# Architecture

## 目的

このファイルは、プロジェクト全体で守る不変条件、責務分離、設計方針を定義します。

## 読むべき場面

- 共通原則を変えるとき
- 責務境界を見直すとき
- 仕様や実装に横断影響があるとき

## 不変条件

- `.md` が設計書兼ソースの正本であり、implementation md には実装レベルのコードを含め、生成コードはその派生物とする。
- mds は設計説明から AI にコードを書かせる仕組みではなく、Markdown 内のコードブロックとメタ情報を generator / language adapter が処理する仕組みとする。
- 1 つの implementation md は 1 機能だけを扱う。
- import / use / require はコードブロック外の `Uses` に記録し、language adapter が生成する。
- 設定ファイルは `mds.config.toml` 固定とし、セクションの意味や必須構造は設定で変更しない。
- `Expose` は公開面を示し、`Uses` は依存を示す。

## 責務分離

- root はプロジェクト全体の入口とし、言語ごとの workspace / distribution metadata は各言語ディレクトリに閉じる。
- Rust core は言語横断の中核処理を担う。
- CLI は native binary として mds の各コマンドを提供する。
- language adapter は言語固有の import 生成、lint、lint --fix、test runner 接続、ファイル名規約、出力規則を担う。
- npm wrapper は native CLI の配布と起動だけを担い、Markdown model や core の意味体系を変更しない。
- `index.md` は階層の設計、責務、公開面、ルールを説明する。
- `package.md` は package metadata と package 単位のルールを説明する。
- implementation md は `Purpose`、`Contract`、`Types`、`Source`、`Cases`、`Test` を持ち、`Types`、`Source`、`Test` には生成元となる実コードを置く 1 機能の正本とする。

## Workspace 構成

- `crates/Cargo.toml` は Rust workspace manifest とし、`crates/mds-core`、`crates/mds-cli`、`crates/mds-lang-rs` を束ねる。
- `packages/package.json` は npm workspace manifest とし、pnpm ではなく npm 10+ workspaces で `packages/*` を束ねる。
- `packages/cli` は npm wrapper package とし、`packages/core` と `packages/lang-*` は実装 entrypoint を持つまでは private placeholder metadata とする。
- `python/mds_cli` は Python CLI wrapper distribution、`python/mds_lang_py` は Python language adapter distribution とする。
- `.agents/` は AI 専用の制約、作業手順、skill、task 文脈キャッシュを置き、`docs/project/` は人間向け正本を置く。

## 設計方針

- 自由度より規約を優先する。
- Node / Rust / Python 環境を横断して使える配布形態にする。
- Obsidian でそのまま読める標準寄り Markdown を使う。
- package 単位で mds の有効 / 無効を切り替えられる monorepo 対応を前提にする。
- lint / lint --fix / test は生成後コードだけでなく、md の状態にも適用できるようにする。
- package manager hook や registry publish のような外部影響が大きい処理は、既定で暗黙実行せず明示有効化を前提にする。

## 関連資料

- `index.md`
- `validation.md`
- `tech-stack.md`
