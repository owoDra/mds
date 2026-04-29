# 言語アダプター

このページでは、mds の言語アダプターの役割を説明します。

## 役割

言語アダプターは、言語ごとの違いを扱う部品です。

mds の中核処理は、Markdown の読み取り、構造検査、生成計画を担当します。TypeScript、Python、Rust ごとの出力規則や検査ツールとの接続は、言語アダプターが担当します。

## 担当する処理

言語アダプターは、主に次の処理を担当します。

- `Uses` から依存宣言を生成します。
- `Source`、`Types`、`Test` の出力ファイル名を決めます。
- 言語ごとの追加生成物を管理します。
- 静的検査、自動修正、テスト実行のコマンドへ接続します。
- 診断結果を Markdown 上の位置に戻せるようにします。

## TypeScript

TypeScript では、`*.ts.md` を対象にします。

既定の生成例です。

| 種別 | 生成先の例 |
| --- | --- |
| `Source` | `src/foo/bar.ts` |
| `Types` | `src/foo/bar.types.ts` |
| `Test` | `tests/foo/bar.test.ts` |

依存宣言は、TypeScript の import として生成します。内部依存の相対 import は、拡張子なしで生成します。

## Python

Python では、`*.py.md` を対象にします。

既定の生成例です。

| 種別 | 生成先の例 |
| --- | --- |
| `Source` | `src/pkg/foo.py` |
| `Types` | `src/pkg/foo.pyi` |
| `Test` | `tests/pkg/test_foo.py` |

内部依存は、生成先のソースルートから見た絶対パッケージ import として生成します。

## Rust

Rust では、`*.rs.md` を対象にします。

既定の生成例です。

| 種別 | 生成先の例 |
| --- | --- |
| `Source` | `src/foo/bar.rs` |
| `Types` | `src/foo/bar_types.rs` |
| `Test` | `tests/foo_bar_test.rs` |

Rust では、生成されたモジュールを公開するための mds 管理ブロックも扱います。

## 品質検査との関係

言語アダプターは、対象言語の検査ツールやテスト実行と接続します。

| 言語 | 検査 | 修正 | テスト |
| --- | --- | --- | --- |
| TypeScript | ESLint | Prettier | Vitest |
| Python | Ruff | Ruff | Pytest |
| Rust | Cargo Clippy | rustfmt | Cargo test |

これらのツールは、利用する機能に応じて実行環境に必要です。
