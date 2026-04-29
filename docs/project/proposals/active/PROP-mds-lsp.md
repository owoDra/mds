# mds LSP 設計調査

## 概要

mds 用の Language Server Protocol (LSP) 実装を検討する。エディタ上でリアルタイムに Markdown の構造検証、コード補完、ナビゲーションを提供することで、mds 利用者の開発体験を大幅に向上させることが目的。

## 期待される機能

### Phase 1: リアルタイム診断（最小 MVP）

- **セクション構造の検証**: Purpose, Expose, Uses, Types, Source, Test の存在と順序
- **テーブル形式の検証**: Expose, Uses テーブルのカラム名と形式
- **言語判定**: ファイル名末尾（`.ts.md`, `.py.md`, `.rs.md`）からの言語検出
- **コードブロック言語の一致確認**: ファイル言語とコードブロック言語ラベルの一致
- **mds.config.toml の検証**: 設定ファイルの構文と必須フィールド

### Phase 2: コードナビゲーション

- **Go to Definition**: Uses テーブルの `From` カラムから参照先の実装 Markdown へジャンプ
- **Find References**: Expose された名前がどの実装 Markdown の Uses で参照されているか
- **Document Symbols**: セクション見出しをシンボルとして提供（アウトライン表示）
- **Workspace Symbols**: `src-md/` 配下の全実装 Markdown の Expose 名を検索

### Phase 3: コードアシスト

- **セクション名の補完**: `## ` の後に Purpose, Expose, Uses 等を候補表示
- **テーブルカラムの補完**: `| ` の後にテーブル種別に応じたカラム名を候補表示
- **コードブロック言語の自動挿入**: ファイル名から推測した言語ラベルを自動設定
- **スニペット**: 新規セクション、テーブル行、コードブロックのスニペット
- **Hover 情報**: Expose/Uses の名前にホバーで定義元の Purpose を表示

## 技術的な検討

### 実装言語

**推奨: Rust**

- mds-core のパーサーを直接再利用できる
- `tower-lsp` クレートで LSP サーバーを構築
- パフォーマンスに優れる
- 既存の Cargo ワークスペースに `crates/mds-lsp/` として追加可能

### アーキテクチャ

```
crates/
  mds-core/       # 既存: パーサー、検証ロジック
  mds-lsp/        # 新規: LSP サーバー
    src/
      main.rs      # LSP サーバー起動
      handler.rs   # リクエストハンドラー
      diagnostics.rs  # 診断変換
      completion.rs   # 補完プロバイダー
      navigation.rs   # ナビゲーション
```

### 依存クレート

| クレート | 用途 |
| --- | --- |
| `tower-lsp` | LSP プロトコル実装 |
| `tokio` | 非同期ランタイム |
| `mds-core` | パーサーと検証ロジック |

### mds-core の変更

現在の mds-core は CLI バッチ処理向けに設計されている。LSP 向けには以下の拡張が必要:

1. **増分解析**: ファイル変更時に全ファイル再解析せず、変更箇所のみ再解析
2. **位置情報の保持**: 診断メッセージにバイトオフセット・行番号を付与（一部は `at_line()` で対応済み）
3. **パーサーの公開**: `markdown` モジュールを `pub(crate)` から `pub` に変更するか、LSP 用の API を追加

### エディタ統合

**VSCode 拡張**:
- `vscode-languageclient` を使用
- `crates/mds-lsp` のバイナリを `bundledServerPath` で同梱
- 拡張子 `.ts.md`, `.py.md`, `.rs.md` に対して自動有効化
- `mds.config.toml` にも TOML LSP と併用

**その他のエディタ**:
- Neovim: `nvim-lspconfig` に設定を追加
- Helix: `languages.toml` に設定を追加
- 標準 LSP プロトコルに準拠するため、任意のエディタで利用可能

## 実装計画

### Phase 1（MVP）: 2-3 週間

1. `crates/mds-lsp/` クレートを追加
2. `tower-lsp` でサーバースケルトンを実装
3. ファイルオープン/変更時にセクション構造を検証
4. 診断メッセージを LSP Diagnostic に変換
5. VSCode 拡張の最小版を作成

### Phase 2: 2-3 週間

1. Document Symbols の提供
2. Go to Definition（Uses → 参照先）
3. Find References（Expose → 利用箇所）
4. Workspace Symbols

### Phase 3: 2-3 週間

1. 補完プロバイダー
2. Hover 情報
3. スニペット
4. コードアクション（欠損セクションの自動追加）

## リスクと課題

- **mds-core パーサーの API 安定性**: 内部 API が頻繁に変更される段階では LSP の保守コストが高い
- **増分解析のコスト**: 大規模プロジェクトでの応答性能
- **マルチ言語コードブロック**: Markdown 内の TS/Py/RS コードに対する言語サーバーの委譲（将来課題）

## 次のステップ

1. この設計文書をレビューし、Phase 1 の範囲を確定する
2. mds-core のパーサー公開範囲を決定する
3. `crates/mds-lsp/` のスケルトンを作成する
4. VSCode 拡張の最小版を作成する
