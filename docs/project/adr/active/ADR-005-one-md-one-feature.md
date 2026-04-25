---
id: ADR-005-one-md-one-feature
status: 採用
related:
  - docs/project/requirements/REQ-implementation-one-md-one-feature.md
---

# Implementation md は 1 機能だけを扱う

## 背景

implementation md は生成コード、型、テストの起点になるため、責務が混ざると生成対象と検証対象が曖昧になる。

## 判断

1 つの implementation md は 1 つの機能だけを扱う。自由な `file=` 指定で別ファイルへ飛ばす方式は採用しない。

## 代替案

- 1 md に複数機能をまとめる: 関連機能を一望しやすいが、生成とテストの単位が曖昧になる。
- `file=` 指定で任意出力する: 柔軟だが、path と命名規約による予測可能性が失われる。

## 結果

Source、Types、Test の出力先は implementation md の path と adapter の命名規約から決める。

## 関連資料

- `../../requirements/REQ-implementation-one-md-one-feature.md`
- `../../specs/shared/SPEC-code-generation-output.md`
- `../../patterns/impl-one-feature-markdown.md`
