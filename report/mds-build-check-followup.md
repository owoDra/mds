# mds build / check Follow-up

## 対応した内容

### 1. `mds build` への self-hosted mirror 同期統合

- `Command::Build { mode: Write }` 成功時に、workspace 配下の Rust package から `.build/rust` を再生成する同期処理を追加した。
- package local の `src/`、`tests/`、special file としての `build.rs` を mirror へコピーする。
- release artifact 生成 script も `sync-build.sh` ではなく `cargo run -p mds-cli -- build --verbose` を前提に更新した。

### 2. `[check]` による validator 個別切り替え

- `mds.config.toml` の `[check]` / `[checks]` に次の on/off を追加した。
  - `code_blocks_required`
  - `code_fence_integrity`
  - `duplicate_h2_sections`
  - `markdown_links`
  - `import_with_implementation`
  - `top_level_fence_required`
  - `doc_comments_outside_code`
- default はすべて `true` にして、既定動作では今までより厳格に保つようにした。

### 3. doc comment / docstring の扱いを厳格化

- implementation md の code block では、Rust の `///` / `//!` / `/**` / `/*!`、Python の triple-quoted docstring、TypeScript の `/**` を reject するようにした。
- 説明は code block の外側、通常の Markdown 本文に寄せる前提へ揃えた。
- examples の Python / Rust fixture もこのルールに合わせて書き換えた。

### 4. template Markdown asset の build 取り扱い修正

- `.mds/source/init/templates/**` 配下の `.md` / `.prompt.md` は implementation md として parse せず、asset として `src/init/templates/**` へコピーするようにした。
- source asset は source 側の内容が変わったとき、generated 側を更新できるようにした。
- これで AI template の source of truth を `.mds/source/init/templates/**` に戻しつつ、`mds build` だけで generated copy を更新できる状態にした。

## 実施済み確認

- `cargo run -p mds-cli -- build --package mds/core --verbose`
  - 成功
- `cargo test -p mds-core --test parser_generation_mvp_test`
  - 54 tests passed
- `cargo run -p mds-cli -- build --verbose`
  - examples / `mds/cli` / `mds/core` / `mds/lsp` すべて成功
- `cargo run -p mds-cli -- check --verbose`
  - workspace 内の全 mds package で成功
- コミット
  - `aad96d6 mds buildにmirror同期とcheck設定を統合`

## まだ確認したいこと

### 1. `[check]` の粒度と命名

確認したいこと:

- この 7 項目で十分か。
- `top_level_fence_required` や `doc_comments_outside_code` という名前でよいか。
- alias を増やすべきか、逆に canonical 名だけに寄せるべきか。

気にしている理由:

- 一度 config surface に出すと後方互換の負債になりやすい。
- validator の意味が分かりにくい名前だと、project 側で誤設定されやすい。

### 2. doc comment ルールの既定値

確認したいこと:

- doc comment / docstring を既定で禁止したままでよいか。
- 言語ごとの対象範囲が妥当か。
  - Rust: `///`, `//!`, `/**`, `/*!`
  - Python: triple-quoted docstring
  - TypeScript: `/**`

気にしている理由:

- authoring 上は Markdown 側へ説明を寄せた方が読みやすい。
- 一方で、言語利用者として code-local documentation を残したい要求が出る可能性はある。

### 3. template asset 扱いの境界

確認したいこと:

- implementation md discovery から除外する asset subtree は `templates` だけで十分か。
- 将来的に `.mds/source/reference/**` や他の asset-only subtree を増やすなら、同じ扱いに寄せるべきか。

気にしている理由:

- 今回は `github-copilot-cli/*.prompt.md` を安全に扱うために `templates` を特別扱いした。
- asset subtree が増えると、この判定を構成化するべきかもしれない。

### 4. self-hosted mirror の検証基準

確認したいこと:

- `mds build` 後の `.build/rust` に対して、最終的にどこまで green を要求するか。
  - `cargo fmt --all --check`
  - `cargo test --manifest-path .build/rust/Cargo.toml`

気にしている理由:

- 今回の変更とは別系統の既存差分で、mirror workspace にはまだ unresolved な failure がある。
- `cargo fmt --all --check` は generated formatting 差分で失敗した。
- `cargo test --manifest-path .build/rust/Cargo.toml -q` は既存の [mds/lsp/tests/diagnostics_test.rs](mds/lsp/tests/diagnostics_test.rs) 破損で失敗した。

### 5. `sync-build` 廃止の docs 完了条件

確認したいこと:

- 今回の task では、正本 docs と release helper の更新までで十分か。
- wiki や過去 task の履歴メモまで完全に消し込むべきか。

気にしている理由:

- 現在の commit では、実運用に効く入口は `mds build` ベースへ揃えた。
- ただし archive / task history には `sync-build` 文字列が残る。

## 次に切るならこの順

1. `.build/rust` の `cargo fmt --all --check` を通すため、generated formatting 差分の source markdown を整理する。
2. [mds/lsp/tests/diagnostics_test.rs](mds/lsp/tests/diagnostics_test.rs) の破損生成を別 task として修正し、mirror workspace の `cargo test` を green にする。
3. `[check]` の key 名と default policy を確定させ、必要なら requirement / spec へ昇格する。