# mds

mds は、Markdown を設計、実装、テストの正本として扱い、Markdown に書かれた実装コードから言語ごとの派生コードを生成する開発ツールチェーンです。

## mds とは

mds は、設計文書とソースコードを別々に管理するのではなく、ひとつの Markdown 文書に目的、契約、依存関係、公開面、実装コード、テストコードをまとめます。

Markdown は単なる説明文ではありません。`Types`、`Source`、`Test` に書いたコードブロックが生成元になり、`.ts`、`.py`、`.rs` などのファイルが派生物として作られます。

## 解決したい課題

- 設計書と実装が時間とともにずれる問題を減らします。
- 実装の意図、依存関係、テスト観点をひとつの文書から確認できるようにします。
- 生成コードを手で直す運用ではなく、Markdown の正本から再生成できる状態を保ちます。
- 複数の言語や複数のパッケージを含むリポジトリでも、同じ考え方で生成と検証を扱えるようにします。

## 主な特徴

- Markdown を正本として扱います。
- ひとつの実装 Markdown は、ひとつの機能だけを担当します。
- `Expose` で公開面を明示し、`Uses` で依存関係を明示します。
- TypeScript、Python、Rust の生成を対象にします。
- `mds check` で Markdown の構造、表、生成先を検査します。
- `mds build` で Markdown から派生コードを生成します。
- `mds build --dry-run` で書き込み前に生成予定と差分を確認できます。
- 生成ファイルには mds 管理ヘッダーを付け、管理対象ではない既存ファイルの上書きを防ぎます。

## 現在の状態

このリポジトリは開発中です。

現在の中心範囲は、Markdown の解析、構造検査、TypeScript、Python、Rust の派生コード生成です。公開パッケージとしての配布、品質検査、環境診断、パッケージ情報同期、公開前検査も設計と実装の対象に含まれています。

## 動作環境

- Rust 1.86 以上
- Node.js 24 以上
- Python 3.13 以上
- npm 10 以上

利用する機能によって必要な実行環境は変わります。TypeScript、Python、Rust の検査、修正、テストで使うツールは `mds init` の言語別オプションや `mds.config.toml` で選択できます。未選択のツールは暗黙には必須になりません。

## クイックスタート

現時点では、リポジトリから Rust のコマンドとして実行する方法が最も確実です。

```bash
cd crates
cargo run -p mds-cli -- check --package ../path/to/package
```

生成予定を確認する場合は、次のように実行します。

```bash
cd crates
cargo run -p mds-cli -- build --package ../path/to/package --dry-run
```

生成ファイルを書き込む場合は、次のように実行します。

```bash
cd crates
cargo run -p mds-cli -- build --package ../path/to/package
```

## 最小構成の考え方

mds の対象パッケージには、少なくとも次の要素が必要です。

- `mds.config.toml`
- `package.md`
- `src-md` 配下の実装 Markdown
- 対象言語のパッケージ情報ファイル

実装 Markdown は、たとえば `src-md/foo/bar.ts.md`、`src-md/pkg/foo.py.md`、`src-md/foo/bar.rs.md` のような名前にします。

## コマンド概要

| コマンド | 目的 |
| --- | --- |
| `mds check` | Markdown の構造、表、設定、生成先を検査します。 |
| `mds build` | Markdown から派生コードを生成します。 |
| `mds build --dry-run` | ファイルを書き込まず、生成予定と差分を表示します。 |
| `mds lint` | Markdown 内のコードブロックを対象に検査を実行します。 |
| `mds lint --fix` | Markdown 内のコードブロックに自動修正を適用します。 |
| `mds test` | Markdown 内のテストコードを対象にテストを実行します。 |
| `mds doctor` | 実行環境と必要なツールを診断します。 |
| `mds package sync` | パッケージ情報から `package.md` の管理部分を同期します。 |
| `mds init` | mds を使うための初期化を行います。 |
| `mds release check` | 公開前の成果物検査を行います。 |

## 対応する言語と配布形態

mds は、Rust の中核処理とコマンドを中心に、複数の言語環境から利用できる形を目指しています。

| 種別 | 対象 |
| --- | --- |
| 中核処理 | Rust |
| コマンド | ネイティブ実行ファイル |
| 生成対象 | TypeScript、Python、Rust |
| 配布経路 | Cargo、npm、Python パッケージ、ネイティブ実行ファイル |

## ドキュメント

詳しい説明は日本語版の wiki にあります。

- [wiki 入口](docs/wiki/ja/index.md)
- [はじめに](docs/wiki/ja/getting-started.md)
- [基本概念](docs/wiki/ja/concepts.md)
- [Markdown 正本](docs/wiki/ja/markdown-source.md)
- [コマンド](docs/wiki/ja/commands.md)
- [設定](docs/wiki/ja/configuration.md)
- [生成の仕組み](docs/wiki/ja/generation.md)
- [AI エージェント連携](docs/wiki/ja/ai-agent-integration.md)

## AI エージェント連携

`mds init --ai` は、AI コーディングエージェント向けの設定ファイル (agent kit) を生成します。各 AI CLI のベストプラクティスに沿った形式で、mds のワークフローとマークダウンフォーマットの知識をエージェントに提供します。

### 対応 AI CLI

| AI CLI | 生成先 | 説明 |
| --- | --- | --- |
| Claude Code | `.claude/rules/`, `.claude/skills/`, `.claude/commands/` | path-scoped rules, skills, slash commands |
| Codex CLI | `.codex/instructions.md`, `.codex/skills/` | instructions, skills |
| Opencode | `.opencode/agents/`, `.opencode/skills/` | subagents, skills |
| GitHub Copilot | `.github/instructions/`, `.github/prompts/` | path-specific instructions, prompt files |

### 使い方

```bash
# 全 AI CLI 向けに全カテゴリを生成
mds init --ai --target all --categories all --yes

# Claude Code 向けのみ生成
mds init --ai --target claude-code --yes

# 特定のカテゴリのみ生成
mds init --ai --target all --categories instructions,skills --yes
```

### 設計方針

- **メインファイル非侵害**: CLAUDE.md、AGENTS.md、copilot-instructions.md などのユーザー所有ファイルは一切生成・変更しません。各 CLI のネイティブ参照パスに配置し、統合ガイドを stdout に表示します。
- **frontmatter 管理**: 生成ファイルには `mds-managed: true` を含み、再実行時に安全に更新できます。
- **データ駆動テンプレート**: `crates/mds-core/src/init/templates/` に manifest.toml とテンプレートファイルを配置するだけで新しい AI CLI を追加できます。

## コントリビューション

不具合報告、仕様の確認、ドキュメント改善、実装改善を歓迎します。

## ライセンス

MIT License です。詳しくは [LICENSE](LICENSE) を参照してください。
