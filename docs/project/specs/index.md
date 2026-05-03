# Specs

> Deprecated: 新しい振る舞いの正本は `mds/*/.mds/source/overview.md` と implementation md に置く。ここは移行残置の legacy spec 入口としてのみ維持する。

## 役割

このディレクトリは、過去に requirements を具体的な振る舞いへ落とし込んだ spec を一時的に保持する legacy migration surface です。

## 置いてよいもの

- 入出力契約
- 状態遷移
- エラー条件
- 横断ルール
- 検証観点

## 置いてはいけないもの

- 要求の背景説明の大半
- 一時的な設計メモ
- ハーネス運用ルール

## 命名規則

- 共有仕様: `shared/SPEC-<category>-<short-title>.md`
- subproject 固有仕様: `<subproject>/SPEC-<category>-<short-title>.md`

## 参照ルール

- 新しい spec 個票は追加しない
- 既存 spec の内容は package overview または implementation md へ移したうえで参照を解消する
- 2 つ以上の subproject にまたがる旧 spec は、まず対象 package overview 群へ責務分解してから archive 化する
- `src-md/project/specs/` は作らない

## 参照

- `shared/index.md`: 複数 subproject にまたがる legacy spec の入口
- `crates-mds-cli/index.md`: legacy の CLI subproject spec 入口
- `crates-mds-core/index.md`: legacy の core subproject spec 入口
- `../../../mds/core/.mds/source/overview.md`: 現行の core behavioral source of truth
- `../../../mds/cli/.mds/source/overview.md`: 現行の CLI surface source of truth
- `../../../mds/lsp/.mds/source/overview.md`: 現行の LSP package source of truth

## 追加された共有仕様

- `shared/SPEC-ai-agent-cli-initialization.md`: AI agent CLI 初期化仕様
- `shared/SPEC-adapter-rust-generation.md`: Rust adapter 生成仕様
- `shared/SPEC-adapter-typescript-generation.md`: TypeScript adapter 生成仕様
- `shared/SPEC-adapter-python-generation.md`: Python adapter 生成仕様
- `shared/SPEC-init-development-environment-setup.md`: 開発環境セットアップ初期化仕様
- `shared/SPEC-release-prepublish-quality.md`: 公開前品質仕様
