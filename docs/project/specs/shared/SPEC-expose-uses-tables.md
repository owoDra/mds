---
id: SPEC-expose-uses-tables
status: 採用
related:
  - docs/project/requirements/REQ-metadata-expose-uses.md
  - docs/project/adr/active/ADR-004-expose-uses-metadata.md
---

# Expose / Uses テーブル

## 概要

mds は公開面を `Expose`、依存を `Uses` の Markdown テーブルで表す。

## 関連要求

- `../../requirements/REQ-metadata-expose-uses.md`

## 入力

- `Expose` テーブル
- `Uses` テーブル
- label override 後の canonical column

## 出力

- 公開 symbol の一覧
- セクション単位の依存一覧
- adapter に渡す仮想 import / use / require の材料

## 挙動

- `Expose` は `Kind`、`Name`、`Summary` 列を持つ。
- `Expose.Kind` は Markdown 記述上は全言語共通 kind とし、`type`、`value`、`function`、`class`、`module` の5種を正式一覧とする。
- language adapter は共通 kind を言語ごとの公開表現へ変換する。
- `Uses` は `From`、`Target`、`Expose`、`Summary` 列を持つ。
- `Uses` は `Types`、`Source`、`Test` ごとに別々に持つ。
- `From` は `internal`、`workspace`、`package`、`builtin` を扱う。
- `Target` は import に使える module specifier とし、拡張子や `.md` は書かない。
- `internal` の `Target` は `markdown_root` からの拡張子なし相対 module path とし、adapter が生成先 root での相対 import / use に変換する。
- `workspace` の `Target` は `<package-name>/<module-path>` 形式とし、先頭 segment を workspace package 名、残りをその package の module path として解決する。
- `Uses.Expose` で複数名を使う場合は `A, B, C` のカンマ区切りで書く。
- `Uses.Expose` に名前がある場合は named import / use とし、空の場合は module import / side-effect 相当として扱う。
- import / use / require はコードブロック内に書かず、language adapter が `Uses` から生成する。
- `Types` の `Uses` は型専用 import がある言語では型 import として扱い、`Source` と `Test` の `Uses` は通常 import / use として扱う。
- `index.md` の `Exposes` は `Kind`、`Name`、`Target`、`Summary` 列を持つ。

## 状態遷移 / 不変条件

- `Expose` と `Uses` の意味は設定で変更できない。
- 型依存、実装依存、テスト依存は混在させない。
- 公開名は `Expose` で表し、`symbol` フィールドは採用しない。
- Markdown 上の kind は言語固有語に寄せず、adapter が言語別の宣言種別へ対応付ける。

## エラー / 例外

- 必須列がないテーブルは構造エラーにする。
- 正式一覧にない `From` や `Kind` は検査エラーにする。
- コードブロック内の import / use / require は規約違反として扱う。

## 横断ルール

- テーブル列の表示名は override できるが、canonical column は維持する。
- 言語固有 kind は adapter に閉じ込める。

## 検証観点

- `Expose` から公開面を抽出できることを確認する。
- `Expose.Kind` が `type`、`value`、`function`、`class`、`module` の5種だけを受け付けることを確認する。
- `Uses` から仮想 import が作れることを確認する。
- `Types` / `Source` / `Test` ごとの依存分離を確認する。
- `internal`、`workspace`、`package`、`builtin` の Target 解決を fixture で確認する。

## 関連資料

- `../../requirements/REQ-metadata-expose-uses.md`
- `../../patterns/data-table-metadata.md`
- `../../adr/active/ADR-004-expose-uses-metadata.md`
