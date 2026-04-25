---
id: REQ-config-toml-fixed-config
status: 採用
related:
  - README.md
  - docs/project/architecture.md
  - docs/project/validation.md
---

# TOML 固定設定

## 目標

mds の設定ファイルは `mds.config.toml` に固定し、root と subproject の設定継承を扱えること。

## 根拠

Node、Rust、Python の各環境から自然に読める設定形式にし、設定形式の分岐を避けるため。

## 対象範囲

- `mds.config.toml` を唯一の設定ファイルとして扱うこと
- built-in default、root 設定、subproject 設定の優先順位を持つこと
- package 有効 / 無効、adapter 設定、lint / format / test 設定、出力ルート、除外パスを設定できること
- セクション名とテーブル列名の表示名を override できること

## 対象外

- `mds.config.ts`、`mds.config.json` などの別形式設定
- 設定による `Uses`、`Expose`、必須構造の意味変更
- 設定継承順位を利用者が任意に変更すること

## 成功指標

- 未設定、root 設定、subproject 設定の fixture で近い設定が勝つ
- 表示名 override 後も canonical key による意味が維持される
- セクションの意味変更や必須構造の破壊が reject される

## 制約 / 品質条件

- 見た目の語彙は変えられるが、意味は変えられない
- 設定仕様は Node 固有機能に依存しない

## 関連資料

- `../../README.md`
- `../architecture.md`
- `../validation.md`
