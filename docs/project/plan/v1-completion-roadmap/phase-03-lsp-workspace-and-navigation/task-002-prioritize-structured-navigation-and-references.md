# Task 002: Prioritize Structured Navigation And References

## 目的

definition / references / symbol 探索を structured-first に整え、heuristic reference を v1 の best-effort として分離できる状態にする。

## 前提条件

- workspace index が impl/test/overview を含めて更新される
- shared navigation spec の structured vs heuristic 方針が参照できる

## 作業内容

- module、shared definition、link、covers など構造参照を優先する navigation 実装へ見直す
- heuristic textual match は fallback とし、誤誘導を避けるルールを明確にする
- 曖昧候補、未解決候補、generated remap 結果の返し方を整える

## 完了条件

- 構造参照がある場合は heuristic より先に返る
- heuristic references が best-effort であることを壊さず、空振りや誤誘導を抑えられる
- generated -> Markdown remap を含む navigation が説明可能である

## 検証方法

- navigation / references / symbols test を追加する
- success-path fixture と broken/remap fixture で link / symbol / remap を確認する

## 依存関係

- `task-001-index-overview-test-and-refresh-events.md`

## 成果物

- `mds/lsp/src/capabilities/navigation.rs`
- `mds/lsp/src/capabilities/symbols.rs`
- `mds/lsp/tests/`
