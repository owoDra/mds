---
id: REQ-quality-md-state-validation
status: 採用
related:
  - README.md
  - docs/project/architecture.md
  - docs/project/validation.md
---

# Markdown 状態での品質確認

## 目標

mds は生成後コードだけでなく、Markdown の状態に対して check、lint、lint --fix、test を適用できること。

## 根拠

Markdown を正本とする以上、品質確認も生成後コードだけでなく、Markdown 内に書かれた実コードに対して行える必要があるため。

## 対象範囲

- Markdown から `Types`、`Source`、`Test` のコードブロックを抽出すること
- `Uses` から仮想 import / use / require を生成すること
- formatter、linter、test runner を language adapter 経由で接続すること
- 結果を必要に応じて Markdown のコードブロックへ反映できること

## 対象外

- 生成後コードだけを lint / lint --fix / test の対象にすること
- adapter を介さず core が各言語 toolchain を直接実行すること
- Markdown の意味構造を無視した単純なコードブロック処理
- 設計説明だけを lint / lint --fix / test の対象コードとして扱うこと

## 成功指標

- TypeScript、Python、Rust の代表 fixture で md 状態の lint / lint --fix / test が実行できる
- 仮想 import を含めたコードが各言語 toolchain に渡される
- format 結果を Markdown 正本へ戻しても構造が壊れない

## 制約 / 品質条件

- language adapter ごとの toolchain 差分を明示する
- 正本の構造検査と言語 toolchain の失敗を区別できる

## 関連資料

- `../../README.md`
- `../architecture.md`
- `../validation.md`
