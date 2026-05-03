# self-hosted split / alpha2 readiness

## 変更概要

- `mds build` から mds 自身の self-hosted Rust mirror 同期を外した
- repo-local helper として `./.github/script/sync-self-hosted-rust.sh` を追加した
- doc comment / docstring 判定を descriptor TOML 駆動へ置き換えた
- import / export 表現比較は `docs/project/research/research-import-export-authoring-shape.md` に記録した
- commit: `bb9247e self-hosted mirrorをrepo helperへ分離しdoc comment判定をdescriptor化`

## 実施した確認

### package-level

- `cargo run -p mds-cli -- build --package mds/core --verbose`
  - 成功
- `cargo test -p mds-core --test parser_generation_mvp_test doc_comment`
  - 3 passed
- `cargo test -p mds-core --test parser_generation_mvp_test package_check_uses_language_metadata_without_markdown_mirror`
  - 1 passed
- `cargo test -p mds-core --test parser_generation_mvp_test`
  - 55 passed
- `cargo run -p mds-cli -- build --verbose`
  - 成功
- `cargo run -p mds-cli -- check --verbose`
  - examples / `mds/cli` / `mds/core` / `mds/lsp` すべて成功

### self-hosted mirror

- `./.github/script/sync-self-hosted-rust.sh`
  - 成功
- `cargo test --manifest-path .build/rust/Cargo.toml -q`
  - 失敗

## release 判定

### 結論

- 現時点では alpha2 をそのまま release してよい状態ではない

### blocker

- self-hosted mirror workspace の Cargo test が失敗する
- 失敗箇所は `mds/lsp/tests/diagnostics_test.rs`
- 現在の generated file に Python 行と fence 断片が混入しており、Rust test file として parse できない

### 根拠ログ要約

- `cargo test --manifest-path .build/rust/Cargo.toml -q` で以下が発生した
  - `mds/lsp/tests/diagnostics_test.rs:6:1` で fence token ````rs` を Rust が解釈できず失敗
  - `mds/lsp/tests/diagnostics_test.rs:3:5` で `def test_it(): assert True` が混入していて失敗

## 判断メモ

- package-level の `mds build` / `mds check` / `mds-core` parser test は green なので、今回の変更自体は狙いどおり入っている
- ただし release 可否は self-hosted mirror を含む Rust workspace の test green を外せないため、alpha2 は保留が妥当
- 次タスクは `mds/lsp/tests/diagnostics_test.rs` の生成破損修正が最優先