# Specs

## 役割

このディレクトリは、要求を具体的な振る舞いへ落とし込む正本です。

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

- 2 つ以上の subproject にまたがる仕様は `shared/` に置く
- 1 つの subproject に閉じる仕様はその subproject 配下に置く

## 参照

- `shared/index.md`: 複数 subproject にまたがる共有仕様の入口
- `crates-mds-core/index.md`: `crates/mds-core` 固有仕様の入口
- `crates-mds-cli/index.md`: `crates/mds-cli` 固有仕様の入口
- `crates-mds-lang-rs/index.md`: `crates/mds-lang-rs` 固有仕様の入口
- `packages-core/index.md`: `packages/core` 固有仕様の入口
- `packages-cli/index.md`: `packages/cli` 固有仕様の入口
- `packages-lang-ts/index.md`: `packages/lang-ts` 固有仕様の入口
- `packages-lang-py/index.md`: `packages/lang-py` 固有仕様の入口
- `packages-lang-rs/index.md`: `packages/lang-rs` 固有仕様の入口
- `python-mds/index.md`: `python/mds` 固有仕様の入口
- `python-mds-lang-py/index.md`: `python/mds_lang_py` 固有仕様の入口

## 追加された共有仕様

- `shared/SPEC-ai-agent-cli-initialization.md`: AI agent CLI 初期化仕様
- `shared/SPEC-init-development-environment-setup.md`: 開発環境セットアップ初期化仕様
- `shared/SPEC-release-prepublish-quality.md`: 公開前品質仕様
