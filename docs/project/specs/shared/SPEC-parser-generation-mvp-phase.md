---
id: SPEC-parser-generation-mvp-phase
status: 採用
related:
  - docs/project/requirements/REQ-cli-command-surface.md
  - docs/project/requirements/REQ-generation-code-output-rules.md
  - docs/project/requirements/REQ-doc-model-markdown-document-types.md
  - docs/project/requirements/REQ-metadata-expose-uses.md
  - docs/project/requirements/REQ-monorepo-package-boundary.md
  - docs/project/specs/shared/SPEC-cli-commands.md
  - docs/project/specs/shared/SPEC-code-generation-output.md
  - docs/project/specs/shared/SPEC-markdown-document-model.md
  - docs/project/specs/shared/SPEC-expose-uses-tables.md
---

# Parser + 生成 MVP フェーズ

## 概要

この仕様は、Parser + 生成 MVP の本実装で扱う範囲、入力検査、生成出力、今フェーズで確定した細部を定義する。

## 関連要求

- `../../requirements/REQ-cli-command-surface.md`
- `../../requirements/REQ-generation-code-output-rules.md`
- `../../requirements/REQ-doc-model-markdown-document-types.md`
- `../../requirements/REQ-metadata-expose-uses.md`
- `../../requirements/REQ-monorepo-package-boundary.md`

## 入力

- `mds.config.toml`
- package root の `package.md`
- package metadata: `package.json`、`pyproject.toml`、`Cargo.toml`
- `src-md` 配下の implementation md
- `index.md` の `Exposes`
- `Expose` / `Uses` table
- `.mds/manifest.toml`

## 出力

- `mds check` の診断
- `mds build --dry-run` の生成計画と unified diff
- `mds build` の生成ファイル
- `.mds/manifest.toml`
- Rust の mds 管理 module block

## 挙動

- 今フェーズの CLI は `mds check`、`mds build --dry-run`、`mds build` に限定する。
- `mds.config.toml` の MVP 対象 key は `[package] enabled/allow_raw_source`、`[roots] markdown/source/types/test`、`[adapters.<lang>] enabled` とする。
- MVP 対象外の config key は warning として報告し、処理は継続する。
- `package.md` は存在、必須セクション、package metadata との詳細同期まで検査する。
- package metadata と `package.md` の `Package`、`Dependencies`、`Dev Dependencies` が矛盾する場合は `mds check` の診断にする。
- dependency version は package metadata を正とし、`package.md` の値が異なる場合は診断する。
- Markdown table の列名は trim し、case-insensitive に canonical column へ解決する。
- `Expose.Kind` と `Uses.From` の enum 値は小文字 canonical のみ受け付ける。
- table の余分な列は warning として無視する。
- 同じ `Expose.Kind + Expose.Name`、同じ `Uses.From + Uses.Target + Uses.Expose` の意味重複は error にする。
- `Uses.Target` は正規形のみ受け付ける。
- `internal` の `Uses.Target` は `markdown_root` からの root 相対 module path とし、`./`、`../`、絶対 path、拡張子、末尾 slash、バックスラッシュを error にする。
- `Uses.Expose` が空欄の場合は module import / side-effect import として扱う。
- import / use は `Uses.From` の種別順で生成し、同じ種別内では `Uses` の出現順を維持する。
- `Uses.From` の種別順は `builtin`、`package`、`workspace`、`internal` とする。
- 同一 target への named import / use は target 単位で統合する。
- TypeScript の相対 import は拡張子なしで生成する。
- Python の `internal` import は生成先 source root からの absolute package import として生成する。
- manifest の `path` は package root 相対で保存する。
- manifest の source hash は、生成に使う section、table、code block の正規化結果を SHA-256 で計算する。
- 既存 `.mds/manifest.toml` が TOML として読めない、または schema 不整合の場合は check error とし、build は書き込みを行わない。
- `mds build --dry-run` は Source、Types、Test、manifest、Rust module block を含む全生成物の diff を表示する。
- Rust の mds 管理 module block は `pub mod` を生成する。
- Rust の nested module は MVP で扱う。
- Rust の Types file も module 管理対象にする。

## 状態遷移 / 不変条件

- package metadata は package 情報の正とし、`package.md` は同期対象の正本表示として扱う。
- `mds build` は mds 管理 marker と manifest によって管理対象と判断できる生成物だけ上書きする。
- manifest 破損時は安全側に倒し、生成物を書き換えない。

## エラー / 例外

- 必須 table column がない場合は error にする。
- enum 値の大小文字違いは error にする。
- `Uses.Target` が正規形でない場合は error にする。
- table の意味重複は error にする。
- mds 管理 marker の begin / end が不整合な場合は error にする。
- MVP 対象外 config key は warning にする。
- table の余分な列は warning にする。

## 横断ルール

- 今フェーズの仕様は Parser + 生成 MVP の実装正本とする。
- 実装中に追加判断が必要になった場合は、この spec または関連 adapter spec を先に更新してからコードへ反映する。

## 最小 fixture 期待出力

- TypeScript: `src-md/foo/bar.ts.md` は `src/foo/bar.ts`、`src/foo/bar.types.ts`、`tests/foo/bar.test.ts` を生成し、相対 import は拡張子なしにする。
- Python: `src-md/pkg/foo.py.md` は `src/pkg/foo.py`、`src/pkg/foo.pyi`、`tests/pkg/test_foo.py` を生成し、internal import は absolute package import にする。
- Rust: `src-md/foo/bar.rs.md` は `src/foo/bar.rs`、`src/foo/bar_types.rs`、`tests/foo_bar_test.rs` を生成し、`lib.rs` または `mod.rs` の mds 管理 block に nested module と types module を `pub mod` で追加する。
- すべての fixture で `.mds/manifest.toml`、生成 header、`build --dry-run` の unified diff を期待出力として固定する。

## 検証観点

- MVP 対象 config key と対象外 key warning を fixture で確認する。
- `package.md` と package metadata の詳細同期診断を fixture で確認する。
- table column、enum、余分な列、意味重複、`Uses.Target` 正規形を fixture で確認する。
- TypeScript / Python / Rust の import / use 生成順、統合、拡張子、absolute import を fixture で確認する。
- manifest path、source hash、破損 manifest error を fixture で確認する。
- dry-run diff が全生成物を表示することを fixture で確認する。
- Rust nested module と types module が mds 管理 block に出力されることを fixture で確認する。

## 関連資料

- `SPEC-cli-commands.md`
- `SPEC-code-generation-output.md`
- `SPEC-markdown-document-model.md`
- `SPEC-expose-uses-tables.md`
- `SPEC-package-boundary-detection.md`
- `SPEC-config-toml-resolution.md`
- `SPEC-adapter-typescript-generation.md`
- `SPEC-adapter-python-generation.md`
- `SPEC-adapter-rust-generation.md`
- `../../validation.md`
