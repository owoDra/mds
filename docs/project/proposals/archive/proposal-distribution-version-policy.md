# 配布と Version 方針

## 状態

archived: 2026-04-26 に採用し、`docs/project/specs/shared/SPEC-distribution-and-versions.md` と `docs/project/tech-stack.md` へ昇格した。2026-05-01 時点の現行構成では `packages/` と `python/` 配布単位は扱わない。

## 背景

`REQ-platform-multi-ecosystem-distribution` は npm、Cargo、uv から mds を導入できることを要求している。`tech-stack.md` は現時点で version が `tbd` のため、残要件達成に向けて最低対応 version と wrapper 契約を固定する。

## 提案内容

- 最低対応 version は最新寄せで固定する。
- Rust は 1.86+、Node.js は 24+、Python は 3.13+ を最低対応候補とする。
- Cargo は native CLI と Rust language adapter を Rust crate として扱う。
- Rust core の言語横断契約を正とする。
- toolchain は Prettier、ESLint、Vitest、Ruff、Pytest、rustfmt、clippy、cargo test の最新安定系列を対象にする。

## 代替案

- 保守的に広い version を支える: 利用者範囲は広がるが、初期実装と検証 matrix が大きくなる。
- version を固定しない: 柔軟だが、doctor と品質操作の検証条件が曖昧になる。
- npm 配布だけ先行する: 実装は楽だが、multi-ecosystem distribution 要求を満たさない。

## 利点

- 実装と検証 matrix を現代的な toolchain に限定できる。
- doctor が version mismatch を診断できる。
- 配布 wrapper の責務境界が明確になる。

## リスク

- 最新寄せにより、一部利用環境では導入障壁が高くなる。
- Node 24 / Python 3.13 などの採用状況により、エコシステム側の依存が追いつかない可能性がある。
- 各 package manager の公開名や binary 名の調整が必要になる。

## 未確定事項

- 正式な npm package name、Cargo crate name、Python package name。
- CLI binary name をすべて `mds` に揃えるか。
- version mismatch を warning にするか environment error にするか。
- 各 toolchain の最低 version を exact に固定するか、major 系列だけ固定するか。

## 正式化先候補

- `../specs/shared/SPEC-distribution-and-versions.md`
- `../tech-stack.md`
- `../validation.md`

## 関連資料

- `../../requirements/REQ-platform-multi-ecosystem-distribution.md`
- `../../requirements/REQ-adapter-required-language-adapters.md`
- `../../architecture.md`
- `../../tech-stack.md`
