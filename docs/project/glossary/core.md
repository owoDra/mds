# Core Glossary

このファイルは、プロジェクト全体で必読となる共通用語を置く正本です。

## 必読用語

| 用語 | 意味 |
| --- | --- |
| mds | Markdown を設計書兼ソースの正本として扱い、Markdown 内の実装レベルのコードから派生コードを生成する開発ツールチェーン。 |
| 正本 | 人間と AI エージェントが参照する一次情報。mds では実コードを含む `.md` を正本とする。 |
| 生成コード | `.md` 内の `Types`、`Source`、`Test` などのコードブロックとメタ情報から生成される `.ts`、`.py`、`.rs` などの派生物。 |
| language adapter | 言語ごとの import 生成、lint、lint --fix、test runner 接続、出力規則を担う部品。 |
| package root module md | package / directory root の `Imports`、`Exports` を担当する言語別 Markdown。Rust は `lib.rs.md` / `mod.rs.md`、TypeScript は `index.ts.md` などを使う。 |
| `overview.md` | `.mds/source` / `.mds/test` の overview、architecture、navigation、ルールを担当する文書。`Imports` / `Exports` は置かない。 |
| source md | `.mds/source/**/*.{lang-ext}.md` 形式の文書。`Source` / `Types` の生成対象コードがない間は spec state、生成対象コードが入ると impl state として扱う。 |
| spec state | source md の lifecycle state。設計、契約、公開面、期待結果を記述するが、まだ `Source` / `Types` の生成対象コードを持たない。 |
| impl state | source md の lifecycle state。設計説明に加えて、生成対象となる `Source` / `Types` の実装コードを持つ。 |
| implementation md | 1 機能 1 実装を表す impl state の source md。設計説明だけでなく、実装レベルの `Types`、`Source` コードを含む。 |
| Exports | implementation md または package root module md が公開面を表すための表形式メタ情報。互換期間だけ Expose も読める。 |
| Imports | `Types`、`Source`、`Test` ごとの依存を表す表形式メタ情報。互換期間だけ Uses も読める。 |
| H5 shared definition | `##### Name` 形式の見出しで表す参照可能な定義説明。`Exports` で公開される定義や `Imports.Reference` から参照される定義に置く。 |
| Cases | 人間と AI 向けに期待結果を要約する実装 md セクション。 |
