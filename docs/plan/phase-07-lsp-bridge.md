# Phase 07: LSP bridge 準備

## 前提

- source map が build または workspace indexing から取得できる。
- core は TypeScript-specific extractor を標準提供していない。
- wiki-link module resolution は mds-native behavior として残す。
- external language server 連携は段階導入する。

## やること

- generated-file mode を優先して実装する。
- generated file location を source map で Markdown location へ remap する API を LSP 側に組み込む。
- code fence 内 definition / hover / diagnostics が generated location を返す場合、Markdown location へ戻す。
- `mds-lsp` の TypeScript extractor-based symbol index を削除する。
- code import navigation が `extract_imports_for_lang` に依存しないようにする。
- `mds-virtual:` URI の設計だけを決め、virtual-document mode は後続候補にする。
- VS Code extension が `vscode.execute*Provider` を呼ぶ方式と、`mds-lsp` が language server client になる方式を比較する。
- 初回 slice では実装量が少ない VS Code extension delegation を優先候補にする。

## 完了条件

- generated file 上の definition result を Markdown code fence location へ戻せる。
- generated diagnostics を Markdown code fence location へ戻せる。
- code fence 内 hover / definition が core language-specific parsing に依存しない。
- virtual-document mode は URI design と後続課題が明文化されている。
- LSP tests に generated-to-Markdown remap の dedicated coverage がある。

## 注意事項

- すべての language server bridge を一括完成させない。
- generated-file mode は既存 language server との互換性を優先するための最初の実装である。
- `mds-embedded` VS Code support は暫定 bridge として残してよいが、source map-backed behavior へ寄せる。
- symbol-level wiki-link resolution は外部 language server なしでは弱くなる可能性がある。診断 severity と UX を Phase 08 と合わせて調整する。
