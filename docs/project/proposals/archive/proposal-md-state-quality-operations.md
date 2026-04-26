# Markdown 状態の品質操作

## 状態

archived: 2026-04-26 に採用し、`docs/project/specs/shared/SPEC-md-state-quality-operations.md` へ昇格した。

## 背景

`REQ-quality-md-state-validation` は、生成後コードだけでなく Markdown 状態に対して check、lint、lint --fix、test を適用できることを求めている。Parser + 生成 MVP では実言語 toolchain 呼び出しを対象外にしたため、次の仕様化対象として確定する。

## 提案内容

- `mds lint`、`mds lint --fix`、`mds test` は Markdown 状態中心の操作とする。
- 対象コードは implementation md の `Types`、`Source`、`Test` のコードブロックとし、`Purpose`、`Contract`、`Cases` は実行対象コードにしない。
- `Uses` から adapter が仮想 import / use / require を生成し、toolchain 実行時の一時コードへ付与する。
- TypeScript は Prettier / ESLint / Vitest、Python は Ruff format / Ruff / Pytest、Rust は rustfmt / clippy / cargo test を代表 toolchain とする。
- `mds format` は廃止し、Markdown 正本への自動修正は `mds lint --fix` に統合する。
- format 失敗時は、成功したコードブロックだけを書き戻すブロック単位更新とする。
- 書き戻しは fenced code block の中身だけに限定し、見出し、表、説明文、`Uses`、`Expose` は更新しない。
- toolchain へ渡す一時ファイルは adapter が作成し、core は Markdown document model と診断集約を担う。
- CLI exit code は成功 0、lint / test / format 診断あり 1、usage/config error 2、internal error 3、toolchain / environment 不足 4 とする。

## 代替案

- 生成済みコード中心にする: 実装は容易だが、Markdown 正本の品質確認要求を満たしにくい。
- format は書き戻さない: 安全だが、Markdown 正本を常に手作業で整形する必要が残る。
- 失敗時は全体ロールバックする: 一貫性は高いが、独立したコードブロックの成功結果まで失う。

## 利点

- Markdown を正本とする要求に沿って、実コード品質を正本上で維持できる。
- adapter ごとの toolchain 差分を閉じ込められる。
- ブロック単位更新により、独立した成功結果を失わずに失敗箇所を診断できる。

## リスク

- ブロック単位更新では、複数ブロック間の整合性が一時的にずれる可能性がある。
- 仮想 import を含む一時コードと Markdown 内コードブロックの行番号対応が複雑になる。
- toolchain の version 差分により診断や format 結果が変わる可能性がある。

## 未確定事項

- `mds lint --fix` の check-only option 名。
- format 差分の stdout 形式。
- 一時コード上の行番号を Markdown file / section / code block line へ戻す診断 location schema。
- toolchain config file の探索範囲と mds config との優先順位。

## 正式化先候補

- `../specs/shared/SPEC-md-state-quality-operations.md`
- `../specs/shared/SPEC-cli-commands.md`
- `../specs/shared/SPEC-config-toml-resolution.md`
- `../specs/packages-lang-ts/SPEC-adapter-typescript-generation.md`
- `../specs/packages-lang-py/SPEC-adapter-python-generation.md`
- `../specs/crates-mds-lang-rs/SPEC-adapter-rust-generation.md`
- `../validation.md`

## 関連資料

- `../../requirements/REQ-quality-md-state-validation.md`
- `../../requirements/REQ-cli-command-surface.md`
- `../../requirements/REQ-adapter-required-language-adapters.md`
- `../../validation.md`
