# Phase 06: package output config

## 前提

- source map が planned output path と連動できる。
- core language semantics が削減されている。
- `Types` output kind は廃止方向である。
- output path は language descriptor ではなく package config pattern を正とする。

## やること

- `[roots]` に `source_md`、`test_md`、`source_out`、`test_out` を追加する。
- `[output]` に `source` と `test` pattern を追加する。
- `{source_out}`、`{test_out}`、`{module}`、`{ext}` などの placeholder を定義する。
- pattern escaping rule と unknown placeholder の error handling を決める。
- `build.rs.md` のような special file は package config override で表せるようにする。
- descriptor-based `files.source`、`files.types`、`files.test`、`special_files` を required output planning から外す。
- `Roots.types` と `OutputKind::Types` を削除する。
- `roots.markdown` と `src-md` fallback を削除する。

## 完了条件

- source / test output path が package config pattern から決まる。
- Rust special file placement が package config override で表現できる。
- `Types` は supported output kind ではない。
- `src-md` は fallback authoring root として受理されない。
- output planning tests が descriptor-driven path ではなく package output config を検証している。

## 注意事項

- package output config は language semantics を持たない。`ext` は file suffix 由来の opaque key として扱う。
- special file override は汎用 path override であり、Rust 専用 hardcode にしない。
- config field rename は破壊的変更として扱い、migration fallback は足さない。
- existing fixtures は大きく壊れる可能性があるため、old fixture を移すより新しい最小 fixture に作り直す。
