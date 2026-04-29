---
id: REQ-ai-agent-cli-initialization
status: 採用
related:
  - docs/project/architecture.md
  - docs/project/specs/shared/SPEC-ai-agent-cli-initialization.md
---

# AI Agent CLI 初期化

## 目標

mds は AI エージェント利用を前提に、主要 AI CLI が mds の正本構造と作業規約を理解できる instruction、skill、command、workflow、docs を初期化できること。

## 根拠

mds は Markdown を設計書兼ソースの正本として扱い、人間と AI エージェントが同じ正本を参照して実装、検証、修正できることを価値にするため。

## 対象範囲

- Claude Code、Codex CLI、Opencode、GitHub Copilot CLI の 4 種を初期必須対応に含めること
- `mds init` の初期化フロー内で AI 初期化を選択できること
- AI 初期化だけを実行する場合は `mds init --ai` で開始できること
- 各 AI CLI が対応する範囲で full agent kit を生成できること
- 初期化時に instructions、skills、commands、workflows、docs などのカテゴリ単位で生成項目を選択できること
- AI CLI ごとの差分を template plugin に閉じ込めること

## 対象外

- AI CLI に mds の core 意味体系を変更させること
- AI に設計説明から実装を丸投げする仕組みを mds の中核機能にすること
- template plugin に任意コマンド実行を許可すること
- Claude Code、Codex CLI、Opencode、GitHub Copilot CLI 以外を初期必須対応に含めること

## 成功指標

- 4 種の AI CLI 向け初期化 fixture が生成物、差分、上書き確認を検証できる
- 生成された instruction / skill / command / workflow が mds の正本、要件、仕様、検証導線を参照できる
- 既存ファイルを変更する場合は diff と確認を経由し、意図しない上書きを避けられる

## 制約 / 品質条件

- 標準 template plugin は mds 本体に同梱し、version は mds 本体と同期する
- plugin は template、置換変数、対応 CLI metadata のみを持ち、任意コマンドを実行しない
- AI CLI 固有のファイル配置や形式は plugin 境界に閉じ込める

## 関連資料

- `../architecture.md`
- `../specs/shared/SPEC-ai-agent-cli-initialization.md`
- `../adr/active/ADR-006-ai-agent-init-and-dev-setup.md`
