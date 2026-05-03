# コマンド

このページでは、mds のコマンドの目的と使い方を説明します。

## 基本形

mds コマンドは、対象パッケージを指定して実行できます。

```bash
mds lint --package path/to/package
```

`--package` を省略した場合は、現在のディレクトリ以下から mds が有効なパッケージを探します。

## `mds lint`

`mds lint` は、Markdown の構造、表、設定、生成先を検査し、その後に code block へ lint を実行します。

主に次の内容を確認します。

- 必須セクションが存在するか
- `Imports`、`Exports`、`Expose`、`Uses` の表が正しいか
- 対象言語を判断できるか
- 生成先がパッケージの外に出ていないか
- 管理対象ではない既存ファイルを上書きしようとしていないか

実行例です。

```bash
mds lint --package path/to/package
```

## `mds typecheck`

`mds typecheck` は、対象言語ごとに設定した型検査を Markdown の code block へ実行します。

```bash
mds typecheck --package path/to/package
```

## `mds build`

`mds build` は、Markdown から派生コードを生成します。

```bash
mds build --package path/to/package
```

生成対象は、実装 Markdown の `Types`、`Source`、`Test` に書かれたコードブロックです。

## `mds build --dry-run`

`mds build --dry-run` は、ファイルを書き込まずに生成予定と差分を表示します。

```bash
mds build --package path/to/package --dry-run
```

初めて生成する場合や、生成規則を変更した場合は、先にこのコマンドで差分を確認してください。

## `mds lint`

`mds lint` は、Markdown 内のコードブロックを対象に、構造検査の後で言語ごとの検査ツールを実行します。

```bash
mds lint --package path/to/package
```

TypeScript、Python、Rust では利用する検査ツールが異なります。具体的な接続は言語アダプターが担当します。

## `mds lint --fix`

`mds lint --fix` は、Markdown 内のコードブロックに自動修正を適用します。

```bash
mds lint --package path/to/package --fix
```

確認だけを行いたい場合は、`--check` を追加します。

```bash
mds lint --package path/to/package --fix --check
```

自動修正は、Markdown 全体を書き換えるのではなく、対象のコードブロックの中身を更新します。

## `mds test`

`mds test` は、Markdown 内の `Test` セクションにあるテストコードを対象に、言語ごとのテスト実行を行います。

```bash
mds test --package path/to/package
```

## `mds doctor`

`mds doctor` は、実行環境と必要なツールを診断します。

```bash
mds doctor --package path/to/package
```

表示形式を JSON にしたい場合は、次のように実行します。

```bash
mds doctor --package path/to/package --format json
```

## `mds package sync`

`mds package sync` は、対象言語のパッケージ情報をもとに `package.md` の管理部分を同期します。

```bash
mds package sync --package path/to/package
```

差分の確認だけを行う場合は、`--check` を追加します。

```bash
mds package sync --package path/to/package --check
```

## `mds init`

`mds init` は、mds を使うための初期化を行います。

```bash
mds init --package path/to/package
```

初期化では、プロジェクト構成、支援ツール向けの設定、開発環境の準備を扱います。外部コマンドの実行や環境変更は、利用者が明示した場合に行う方針です。

### AI エージェント向け初期化

`mds init --ai` は、AI コーディングエージェント向けの設定ファイル (agent kit) を生成します。

```bash
# 全 AI CLI 向けに全カテゴリを生成
mds init --ai --target all --categories all --yes

# Claude Code 向けのみ生成
mds init --ai --target claude-code --yes

# 特定のカテゴリのみ生成
mds init --ai --target all --categories instructions,skills --yes
```

対応している AI CLI は次のとおりです。

| AI CLI | 指定名 | 生成先 |
| --- | --- | --- |
| Claude Code | `claude-code`, `claude` | `.claude/rules/`, `.claude/skills/`, `.claude/commands/` |
| Codex CLI | `codex-cli`, `codex` | `.codex/instructions.md`, `.codex/skills/` |
| Opencode | `opencode` | `.opencode/agents/`, `.opencode/skills/` |
| GitHub Copilot | `github-copilot-cli`, `copilot` | `.github/instructions/`, `.github/prompts/` |

生成カテゴリは `instructions`、`skills`、`commands` です。

CLAUDE.md、AGENTS.md、copilot-instructions.md などのメインファイルは生成しません。各 CLI のネイティブ参照パスに配置し、生成後に統合方法のガイドを表示します。

生成ファイルには `mds-managed: true` の frontmatter が含まれ、再実行時に安全に更新できます。非管理ファイルの上書きは `--force` がない限り拒否されます。

### 品質検査ツール選択

品質検査で使うツールは、言語ごとに選択できます。

```bash
mds init --package path/to/package --ts-tools biome,jest --py-tools ruff,black,pytest --rs-tools rustfmt,cargo-test
```

使わない言語や品質検査は `none` で無効にできます。

```bash
mds init --package path/to/package --ts-tools none --py-tools pytest --rs-tools clippy,nextest
```

`default` を指定すると、mds の代表的な組み合わせを使います。選択内容は `mds.config.toml` の `[quality.ts]`、`[quality.py]`、`[quality.rs]` に書き込まれます。

### 対話型モード

`mds init` を引数なし（または `--package` のみ）で実行すると、対話型ウィザードが起動します。

```bash
mds init
mds init --package path/to/package
```

ウィザードは英語表示で開始します。Label 言語を選択した後は、以降のタイトル、説明文、選択項目、コマンド入力の補足が選択した言語で表示されます。

ウィザードでは、Label 言語、ツールチェーン、AI Kit 生成項目を順に選択します。ツールチェーンでは `package.json`、`pyproject.toml`、`Cargo.toml` を自動検知し、検知された metadata ごとに type check、lint check、test check command の候補を表示して入力を受け付けます。AI Kit では生成が必要かを選び、必要な場合は AI CLI と各 CLI の生成項目を選択します。

従来のフラグ指定方式も引き続き利用できます。

## `mds new`

`mds new` は、新しい実装 Markdown のスキャフォールドを生成します。

```bash
mds new greet.ts.md
mds new utils/helper.py.md --package path/to/package
mds new parser.rs.md --force
```

ファイル名の末尾で言語を判定します。

| 末尾 | 言語 |
| --- | --- |
| `.ts.md` | TypeScript |
| `.py.md` | Python |
| `.rs.md` | Rust |

生成されるテンプレートには、Purpose、Expose、Uses、Types、Source、Test の全セクションが含まれます。生成先は `src-md/` 配下です。

既存ファイルがある場合は上書きしません。`--force` で強制上書きできます。

## 終了コード

mds は、失敗の種類を区別できるように終了コードを使い分けます。

| 終了コード | 意味 |
| --- | --- |
| `0` | 成功しました。 |
| `1` | 診断または検査エラーがあります。 |
| `2` | コマンドの使い方または設定に誤りがあります。 |
| `3` | 内部エラーが発生しました。 |
| `4` | 実行環境または必要なツールが不足しています。 |
