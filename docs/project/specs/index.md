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
- mds 自身の実装に結びつく設計は、今後は `src-md/` 配下の該当 `index.md` または implementation md へ移す
- `src-md/project/specs/` は作らない

## 参照

- `shared/index.md`: 複数 subproject にまたがる共有仕様の入口
- `crates-mds-core/index.md`: 移行前の `mds-core` 固有仕様の入口。新しい設計は `src-md/mds-core/index.md` または該当 implementation md に移す
- `crates-mds-cli/index.md`: 移行前の `mds-cli` 固有仕様の入口。新しい設計は `src-md/mds-cli/index.md` または該当 implementation md に移す

## 追加された共有仕様

- `shared/SPEC-ai-agent-cli-initialization.md`: AI agent CLI 初期化仕様
- `shared/SPEC-adapter-rust-generation.md`: Rust adapter 生成仕様
- `shared/SPEC-adapter-typescript-generation.md`: TypeScript adapter 生成仕様
- `shared/SPEC-adapter-python-generation.md`: Python adapter 生成仕様
- `shared/SPEC-init-development-environment-setup.md`: 開発環境セットアップ初期化仕様
- `shared/SPEC-release-prepublish-quality.md`: 公開前品質仕様
