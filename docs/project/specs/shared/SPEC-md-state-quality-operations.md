---
id: SPEC-md-state-quality-operations
status: 採用
related:
  - docs/project/requirements/REQ-quality-md-state-validation.md
  - docs/project/requirements/REQ-cli-command-surface.md
  - docs/project/requirements/REQ-adapter-required-language-adapters.md
---

# Markdown 状態の品質操作

## 概要

mds は Markdown 正本内の `Types`、`Source`、`Test` コードブロックを対象に、language adapter 経由で lint、lint --fix、test を実行する。`mds format` コマンドは提供せず、自動修正は `mds lint --fix` に統合する。

## 関連要求

- `../../requirements/REQ-quality-md-state-validation.md`
- `../../requirements/REQ-cli-command-surface.md`
- `../../requirements/REQ-adapter-required-language-adapters.md`

## 入力

- implementation md
- `Uses` table
- 解決済み `mds.config.toml`
- adapter ごとの linter / fixer / test runner 設定

## 出力

- lint / lint --fix / test の診断
- fix 後の Markdown 正本
- fix 差分
- toolchain 実行結果

## 挙動

- `mds lint`、`mds lint --fix`、`mds test` は Markdown 状態中心の操作とする。
- 対象コードは implementation md の `Types`、`Source`、`Test` の fenced code block とする。
- `Purpose`、`Contract`、`Cases`、説明文、table は toolchain 実行対象コードにしない。
- adapter は `Uses` から仮想 import / use / require を生成し、一時コードへ付与して toolchain に渡す。
- `mds lint --fix` は adapter / fixer が修正可能な code block を全自動修正し、Markdown 正本へ書き戻す。
- `mds lint --fix` の書き戻しは fenced code block の中身だけに限定する。
- `mds lint --fix` の対象は `Types`、`Source`、`Test` の fenced code block に限定し、`Purpose`、`Contract`、`Cases` などの説明用 code block は変更しない。
- `mds lint --fix` は修正必要箇所と修正例として unified diff を常に stdout へ表示する。
- `mds lint --fix --check` は書き込みを行わず、修正が必要な場合は diagnostics と unified diff を表示し、exit code 1 にする。
- fix が一部失敗した場合、成功した code block は書き戻し、失敗した code block は変更しない。
- TypeScript adapter は ESLint / Prettier / Biome と Vitest / Jest を選択可能な代表 toolchain とする。
- Python adapter は Ruff / Black と Pytest / unittest を選択可能な代表 toolchain とする。
- Rust adapter は rustfmt / clippy と cargo test / cargo-nextest を選択可能な代表 toolchain とする。
- 未選択の linter、fixer、test runner は実行せず、未選択 tool を environment 不足として扱わない。

## 状態遷移 / 不変条件

- Markdown 正本が品質操作の主対象であり、生成後コードだけを正本扱いしない。
- adapter は一時コードと Markdown location の対応を保持する。
- core は toolchain 固有処理を持たず、adapter の結果を共通診断へ集約する。
- `mds lint --fix` は Markdown の見出し、table、説明文、`Uses`、`Expose` を変更しない。

## エラー / 例外

- Markdown 構造エラーは toolchain 実行前に診断し、対象 document の品質操作を行わない。
- toolchain 実行失敗は lint / lint --fix / test 診断として扱い、exit code 1 に集約する。
- toolchain または runtime 不足は environment 不足として exit code 4 にする。
- usage / config error は exit code 2、internal error は exit code 3 にする。
- adapter が仮想 import を生成できない場合は adapter 診断にする。

## 横断ルール

- `mds check` は実言語 lint / test を実行しない。
- `mds lint` / `mds lint --fix` / `mds test` は `mds check` と同じ package 境界、設定解決、文書種別検査を使う。
- 診断 location は Markdown file、section、code block、line を参照できる形にする。
- 診断 code は `LINT001_TOOLCHAIN_FAILED` のようにカテゴリ prefix と短名で表す。

## 検証観点

- TypeScript / Python / Rust fixture で Markdown 状態の lint / lint --fix / test を確認する。
- `Uses` 由来の仮想 import を含む一時コードが toolchain に渡ることを確認する。
- `mds lint --fix` が code block の中身だけを書き戻すことを確認する。
- `mds lint --fix` 一部失敗時に成功 block だけ更新されることを確認する。
- `Purpose`、`Contract`、`Cases` などの説明用 code block が `lint --fix` で変更されないことを確認する。
- `mds lint --fix --check` が書き込みなしで差分診断を返すことを確認する。
- environment 不足が exit code 4 になることを確認する。

## 関連資料

- `../../requirements/REQ-quality-md-state-validation.md`
- `../../requirements/REQ-cli-command-surface.md`
- `SPEC-cli-commands.md`
- `SPEC-expose-uses-tables.md`
- `../../validation.md`
