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

- Rust core は言語横断の中核処理を担う。
- CLI は native binary として mds の各コマンドを提供する。
- language adapter は言語固有の import 生成、lint、format、test runner 接続、ファイル名規約、出力規則を担う。
- `index.md` は階層の設計、責務、公開面、ルールを説明する。
- `package.md` は package metadata と package 単位のルールを説明する。
- implementation md は `Purpose`、`Contract`、`Types`、`Source`、`Cases`、`Test` を持ち、`Types`、`Source`、`Test` には生成元となる実コードを置く 1 機能の正本とする。

## 設計方針

- 自由度より規約を優先する。
- Node / Rust / Python 環境を横断して使える配布形態にする。
- Obsidian でそのまま読める標準寄り Markdown を使う。
- package 単位で mds の有効 / 無効を切り替えられる monorepo 対応を前提にする。
- lint / format / test は生成後コードだけでなく、md の状態にも適用できるようにする。

## 関連資料

- `index.md`
- `validation.md`
- `tech-stack.md`
