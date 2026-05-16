# コマンド

このページでは、current な mds CLI を要約します。

## 基本形

```bash
mds <command> --package ./path/to/package
```

`--package` を省略した場合は、現在のディレクトリ以下から enabled package を探します。

## command 一覧

| command | 目的 |
| --- | --- |
| `mds init` | package 設定、quality 設定、必要なら AI kit workflow を初期化 |
| `mds new` | current な tableless Markdown template を scaffold |
| `mds build` | generated output を計画して書き込む |
| `mds lint` | Markdown 構造を検証し、選択した linter を実行 |
| `mds typecheck` | 選択した typecheck command を実行 |
| `mds test` | 選択した test command を実行 |
| `mds doctor` | 必要な tool と runtime を確認 |
| `mds package sync` | `package.md` の managed package metadata を同期 |

## `mds init`

`mds init` は package config を作成または更新します。

```bash
mds init --package ./path/to/package
```

current template は canonical `.mds/source` / `.mds/test` と `[output]` pattern を前提にしています。

### `mds init --ai`

`mds init --ai` は対応する AI CLI 向けの agent kit file を生成します。

```bash
mds init --ai --target all --categories all --yes
```

対応 target は Claude Code、Codex CLI、Opencode、GitHub Copilot です。

## `mds new`

`mds new` は current な tableless doc を scaffold します。

```bash
mds new greet.ts.md
mds new overview.md
mds new index.ts.md
```

source doc、hierarchy overview、language root module doc の scaffold に使います。挙動を executable verification したいときは `.mds/test` に対応する test doc も用意します。

## `mds build`

`mds build` は generated output を書き込みます。

```bash
mds build --package ./path/to/package
```

初回や output pattern を変えた後は dry-run を先に使います。

```bash
mds build --package ./path/to/package --dry-run
```

## `mds lint`

`mds lint` は document structure、output planning、selected toolchain command を検証します。

```bash
mds lint --package ./path/to/package
```

`mds lint --fix` は設定した fixer で code fence を更新します。

```bash
mds lint --package ./path/to/package --fix
```

## `mds typecheck`

```bash
mds typecheck --package ./path/to/package
```

関連する `[quality.<lang>]` section に設定した typecheck command を実行します。

## `mds test`

```bash
mds test --package ./path/to/package
```

関連する `[quality.<lang>]` section に設定した test command を実行します。

## `mds doctor`

```bash
mds doctor --package ./path/to/package
```

machine-readable な出力が必要なら `--format json` を付けます。

## `mds package sync`

```bash
mds package sync --package ./path/to/package
```

差分だけ確認したい場合は `--check` を使います。

## 推奨フロー

1. `mds lint`
2. `mds build --dry-run`
3. `mds build`
4. `mds typecheck`
5. `mds test`
6. `mds doctor`

## 終了コード

| 終了コード | 意味 |
| --- | --- |
| `0` | 成功 |
| `1` | 診断または check failure |
| `2` | usage または configuration error |
| `3` | internal error |
| `4` | runtime または required tool 不足 |