# エディタ統合 (LSP)

mds は、authoring-v2 package 向けの Language Server Protocol server として `mds-lsp` を提供します。

## 主な機能

| 機能 | 内容 |
| --- | --- |
| Diagnostics | section structure、canonical roots、legacy table warning、source/test mixing、unresolved wiki-style link |
| Hover / Definition | generated file へ委譲してから source map bridge で Markdown に戻す |
| Completion / Snippets | current source/test heading、code fence、config snippet |
| Code Action | tableless doc の不足 heading を補う |
| Outline | Markdown 見出しの document symbols |

## インストール

### VS Code

```bash
code --install-extension owo-x-project.mds
```

Marketplace 版の拡張には対応する `mds-lsp` binary が同梱されます。

### 他 editor

```bash
curl -fsSL https://raw.githubusercontent.com/owo-x-project/owox-mds/latest/install.sh | sh
```

この script は `mds` と `mds-lsp` を両方インストールします。

### ソースからビルド

```bash
cargo build -p mds-lsp --release
```

この repository 内では root Cargo workspace から `mds-lsp` をビルドします。self-hosted sync は不要です。

## VS Code 設定

| 設定 | 既定値 | 意味 |
| --- | --- | --- |
| `mds.lsp.path` | `""` | `mds-lsp` の明示 path。空なら bundled server を優先 |
| `mds.lsp.enabled` | `true` | LSP server の有効 / 無効 |
| `mds.lsp.logLevel` | `"info"` | server log level |
| `mds.lsp.additionalLanguages` | `[]` | editor feature の対象に追加する `*.lang.md` suffix |

## activation model

拡張は workspace に `mds.config.toml` があると有効化されます。既定では canonical な `.mds/source` と `.mds/test` を監視し、`mds.lsp.additionalLanguages` で editor-only の追加 suffix を扱えます。

## generated-file bridge

`mds-lsp` は generated output の source map を保持します。VS Code 拡張は bridge command を使って次を行います。

- hover と definition を generated file 側に委譲する
- 結果 range を元の Markdown code fence に remap する
- generated diagnostics を indexed Markdown document に mirror する

completion では、必要に応じて shadow document fallback も使います。

## 他 editor

stdio で `mds-lsp` を起動し、root marker は `mds.config.toml` にします。

推奨対象:

- `.mds/source/**/*.md`
- `.mds/test/**/*.md`
- `mds.config.toml`
- `package.md`

## ローカル開発

```bash
cd editors/vscode
npm install
npm run compile
```

その後 VS Code の Extension Development Host で起動します。

## トラブルシューティング

- diagnostics が出ない場合は file が `.mds/source` か `.mds/test` の配下にあるか確認
- bridge 結果が stale に見える場合は package を再 build するか workspace を開き直して index を更新
- `mds.lsp.path` を上書きしている場合は extension version と binary version の整合を確認