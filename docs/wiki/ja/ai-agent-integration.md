# AI エージェント連携

このページでは、mds の current な AI agent kit flow を説明します。

## 概要

`mds init --ai` は、対応する AI CLI 向けに instructions、skills、commands を生成し、agent が authoring-v2 package を独自解釈せず扱えるようにします。

生成される guidance は次を教えます。

- canonical `.mds/source` と `.mds/test`
- tableless な source/test document
- 新規 doc の scaffold に `mds new` を使うこと
- generated output の通常 validation flow

## 対応 AI CLI

| AI CLI | 指定名 | 出力先 |
| --- | --- | --- |
| Claude Code | `claude-code`, `claude` | `.claude/rules/`, `.claude/skills/`, `.claude/commands/` |
| Codex CLI | `codex-cli`, `codex` | `.codex/instructions.md`, `.codex/skills/` |
| Opencode | `opencode` | `.opencode/agents/`, `.opencode/skills/` |
| GitHub Copilot | `github-copilot-cli`, `copilot` | `.github/instructions/`, `.github/prompts/` |

## 基本的な使い方

```bash
# plan だけ表示
mds init --ai --target all --categories all

# plan を適用
mds init --ai --target all --categories all --yes

# 1 つの CLI だけ生成
mds init --ai --target claude-code --yes
```

## カテゴリ

| カテゴリ | 目的 |
| --- | --- |
| `instructions` | 常時参照される rule と workflow guidance |
| `skills` | 必要時に読む詳細 reference |
| `commands` | 対象 CLI でそのまま実行しやすい command snippet |

## 設計ルール

- `CLAUDE.md`、`AGENTS.md`、`copilot-instructions.md` などの user-owned file は書き換えません。
- generated file には `mds-managed: true` を付け、再実行時に安全に更新します。
- 非管理 file の上書きは `--force` がある場合だけです。

## template が教えるべき内容

template には current live surface を書きます。

1. source doc は `.mds/source`、test doc は `.mds/test` に置く
2. source doc は `Purpose`、`Contract`、`API`、`Source`、`Cases` を使う
3. test doc は `Purpose`、`Covers`、`Cases`、`Test` を使う
4. 新規 doc は `mds new` で scaffold する
5. validation は通常 `mds lint`、`mds build --dry-run`、`mds build`、`mds typecheck`、`mds test` の順で行う

## 新しい AI CLI の追加

`mds/core/src/init/templates/<target-key>/` の下に `manifest.toml` と category template を作成します。

典型的な構成:

```text
mds/core/src/init/templates/<target-key>/
├── manifest.toml
├── instructions.md
├── skill.md
└── command-check.md
```

その後:

1. `AiTarget` に target を追加
2. template の出力先を新しい CLI の native path に合わせる
3. generated frontmatter に `mds-managed: true` を含める
4. `cargo check --workspace`、`cargo test --workspace`、focused な `mds init --ai` smoke test で検証する

`build.rs` が template manifest を自動登録するため、別の sync step は不要です。