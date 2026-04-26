# Proposals

## 役割

このディレクトリは、正式仕様の前段階にある設計草案や提案を置くための場所です。

## 置いてよいもの

- 設計草案
- API 草案
- データ構造案
- 構成変更案
- 移行案

## 置いてはいけないもの

- 採用済み仕様の正本
- 過去ログだけのメモ
- ハーネス運用ルール

## 命名規則

- `active/proposal-<topic>.md`
- `archive/proposal-<topic>.md`

## 昇格ルール

- 採用した proposal は requirement / spec / ADR への昇格を確認する
- 参照優先度を落とした proposal は `archive/` へ移す

## 参照

- `active/`: 採否判断前の proposal の配置先
- `active/`: 現在 active proposal はない
- `archive/`: 参照優先度を落とした proposal の配置先
- `archive/proposal-markdown-grammar-open-details.md`: Markdown grammar の未確定細部に関する採否判断済み proposal
- `archive/proposal-post-mvp-generation-followups.md`: Post-MVP の残要件達成計画に関する採否判断済み proposal
- `archive/proposal-md-state-quality-operations.md`: Markdown 状態の lint / lint --fix / test に関する採否判断済み proposal
- `archive/proposal-graph-doctor-commands.md`: 旧 `mds graph` / `mds doctor` に関する採否判断済み proposal。`mds graph` は後続判断で削除済み
- `archive/proposal-package-sync-hooks.md`: `mds package sync` と任意 post hook に関する採否判断済み proposal
- `archive/proposal-import-expression-extensions.md`: `Uses` import 表現拡張に関する採否判断済み proposal
- `archive/proposal-distribution-version-policy.md`: npm / Cargo / uv 配布と version 方針に関する採否判断済み proposal
