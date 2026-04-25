---
id: SPEC-adapter-python-generation
status: 採用
related:
  - docs/project/requirements/REQ-adapter-required-language-adapters.md
  - docs/project/requirements/REQ-generation-code-output-rules.md
  - docs/project/specs/shared/SPEC-code-generation-output.md
  - docs/project/specs/shared/SPEC-md-state-quality-operations.md
---

# Python Adapter 生成

## 概要

`packages/lang-py` は Python の生成 file pattern、import 生成、Markdown 状態の品質操作に必要な toolchain 接続を担う。

## 関連要求

- `../../requirements/REQ-adapter-required-language-adapters.md`
- `../../requirements/REQ-generation-code-output-rules.md`

## 入力

- `.py.md` implementation md
- `Types`、`Source`、`Test` のコードブロック
- `Uses` テーブル
- 解決済みの `markdown_root`、`source_root`、`types_root`、`test_root`

## 出力

- Python Source ファイル
- Python Types stub ファイル
- Python Test ファイル
- Python import 文
- Python toolchain 診断

## 挙動

- 既定 pattern は、`src-md/pkg/foo.py.md` から Source `src/pkg/foo.py`、Types `src/pkg/foo.pyi`、Test `tests/pkg/test_foo.py` を生成する。
- `Uses` の `Types`、`Source`、`Test` 依存は Python import として生成する。
- `Uses.Expose` が空の場合は module import 相当として扱う。
- Markdown 状態の品質操作では Ruff format、Ruff、Pytest を代表 toolchain とする。

## 状態遷移 / 不変条件

- Python 固有の file pattern は core の Markdown model を変更しない。
- Types は既定で `.pyi` stub として出力する。
- default import と alias は MVP では扱わない。

## エラー / 例外

- `.py.md` 以外の implementation md を Python adapter の生成対象にしない。
- Python import に変換できない `Uses` は adapter 診断にする。
- toolchain 実行失敗は Markdown 構造エラーと区別して報告する。

## 横断ルール

- shared spec の生成 lifecycle、manifest、header、上書き規則に従う。
- shared spec の `Expose` / `Uses` canonical schema を変更しない。

## 検証観点

- `src-md/pkg/foo.py.md` から `src/pkg/foo.py`、`src/pkg/foo.pyi`、`tests/pkg/test_foo.py` が導出できることを確認する。
- `Uses` から Python import が生成できることを fixture で確認する。
- Ruff format、Ruff、Pytest の接続は toolchain 接続仕様を実装するときに代表 fixture で確認する。

## 関連資料

- `../shared/SPEC-code-generation-output.md`
- `../shared/SPEC-expose-uses-tables.md`
- `../shared/SPEC-md-state-quality-operations.md`
- `../../patterns/impl-adapter-boundary.md`
