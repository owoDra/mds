---
id: SPEC-config-toml-resolution
status: 採用
related:
  - docs/project/requirements/REQ-config-toml-fixed-config.md
  - docs/project/adr/active/ADR-002-toml-only-config.md
---

# TOML 設定解決

## 概要

mds の設定ファイルは `mds.config.toml` 固定とし、built-in default、root 設定、subproject 設定の順に解決する。

## 関連要求

- `../../requirements/REQ-config-toml-fixed-config.md`

## 入力

- built-in default
- repository root の `mds.config.toml`
- subproject root の `mds.config.toml`

## 出力

- package ごとの有効 / 無効
- language adapter 設定
- 出力ルート
- セクション名とテーブル列名の表示名

## 挙動

- 設定優先順位は built-in default、root 設定、subproject 設定の順とする。
- 同じ設定項目は対象 package に近い設定が勝つ。
- table は key 単位で merge し、scalar / array は近い設定で置換する。
- `mds.config.ts`、`mds.config.json` などの別形式は設定ファイルとして扱わない。
- 表示名 override は canonical key に紐づけて解決する。
- MVP の `mds.config.toml` は `[package] enabled/allow_raw_source`、`[roots] markdown/source/types/test`、`[adapters.<lang>] enabled` を扱う。
- `roots.markdown` の既定値は `src-md` とする。
- `source_root`、`types_root`、`test_root` の具体 default は adapter が言語慣習に合わせる。

## 状態遷移 / 不変条件

- `Uses`、`Expose`、必須セクションの意味は設定で変更できない。
- 設定解決後も Markdown 正本の canonical な意味は変わらない。
- package を mds 対象にするには `[package] enabled = true`、`package.md`、実体 package metadata が揃っている必要がある。

## エラー / 例外

- TOML として読めない設定は設定エラーにする。
- 意味変更や必須構造の破壊につながる override は reject する。
- 複数形式の設定ファイルがあっても `mds.config.toml` 以外は無視または対象外として報告する。
- `[package] enabled = true` でも `package.md` または実体 package metadata がない場合は構造または設定の不備として報告する。

## 横断ルール

- Node、Rust、Python の各環境で同じ設定解決結果を使う。
- adapter 固有項目は adapter に渡し、core の意味体系を変更しない。

## 検証観点

- 未設定、root 設定、subproject 設定の fixture で優先順位を確認する。
- table merge と scalar / array 置換を fixture で確認する。
- label override 後も canonical key が維持されることを確認する。
- 不正な形式や意味変更が失敗になることを確認する。

## 関連資料

- `../../requirements/REQ-config-toml-fixed-config.md`
- `../../adr/active/ADR-002-toml-only-config.md`
- `../../validation.md`
