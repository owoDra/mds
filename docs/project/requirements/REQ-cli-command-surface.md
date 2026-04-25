---
id: REQ-cli-command-surface
status: 採用
related:
  - README.md
  - docs/project/validation.md
---

# CLI コマンド面

## 目標

mds は build、check、graph、lint、format、test、doctor、package sync の各操作を CLI から実行できること。

## 根拠

Markdown 正本からの派生コード生成、検査、品質確認、package metadata 同期を利用者が一貫した導線で実行できるようにするため。

## 対象範囲

- `mds build` で Markdown 内のコードブロックとメタ情報から派生コードを生成すること
- `mds check` で構造、参照、表を検証すること
- `mds graph` で Markdown 依存グラフを表示すること
- `mds lint`、`mds format`、`mds test` で adapter 経由の品質確認を行うこと
- `mds doctor` で環境を確認すること
- `mds package sync` で package metadata から `package.md` を更新すること

## 対象外

- CLI 以外の UI を必須機能にすること
- すべてのコマンドを language adapter なしで完結させること
- package metadata 以外の任意文書を `package sync` で生成すること

## 成功指標

- 各コマンドの正常系、入力不備、対象なし、部分失敗の挙動が仕様化される
- 失敗時に終了コードとエラー出力から次の対応が分かる
- `package sync` が package metadata と `package.md` の整合性を保てる

## 制約 / 品質条件

- 破壊的な失敗動作を避ける
- コマンド間で対象検出と設定解決の結果が矛盾しない

## 関連資料

- `../../README.md`
- `../validation.md`
