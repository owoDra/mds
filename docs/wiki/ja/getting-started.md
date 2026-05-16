# はじめに

このページでは、mds を試すための current な最小構成を説明します。

## インストール

GitHub Releases の platform 別バイナリをインストールします。

```bash
curl -fsSL https://raw.githubusercontent.com/owo-x-project/owox-mds/latest/install.sh | sh
```

既定では `mds` と `mds-lsp` の両方が `~/.local/bin` に入ります。

### VS Code 拡張

```bash
code --install-extension owo-x-project.mds
```

Marketplace 版の拡張には対応する `mds-lsp` バイナリが同梱されます。

## 実行環境

| 用途 | 必要なもの |
| --- | --- |
| `mds` 自体の実行 | ビルド済み binary を使う場合は追加不要 |
| TypeScript の検査 | Node.js と `[quality.ts]` で選んだ tool |
| Python の検査 | Python と `[quality.py]` で選んだ tool |
| Rust の検査 | Rust/Cargo と `[quality.rs]` で選んだ tool |

未選択の tool は不足扱いになりません。

## 最小 package 構成

```text
my-package/
├── mds.config.toml
├── package.md
├── package.json
├── .mds/
│   ├── source/
│   │   ├── overview.md
│   │   └── greet.ts.md
│   └── test/
│       ├── overview.md
│       └── greet.ts.md
├── src/
└── tests/
```

`package.json`、`pyproject.toml`、`Cargo.toml` などの package manager metadata はそのまま authoritative source として扱います。`package.md` は mds 側の package 文書です。

## 最小設定

```toml
[package]
enabled = true
allow_raw_source = false

[roots]
source_md = ".mds/source"
test_md = ".mds/test"
source_out = "src"
test_out = "tests"

[output]
source = "{source_out}/{module}.{ext}"
test = "{test_out}/{module}.test.{ext}"
```

非既定の file naming が必要なら、Markdown root を変えるのではなく `[[output.override]]` を使います。

## authoring model

- source doc は `.mds/source/**/*.lang.md` に置きます。
- test doc は `.mds/test/**/*.md` に置きます。
- `mds new greet.ts.md`、`mds new overview.md`、`mds new index.ts.md` で current tableless template を作れます。
- source behavior は source doc、executable verification は test doc に分けます。

## 最初のフロー

```bash
mds init --package ./path/to/package
mds lint --package ./path/to/package
mds build --package ./path/to/package --dry-run
mds build --package ./path/to/package
mds typecheck --package ./path/to/package
mds test --package ./path/to/package
```

## 既定の出力対応

| Markdown doc | 既定の出力先 |
| --- | --- |
| `.mds/source/greet.ts.md` | `src/greet.ts` |
| `.mds/test/greet.ts.md` | `tests/greet.test.ts` |
| `.mds/source/lib.rs.md` | `src/lib.rs` |
| `.mds/test/lib.rs.md` | override が無ければ `tests/lib.test.rs` |

logical module id は `.mds/source` または `.mds/test` からの相対 path から `.md` と最後の言語 suffix を除いて決まります。

## 次に読むページ

- [設定](configuration.md)
- [Markdown 正本](markdown-source.md)
- [コマンド](commands.md)
- [生成の仕組み](generation.md)