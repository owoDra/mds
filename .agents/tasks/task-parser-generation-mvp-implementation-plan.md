# Task

## 目的

Parser + 生成 MVP フェーズの採用済み要件・仕様を、実装コード、fixture、検証、必要な正本更新まで含めて完全実装するための実装計画として整理する。

## 状態

completed

## 依頼内容

今計画されているフェーズの要件・仕様について、完全実装するための実装計画を立てて task にまとめる。

## 確定前提

- 今フェーズは Parser + 生成 MVP とする。
- Parser + 生成 MVP は `mds check`、`mds build --dry-run`、`mds build` を対象にする。
- MVP 対象 language adapter は TypeScript、Python、Rust の 3 つとする。
- 現状リポジトリには実装コード、workspace manifest、package metadata、test suite はまだ存在しない。
- 実装は Rust core / native CLI を先に作り、TypeScript / Python / Rust adapter 生成規則を fixture で固定しながら進める。
- npm / Python 配布 wrapper、lint / format / test runner 接続、graph、doctor、package sync、package manager hook は MVP 外とする。
- MVP 対象外 config key は warning として報告し、処理は継続する。
- 実装中に追加判断が必要な場合は、先に `docs/project/specs/shared/SPEC-parser-generation-mvp-phase.md` または関連 adapter spec を更新してからコードへ反映する。

## 未確定事項

- Rust crate / module の内部命名は実装時に最小で決める。ただし公開面は `mds check` / `mds build` と fixture 期待値を優先して固定する。
- package metadata と `package.md` の詳細同期で扱う表 schema は、MVP fixture に必要な最小 schema から実装し、不足が出た場合は spec を先に追記する。
- adapter の import / use 文字列の詳細で spec に明記がないものは、最小 fixture の期待出力として固定し、関連 spec へ追記する。
- dry-run unified diff の完全な git 互換性は MVP では「git 互換寄り」に留める。差分形式の細部が実装判断を左右する場合は spec に追記する。

## 対象範囲

- repository 初期実装構成
- `crates/mds-core`: Markdown parser、document model、table parser、config 解決、package 検出、generation planning、manifest、diagnostics、diff 生成の中核
- `crates/mds-cli`: native CLI の `check` / `build` 実行、stdout / stderr、exit code
- `crates/mds-lang-rs`: Rust adapter の path / use / module block 生成
- `packages/lang-ts`: TypeScript adapter の path / import 生成に必要な最小実装または adapter spec fixture
- `packages/lang-py`: Python adapter の path / import 生成に必要な最小実装または adapter spec fixture
- TypeScript / Python / Rust の正常系 fixture
- config、package boundary、table parser、manifest、dry-run、Rust module block、診断系 fixture
- 実装に伴う `docs/project/specs/`、`docs/project/validation.md`、関連 index の必要最小更新

## 対象外

- `mds lint`、`mds format`、`mds test` の実言語 toolchain 呼び出し
- `mds graph`、`mds doctor`、`mds package sync`
- package manager 実行後の `package.md` 自動同期 hook
- npm / uv / Cargo への公開作業
- TypeScript / Python runtime package の完全な配布 wrapper 実装
- archive 資料の詳細再レビュー
- tech-stack の具体 version 方針確定

## 守るべき不変条件

- `.md` が設計書兼ソースの正本であり、生成コードは派生物である。
- implementation md は 1 機能だけを扱う。
- import / use / require はコードブロック外の `Uses` に記録し、language adapter が生成する。
- `Expose` は公開面を示し、`Uses` は依存を示す。
- 設定ファイルは `mds.config.toml` 固定とし、設定で必須構造や意味を変更しない。
- 言語固有処理は language adapter に閉じ込める。
- `mds build` は mds 管理 marker と manifest で管理対象と判断できる生成物だけを上書きする。
- manifest 破損時は安全側に倒し、生成物を書き換えない。

## 参照する正本

- `.agents/project.md`
- `.agents/tasks/task-requirements-spec-gap-analysis.md`
- `.agents/tasks/task-implementation-readiness-review.md`
- `docs/project/index.md`
- `docs/project/glossary/core.md`
- `docs/project/tech-stack.md`
- `docs/project/patterns/index.md`
- `docs/project/patterns/impl-one-feature-markdown.md`
- `docs/project/patterns/impl-adapter-boundary.md`
- `docs/project/patterns/data-table-metadata.md`
- `docs/project/architecture.md`
- `docs/project/requirements/index.md`
- `docs/project/specs/index.md`
- `docs/project/specs/shared/SPEC-parser-generation-mvp-phase.md`
- `docs/project/specs/shared/SPEC-markdown-document-model.md`
- `docs/project/specs/shared/SPEC-expose-uses-tables.md`
- `docs/project/specs/shared/SPEC-code-generation-output.md`
- `docs/project/specs/shared/SPEC-cli-commands.md`
- `docs/project/specs/shared/SPEC-config-toml-resolution.md`
- `docs/project/specs/shared/SPEC-package-boundary-detection.md`
- `docs/project/specs/packages-lang-ts/SPEC-adapter-typescript-generation.md`
- `docs/project/specs/packages-lang-py/SPEC-adapter-python-generation.md`
- `docs/project/specs/crates-mds-lang-rs/SPEC-adapter-rust-generation.md`
- `docs/project/validation.md`

## 今回読まなくてよい資料

- `docs/project/adr/archive/`
- `docs/project/proposals/archive/`
- `.opencode/node_modules/`
- MVP 外コマンドの未採用 proposal

## 実施方針

- 作業モードは自走。仕様変更、破壊的な既存ファイル変更、MVP スコープ拡大、公開 API の大きな変更が必要になった場合だけ確認する。
- 最初に workspace と最小 crate を作り、すぐに fixture 駆動のテストを置ける状態にする。
- 実装順は、共通 model と診断、package/config 解決、Markdown/table parser、generation plan、adapter、manifest/diff、CLI の順に積み上げる。
- TypeScript / Python / Rust の 3 言語 fixture を早期に作り、以後の変更は fixture の期待出力を壊さないように進める。
- 仕様が薄い箇所はコードで暗黙決定せず、該当 spec を先に更新してから実装する。
- 重要な module、fixture、test には必要に応じて `@spec SPEC-parser-generation-mvp-phase` などの trace tag を残す。

## 実施手順

1. Repository 初期化
   - root `Cargo.toml` workspace を追加する。
   - `crates/mds-core`、`crates/mds-cli`、`crates/mds-lang-rs` を追加する。
   - 必要最小の Rust toolchain 設定、format、test 実行導線を整える。
   - npm / Python package は MVP 実装の対象外なら空構成を作らず、正本との差異を task に記録する。

2. Core domain model と diagnostics
   - document kind、section、code block、Expose、Uses、package、config、adapter、output kind、manifest entry、diagnostic severity を型として定義する。
   - 診断は error / warning を分け、CLI exit code へ集約できる構造にする。
   - source location は最低限 file path と line を持たせ、fixture で診断対象を追えるようにする。

3. Config 解決
   - built-in default、root `mds.config.toml`、package `mds.config.toml` の順に解決する。
   - table は key 単位 merge、scalar / array は近い設定で置換する。
   - MVP 対象 key は `[package] enabled/allow_raw_source`、`[roots] markdown/source/types/test`、`[adapters.<lang>] enabled` に限定する。
   - MVP 対象外 key は warning にする。
   - TOML parse error と意味変更につながる override は error にする。

4. Package boundary 検出
   - `enabled = true`、`package.md`、実体 package metadata が揃う directory を mds 対象 package にする。
   - JS / TS は `package.json`、Python は `pyproject.toml`、Rust は `Cargo.toml` を実体 metadata とする。
   - `--package <path>` と cwd 配下探索の両方を実装する。
   - 対象 package 0 件、部分失敗、対象外 package 非破壊を fixture で確認する。

5. Markdown parser と文書種別検査
   - `index.md`、`package.md`、`*.{lang-ext}.md` を識別する。
   - H2 固定の必須セクションと H3-H4 補助見出し制約を検査する。
   - `Types` / `Source` / `Test` のコードブロックを出現順で抽出する。
   - `Purpose` / `Contract` / `Cases` は実コード生成入力にしない。
   - code block 内の import / use / require を MVP で検出可能な範囲で規約違反にする。

6. Markdown table parser
   - `Expose`、`Uses`、`index.md` の `Exposes`、`package.md` の `Package` / `Dependencies` / `Dev Dependencies` を解析する。
   - 列名は trim し、case-insensitive に canonical column へ解決する。
   - 必須 column 欠落、enum 不正、大小文字違い、意味重複を error にする。
   - 余分な列は warning として無視する。
   - `Uses.Target` の正規形を検査し、`internal` は `./`、`../`、絶対 path、拡張子、末尾 slash、バックスラッシュを error にする。

7. Generation planning
   - `markdown_root` からの相対 path を保って Source / Types / Test の出力 path を計算する。
   - 出力先が package 範囲外になる場合は error にする。
   - 既存ファイルに mds 管理 marker がない場合は error にする。
   - stale 生成物は warning に留め、自動削除しない。
   - 複数コードブロックは LF 1 行で連結し、出力末尾も LF 1 行にする。

8. Adapter 実装
   - TypeScript: `.ts.md` から `src/**/*.ts`、`src/**/*.types.ts`、`tests/**/*.test.ts` を生成し、`Types` は type-only import、`Source` / `Test` は通常 import にする。
   - TypeScript: 相対 import は拡張子なしにする。
   - Python: `.py.md` から `src/**/*.py`、`src/**/*.pyi`、`tests/**/test_*.py` を生成し、`internal` import は source root からの absolute package import にする。
   - Rust: `.rs.md` から `src/**/*.rs`、`src/**/*_types.rs`、`tests/*_test.rs` を生成し、`use` と `pub mod` の mds 管理 block を生成する。
   - import / use は `builtin`、`package`、`workspace`、`internal` の順に生成し、同一 target の named import / use は統合する。
   - 言語固有の変換不能な `Uses` は adapter 診断にする。

9. Manifest と hash
   - package root の `.mds/manifest.toml` を読み書きする。
   - `path` は package root 相対で保存する。
   - source hash は生成に使う section、table、code block の正規化結果を SHA-256 で計算する。
   - output hash は生成後内容から計算する。
   - manifest が TOML として読めない、または schema 不整合の場合は check error とし、build は書き込みを行わない。

10. Build / dry-run / diff
   - build は Source、Types、Test、manifest、Rust module block を生成する。
   - `--dry-run` は書き込みを行わず、生成計画と全生成物の unified diff を stdout に表示する。
   - 新規ファイル、既存管理ファイル更新、manifest 更新、Rust module block 更新を diff fixture で固定する。
   - 管理 marker 不整合や manifest 破損時は生成物を書き換えない。

11. CLI 実装
   - `mds check`、`mds build`、`mds build --dry-run` を実装する。
   - option は `--package <path>`、`--dry-run`、`--verbose` に限定する。
   - 成功時の要約と生成一覧は stdout、warning / error / diagnostic は stderr に出す。
   - exit code は 0 成功、1 check/build 診断あり、2 CLI usage/config error、3 internal error とする。
   - 複数 package の一部失敗時は可能な package の処理を続け、最後に失敗 package をまとめて診断する。

12. Fixture / test 整備
   - TypeScript / Python / Rust の最小正常 fixture を作る。
   - 各 fixture で `mds check`、`mds build --dry-run`、`mds build` を検証する。
   - 期待ファイル、manifest、header、dry-run diff、Rust mod 更新を golden file として固定する。
   - config warning、package metadata 同期診断、table parser error、Uses.Target error、manifest 破損、出力先衝突、対象 package 0 件、部分失敗を回帰 test にする。

13. 正本同期
   - 実装中に fixture で固定した未詳述の振る舞いを該当 spec に追記する。
   - spec を更新した場合は `docs/project/specs/index.md`、subproject index、`docs/project/validation.md` への影響を確認する。
   - MVP 外に残した事項は task の未完了事項または後続 task に分離する。

14. 最終検証
   - `cargo fmt --check` を実行する。
   - `cargo test` を実行する。
   - fixture で `mds check`、`mds build --dry-run`、`mds build` が TypeScript / Python / Rust すべてで通ることを確認する。
   - `bash /workspace/.agents/scripts/validate_harness.sh --root /workspace --verbose` を実行する。
   - 未実施検証、残リスク、更新した正本を task に記録する。

## 実装マイルストーン

1. M0: workspace / crates / test harness が作成され、空の CLI が起動できる。
2. M1: config 解決と package boundary 検出が fixture で検証できる。
3. M2: Markdown 文書種別、必須セクション、table parser、Uses.Target 検査が実装される。
4. M3: generation plan、path 解決、コードブロック連結、出力先衝突検査が実装される。
5. M4: TypeScript / Python / Rust adapter の path と import / use 生成が fixture で固定される。
6. M5: manifest、header、hash、safe overwrite、stale warning が実装される。
7. M6: dry-run unified diff と Rust module block 更新が実装される。
8. M7: CLI の stdout / stderr / exit code / 複数 package 処理が仕様どおりになる。
9. M8: 3 言語 fixture で `check` / `build --dry-run` / `build` が通り、正本同期と harness validation が完了する。

## 検証項目

- Parser + 生成 MVP の範囲が `check` / `build --dry-run` / `build` に限定されている。
- `mds.config.toml` の merge、MVP 対象 key、対象外 key warning が fixture で確認できる。
- mds 対象 package、非対象 package、対象 0 件、部分失敗が fixture で確認できる。
- `index.md`、`package.md`、implementation md の必須セクションと文書種別が検査される。
- `Expose` / `Uses` の canonical column、enum、余分な列、意味重複が検査される。
- `Uses.Target` の正規形違反が error になる。
- import / use の生成順、同一 target 統合、section-aware 生成が各 adapter fixture で確認できる。
- TypeScript / Python / Rust の既定 path pattern が期待ファイルを生成する。
- `.mds/manifest.toml` の path、hash、破損 manifest error が確認できる。
- 生成 header と safe overwrite が確認できる。
- `mds build --dry-run` が書き込みなしで全生成物の diff を表示する。
- Rust nested module と types module が mds 管理 block に `pub mod` として出力される。
- stdout / stderr / exit code が CLI spec と一致する。
- `cargo fmt --check`、`cargo test`、harness validation が成功する。

## 完了条件

- TypeScript / Python / Rust の最小 fixture で `mds check`、`mds build --dry-run`、`mds build` が通る。
- 期待 Source / Types / Test、`.mds/manifest.toml`、生成 header、dry-run diff、Rust module block がテストで固定されている。
- 診断系 fixture が、config、package、Markdown 構造、table、Uses.Target、manifest、出力先衝突、対象 package 0 件、部分失敗を覆っている。
- 実装中に確定した仕様差分が関連 spec / validation / index に反映されている。
- MVP 外の事項が実装ブロッカーとして残っていない。
- `cargo fmt --check`、`cargo test`、`bash /workspace/.agents/scripts/validate_harness.sh --root /workspace --verbose` が成功している。

## 進捗記録

- 2026-04-26: `.agents/project.md`、既存 task、project index、glossary、tech-stack、patterns、architecture、requirements / specs index、Parser + 生成 MVP 関連 spec、validation を確認した。
- 2026-04-26: 現状リポジトリには実装コード、Cargo workspace、npm / Python package metadata、test suite がまだ存在しないことを確認した。
- 2026-04-26: Parser + 生成 MVP の完全実装に向けた実施手順、マイルストーン、検証項目、完了条件を本 task に整理した。
- 2026-04-26: ユーザー依頼により Parser + 生成 MVP の実装を開始した。
- 2026-04-26: root `Cargo.toml`、`crates/mds-core`、`crates/mds-cli`、`crates/mds-lang-rs` を追加した。
- 2026-04-26: `mds-core` に config 解決、package 検出、Markdown section / code block / Uses table parser、生成 planning、adapter 別 path / import / use 生成、manifest、dry-run diff、safe write、Rust module block 更新の MVP 実装を追加した。
- 2026-04-26: `mds-cli` に `mds check` / `mds build` / `mds build --dry-run`、`--package`、`--verbose` の CLI を追加した。
- 2026-04-26: `mds-core` に TypeScript / Python / Rust の最小 fixture test と config warning / Uses.Target error の回帰 test を追加した。
- 2026-04-26: `bash /workspace/.agents/scripts/validate_harness.sh --root /workspace --verbose` は OK、0 warnings。
- 2026-04-26: `apt-get` で Rust toolchain と rustfmt を導入し、依存なし workspace 構成へ変更した。
- 2026-04-26: `mds-core` に package metadata 同期診断、manifest 破損時の build 停止、非管理ファイル上書き拒否、SHA-256 の回帰 test を追加した。
- 2026-04-26: `mds-cli` に CLI option parser の unit test を追加した。
- 2026-04-26: `cargo fmt --check`、`cargo test`、`bash /workspace/.agents/scripts/validate_harness.sh --root /workspace --verbose` は成功した。
- 2026-04-26: root `mds.config.toml` と package `mds.config.toml` の key 単位 merge、`package.md` Dependencies / Dev Dependencies と metadata の version 同期診断、table 必須列エラー、`index.md` Exposes 由来の Rust module block 生成、dry-run diff 対象確認を追加した。
- 2026-04-26: Parser + 生成 MVP の未完了事項を覆う regression test を追加し、`cargo fmt --check`、`cargo test`、`bash /workspace/.agents/scripts/validate_harness.sh --root /workspace --verbose` は成功した。
- 2026-04-26: MVP 完了後検証として `.agents/project.md`、`docs/project/validation.md`、関連 MVP spec、adapter spec を再確認し、`cargo fmt --check`、`cargo test`、`bash /workspace/.agents/scripts/validate_harness.sh --root /workspace --verbose` が成功することを確認した。
- 2026-04-26: `target/mds-validation/pkg` に TypeScript / Python / Rust の一時 fixture を作成し、native CLI で `cargo run -q --bin mds -- check --package target/mds-validation/pkg`、`cargo run -q --bin mds -- build --dry-run --package target/mds-validation/pkg`、`cargo run -q --bin mds -- build --package target/mds-validation/pkg`、build 後の再 `check` が成功することを確認した。
- 2026-04-26: native CLI の dry-run は書き込みなしで Source / Types / Test、Rust `src/lib.rs`、`.mds/manifest.toml` の unified diff を表示し、build は 14 files written として 3 言語の期待 path、生成 header、manifest、Rust module block を生成することを確認した。

## 未実施検証 / 残リスク

- npm / Python 配布 wrapper は MVP 対象外として未作成。
- Parser + 生成 MVP としての実装・検証ブロッカーは残っていない。

## 次に読むもの

- `.agents/project.md`
- `.agents/tasks/task-parser-generation-mvp-implementation-plan.md`
- `.agents/tasks/task-implementation-readiness-review.md`
- `.agents/tasks/task-requirements-spec-gap-analysis.md`
- `docs/project/specs/shared/SPEC-parser-generation-mvp-phase.md`
- `docs/project/specs/shared/SPEC-markdown-document-model.md`
- `docs/project/specs/shared/SPEC-expose-uses-tables.md`
- `docs/project/specs/shared/SPEC-code-generation-output.md`
- `docs/project/specs/shared/SPEC-cli-commands.md`
- `docs/project/specs/shared/SPEC-config-toml-resolution.md`
- `docs/project/specs/shared/SPEC-package-boundary-detection.md`
- `docs/project/specs/packages-lang-ts/SPEC-adapter-typescript-generation.md`
- `docs/project/specs/packages-lang-py/SPEC-adapter-python-generation.md`
- `docs/project/specs/crates-mds-lang-rs/SPEC-adapter-rust-generation.md`
- `docs/project/validation.md`
