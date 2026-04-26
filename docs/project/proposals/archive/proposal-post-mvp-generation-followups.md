# Post-MVP 要件達成計画

## 状態

archived: 2026-04-26 に採用し、関連 spec へ昇格した。

## 背景

Parser + 生成 MVP では、`mds check`、`mds build --dry-run`、`mds build` と TypeScript / Python / Rust の生成規則を先行して実装した。採用済み要件には、MVP 外として残した品質操作、doctor、package sync、配布、version、import 表現拡張が残っている。

## 提案内容

- 残要件は機能ごとに proposal を分割し、確認後に spec へ昇格する。
- `mds lint` / `mds lint --fix` / `mds test` は Markdown 状態中心で仕様化する。
- `mds doctor` は別 proposal として、CLI 出力、診断、環境確認を仕様化する。
- `mds package sync` と package manager post hook は別 proposal として仕様化する。
- `Uses` の default / alias / namespace / 言語固有 import 表現は別 proposal として仕様化する。
- npm / Cargo / uv 配布と最低対応 version は別 proposal として仕様化する。

## 代替案

- 1 つの巨大 spec にまとめる: 要件の全体像は見えやすいが、各機能の実装単位と検証単位が曖昧になるため採用しない。
- MVP spec へ追記する: 完了済み MVP の範囲が膨らみ、実装済み範囲と将来範囲が混ざるため採用しない。
- 優先機能だけ先行する: 進捗は早いが、requirements に挙がった範囲をすべて達成する今回の目的に合わないため採用しない。

## 利点

- requirements に残る未達事項を、仕様と実装の単位へ分解できる。
- proposal と採用済み spec の境界を維持できる。
- ユーザー確認後に、対象 spec へ段階的に昇格しやすい。

## リスク

- 機能間の共通事項、特に CLI exit code、config schema、diagnostic schema が重複しやすい。
- 複数 proposal を同時に更新するため、index と参照更新漏れが起きやすい。

## 未確定事項

- 各 proposal の完成計画を採用して spec 昇格へ進めてよいか。
- 横断的な diagnostic schema を単独 spec にするか、CLI spec に含めるか。

## 正式化先候補

- `../specs/shared/SPEC-cli-commands.md`
- `../specs/shared/SPEC-config-toml-resolution.md`
- `../specs/shared/SPEC-expose-uses-tables.md`
- `../specs/shared/SPEC-md-state-quality-operations.md`
- `../specs/shared/SPEC-doctor-command.md`
- `../specs/shared/SPEC-package-sync.md`
- `../specs/shared/SPEC-distribution-and-versions.md`
- `../tech-stack.md`

## 関連資料

- `../../requirements/REQ-cli-command-surface.md`
- `../../requirements/REQ-quality-md-state-validation.md`
- `../../requirements/REQ-platform-multi-ecosystem-distribution.md`
- `../../requirements/REQ-adapter-required-language-adapters.md`
- `proposal-md-state-quality-operations.md`
- `proposal-graph-doctor-commands.md`
- `proposal-package-sync-hooks.md`
- `proposal-import-expression-extensions.md`
- `proposal-distribution-version-policy.md`
