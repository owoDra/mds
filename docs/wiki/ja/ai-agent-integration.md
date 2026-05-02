# AI エージェント連携

このページでは、mds の AI コーディングエージェント向け設定ファイル (agent kit) の仕組みと、新しい AI CLI 向けテンプレートの追加方法を説明します。

## 概要

`mds init --ai` は、AI コーディングエージェントが mds プロジェクトで正しく作業できるように、各 AI CLI のベストプラクティスに沿った設定ファイルを生成します。

生成されるファイルには、mds のマークダウンフォーマット仕様 (Uses テーブル、セクション構造、制約) が含まれており、AI エージェントが正しい形式で実装マークダウンを作成できるようになります。

## 対応 AI CLI

| AI CLI | 指定名 | 生成先 | 特徴 |
| --- | --- | --- | --- |
| Claude Code | `claude-code`, `claude` | `.claude/rules/`, `.claude/skills/`, `.claude/commands/` | path-scoped rules で自動読み込み、skill でオンデマンド参照、slash commands |
| Codex CLI | `codex-cli`, `codex` | `.codex/instructions.md`, `.codex/skills/` | AGENTS.md 補完用の instructions と skill |
| Opencode | `opencode` | `.opencode/agents/`, `.opencode/skills/` | subagent 定義 (build/check)、YAML frontmatter 付き skill |
| GitHub Copilot | `github-copilot-cli`, `copilot` | `.github/instructions/`, `.github/prompts/` | applyTo frontmatter 付き path-specific instructions、prompt files |

## 使い方

```bash
# 全 AI CLI 向けに全カテゴリを生成 (計画の表示のみ)
mds init --ai --target all --categories all

# 計画を適用
mds init --ai --target all --categories all --yes

# Claude Code 向けのみ
mds init --ai --target claude-code --yes

# 特定カテゴリのみ
mds init --ai --target all --categories instructions,skills --yes
```

### カテゴリ

| カテゴリ | 説明 |
| --- | --- |
| `instructions` | AI CLI のルールファイル。mds のワークフローとマークダウンフォーマットを記載 |
| `skills` | オンデマンドで参照される詳細なスキル定義 |
| `commands` | 即座に実行可能なコマンド定義 (mds check, mds build 等) |

### オプション

| フラグ | 説明 |
| --- | --- |
| `--ai` | AI 初期化のみ実行 (プロジェクト初期化をスキップ) |
| `--target <list>` | 対象 AI CLI をカンマ区切りで指定。`all` で全対象 |
| `--categories <list>` | 生成カテゴリをカンマ区切りで指定。`all` で全カテゴリ |
| `--yes` | 計画を実際に適用する |
| `--force` | 非管理ファイルの上書きを許可する |

## 設計方針

### メインファイル非侵害

CLAUDE.md、AGENTS.md、copilot-instructions.md などの**ユーザー所有ファイルは一切生成・変更しません**。各 CLI のネイティブ参照パスに配置し、生成後にメインファイルへの統合方法をガイド表示します。

### frontmatter 管理

生成ファイルには YAML frontmatter に `mds-managed: true` が含まれます。これにより:

- `mds init` 再実行時に安全に更新できる
- 非管理ファイル (手動作成) との区別が明確
- `--force` なしでは非管理ファイルを上書きしない

### 各 AI CLI のネイティブ形式

テンプレートは各 AI CLI のベストプラクティスに従います:

- **Claude Code**: `.claude/rules/` の path-scoped rules (frontmatter の `paths` で対象ファイルを指定)
- **Opencode**: `.opencode/agents/` の subagent 定義 (frontmatter の `mode`, `tools` で権限制御)
- **GitHub Copilot**: `.github/instructions/` の path-specific instructions (frontmatter の `applyTo` で対象指定)
- **Codex CLI**: `.codex/` の instructions と skills

## 新しい AI CLI の追加方法 (mds 開発者向け)

mds はデータ駆動のテンプレートシステムを採用しています。新しい AI CLI のサポートを追加するには、以下の手順に従います。

### 1. テンプレートディレクトリを作成

```
src-md/mds/core/src/init/templates/<target-key>/
├── manifest.toml       ← ファイルマッピング定義
├── instructions.md     ← instructions カテゴリのテンプレート
├── skill.md            ← skills カテゴリのテンプレート
└── command-check.md    ← commands カテゴリのテンプレート
```

`<target-key>` は `AiTarget` enum の `key()` メソッドが返す文字列と一致させます。

### 2. manifest.toml を定義

```toml
# 各 [[file]] エントリがテンプレートファイルと出力先のマッピング

[[file]]
template = "instructions.md"     # テンプレートファイル名
output_path = ".new-cli/rules.md"  # プロジェクトルートからの相対パス
category = "instructions"         # instructions, skills, commands のいずれか

[[file]]
template = "skill.md"
output_path = ".new-cli/skills/mds.md"
category = "skills"
```

### 3. テンプレートファイルを作成

テンプレートには以下を含めます:

- 対象 AI CLI のネイティブ frontmatter 形式
- `mds-managed: true` (再実行時の更新判定用)
- mds のマークダウンフォーマットリファレンス (Uses テーブル、セクション構造、制約)
- mds のコマンド一覧 (check, build, lint, test)

### 4. AiTarget enum にバリアントを追加

`src-md/mds/core/src/model/mod.rs.md` の `AiTarget` enum に:

```rust
pub enum AiTarget {
    // ...existing...
    NewCli,
}
```

`key()` メソッドで `"new-cli"` を返すようにし、`parse()` で受け付けるエイリアスを定義します。

### 5. ビルドして確認

```bash
./.github/script/sync-build.sh && cd .build/rust && cargo build && cargo test
```

build.rs が manifest.toml を自動検出し、テンプレートレジストリに登録します。init ロジックの変更は不要です。

## テンプレートに含めるべき内容

AI エージェントが mds プロジェクトで正しく作業するために、テンプレートには以下の情報が必要です:

1. **ワークフロー**: `mds check` → `mds build --dry-run` → `mds build` → `mds test`
2. **ファイル命名規約**: `src-md/name.{lang}.md` → `src/name.{lang}`
3. **必須セクション構造**: Purpose, Contract, Types, Source, Cases, Test (H2, 順序固定)
4. **Uses テーブル仕様**: From (internal/package/builtin/workspace), Target, Expose, Summary
5. **致命的制約**: コードブロック内に import/use/require を書かない
6. **Expose トークン構文**: `Name`, `Name as Alias`, `default: Name`, `* as ns`
7. **index.md の Exposes テーブル**: Kind, Name, Target, Summary
8. **見出し制約**: 実装 md に H1 なし、H5+ なし
