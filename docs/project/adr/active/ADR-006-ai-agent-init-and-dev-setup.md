---
id: ADR-006-ai-agent-init-and-dev-setup
status: 採用
related:
  - docs/project/requirements/REQ-ai-agent-cli-initialization.md
  - docs/project/requirements/REQ-init-development-environment-setup.md
  - docs/project/requirements/REQ-release-prepublish-quality.md
---

# AI Agent 初期化と開発環境セットアップを CLI に統合する

## 判断

mds は `mds init` に project 初期化、AI agent kit 生成、開発環境セットアップを統合する。AI CLI 対応はテンプレートのみの plugin として扱い、標準対応の Claude Code、Codex CLI、Opencode、GitHub Copilot CLI 向け template は mds 本体に同梱する。

## 背景

mds は人間と AI エージェントが同じ Markdown 正本を参照する前提のツールチェーンである。利用開始時に AI CLI ごとの instructions や skills を手作業で整えると、正本参照、検証導線、禁止事項が環境ごとにずれやすい。

また、mds は Rust、Node.js、Python、language toolchain、AI CLI を横断するため、環境セットアップの失敗や不足が導入障壁になりやすい。

## 採用案

- `mds init` を初期化の入口にする。
- AI だけを初期化する場合は `mds init --ai` を使う。
- AI CLI 対応は template plugin とし、任意コマンド実行を許可しない。
- 標準 template は mds 本体に同梱し、mds 本体 version と同期する。
- 開発環境セットアップは interactive default とし、project dependencies、toolchains、global AI CLI の導入を確認後に扱えるようにする。
- 非対話実行は明示 option がある場合だけ変更する。
- 公開前品質は全配布経路で checksum、署名、SBOM、provenance、install smoke test を必須にする。

## 代替案

- AI CLI 対応を built-in 固定にする: 実装は単純だが、AI CLI ごとの変化を mds core に混ぜやすいため採用しない。
- plugin に任意コマンド実行を許可する: 拡張性は高いが、初期化時の supply-chain risk が大きく、sandbox や署名検証が先に必要になるため採用しない。
- `mds setup` を独立コマンドにする: 責務は分かりやすいが、利用開始導線が分かれ、初期化時の必要確認が重複するため採用しない。
- 配布品質を smoke test のみにする: 初期実装は容易だが、全配布経路の完全性と provenance を保証しにくいため採用しない。

## 影響

- `mds init` の CLI 面が新たな中核機能になる。
- AI CLI template plugin の schema、fixture、release 検証が必要になる。
- bootstrap は `npx`、Cargo、`uvx` の 3 経路を正式に扱う。
- release 前検証に supply-chain 成果物と install smoke test が加わる。

## 関連資料

- `../../requirements/REQ-ai-agent-cli-initialization.md`
- `../../requirements/REQ-init-development-environment-setup.md`
- `../../requirements/REQ-release-prepublish-quality.md`
- `../../specs/shared/SPEC-ai-agent-cli-initialization.md`
- `../../specs/shared/SPEC-init-development-environment-setup.md`
- `../../specs/shared/SPEC-release-prepublish-quality.md`
