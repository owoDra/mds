# 配布方針

このページでは、mds の配布方針を説明します。

## 基本方針

mds は、Rust で実装した単一の静的バイナリとして配布します。ランタイム依存はありません。

## 配布経路

| 経路 | 方法 |
| --- | --- |
| GitHub Releases | プラットフォーム別バイナリ（推奨） |
| install.sh | `curl -fsSL .../install.sh \| sh` でワンライナーインストール |
| Cargo | `cargo install mds-cli` でソースからビルド |

## インストール

```bash
# 推奨: インストールスクリプト
curl -fsSL https://raw.githubusercontent.com/owo-x-project/owox-mds/main/install.sh | sh

# バージョン指定
curl -fsSL .../install.sh | sh -s -- --version 0.3.0

# Cargo 経由（Rust 環境が必要）
cargo install mds-cli
```

## セルフアップデート

```bash
mds update              # 最新版に更新
mds update --version 0.4.0  # 指定バージョンに更新
```

## バージョン固定

プロジェクトで使用する mds バージョンを `mds.config.toml` で指定できます:

```toml
[package]
enabled = true
mds_version = "0.3.0"
```

`mds doctor` がバージョン不一致を検出して警告します。

## 含まれるバイナリ

| バイナリ | 用途 |
| --- | --- |
| `mds` | CLI メインコマンド |
| `mds-lsp` | Language Server（エディタ連携用） |

## 公開前の品質確認

リリース前に以下を確認します:

- 配布物の存在とチェックサム
- 署名
- ソフトウェア部品表 (SBOM)
- 来歴情報 (provenance)
- インストール後の簡易動作確認
