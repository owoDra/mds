# Phase 04: core 言語非依存化

## 前提

- first-party self-hosting は停止済み、または少なくとも canonical ではない。
- `mds-core` の breaking API change が許容されている。
- import/export、definition、hover、references、rename、type-aware diagnostics は core ではなく LSP / editor / optional provider へ委譲する方針である。

## やること

- `Lang::TypeScript`、`Lang::Python`、`Lang::Rust` enum variants を削除または非標準化し、opaque extension key へ寄せる。
- `TypeScriptExtractor` を core build path から削除する。
- `ImportExtractor` と `SymbolExtractor` を core の標準 API から削除する。残す場合は optional provider boundary へ隔離する。
- `extract_imports_for_lang` と `extract_exports_for_lang` を `mds-core::markdown` の標準 API から削除する。
- build generation が `Imports` table から import statement を prepend しないようにする。
- descriptor import renderer を required build path から外す。
- language-specific syntax rules による import line、doc comment、top-level declaration の必須 check を core から外す。
- core の責務を Markdown parsing、doc kind detection、code fence extraction、logical module id、wiki-link resolution、source map、output planning に限定する。

## 完了条件

- core build が import/export extraction なしで成立する。
- core build が `ts`、`py`、`rs` の意味分岐を持たない。
- file suffix は extension key として扱われ、core は意味を解釈しない。
- `mds-lsp` や editor bridge は core extractor に依存しない。
- language-specific parser が core に追加されていない。

## 注意事項

- `Lang` 型を一気に消すか、内部表現として縮小するかは影響範囲を見て決める。
- `header_prefix` のような出力コメント prefix も language semantics になり得るため、package output config または generic default へ寄せる。
- import/export extraction 削除により LSP symbol index の一部が一時的に弱くなる可能性がある。Phase 07 の source map / external LSP bridge で回復する。
- descriptor を完全削除する phase ではない。package manager / quality tool metadata と language output rules を分けて扱う。
