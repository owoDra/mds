# mds

mds は、Markdown を設計、実装、テストの正本として扱い、Markdown 内に書かれた実装レベルのコードから言語ごとのコード、型、テストを生成する強規約ツールチェーンです。

## 概要

mds は、人間と AI エージェントが同じ Markdown 文書を読んで作業するプロジェクトを対象にしています。Markdown は設計だけを書くプロンプトではなく、`Source`、`Types`、`Test` などに実際のコードブロックを含むソース正本です。生成された `.ts`、`.py`、`.rs` などのファイルは、そのコードブロックとメタ情報から作られる派生物です。

## プロジェクト資料

正本となるプロジェクト資料は `docs/project/` にあります。

- `docs/project/index.md`: プロジェクト資料の入口
- `docs/project/requirements/index.md`: 要求
- `docs/project/specs/index.md`: 振る舞いと構造の仕様
- `docs/project/adr/index.md`: 採用済みの判断
- `docs/project/patterns/index.md`: 再利用するプロジェクトパターン
- `docs/project/proposals/index.md`: 未確定の設計提案
- `docs/project/validation.md`: 検証方針
- `docs/project/tech-stack.md`: 採用技術

## 現在の対象

mds は monorepo 構成を前提に、Rust core、native CLI、npm package、Cargo 配布、uv ベースの Python 配布、TypeScript / Python / Rust 向け language adapter を対象にしています。

## 状態

この repository はプロジェクト定義の初期段階です。要求、仕様、判断、未確定の提案は `docs/project/index.md` を入口に確認してください。
