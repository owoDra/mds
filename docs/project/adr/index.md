# ADR

## 役割

このディレクトリは、重要な判断とその理由を記録する正本です。

## 置いてよいもの

- 継続的な影響を持つ判断
- 代替案と採否理由
- 後続資料への関連付け

## 置いてはいけないもの

- 単なる作業ログ
- 会議メモだけの記録
- 手順詳細

## 命名規則

- `active/ADR-<NNN>-<short-title>.md`
- `archive/ADR-<NNN>-<short-title>.md`

## 参照ルール

- 現在有効な判断は `active/`
- 参照優先度を落とした過去資料は `archive/`
- 置換関係は状態名ではなく本文または front matter の関連情報で示す

## 参照

- `active/`: 現在有効な ADR の配置先
- `active/ADR-001-markdown-source-of-truth.md`: Markdown を正本、生成コードを派生物とする判断
- `active/ADR-002-toml-only-config.md`: 設定ファイルを `mds.config.toml` に固定する判断
- `active/ADR-003-multi-ecosystem-rust-core.md`: Rust core とマルチエコシステム配布を採用する判断
- `active/ADR-004-expose-uses-metadata.md`: `Expose` と `Uses` を Markdown 表にする判断
- `active/ADR-005-one-md-one-feature.md`: implementation md を 1 機能に限定する判断
- `active/ADR-006-ai-agent-init-and-dev-setup.md`: AI agent 初期化と開発環境セットアップを CLI に統合する判断
- `active/ADR-007-self-hosted-src-md-build.md`: mds 自身の正本を `src-md/` に置き生成物を `.build/` に集約する判断
- `archive/`: 参照優先度を落とした ADR の配置先。現在は個票なし
