# Shared Specs

## 役割

このディレクトリは、複数の subproject にまたがる仕様を置くための場所です。

## 置いてよいもの

- 共通 API 契約
- 共通状態ルール
- 共通認可や横断要件に関する詳細仕様

## 置いてはいけないもの

- 1 つの subproject に閉じる仕様
- 設計草案や調査メモ

## 命名規則

- `SPEC-<category>-<short-title>.md`

## 参照ルール

- subproject 固有仕様が必要な場合は、対象の `<subproject>/index.md` と個票を追加する

## 参照

- `SPEC-config-toml-resolution.md`: `mds.config.toml` の設定解決仕様
- `SPEC-package-boundary-detection.md`: monorepo package 境界検出仕様
- `SPEC-markdown-document-model.md`: `index.md`、`package.md`、implementation md の文書モデル仕様
- `SPEC-expose-uses-tables.md`: `Expose` / `Uses` テーブル仕様
- `SPEC-code-generation-output.md`: Source / Types / Test の生成出力仕様
- `SPEC-parser-generation-mvp-phase.md`: Parser + 生成 MVP の今フェーズ実装範囲と詳細判断
- `SPEC-cli-commands.md`: CLI コマンド仕様
- `SPEC-obsidian-readable-markdown.md`: Obsidian で読める Markdown 仕様
- `SPEC-md-state-quality-operations.md`: Markdown 状態での lint / lint --fix / test 仕様
- `SPEC-doctor-command.md`: `mds doctor` の環境診断仕様
- `SPEC-package-sync.md`: `mds package sync` と任意 post hook 仕様
- `SPEC-distribution-and-versions.md`: npm / Cargo / uv 配布と最低対応 version 仕様
- `SPEC-ai-agent-cli-initialization.md`: Claude Code / Codex CLI / Opencode / GitHub Copilot CLI 向け AI agent kit 初期化仕様
- `SPEC-init-development-environment-setup.md`: `mds init` による bootstrap と開発環境セットアップ仕様
- `SPEC-release-prepublish-quality.md`: 全配布経路の公開前品質 gate 仕様
