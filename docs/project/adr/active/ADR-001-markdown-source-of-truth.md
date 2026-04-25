---
id: ADR-001-markdown-source-of-truth
status: 採用
related:
  - docs/project/requirements/REQ-core-markdown-source-of-truth.md
---

# Markdown を正本にする

## 背景

mds は人間と AI エージェントが同じ資料を読み、設計、実装、修正、検証を進めるための toolchain である。Markdown は設計説明だけではなく、生成元となる実装レベルのコードを含む。

## 判断

`.md` を設計書兼ソースの正本とし、`Types`、`Source`、`Test` のコードブロックに実コードを置く。生成された `.ts`、`.py`、`.rs` などは、その実コードとメタ情報から作られる派生物として扱う。

## 代替案

- 生成コードを正本にする: 通常の開発体験に近いが、設計、仕様、テストとの同期が崩れやすい。
- Markdown を補助文書に留める: 人間向け資料にはなるが、AI エージェントと generator の一次情報にならない。
- Markdown に設計だけを書き、AI が実装を生成する: 生成結果の根拠が正本に残らず、人間と AI が同じ一次情報を読めない。

## 結果

実装と検証は Markdown 正本内の説明、メタ情報、実コードを起点に行う。生成物の差分は正本または generator の修正で解消し、設計説明から暗黙にコードを補完しない。

## 関連資料

- `../../requirements/REQ-core-markdown-source-of-truth.md`
- `../../architecture.md`
