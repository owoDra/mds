# Task 003: Enforce Link Policy And Overview Contract

## 目的

link policy 3 mode、`lint --fix` 文書正規化、`overview.md` managed region / `package sync` 契約を v1 spec どおりに完成させる。

## 前提条件

- core の config/schema runtime が package policy を解釈できる
- `SPEC-authoring-markdown-format` と `SPEC-core-overview-and-package-sync` の契約が参照できる

## 作業内容

- package 単位 `wiki-only` `markdown-only` `mixed` link policy 解釈と validation を追加する
- `mds lint --fix` に wiki-link / Markdown link 相互変換を追加する
- `overview.md` managed region の必須要件と `package sync --check` / write mode の差分を実装する
- package sync hook の案内または実行契約を spec に合わせて整える

## 完了条件

- link policy 違反が lint error または fix 対象として観測できる
- `lint --fix` が文書正規化として link policy を揃えられる
- `overview.md` 欠落や managed region 欠落が package error / sync error として観測できる

## 検証方法

- link policy 3 mode の parser / lint / fix test を追加する
- `examples/minimal-ts` で `package sync --check` と write mode を確認する

## 依存関係

- `task-001-complete-language-and-schema-resolution.md`
- `task-002-complete-quality-and-doctor-policy.md`

## 成果物

- `mds/core/src/config.rs`
- `mds/core/src/markdown.rs`
- `mds/core/src/quality.rs`
- `mds/core/src/package_sync.rs`
- `mds/core/tests/`
