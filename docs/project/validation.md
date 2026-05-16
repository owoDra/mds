# Validation

## 目的

このファイルは、変更時に確認すべき検証方針を記録します。

## 読むべき場面

- 変更後に何をどう確認すべきか整理したいとき
- 検証観点を追加または更新したいとき

## 検証項目

- `mds-core` `mds-cli` `mds-lsp` を変更したら、少なくとも対象 crate の build と test を確認する。
- `editors/vscode` を変更したら `npm run compile` を通す。
- 生成、config、schema、quality、source map 挙動を変えたら、少なくとも `examples/minimal-ts` を使って lint / build / doctor / package sync 系の使用感を確認する。
- 仕様更新や authoring 体験変更がある場合、対応する `examples/` を必ず更新し、開発者体験と使いやすさをレビューする。
- diagnostic remap を変えたら、成功系 fixture と分離された失敗系 TypeScript fixture の両方で確認する。
- `mds file` の構造ルールを変えたら、一般的な Markdown としての可読性と、機械検証可能性の両方を確認する。
- 参照配置や file 分割方針を変えたら、人間と AI の両方にとって探索コストが下がるか、少なくとも悪化しないかを確認する。
- LSP / VS Code extension の変更では、記法未習得の利用者でも completion / snippet / diagnostics 補助で最小 `mds file` を作れるか確認する。
- 埋め込み code bridge や言語認識を変えたら、active language 表示、言語 LSP 機能の再利用、Markdown 位置への再対応付けを確認する。
- navigation を変えたら、definition / references / related symbol 探索が `mds file` 起点で成立するか確認する。

## 実行メモ

- Rust workspace 全体確認は `cargo test` と必要な `cargo build` を基準にする。
- examples 回帰確認は `mds-cli` を使う実運用寄りのコマンドを優先し、v1 必須セットは `examples/minimal-ts` を基準にする。
