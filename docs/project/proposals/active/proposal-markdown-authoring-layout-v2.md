# Markdown Authoring Model V2

## 背景

現在の mds は `roots.markdown` で 1 つの Markdown root を選び、その配下の implementation md から生成物を導出する前提になっている。この前提だと、package 内の source / test の責務が Markdown 上で分かれにくく、internal import / export の関係も生成後 path を頭の中で補完しないと読み取りづらい。

self-hosted 移行では特に次の問題が目立った。

- 単一 Markdown root に source と test 相当の情報が混在し、生成後 directory との対応が直感的でない。
- 1 つの implementation md に実装とテストを同居させる前提が、文脈肥大化と AI による勝手な分割を招く。
- package metadata は正である一方、`overview.md` には dependency の俯瞰がなく、package ルールと dependency の両方を読むために複数ファイルを行き来する必要がある。
- `mds lint` は現状レイアウトのつらさを十分に診断できず、望ましい authoring model へ強く誘導できない。

## 提案内容

0. authoring style は「読みやすい仕様書 + 実コード解析」を標準にする。

- 新規 source md は `Purpose / Contract / Exports / Imports / Source / Cases / Test` の固定英語 section と表形式メタ情報を必須にしない。
- Markdown 本文は自然な仕様書として保ち、標準 section は日本語では `仕様`、`API`、`実装`、`検証`、test md では `対象`、`ケース`、`実装` を使う。
- `Imports` / `Exports` / `Uses` table は legacy 互換として読み続けるが、新規生成、snippet、example では使わない。
- `import` / `export` の事実は `## 実装` 配下の実コードから解析する。依存の理由だけを自然文と `[[module#symbol]]` 参照で書く。
- frontmatter は原則不要とし、module id は `.mds/source` または `.mds/test` からの logical module id で解決する。
- Markdown 参照は `[[module]]` と `[[module#symbol]]` を canonical とし、docs build では通常の Markdown link へ変換できるようにする。

- 参照形式のデフォルト挙動はパッケージ設定で制御する。既定では wiki-style の `[[module]]` を優先する（例: `wiki_links = true`）ことを推奨しつつ、パッケージ設定で `links_mode` を `normal`（通常リンクのみ）、`wiki`（wiki link のみ）、`mixed`（両方許可）などに切り替えられるようにする。docs build / LSP / lint はこの設定に従って表示と変換のポリシーを適用する。

1. Markdown root は任意指定ではなく固定 logical authoring root にする。

- package root に `mds.config.toml` がある場合、Markdown 正本 root は `.mds/source/` と `.mds/test/` を固定で解決する。
- 現行の `roots.markdown` による任意 directory 指定は廃止し、doc kind と root の関係を convention で固定する。
- `.mds/source/overview.md` は source rule と dependency snapshot、`.mds/test/overview.md` は test rule を担う。`overview.md` には `Imports` / `Exports` を置かない。
- package / directory root の `Imports` / `Exports` は `index.md` ではなく、Rust の `lib.rs.md` / `mod.rs.md`、TypeScript の `index.ts.md` など言語別 root module md に置く。

2. implementation md と test md を完全分離する。

- `.mds/source/**/*.md` は source document とし、`仕様` / `Contract`、`API`、`実装` / `Implementation`、`検証` を自然文中心で扱う。root module md も table なしで成立する。
- `.mds/test/**/*.md` は test document とし、`対象` / `Covers`、`ケース` / `Cases`、`実装` / `Implementation` を扱う。
- `.mds/source` 側に `Test` code block を置くこと、`.mds/test` 側に `Source` / `Types` code block を置くことを禁止する。
- source と test は file 名の一致ではなく、論理 module id と `対象` / `Covers` の `[[module]]` 参照で結び付ける。

3. internal dependency は generated file path ではなく logical module id で表す。

- core は doc graph、code import、Markdown wiki link を index 化し、logical module id を解決する。
- LSP はコード内 import path と import symbol から対応する `.mds/source/**/*.md` と symbol 定義へジャンプする。
- author は build 後 path や拡張子を意識せず、自然文では `[[module#symbol]]`、コードでは通常の import を書けばよい。

4. `.mds/source/overview.md` に metadata 由来の dependency snapshot を持たせる。

- package metadata は引き続き正とする。
- ただし `.mds/source/overview.md` には managed section として package summary、dependencies、dev dependencies の snapshot を自動生成する。
- `Rules` や architecture 説明などの手書き領域は保持し、managed section だけが同期対象になる。
- managed section の唯一の writer は `mds package sync` とし、`mds lint` と `mds build` は同期ずれを診断するだけにとどめる。
- package manager post hook は opt-in 前提を維持したうえで、既定 command を `mds package sync` にする。

5. `mds lint` を authoring model 起点で厳格化する。

- `.mds/` 配下の fixed root と `overview.md` の配置、doc kind ごとの必須 section、許可される code block 種別を検査する。
- default validator では code fence 整合、Markdown link、duplicate H2、import 混在、doc comment / docstring、top-level 実装の code fence 分離を検査し、`[check]` で個別に on/off できるようにする。
- `[[module]]` / `[[module#symbol]]`、`対象` / `Covers` の参照解決、code import / export 解析、managed dependency snapshot の同期ずれを診断する。
- legacy `Imports` / `Exports` / `Uses` table は既定で warning とし、CI では `legacy_tables = "error"` へ昇格できるようにする。
- 旧 1-root / 1-md model は authoring-v2 を採用する package では warning ではなく migration error として扱う。ただしこの repository の first-party package は本 proposal の移行対象に含めない。
- `mds lint --fix` による自動修正機能を提供する。`--fix` はパッケージの `links_mode` 設定に従い、既存の参照表記（`[[module]]` / `[[module#symbol]]` と通常の Markdown link）を相互に変換してファイルを置換する。変換は source map と参照解決を用いて安全に行い、冪等性を保つ（必要に応じて変更のプレビューやバックアップ、CI 向けの自動承認フラグを提供する）。

6. core は言語非依存にし、言語意味論は外部 LSP / provider へ委譲する。

- core は Markdown parser、doc kind 判定、code fence 抽出、logical module id 解決、wiki-link 解決、source map、output planning だけを担う。
- core は `.ts.md` / `.rs.md` / `.py.md` のようなファイル名 suffix を opaque な extension key として扱い、`ts` / `rs` / `py` の意味分岐を持たない。
- import / export の厳密解析は core の必須責務にしない。build は import / export index が存在しなくても成立する。
- import / export、symbol definition、hover、references、rename、type-aware diagnostics は、mds LSP または editor extension が既存 language server へ問い合わせて取得する。
- output root、source/test root、source/test file naming は language descriptor ではなく package config の pattern として扱う。
- descriptor は新標準の主機構にしない。残す場合も `ext`、fence label alias、特殊 file mapping などの薄い optional metadata に限定し、built-in language descriptor を増やす方針は採らない。

### 外部 LSP 委譲モデル

- mds LSP は Markdown code fence と generated file の対応を source map として保持する。
- definition / hover / completion / references が code fence 内で要求された場合、mds LSP は Markdown range を virtual/generated code range へ変換する。
- 既存 language server が返した generated code location は、source map により `.mds/source/**/*.md` または `.mds/test/**/*.md` の Markdown range へ戻す。
- editor が virtual document を安定して扱える場合は `mds-virtual:` URI を使える。
- virtual URI を language server が扱いにくい場合は、`mds build` 後の generated file を既存 language server に任せ、mds LSP が generated location を Markdown location へ戻す。

### source map の最小要件

- 各 code fence について、Markdown file path、Markdown range、generated/virtual file path、generated range、language extension key を記録する。
- 複数 code fence を 1 generated file へ連結する場合、fence ごとの offset table を保持する。
- source md と test md は別々の output kind として source map を持つ。
- source map は LSP、docs build、graph、diagnostics の共有 index として使う。

### package output config 案

```toml
[roots]
source_md = ".mds/source"
test_md = ".mds/test"
source_out = "src"
test_out = "tests"

[output]
source = "{source_out}/{module}.{ext}"
test = "{test_out}/{module}.test.{ext}"
```

- `{module}` は `.mds/source` または `.mds/test` からの logical module path を使う。
- `{ext}` は Markdown file name の language suffix から取る。
- 言語固有の特殊配置が必要な場合だけ package config の override で扱い、core に hardcode しない。

7. first-party self-hosting removal は proposal の適用範囲から切り分ける。

- この proposal は product の Markdown authoring model を定義するものであり、この repository の first-party package、examples、init が生成する AI Kit / project skeleton を `.mds` authoring-v2 へ移行する計画は含めない。
- この repository では first-party self-hosting を breaking alpha change として削除し、first-party package 向け migration command は作らない。

## 今回の確認で固定した判断

- logical authoring root 名は hidden directory の `.mds/` に固定する。
- この proposal で固定するのは product の authoring-v2 model であり、この repository の first-party package、examples、init template、AI Kit template は移行対象に含めない。first-party self-hosting は breaking alpha change として削除し、migration command は作らない。
- test md の canonical section は `Covers` を新設する。
- generated test root の canonical は authoring model では固定せず、language descriptor が言語ごとに定義する。
- dependency snapshot managed section の唯一の writer は `mds package sync` とする。
- `overview.md` は `Imports` / `Exports` を持たず、package / directory root の import / export surface は言語別 root module md に置く。
- dependency snapshot が stale な場合、`mds lint` と `mds build` はともに error で止める。
- package manager post hook の既定 command は `mds package sync` とする。
- descriptor-driven adapter は主方針から外し、core は言語非依存を優先する。
- 言語は file suffix から extension key として推論し、core は extension key の意味を解釈しない。
- output path は package config pattern を正とし、言語ごとの descriptor では定義しない。
- import / export 解析、symbol definition、hover、references、rename は mds core ではなく LSP / editor extension / optional provider へ委譲する。
- built-in language descriptor registry は増やさない。必要な metadata は薄い optional metadata または package override に限定する。

## 推奨する移行順序

1. doc kind と fixed root を requirement / spec に昇格し、`.mds/source` / `.mds/test` を canonical にする。
2. `mds lint` と LSP に migration error を追加し、旧 `src-md` / implementation-test 同居と dependency snapshot drift を早期検知できるようにする。
3. `mds package sync` を `.mds/source/overview.md` の managed snapshot writer として実装し、`mds build` は stale snapshot を error として拒否する。
4. この repository では first-party self-hosting を breaking alpha change として削除し、first-party package 向け migration command は作らない。product-facing な authoring-v2 rollout は別途 formalization する。
5. descriptor-driven adapter ではなく、source map と package output config を導入し、LSP / editor extension が外部 language server へ問い合わせられる形へ移行する。

## 代替案

1. 現行の single root を維持し、`mds lint` と運用ルールだけで吸収する。

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

5. core に built-in language descriptor registry を持たせる。

- data-driven にはなるが、descriptor schema、継承、override、言語ごとの差分管理が mds core の責務として残る。
- 新言語対応のたびに mds 側の metadata 追加が必要になり、「Markdown と source map だけを扱う core」という境界が曖昧になる。

6. core に正規表現ベースの import / export extractor を持たせる。

- 短期的には LSP なしでも graph を作れるが、言語ごとの分岐と精度問題が core に入り込む。
- 既存 language server が持つ symbol 解決、rename、references、type-aware diagnostics と二重実装になる。

## 利点

- source と test の責務が logical root と doc kind の両方で明確になる。
- author は logical module id を使うだけでよく、generated path を先読みする負担が下がる。
- `.mds/source/overview.md` で package rule と dependency の俯瞰を同じ場所で確認できる。
- `mds lint` と LSP が「望ましい形」を構造的に説明できる。
- core を言語非依存に保つことで、新言語追加時に mds core の descriptor や分岐を増やさずに済む。
- 既存 language server を利用することで、import/export、definition、hover、references、rename の精度を mds 側で再実装しなくてよい。
- source map を共有 index とすることで、generated file と Markdown 正本の間を LSP / docs / diagnostics で同じ規則で往復できる。

## リスク

- product-facing requirement / spec / fixtures の breaking update と、この repository の first-party package / examples / init template から self-hosted 前提を外す cleanup が必要になる。
- `Covers` など新しい canonical section を導入する場合、label override と parser migration の整理が必要になる。
- dependency snapshot の managed section 方式を誤ると、手書き領域との衝突や package metadata の二重管理に見える危険がある。
- 外部 language server 連携は editor / language server ごとの差分が大きく、virtual document URI を扱えない language server では generated-file mode が必要になる。
- source map の精度が低いと、definition / diagnostics の Markdown range remap が不正確になる。
- hidden directory を使うため、editor 設定や人間の discoverability への配慮が必要になる。

## 未確定事項

- source map の永続化 format と更新タイミング。
- generated-file mode と virtual-document mode の優先順位。
- editor extension 側で外部 language server bridge を担うか、mds-lsp が language server client を内包するか。
- package output config pattern の exact field 名と escaping rule。

## 正式化先候補

- requirement:
  - `docs/project/requirements/REQ-doc-model-markdown-document-types.md`
  - `docs/project/requirements/REQ-generation-code-output-rules.md`
  - `docs/project/requirements/REQ-quality-md-state-validation.md`
  - `docs/project/requirements/REQ-ai-agent-cli-initialization.md`
- implementation / follow-up:
  - この repository の first-party package を `.mds/source` へ移す formalization は行わない。product-facing な adoption scope は requirement / ADR で別途整理する。
- validation:
  - `docs/project/validation.md`
- adr:
  - `docs/project/adr/active/` に authoring model v2 と language descriptor 化の判断を追加する余地がある。

## 関連資料

- `../../architecture.md`
- `../../requirements/REQ-doc-model-markdown-document-types.md`
- `../../validation.md`
