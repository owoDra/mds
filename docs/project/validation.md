# Validation

## 目的

このファイルは、変更時に確認すべき検証方針を記録します。

## 読むべき場面

- 変更後に何をどう確認すべきか整理したいとき
- 検証観点を追加または更新したいとき

## 検証項目

- `mds-core` `mds-cli` `mds-lsp` を変更したら、少なくとも対象 crate の build と test を確認する。
- `editors/vscode` を変更したら `npm run compile` を通す。
- 生成、config、descriptor、quality、source map 挙動を変えたら、`examples/` を使って lint / build / doctor / package sync 系の使用感を確認する。
- 仕様更新や authoring 体験変更がある場合、対応する `examples/` を必ず更新し、開発者体験と使いやすさをレビューする。

## 実行メモ

- Rust workspace 全体確認は `cargo test` と必要な `cargo build` を基準にする。
- examples 回帰確認は `mds-cli` を使う実運用寄りのコマンドを優先する。
