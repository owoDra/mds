# Core Glossary

このファイルは、プロジェクト全体で必読となる共通用語を置く正本です。

## 必読用語

| 用語 | 意味 |
| --- | --- |
| mds | Markdown を設計書兼ソースの正本として扱い、Markdown 内の実装レベルのコードから派生コードを生成する開発ツールチェーン。 |
| 正本 | 人間と AI エージェントが参照する一次情報。mds では実コードを含む `.md` を正本とする。 |
| 生成コード | `.md` 内の `Types`、`Source`、`Test` などのコードブロックとメタ情報から生成される `.ts`、`.py`、`.rs` などの派生物。 |
| language adapter | 言語ごとの import 生成、lint、lint --fix、test runner 接続、出力規則を担う部品。 |
| `index.md` | package root の package metadata、overview、architecture、navigation を担当する文書。 |
| `overview.md` | `src-md/` など source root の overview、architecture、navigation を担当する文書。 |
| implementation md | 1 機能 1 実装を表す `*.{lang-ext}.md` 形式の文書。設計説明だけでなく、実装レベルの `Types`、`Source`、`Test` コードを含む。 |
| Exports | 実装 md または index が公開面を表すための表形式メタ情報。互換期間だけ Expose も読める。 |
| Imports | `Types`、`Source`、`Test` ごとの依存を表す表形式メタ情報。互換期間だけ Uses も読める。 |
| Cases | 人間と AI 向けに期待結果を要約する実装 md セクション。 |
