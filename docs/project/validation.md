# Validation

## 目的

このファイルは、mds を開発する過程で随時確認すべき検証項目を記録します。

## 読むべき場面

- 要求、仕様、設計、実装、テストを追加または変更するとき
- 変更後に何をどう確認すべきか整理したいとき
- 検証観点を追加または更新したいとき

## 仕様整合性

- いつ行うか: requirement、spec、architecture、implementation md、生成処理を変更するとき。
- 何で検証するか: 対象の正本、関連 spec、architecture、実装、テストを相互に読み合わせる。
- 期待する結果: 正本の約束、実装の振る舞い、テストの期待値が矛盾しない。
- 問題があった際にどうするか: 先に正本を修正し、実装またはテストだけで矛盾を吸収しない。

## Markdown 正本構造

- いつ行うか: `index.md`、`package.md`、implementation md、`mds.config.toml` の仕様や parser を変更するとき。
- 何で検証するか: Markdown fixture、構造検査、手元の代表サンプルを使う。
- 期待する結果: 必須セクション、`Expose`、`Uses`、`Cases`、`Types` / `Source` / `Test` の分離が仕様どおり扱われ、`Types` / `Source` / `Test` の実コードが正本として処理される。
- 問題があった際にどうするか: 例外的な入力を暗黙許容せず、仕様化するか明確に reject する。

## 生成コード整合性

- いつ行うか: build、出力パス、ファイル名規約、コードブロック連結、import 生成を変更するとき。
- 何で検証するか: fixture から生成した `.ts`、`.py`、`.rs` と期待出力を比較する。
- 期待する結果: `.md` 内のコードブロックから生成される Source、Types、Test が命名規約と出力先規則に従う。
- 問題があった際にどうするか: 生成物を手修正せず、正本または generator を修正する。

## Self-hosted Build 同期

- いつ行うか: `src-md/`、`scripts/sync-build.sh`、Rust package metadata、`.build/` 配置を変更するとき。
- 何で検証するか: `scripts/sync-build.sh` を実行し、`.build/rust/` で `cargo fmt --check` と `cargo test` を実行する。
- 期待する結果: `src-md/` の implementation md と package metadata から `.build/rust/` の Cargo workspace が再生成され、生成物を手編集せずに build / test できる。
- 問題があった際にどうするか: `.build/` を直接修正せず、`src-md/` または同期処理を修正する。

## Language Adapter 動作

- いつ行うか: language adapter、lint、lint --fix、test runner 接続、import / use / require 生成を変更するとき。
- 何で検証するか: TypeScript、Python、Rust の代表 fixture と各 adapter の lint / lint --fix / test 接続を確認する。
- 期待する結果: adapter ごとの出力、依存解決、fixer、linter、test runner が同じ概念を言語ごとに一貫して扱う。
- 問題があった際にどうするか: 言語固有の差分を adapter に閉じ込め、core の言語横断契約を崩さない。

## 設定継承

- いつ行うか: `mds.config.toml`、root / subproject 設定、label override、package 有効判定を変更するとき。
- 何で検証するか: root 設定、subproject 設定、未設定時の built-in default を含む fixture を使う。
- 期待する結果: built-in default、root、subproject の優先順位が守られ、見た目の語彙変更が意味変更にならない。
- 問題があった際にどうするか: 互換性のために曖昧な優先順位を増やさず、仕様または ADR で判断を確定する。

## Monorepo 境界

- いつ行うか: package 検出、workspace traversal、subproject 出力、混在 package 対応を変更するとき。
- 何で検証するか: mds 有効 package、mds 無効 package、複数言語 package が混在する fixture を使う。
- 期待する結果: `enabled = true`、`package.md`、実体の package 定義による mds package 判定が安定し、対象外 package を壊さない。
- 問題があった際にどうするか: 対象範囲の誤検出を優先的に修正し、未対応構成は明示的に未対応として扱う。

## 回帰防止

- いつ行うか: bug fix、parser / generator / adapter / CLI の変更、仕様変更のたび。
- 何で検証するか: 失敗を再現する fixture または test を追加し、既存 test suite を実行する。
- 期待する結果: 修正対象の失敗が再発せず、既存の代表フローも壊れない。
- 問題があった際にどうするか: 再現 test なしで修正完了にせず、回帰の原因範囲を正本へ反映する。

## CLI 振る舞い

- いつ行うか: `mds build`、`mds check`、`mds lint`、`mds lint --fix`、`mds test`、`mds doctor`、`mds package sync`、`mds init` を変更するとき。
- 何で検証するか: 正常系、入力不備、対象なし、部分失敗の CLI fixture または統合テストを使う。
- 期待する結果: 終了コード、標準出力、標準エラー、生成物、破壊的でない失敗動作が予測可能である。
- 問題があった際にどうするか: ユーザーが次に取るべき行動が分かるエラーへ修正し、曖昧な成功扱いを避ける。

## AI Agent CLI 初期化

- いつ行うか: `mds init`、`mds init --ai`、AI CLI template plugin、agent kit 生成物を変更するとき。
- 何で検証するか: Claude Code、Codex CLI、Opencode、GitHub Copilot CLI の代表 fixture、カテゴリ選択 fixture、上書き確認 fixture を使う。
- 期待する結果: instructions、skills、commands、workflows、docs が選択カテゴリどおり生成され、mds の正本参照導線と作業規約を含み、非管理ファイルは diff と確認なしに上書きされない。
- 問題があった際にどうするか: AI CLI 固有差分を template plugin 側で修正し、core の Markdown model や正本構造へ持ち込まない。

## 開発環境セットアップ

- いつ行うか: `mds init` の bootstrap、project dependencies、toolchains、global AI CLI 導入、非対話 option を変更するとき。
- 何で検証するか: `npx`、Cargo、`uvx` の bootstrap smoke test、対話 / 非対話 fixture、部分成功 / 部分失敗 fixture を使う。
- 期待する結果: interactive default が守られ、非対話実行では明示 option がない限り変更せず、導入失敗時は成功項目と失敗項目を分けて診断する。
- 問題があった際にどうするか: 自動導入の範囲を拡大せず、明示 option、plan、診断を追加して安全側に倒す。

## Markdown 状態の品質操作

- いつ行うか: `mds lint`、`mds lint --fix`、`mds test`、adapter の toolchain 接続、`Uses` import 生成を変更するとき。
- 何で検証するか: TypeScript、Python、Rust の Markdown fixture と toolchain 接続 fixture を使う。
- 期待する結果: Markdown 内の `Types` / `Source` / `Test` が仮想 import 付きで toolchain に渡り、`mds lint --fix` は code block の中身だけを安全に書き戻す。
- 問題があった際にどうするか: 正本構造を壊す fix 書き戻しを禁止し、adapter の診断 location を Markdown 上の位置へ戻す。

## Doctor / Package Sync

- いつ行うか: `mds doctor`、`mds package sync`、package manager hook を変更するとき。
- 何で検証するか: toolchain 有無と最低 version の doctor fixture、Cargo / package metadata sync fixture、package sync hook fixture を使う。
- 期待する結果: doctor は有効 adapter 分の runtime / toolchain を検出し、environment 不足または最低 version 未満を exit code 4 にし、package sync は手書き領域を壊さず package metadata 由来の管理部分だけを更新し、hook は明示有効化時だけ既定 command `mds package sync --check` を使う。
- 問題があった際にどうするか: 破壊的な自動更新を止め、`--check` や診断で利用者が次に取るべき対応を示す。

## 公開前品質

- いつ行うか: Cargo、native binary、VS Code extension の packaging、release metadata、publish 前検証を変更するとき。
- 何で検証するか: 現行配布経路の artifact fixture、checksum、署名、SBOM、provenance / attestations、install smoke test を使う。
- 期待する結果: 全 artifact に `.build/release/` 配下の supply-chain 成果物が紐づき、install 後に `mds --version` 相当と代表 command が動き、欠落や互換性不一致は publish 前 gate を失敗させる。
- 問題があった際にどうするか: publish を止め、artifact、wrapper、release metadata、署名または provenance の不足を修正する。

## ドキュメント同期

- いつ行うか: architecture、spec、ADR、tech-stack、validation、glossary、README のいずれかを変更するとき。
- 何で検証するか: 関連資料の参照リスト、用語、責務境界、検証項目を確認する。
- 期待する結果: 人間と AI エージェントが、説明と実コードを含む同じ正本から実装、修正、検証を再現できる。
- 問題があった際にどうするか: コード変更だけで完了せず、正本の不足を同じ変更または後続 task に明示する。
