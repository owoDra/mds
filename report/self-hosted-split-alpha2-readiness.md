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

- 現時点では alpha2 を release してよい状態まで到達した

### 解消した blocker

- self-hosted mirror workspace の Cargo test を止めていた `mds/lsp/tests/diagnostics_test.rs` の生成破損を修正した
- 原因は、LSP diagnostics test の埋め込み sample Markdown が outer Markdown parser に漏れていたことだった
- 対応として `mds/lsp/.mds/test/diagnostics.rs.md` に `sample_markdown` helper を追加し、埋め込み heading / fence を実行時復元へ変更した

### 最終確認

- `cargo run -p mds-cli -- build --verbose`
  - 成功
- `cargo run -p mds-cli -- check --verbose`
  - 成功
- `./.github/script/sync-self-hosted-rust.sh`
  - 成功
- `cargo test --manifest-path .build/rust/Cargo.toml -q`
  - 成功
- `./.github/script/generate-release-artifacts.sh`
  - 成功
- `cargo run -p mds-cli -- release check --manifest release.mds.toml --verbose`
  - `release quality ok`

## 判断メモ

- package-level の `mds build` / `mds check` / `mds-core` parser test は引き続き green
- self-hosted mirror を含む Rust workspace test も green になった
- release artifact 生成と `mds release check` まで通ったため、alpha2 は現時点で進めてよい
- `mds/lsp/tests/capabilities_test.rs` には unused import warning が残るが、今回の release gate では blocker ではない