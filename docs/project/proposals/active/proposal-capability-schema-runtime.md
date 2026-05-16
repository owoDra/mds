# Capability Schema Runtime Migration

## 背景

- 現行実装は built-in descriptor、tool registry、package manager registry への依存が強い。
- 仕様では、言語 identity は impl md の file suffix と code fence から判定でき、出力先は package config で決められる前提を置ける。
- 今後の新言語、新ツール対応で `mds` 本体に固有知識を継ぎ足す構造は、保守コストと追従コストを増やす。
- 一方で、diagnostic remap、source map、special file、quality slot、editor bridge などは引き続き共通 kernel として必要である。

## 提案内容

- built-in descriptor / tool profile / package manager registry 依存を縮小し、package config と外部 capability schema を中核にする。
- language identity は `*.lang.md` と code fence label を基準にする。
- output rule、special file rule、root module rule、quality slot command、diagnostic capture rule は package config または capability schema で宣言する。
- quality integration は `typecheck` `lint` `fix` `test` の slot semantic を中心にする。
- diagnostic remap は source map と capture 可能な path / line / column 情報に基づいて行う。
- CLI wizard は tool 選択中心ではなく、section semantic、link policy、quality slot summary、AI optional branch を中心にする。

## 代替案

- built-in descriptor を維持し、必要な言語と tool を都度追加する。
  - 不採用理由: 本体改修が増え、将来の拡張コストが高い。
- descriptor を完全廃止し、宣言も持たない。
  - 不採用理由: special file、quality capture、package metadata 読み取り、editor bridge の契約が失われる。
- built-in descriptor を残しつつ、config 上書きだけ許す。
  - 不採用理由: kernel と project policy の責務が曖昧なまま残る。

## 利点

- 新言語 / 新ツール対応の多くを config / schema 追加で進められる。
- `mds` 本体の変更頻度を下げられる。
- section semantic、link policy、quality slot、diagnostic remap など project ごとの差分を正しく宣言できる。
- v2 の project-wide traceability 拡張でも、kernel と policy の責務分離を維持しやすい。

## リスク

- schema 設計が弱いと、built-in descriptor より複雑で使いにくくなる。
- migration 途中では、旧 registry と新 schema が二重化する可能性がある。
- package manager metadata 読み取りや diagnostic capture の共通化が不十分だと、かえって project 側負担が増える。

## 未確定事項

- capability schema の配置形式と参照方法
- 旧 registry から schema への移行期間の扱い
- package manager metadata reader を schema にどこまで持たせるか
- diagnostic capture rule の最小 schema

## 正式化先候補

- `docs/project/architecture.md`
- `docs/project/specs/shared/SPEC-language-extension-contract.md`
- `docs/project/specs/mds-core/SPEC-core-config-and-authoring-policy.md`
- `docs/project/specs/mds-core/SPEC-core-quality-and-fix-pipeline.md`
- `docs/project/specs/mds-cli/SPEC-cli-init-and-new-workflows.md`
- `docs/project/specs/mds-cli/SPEC-cli-init-wizard-screen-flow.md`
- `docs/project/plan/capability-schema-migration/`

## 関連資料

- `../index.md`
- `../../architecture.md`
- `../../specs/shared/SPEC-language-extension-contract.md`
- `../../specs/mds-core/SPEC-core-config-and-authoring-policy.md`
- `../../specs/mds-core/SPEC-core-quality-and-fix-pipeline.md`
