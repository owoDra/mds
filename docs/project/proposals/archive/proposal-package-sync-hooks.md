# Package Sync と任意 Hook

## 状態

archived: 2026-04-26 に採用し、`docs/project/specs/shared/SPEC-package-sync.md` へ昇格した。

## 背景

`REQ-cli-command-surface` と `REQ-monorepo-package-boundary` は、package metadata と `package.md` の整合性を保つことを要求している。MVP では不整合診断までを扱ったため、次に自動同期と package manager hook を仕様化する。

## 提案内容

- `mds package sync` は package metadata を正として、`package.md` の `Package`、`Dependencies`、`Dev Dependencies` を更新する。
- `Rules` など手書き補足領域は更新対象外にする。
- `mds package sync --check` は書き込みなしで差分と診断を返す。
- package manager hook は任意機能として採用し、post 実行のみを対象にする。
- hook は npm / Cargo / uv の依存変更後に `mds package sync --check` または `mds package sync` を呼び出す契約を仕様化する。
- hook 自体は mds の実行時必須依存にせず、利用者が明示的に有効化する。
- CLI exit code は成功 0、同期差分または診断あり 1、usage/config error 2、internal error 3、environment 不足 4 とする。

## 代替案

- check 診断のみ: 安全だが、`package.md` 更新が手作業になり要求の達成度が低い。
- pre + post hook: 変更前後の検証は強いが、package manager ごとの差分が大きく複雑になる。
- hook を自動組み込みする: 便利だが、利用者の package manager 操作へ暗黙に介入するため採用しない。

## 利点

- package metadata と `package.md` のずれを自動で修正できる。
- 手書き領域を保護しながら、生成管理部分だけを同期できる。
- post hook に限定することで package manager 差分を抑えられる。

## リスク

- `package.md` の更新範囲 marker が曖昧だと手書き内容を破壊する恐れがある。
- npm / Cargo / uv の metadata schema 差分が同期仕様を複雑にする。
- hook の既定 command を sync にするか check にするかで破壊性が変わる。

## 未確定事項

- `package.md` の生成管理範囲 marker を導入するか、既存セクション名だけで判定するか。
- hook の既定 command を `sync --check` にするか `sync` にするか。
- dependency group、optional dependency、workspace dependency、path dependency の表現。
- package manager ごとの hook 配置方法。

## 正式化先候補

- `../specs/shared/SPEC-package-sync.md`
- `../specs/shared/SPEC-package-boundary-detection.md`
- `../specs/shared/SPEC-cli-commands.md`
- `../specs/shared/SPEC-config-toml-resolution.md`
- `../validation.md`

## 関連資料

- `../../requirements/REQ-cli-command-surface.md`
- `../../requirements/REQ-monorepo-package-boundary.md`
- `../../validation.md`
