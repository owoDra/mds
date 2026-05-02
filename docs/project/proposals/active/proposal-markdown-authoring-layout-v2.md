# Markdown Authoring Model V2

## 背景

現在の mds は `roots.markdown` で 1 つの Markdown root を選び、その配下の implementation md から生成物を導出する前提になっている。この前提だと、package 内の source / test の責務が Markdown 上で分かれにくく、internal import / export の関係も生成後 path を頭の中で補完しないと読み取りづらい。

self-hosted 移行では特に次の問題が目立った。

- 単一 Markdown root に source と test 相当の情報が混在し、生成後 directory との対応が直感的でない。
- 1 つの implementation md に実装とテストを同居させる前提が、文脈肥大化と AI による勝手な分割を招く。
- package metadata は正である一方、`overview.md` には dependency の俯瞰がなく、package ルールと dependency の両方を読むために複数ファイルを行き来する必要がある。
- `mds check` は現状レイアウトのつらさを十分に診断できず、望ましい authoring model へ強く誘導できない。

## 提案内容

1. Markdown root は任意指定ではなく固定 logical authoring root にする。

- package root に `mds.config.toml` がある場合、Markdown 正本 root は `.mds/source/` と `.mds/test/` を固定で解決する。
- 現行の `roots.markdown` による任意 directory 指定は廃止し、doc kind と root の関係を convention で固定する。
- `.mds/source/overview.md` は source rule と dependency snapshot、`.mds/test/overview.md` は test rule を担う。

2. implementation md と test md を完全分離する。

- `.mds/source/**/*.md` は source implementation md とし、`Purpose`、`Contract`、`Expose`、`Uses`、`Types`、`Source`、`Cases` を扱う。
- `.mds/test/**/*.md` は test md とし、`Purpose`、`Covers`、`Uses`、`Cases`、`Test` を扱う。
- `.mds/source` 側に `Test` code block を置くこと、`.mds/test` 側に `Source` / `Types` code block を置くことを禁止する。
- source と test は file 名の一致ではなく、論理 module id と `Covers` 参照で結び付ける。

3. internal dependency は generated file path ではなく logical module id で表す。

- `Uses.Target` の internal 参照は `foo/bar` のような logical authoring root 相対の logical module id を canonical にする。
- core は doc graph と logical module id を解決し、adapter が実際の import / use / require と file 配置へ変換する。
- author は build 後 path や拡張子を意識せず、論理単位の依存だけを書けばよい。

4. `.mds/source/overview.md` に metadata 由来の dependency snapshot を持たせる。

- package metadata は引き続き正とする。
- ただし `.mds/source/overview.md` には managed section として package summary、dependencies、dev dependencies の snapshot を自動生成する。
- `Rules` や architecture 説明などの手書き領域は保持し、managed section だけが同期対象になる。
- managed section の唯一の writer は `mds package sync` とし、`mds check` と `mds build` は同期ずれを診断するだけにとどめる。
- package manager post hook は opt-in 前提を維持したうえで、既定 command を `mds package sync` にする。

5. `mds check` を authoring model 起点で厳格化する。

- `.mds/` 配下の fixed root と `overview.md` の配置、doc kind ごとの必須 section、許可される code block 種別を検査する。
- `Uses.Target` の logical module id、`Covers` の参照解決、managed dependency snapshot の同期ずれを診断する。
- 旧 1-root / 1-md model は warning ではなく migration error として扱い、first-party package は即時移行対象にする。

6. adapter は core 内の data-driven language descriptor として再編する。

- core は Markdown parser、doc kind 判定、graph 解決、diagnostics、logical module id 解決を担う。
- 言語ごとの import 変換、output path、test path、manifest 連携、module file 更新ルールは bundled な `<ext>.toml` descriptor で定義する。
- generated test root の canonical は language ごとに descriptor が定義し、authoring model 側では `test/` と `tests/` のどちらかへ統一しない。
- package root の `.mds/languages/<ext>.toml` により built-in にない言語、または package 固有 variation を追加できるようにする。
- package descriptor が built-in または別 descriptor を土台にするときは、暗黙継承ではなく明示的な `extends` を使う。
- descriptor override は section 単位の whitelist merge とし、map は key override、array は section ごと置換、未知 key は error にする。
- LSP は同じ descriptor registry を利用し、built-in 言語と custom 言語の両方を同じ意味体系で扱う。

### descriptor 骨格

- descriptor は少なくとも `language`、`files`、`imports`、`quality_defaults` を持つ。
- Rust など追加責務がある言語は `module_management` のような optional section を持てる。
- built-in descriptor id は既存 canonical key に合わせて `ts`、`py`、`rs` のような短縮 key を使う。
- `files` は freeform template string ではなく structured fields で持ち、source / types / test ごとの root kind、basename rule、suffix、extension を定義する。
- `imports` は freeform template や mini DSL ではなく declarative enum と capability flag で持ち、`Uses` から import / use / require へ変換する canonical rule を定義する。
- `quality_defaults` は language 別の既定 lint / format / test / typecheck command と required / optional tool id を定義する。
- `module_management` は generic optional section とし、少なくとも managed_files、markers、insert_strategy、declaration_style の standardized field を持つ。
- built-in descriptor registry は `mds-core` に同梱し、self-hosted 正本は `mds-core/src-md/src/descriptors/` に置く。

7. first-party package と生成 template は一括で切り替える。

- mds 自身の package、examples、init が生成する AI Kit / project skeleton は同じ authoring model を前提に更新する。
- 新旧 model の長期併存は避け、`.mds/` fixed root への breaking change として 1 つの移行で済ませる。

## 今回の確認で固定した判断

- logical authoring root 名は hidden directory の `.mds/` に固定する。
- first-party package、examples、init template、AI Kit template は旧 `src-md` model と長期併存させず、breaking change として一括移行する。
- test md の canonical section は `Covers` を新設する。
- generated test root の canonical は authoring model では固定せず、language descriptor が言語ごとに定義する。
- dependency snapshot managed section の唯一の writer は `mds package sync` とする。
- dependency snapshot が stale な場合、`mds check` と `mds build` はともに error で止める。
- package manager post hook の既定 command は `mds package sync` とする。
- language descriptor の継承は明示的な `extends` を使う。
- language descriptor override は section 単位の whitelist merge を使う。
- built-in descriptor id は既存 canonical short key を使う。
- descriptor の `files` section は structured fields を使う。
- descriptor の `imports` section は declarative enum と capability flag を使う。
- descriptor の `quality_defaults` は default command と required / optional tool id までを持つ。
- descriptor の `module_management` は generic optional section と standardized field を使う。
- built-in descriptor registry は `mds-core` に同梱する。

## 推奨する移行順序

1. doc kind と fixed root を requirement / spec に昇格し、`.mds/source` / `.mds/test` を canonical にする。
2. `mds check` と LSP に migration error を追加し、旧 `src-md` / implementation-test 同居と dependency snapshot drift を早期検知できるようにする。
3. `mds package sync` を `.mds/source/overview.md` の managed snapshot writer として実装し、`mds build` は stale snapshot を error として拒否する。
4. first-party package、examples、init template、AI Kit template を同じ release で `.mds/` model へ切り替える。
5. adapter descriptor と logical module id 解決をその release の内部実装として同時に差し替え、旧 model の互換分岐を残さない。

## 代替案

1. 現行の single root を維持し、`mds check` と運用ルールだけで吸収する。

- 変更量は少ないが、source / test の責務分離が Markdown 上で曖昧なまま残る。
- self-hosted で再発した認知コストを structure ではなく慣れで吸収することになり、AI 利用にも不利。

2. `src-md` / `test-md` のような physical output 名に近い root を固定する。

- output root 名の揺れを Markdown 側へ持ち込むため、test 系の language 差分を authoring model から追い出しきれない。
- self-hosted で困っている「generated path を想像しながら書く」負担を十分には下げられない。

3. `src/` や `tests/` に `.md` を sidecar 配置し、generated code と同居させる。

- path 対応は分かりやすいが、source of truth と generated code の境界が曖昧になる。
- package が大きくなるほど、同一 directory に `.md` と生成ファイルが並ぶ管理コストが増える。

4. adapter を hardcoded Rust / TypeScript / Python 実装のまま増やしていく。

- 短期的には速いが、custom language と LSP 共有の要求を満たしにくい。
- path / import / manifest 差分をコードへ埋め込み続けるため、今の「生成後構造を想像しないと書けない」問題が温存される。

## 利点

- source と test の責務が logical root と doc kind の両方で明確になる。
- author は logical module id を使うだけでよく、generated path を先読みする負担が下がる。
- `.mds/source/overview.md` で package rule と dependency の俯瞰を同じ場所で確認できる。
- `mds check` と LSP が「望ましい形」を構造的に説明できる。
- built-in 言語と custom 言語の扱いを descriptor に寄せることで、core と LSP の意味体系を揃えやすい。

## リスク

- 現行 requirement / spec / fixtures / self-hosted package / init template の広範な breaking update が必要になる。
- `Covers` など新しい canonical section を導入する場合、label override と parser migration の整理が必要になる。
- dependency snapshot の managed section 方式を誤ると、手書き領域との衝突や package metadata の二重管理に見える危険がある。
- language descriptor を TOML 化すると、schema versioning と validation の責務が増える。
- hidden directory を使うため、editor 設定や人間の discoverability への配慮が必要になる。

## 未確定事項

- `module_management.declaration_style` や marker default 名の exact enum / field 名。
- built-in descriptor を core へ埋め込む build step と LSP の reload 戦略。

## 正式化先候補

- requirement:
  - `docs/project/requirements/REQ-doc-model-markdown-document-types.md`
  - `docs/project/requirements/REQ-generation-code-output-rules.md`
  - `docs/project/requirements/REQ-quality-md-state-validation.md`
  - `docs/project/requirements/REQ-ai-agent-cli-initialization.md`
- spec:
  - `docs/project/specs/shared/SPEC-markdown-document-model.md`
  - `docs/project/specs/shared/SPEC-config-toml-resolution.md`
  - `docs/project/specs/shared/SPEC-cli-commands.md`
  - `docs/project/specs/shared/SPEC-package-sync.md`
  - `docs/project/specs/shared/SPEC-adapter-typescript-generation.md`
  - `docs/project/specs/shared/SPEC-adapter-python-generation.md`
  - `docs/project/specs/shared/SPEC-adapter-rust-generation.md`
- validation:
  - `docs/project/validation.md`
- adr:
  - `docs/project/adr/active/` に authoring model v2 と language descriptor 化の判断を追加する余地がある。

## 関連資料

- `../../architecture.md`
- `../../requirements/REQ-doc-model-markdown-document-types.md`
- `../../specs/shared/SPEC-markdown-document-model.md`
- `../../specs/shared/SPEC-config-toml-resolution.md`
- `../../specs/shared/SPEC-cli-commands.md`
- `../../specs/shared/SPEC-package-sync.md`
- `../../validation.md`