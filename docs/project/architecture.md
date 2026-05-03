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
- mds 自身の package 編集入口は各 package の `.mds/source/` と `.mds/test/` とし、`.build/` は生成物置き場として Git 管理しない。
- 1 つの implementation md は 1 機能だけを扱う。
- import / use / require はコードブロック外の `Uses` に記録し、language adapter が生成する。
- implementation md の code block は import / use / require を含めず、doc comment / docstring を持たず、default では 1 code block につき 1 top-level logical unit だけを含める。
- `mds.config.toml` の `[check]` は validator の有効 / 無効を切り替えられるが、正本の意味や canonical 構造は変更しない。
- 設定ファイルは `mds.config.toml` 固定とし、セクションの意味や必須構造は設定で変更しない。
- `Expose` は公開面を示し、`Uses` は依存を示す。

## 責務分離

- root はプロジェクト全体の入口とし、言語ごとの workspace / distribution metadata は各言語ディレクトリに閉じる。
- Rust core は言語横断の中核処理を担う。
- CLI は native binary として mds の各コマンドを提供する。
- language adapter は言語固有の import 生成、lint、lint --fix、test runner 接続、ファイル名規約、出力規則を担う。
- npm wrapper は native CLI の配布と起動だけを担い、Markdown model や core の意味体系を変更しない。
- `mds init` は project 初期化、AI agent kit 生成、開発環境セットアップの入口を担う。
- AI CLI template plugin は AI CLI 固有の instruction、skill、command、workflow、docs 生成差分を担い、任意コマンド実行は行わない。
- package root に mds 用の `index.md` は置かず、package metadata は `Cargo.toml`、`package.json`、`pyproject.toml` などの実体 metadata を正とする。
- source root の `overview.md` は source hierarchy と package 単位の設計、責務、公開面、ルールを説明する。
- implementation md は `Purpose`、`Contract`、`Types`、`Source`、`Cases`、`Test` を持ち、`Types`、`Source`、`Test` には生成元となる実コードを置く 1 機能の正本とする。

## Workspace 構成

- root 直下に mds 自身の source root は置かず、各 package の `.mds/source/overview.md` が package 単位の source overview を担う。
- `mds/core/.mds/source`、`mds/cli/.mds/source`、`mds/lsp/.mds/source` と対応する `.mds/test` は Rust 実装の Markdown 正本であり、`cargo run -p mds-cli -- build --verbose` は package 内の生成 `src/` / `tests/` を更新し、repo 内の `.build/rust/` self-hosted mirror も同じ command で再生成する。
- special file は descriptor の出力規則を優先し、たとえば Rust の `build.rs.md` は `src/build.rs` ではなく package root の `build.rs` に生成する。
- `.build/rust/Cargo.toml` は `mds build` が生成する self-hosted Rust workspace manifest とし、`.build/rust/mds/core`、`.build/rust/mds/cli`、`.build/rust/mds/lsp` を束ねる。
- TypeScript / Python / Rust の language adapter 規則は現時点では Rust core 側の生成処理と共有仕様で管理し、独立した `packages/`、`python/`、`mds-lang-rs` 配布単位は置かない。
- `editors/vscode` は VS Code 拡張とし、syntax highlighting、LSP 連携、snippets を提供する。
- `.agents/` は AI 専用の制約、作業手順、skill、task 文脈キャッシュを置き、`docs/project/` は人間向け正本を置く。

## 設計方針

- 自由度より規約を優先する。
- Node / Rust / Python 環境を横断して使える配布形態にする。
- Obsidian でそのまま読める標準寄り Markdown を使う。
- package 単位で mds の有効 / 無効を切り替えられる monorepo 対応を前提にする。
- lint / lint --fix / test は生成後コードだけでなく、md の状態にも適用できるようにする。
- package manager hook や registry publish のような外部影響が大きい処理は、既定で暗黙実行せず明示有効化を前提にする。
- 開発環境セットアップで project dependencies、toolchains、global AI CLI を導入する場合は interactive default とし、非対話実行では明示 option がある場合だけ変更する。
- 公開前品質では全配布経路について checksum、署名、SBOM、provenance、install smoke test を release gate として扱う。
- ビルド、テスト、release の派生物は `.build/` 配下に集約し、正本と生成物の境界を保つ。

## 関連資料

- `index.md`
- `validation.md`
- `tech-stack.md`
- `adr/active/ADR-007-self-hosted-src-md-build.md`
- `adr/active/ADR-006-ai-agent-init-and-dev-setup.md`
