# 配布方針

このページでは、mds の配布方針を説明します。

## 基本方針

mds は、Rust で実装したビルド済み native binary として配布します。CLI 自体にランタイム依存はなく、エディタ連携では `mds-lsp` バイナリを使います。

## 配布経路

| 経路 | 方法 |
| --- | --- |
| GitHub Releases | `mds` と `mds-lsp` を含む platform 別 archive（推奨） |
| install.sh | OS / architecture に合う GitHub Releases archive を取得するワンライナー |
| VS Code Marketplace | `mds-lsp` 同封済みの platform-specific extension package |

## インストール

```bash
# 推奨: 最新の GitHub Releases archive をインストール
curl -fsSL https://raw.githubusercontent.com/owo-x-project/owox-mds/latest/install.sh | sh
```

release asset は tag と Rust target triple を含む名前で公開されます。

## アップデート

インストーラーを再実行すると最新 release へ更新されます。

## 含まれるバイナリ

| バイナリ | 用途 |
| --- | --- |
| `mds` | CLI メインコマンド |
| `mds-lsp` | VSCode 以外のエディタ、または VSCode の `mds.lsp.path` 上書きで使う Language Server |

## VS Code 拡張

```bash
code --install-extension owo-x-project.mds
```

Marketplace 版の拡張は platform-specific package として公開され、対応する `mds-lsp` バイナリを `server/<target>/` に同封しています。VSCode 利用者は通常 `mds-lsp` を別途インストールする必要はありません。

## 公開前の品質確認

リリース前に以下を確認します:

- 配布物の存在とチェックサム
- 署名
- ソフトウェア部品表 (SBOM)
- 来歴情報 (provenance)
- インストール後の簡易動作確認
