# Parser + 生成 MVP 後続フェーズ

## 背景

Parser + 生成 MVP では、実装開始に必要な文書解析、table schema、path 解決、生成物、manifest、header、Rust module 管理を優先する。lint / format / test、graph、doctor、package sync、toolchain 呼び出し契約まで同時に扱うと、MVP の完了条件と検証対象が過大になる。

## 提案内容

- lint / format / test の toolchain 呼び出し契約を次フェーズで仕様化する。
- Markdown から `Types`、`Source`、`Test` のコードブロックを抽出し、`Uses` から仮想 import / use / require を作り、formatter、linter、test runner に渡す品質操作を次フェーズで仕様化する。
- format 結果を Markdown のコードブロックへ安全に戻す手順、失敗時に Markdown を更新しない条件、説明文を診断文脈として扱うルールを次フェーズで仕様化する。
- TypeScript の Prettier、ESLint、Vitest、Python の Ruff format、Ruff、Pytest、Rust の rustfmt、clippy、cargo test を代表 toolchain 候補として扱う。
- `mds graph` の出力形式、リンク解釈、依存 graph の安定順序を次フェーズで仕様化する。
- `mds doctor` の環境診断対象、adapter toolchain 検出、診断分類を次フェーズで仕様化する。
- `mds package sync` の更新範囲、package manager hook、`package.md` 自動同期を次フェーズで仕様化する。
- config の lint / format / test 設定、除外パス、表示名 override の詳細挙動を次フェーズで仕様化する。
- default import、alias、言語固有 import の高度な変換は次フェーズで扱う。
- `Uses.Expose` の配列表現や alias 表現を追加するかは次フェーズで判断する。
- tech-stack の具体 version 方針は次フェーズまたは専用 task で扱う。

## 代替案

- Parser + 生成 MVP にすべて含める: 一度に完成形へ近づくが、実装と検証が大きくなり、完了判定が曖昧になる。
- すべて未記録のまま残す: MVP は軽くなるが、後続で何を仕様化するか追跡しにくくなる。
- 採用済み spec に次フェーズ内容も混在させる: 参照箇所は減るが、今フェーズの実装義務と将来構想が混ざる。

## 利点

- 今フェーズの実装範囲を Parser + 生成 MVP に固定できる。
- 将来対応を捨てずに、採用済み仕様とは分けて追跡できる。
- 実装者が MVP 外の機能を誤って完了条件に含めるリスクを減らせる。

## リスク

- 次フェーズ proposal を参照せずに実装を進めると、MVP 後の拡張余地を見落とす可能性がある。
- MVP 実装中に次フェーズ項目が必要に見える場合、spec へ昇格する前に実装へ混ぜ込まれる可能性がある。

## 未確定事項

- lint / format / test の仮想コード生成と実 toolchain 呼び出しの境界。
- Markdown 構造エラーと language toolchain 失敗の診断分類。
- format 結果を Markdown 正本へ戻す安全条件。
- `mds graph` の出力形式と stable ordering。
- `mds doctor` の診断分類と exit code への対応。
- `mds package sync` が更新してよい `package.md` の範囲。
- package manager hook の任意実行方式。
- config の表示名 override、除外パス、品質操作設定の具体 schema。
- default import、alias、言語固有 import の追加表現。
- `Uses.Expose` の配列表現を採用するか。
- tech-stack の version 方針。

## 正式化先候補

- `../specs/shared/SPEC-cli-commands.md`
- `../specs/shared/SPEC-config-toml-resolution.md`
- `../specs/shared/SPEC-package-boundary-detection.md`
- `../specs/shared/SPEC-expose-uses-tables.md`
- `../specs/packages-lang-ts/SPEC-adapter-typescript-generation.md`
- `../specs/packages-lang-py/SPEC-adapter-python-generation.md`
- `../specs/crates-mds-lang-rs/SPEC-adapter-rust-generation.md`
- `../tech-stack.md`

## 関連資料

- `../../specs/shared/SPEC-parser-generation-mvp-phase.md`
- `../../specs/shared/SPEC-cli-commands.md`
- `../../specs/shared/SPEC-config-toml-resolution.md`
- `../../specs/shared/SPEC-package-boundary-detection.md`
- `../../specs/shared/SPEC-expose-uses-tables.md`
- `../../tech-stack.md`
