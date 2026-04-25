# Validation

## 目的

このファイルは、変更時に確認すべき検証方針を記録します。

## 読むべき場面

- 変更後に何をどう確認すべきか整理したいとき
- 検証観点を追加または更新したいとき

## 検証項目

- implementation md は `Purpose`、`Contract`、`Types`、`Source`、`Cases`、`Test` を持つこと。
- `Types`、`Source`、`Test` の依存は各セクションの `Uses` に分けて記録すること。
- import / use / require がコードブロック内に直接書かれていないこと。
- `Expose` と `Uses` が定義済みの表形式に従うこと。
- `mds.config.toml` が root または subproject に置かれ、優先順位が守られること。
- mds package は `enabled = true`、`package.md`、実体の package 定義を満たすこと。
- language adapter ごとの lint、format、test runner 接続が md 状態または生成コードに対して確認できること。
