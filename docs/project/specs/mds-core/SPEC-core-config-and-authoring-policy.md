---
id: SPEC-core-config-and-authoring-policy
status: 提案中
related:
  - ../shared/SPEC-model-package-layout.md
  - ../shared/SPEC-authoring-markdown-format.md
  - ../shared/SPEC-language-extension-contract.md
  - ../../requirements/v1/REQ-product-markdown-source-of-truth.md
  - ../../requirements/v1/REQ-ux-section-title-independence.md
  - ../../requirements/v1/REQ-quality-language-and-toolchain-independence.md
subproject: mds-core
---

# mds-core Config And Authoring Policy

## 概要

`mds-core` が解釈する package config、capability schema、authoring policy、label override、doc kind 判定、link policy mode の契約を定義する。

## 関連要求

- `REQ-product-markdown-source-of-truth`
- `REQ-ux-section-title-independence`
- `REQ-quality-language-and-toolchain-independence`

## 入力

- `mds.config.toml`
- package root
- authoring doc path
- optional capability schema
- package manager metadata / scripts

## 出力

- package config
- source/test root 判定
- doc kind / doc profile 判定
- label override
- link policy mode
- doctor policy と tool / version expectation
- quality slot config と capture rule

## 挙動

- `mds-core` は `mds.config.toml` と必要に応じて参照される capability schema を読み、package、check、roots、output、quality、doctor、package sync、labels、language/capture policy を解釈する。
- v1 の canonical root は `.mds/source` `.mds/test` とし、config でもこの root を用いる。
- link policy mode は package 単位で `wiki-only` `markdown-only` `mixed` の 3 モードを持つ。
- link policy の既定値は `wiki-only` とする。
- label preset と label override は canonical semantic に対する表示名マッピングとして package 単位で設定できる。
- doctor policy は required / optional tool と version expectation を config から解釈できる。
- quality slot command と diagnostic capture rule は package config または capability schema から解釈できる。
- source doc は path と内容から impl / spec / overview を判定できる。
- prose-only source doc は root module doc や spec 的 source doc として許容する。

## 状態遷移 / 不変条件

- 1 package の config 解釈結果は決定的であること。
- link policy は package 内で一貫し、validation と fix の両方に使われること。
- source/test root、doc kind、doc profile 判定は generation / validation / LSP で共有されること。

## エラー / 例外

- 不正 TOML や unsupported config value は error または warning とする。
- canonical root に反する構成は v1 では不正として扱える。
- 不明 label override key は error とする。
- link policy と文書内容が矛盾する場合、lint error または fix 対象とする。

## 横断ルール

- config 契約は CLI、LSP、VS Code extension から一貫して観測されること。
- 将来 v2 で資料種別が増えても、package 単位 policy 解釈の形は維持できること。

## 検証観点

- canonical root と doc kind が一貫判定される。
- 3 link policy mode が解釈される。
- label override が section / table column に反映される。
- prose-only source doc が意図どおり許容される。

## 関連資料

- `../shared/SPEC-model-package-layout.md`
- `../shared/SPEC-authoring-markdown-format.md`
- `../shared/SPEC-language-extension-contract.md`
- `../../requirements/v1/REQ-product-markdown-source-of-truth.md`
- `../../requirements/v1/REQ-ux-section-title-independence.md`
- `../../requirements/v1/REQ-quality-language-and-toolchain-independence.md`
