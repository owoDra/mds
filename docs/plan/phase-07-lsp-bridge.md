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

## Task 3 記録

### 固定した実装判断

- 初回実装は generated-file mode を採る。既存 language server が通常の workspace file として generated file を扱えるため、custom scheme 依存の差分を後ろへ送りつつ、source map remap で Markdown authoring surface へ戻す最小 slice を作りやすい。
- bridge 方式は VS Code extension から `vscode.execute*Provider` を呼ぶ delegation を優先する。VS Code が既に持つ各言語 server の起動と workspace 解決を再利用でき、`mds-lsp` に言語別 language server client を内蔵するより実装量と運用負荷が小さい。
- `mds-lsp` の `executeCommand` bridge は generated result を source map で Markdown location へ正規化する責務を持つ。extension は provider 呼び出しと VS Code UI 連携に集中し、generated-to-Markdown remap の判定は `mds-lsp` 側へ寄せる。
- diagnostics mirror は extension 側で index 済みの全 Markdown 文書へ再配布する。Problems / gutter を authoring surface に集約しつつ、index 更新で診断が消えた文書も一括で stale clear できるため。

### virtual-document mode 設計メモ

- `mds-virtual:` URI は source Markdown path を path に持ち、generated 側情報は query に持つ。正本の identity は path に固定し、generated path・language・fence などの切り替え情報だけを query で表す。
- 例 URI: `mds-virtual:/workspace/mds/lsp/.mds/source/capabilities/navigation.rs.md?generated=/workspace/mds/lsp/src/capabilities/navigation.rs&kind=source&lang=rust&fence=0`
- 後続で詰める論点は query key の安定化、source map version / invalidation、custom scheme を各 language server が workspace 文脈として扱う条件、references / relatedInformation など multi-location result の remap 規約、virtual document の edit / save / format UX。

### 実装済み範囲と後続課題

- 初回 slice の実装済み範囲は hover / definition / diagnostics で、generated-file mode と VS Code extension の `execute*Provider` delegation を前提に generated 側結果を Markdown 側へ戻すところまでを対象にする。
- references、diagnostics `relatedInformation` の remap、virtual-document mode 本体、より広い editor portability の検証は後続候補として残す。

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
