# Markdown Grammar Open Details

## 状態

archived: 2026-04-25 に採否判断済み。正式な振る舞いは shared spec に反映した。

## 背景

README の最終要件では中核方針は採用済みだが、Markdown grammar の細部には未確定事項が残っている。

## 提案内容

次の細部を採否判断前の proposal として扱い、個別の spec または ADR に昇格する。

- `Expose.Kind` の正式一覧と、共通 kind / 言語固有 kind の分担
- `Uses.From` の正式一覧と、`workspace` を追加するかどうか
- `Uses.Expose` の複数名表現を `A, B, C` にするか配列表現にするか
- 複数コードブロック連結時の改行規則
- generated files を md 横に置くか `output_root` 配下にまとめるか
- md 内補助見出しの許容深さ

## 代替案

- すぐに shared spec へ採用する: 実装を進めやすいが、未検証の細部を固定してしまう。
- README に未確定事項として残す: 見つけやすいが、README が正本化してしまう。
- 各項目を別 proposal に分ける: 粒度は明確だが、現時点では相互に Markdown grammar の細部としてまとまっている。

## 利点

- README から未確定事項を除去できる。
- 採用済み仕様と未確定草案の境界が明確になる。
- 後続で spec / ADR に昇格しやすい。

## リスク

- 1 つの proposal に複数項目があるため、判断が進んだ項目から分割が必要になる可能性がある。
- adapter 実装が先行すると、未確定事項が事実上固定される可能性がある。

## 未確定事項

- none

## 採用結果

- `Expose.Kind` は Markdown 記述上は全言語共通 kind とし、`type`、`value`、`function`、`class`、`module` の5種を正式一覧にする。
- `Uses.From` は `internal`、`workspace`、`package`、`builtin` の4種にする。
- `Uses.Expose` の複数名表現は `A, B, C` のカンマ区切りにする。
- 複数コードブロック連結時は LF 1 行で連結し、出力末尾も LF 1 行にする。
- generated files は adapter ごとの言語別 root へ出す。
- implementation md 内の補助見出しは H3-H4 まで許容する。

## 正式化先候補

- `docs/project/specs/shared/SPEC-expose-uses-tables.md`
- `docs/project/specs/shared/SPEC-code-generation-output.md`
- `docs/project/specs/shared/SPEC-markdown-document-model.md`
- `docs/project/specs/shared/SPEC-config-toml-resolution.md`
- `docs/project/specs/shared/SPEC-cli-commands.md`
- `docs/project/specs/shared/SPEC-package-boundary-detection.md`

## 関連資料

- `../../../README.md`
- `../../specs/shared/SPEC-expose-uses-tables.md`
- `../../specs/shared/SPEC-code-generation-output.md`
- `../../specs/shared/SPEC-markdown-document-model.md`
