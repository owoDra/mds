# Phase 05: source map 基盤

## 前提

- core の code fence extraction が range を保持できる状態にある。
- output planning が language-specific import generation に依存していない。
- generated header や separator の扱いが決まっている。
- LSP / diagnostics / docs build が同じ remap layer を使う方針である。

## やること

- `SourceSpan` を設計・実装する。
- `SourceSpan` には Markdown path、Markdown range、generated path、generated range、output kind、extension key、code fence index を持たせる。
- `SourceMap` を設計・実装する。
- Markdown location から generated location、generated location から Markdown location を引ける API を追加する。
- 複数 code fence を 1 generated file へ連結する場合の offset table を保持する。
- source md output と test md output を別 output kind として source map に記録する。
- build planning から file write なしで source map を得られるようにする。
- diagnostics と LSP が使う range representation を source map と揃える。

## 完了条件

- 各 generated source / test file が Markdown code fence range に戻せる。
- generated diagnostic line が Markdown code fence 内の正しい line へ戻る。
- 複数 code fence の結合時も remap が正確である。
- source output と test output の source map が区別できる。
- source map remap 用 dedicated tests が存在する。

## 注意事項

- Markdown range は prose ではなく code fence content range を明確に扱う。
- generated header の行数と separator の行数を source map に反映しないと diagnostics がずれる。
- CRLF / LF の違いで byte offset と line/column がずれないよう、line-based mapping と byte-based mapping の責務を分ける。
- source map は LSP 専用にしない。docs build と diagnostics も共有する前提で core に置く。
