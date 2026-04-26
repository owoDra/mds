# Graph / Doctor コマンド

## 状態

archived: 2026-04-26 に採用したが、後続判断で `mds graph` は要件・仕様から削除し、`mds doctor` のみ `docs/project/specs/shared/SPEC-doctor-command.md` として維持する。

## 背景

当初は `mds graph` と `mds doctor` を CLI 面に含めていたが、後続判断で `mds graph` は削除し、環境診断として `mds doctor` のみを維持する。

## 提案内容

- `mds graph` は採用済み仕様から削除したため、実装対象にしない。
- `mds doctor` は mds core / CLI version、config 読み込み、package 検出、adapter 有効性、toolchain 検出を診断する。
- `mds doctor` は不足を `required` と `optional` に分類し、必須 toolchain 不足は exit code 4 とする。
- CLI exit code は成功 0、診断あり 1、usage/config error 2、internal error 3、environment 不足 4 とする。

## 代替案

- graph を JSON 中心にする: 機械処理には向くが、人間が CLI で確認する用途が弱くなる。
- DOT も必須にする: 可視化しやすいが、初期必須出力が増え、Graphviz 前提に見えやすい。
- doctor を check に統合する: コマンド数は減るが、Markdown 構造診断と環境診断が混ざる。

## 利点

- Markdown 正本の依存関係を人間とツールの両方が追跡できる。
- environment 不足を専用 exit code で扱える。
- check / build と同じ package 検出・config 解決を再利用できる。

## リスク

- link 解決と `Uses` 解決の責務境界が曖昧になる可能性がある。
- doctor が OS / package manager 差分を多く抱えすぎる可能性がある。
- graph の循環表示や未解決 link の扱いを曖昧にすると実装差が出る。

## 未確定事項

- graph JSON schema。
- 循環 graph の表示形式。
- 未解決 link を error / warning のどちらにするか。
- doctor の required / optional toolchain 一覧。
- doctor が version mismatch を warning にするか environment error にするか。

## 正式化先候補

- `../specs/shared/SPEC-doctor-command.md`
- `../specs/shared/SPEC-cli-commands.md`
- `../specs/shared/SPEC-obsidian-readable-markdown.md`
- `../validation.md`

## 関連資料

- `../../requirements/REQ-cli-command-surface.md`
- `../../requirements/REQ-ux-obsidian-readable-markdown.md`
- `../../validation.md`
