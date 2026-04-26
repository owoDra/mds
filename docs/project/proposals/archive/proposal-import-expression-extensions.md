# Uses Import 表現拡張

## 状態

archived: 2026-04-26 に採用し、`docs/project/specs/shared/SPEC-expose-uses-tables.md` と adapter spec への昇格対象にした。

## 背景

Parser + 生成 MVP では `Uses.Expose` の named import / use と空欄による module import / side-effect 相当を対象にした。残要件をすべて達成する段階では、default import、alias、namespace import、言語固有 import 表現も扱う必要がある。

## 提案内容

- `Uses` の高度な import 表現は `Expose` cell の構文拡張で表す。
- named import は既存どおり `A, B` と書く。
- alias は `A as B` と書く。
- default import は `default: Foo` と書く。
- namespace import は `* as ns` と書く。
- module import / side-effect 相当は既存どおり `Expose` 空欄で表す。
- 言語固有 import が必要な場合は adapter が許容する prefix を `lang:<adapter>:<expr>` 形式で扱う案を候補にする。
- 変換不能な表現は adapter 診断にする。

## 代替案

- 列追加で表現する: 機械処理しやすいが、table が横に広がり Markdown の可読性が落ちる。
- 別 `Imports` table を導入する: 高度表現は分離できるが、`Uses` と依存情報が分裂する。
- 高度 import を採用しない: MVP の単純性は保てるが、実プロジェクトの import 表現を十分に扱えない。

## 利点

- 既存 `Uses` table を維持しながら表現力を拡張できる。
- Markdown table の列数を増やさず、Obsidian での可読性を保ちやすい。
- adapter が言語ごとの差分を閉じ込められる。

## リスク

- `Expose` cell 内 grammar が複雑になり、escape や comma parsing が難しくなる。
- 言語固有 import を許容しすぎると共通概念が崩れる。
- TypeScript / Python / Rust で同じ構文が自然に対応しない場合がある。

## 未確定事項

- `Expose` cell grammar の exact syntax と escape 規則。
- 複数 default や default + namespace の不正組み合わせ検査。
- 言語固有 prefix を採用するか、adapter spec に閉じるか。
- Python / Rust で default / namespace 相当をどう診断または変換するか。

## 正式化先候補

- `../specs/shared/SPEC-expose-uses-tables.md`
- `../specs/packages-lang-ts/SPEC-adapter-typescript-generation.md`
- `../specs/packages-lang-py/SPEC-adapter-python-generation.md`
- `../specs/crates-mds-lang-rs/SPEC-adapter-rust-generation.md`
- `../validation.md`

## 関連資料

- `../../requirements/REQ-metadata-expose-uses.md`
- `../../requirements/REQ-adapter-required-language-adapters.md`
- `../../patterns/data-table-metadata.md`
