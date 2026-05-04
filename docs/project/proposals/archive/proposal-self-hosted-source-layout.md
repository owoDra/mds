# mds 自身の mds 化と生成物配置の再編案

Status: 採用済み。`docs/project/adr/active/ADR-007-self-hosted-src-md-build.md` に昇格。

## 背景

mds は Markdown を設計書兼ソースの正本として扱い、生成コードを派生物とするツールチェーンである。一方で、現在の mds 自身の実装は `crates/` 配下の Rust workspace を中心に管理されており、mds の規定する `index.md`、`package.md`、implementation md を mds 自身の開発入口として十分に使えていない。

また、ビルド生成物は `target/`、`out/`、release 用ディレクトリなどに分散しており、生成物の配置と Git 管理方針を理解しづらい。今後は mds 自身も mds で開発できるようにし、編集対象と生成対象を明確に分離する必要がある。

さらに、現在の `crates/` というディレクトリ名は Rust crate を手書き開発する前提を強く示す。今後は mds 正本から Rust workspace を生成する方向へ移行するため、開発上の主語としての `crates/` は廃止したい。

## 提案内容

### 正本と生成物の分離

mds 自身の編集入口を `src-md/` に集約する。

```text
src-md/
  index.md
  mds/core/
    package.md
    index.md
    src/
      markdown/
        index.md
        parser.rs.md
      generation/
        index.md
        builder.rs.md
      config/
        index.md
        resolver.rs.md
  mds/cli/
    package.md
    index.md
    src/
      index.md
      args.rs.md
      commands.rs.md
  mds/lsp/
    package.md
    index.md
    src/
      index.md
      server.rs.md
  vscode/
    package.md
    index.md
    src/
      extension.ts.md
```

`src-md/index.md` は mds 自身の source root の入口とする。各 package の `index.md` は責務、構成、公開面、ディレクトリ単位の設計を記録する。1 機能の契約、実装、テストは `*.rs.md`、`*.ts.md` などの implementation md に記録する。

### 全体設計の配置

`src-md/project/specs/` は作らない。全体に関わる設計や階層設計は、mds の規定に従って該当階層の `index.md` に書く。

`docs/project/specs/` にある既存仕様は、段階的に次のいずれかへ移す。

| 現在の内容 | 移行先 |
| --- | --- |
| source root 全体の構成、責務境界、横断設計 | `src-md/index.md` |
| package 固有の設計 | `src-md/<package>/index.md` |
| subdirectory 固有の設計 | `src-md/<package>/src/**/index.md` |
| 1 機能の契約、実装、テスト | implementation md |
| 品質確認、検証観点、受け入れ確認 | `docs/project/validation.md` |
| 重要判断と理由 | `docs/project/adr/active/*.md` |

`docs/project/validation.md` は引き続き検証事項の正本として維持する。

### `.build/` への生成物集約

ビルド生成物と派生コードはすべて `.build/` 配下へ出力する。

```text
.build/
  rust/
    Cargo.toml
    Cargo.lock
    mds/core/
    mds/cli/
    mds/lsp/
    target/
  node/
    vscode/
  test/
  release/
```

Rust の Cargo workspace は `.build/rust/` に生成する。Cargo の build artifact は `.build/rust/target/` に置く。VS Code extension など Node.js 系の生成物は `.build/node/` 配下へ置く。release artifact は `.build/release/` に置く。

### `crates/` の廃止

`crates/` は mds 自身の開発上の正本ではなくなる。移行後は `.build/rust/` が Cargo workspace として生成され、`src-md/` が編集対象になる。

移行は段階的に行う。

1. `src-md/` に mds 自身の package / index / implementation md を追加する
2. `.build/rust/` に現在の Rust workspace 相当を生成できるようにする
3. 既存 `crates/` と `.build/rust/` の差分を検証する
4. CI と開発手順を `.build/` ベースへ切り替える
5. `crates/` を削除する

### Git 管理方針

原則として、Git 管理するのは正本である `src-md/`、`docs/project/`、利用者向け docs、設定、テンプレート、テスト入力とする。`.build/` は生成物置き場として Git 管理しない。

移行中に限り、既存 `crates/` は比較対象として残す。`crates/` 削除後は `.build/` を復元可能な生成物として扱う。

### 移行フェーズ

| Phase | 内容 | 完了条件 |
| --- | --- | --- |
| 1 | 方針の正本化 | architecture、validation、ADR 候補、開発 docs の更新対象が明確である |
| 2 | `.build/` 生成先の整備 | Rust / Node / release / test の生成物が `.build/` に集約される |
| 3 | `src-md/` 入口作成 | `src-md/index.md` と各 package の `package.md` / `index.md` が存在する |
| 4 | 小規模 self-hosting | 1 つの小さい機能を implementation md から `.build/rust/` へ生成できる |
| 5 | specs の再配置 | 既存 `docs/project/specs` の内容が `index.md`、implementation md、validation、ADR へ分類される |
| 6 | `crates/` 廃止 | CI と開発手順が `.build/rust/` ベースになり、`crates/` を削除できる |

## 代替案

### `crates/` を維持する

Rust 開発者には分かりやすく、既存 tooling への影響が少ない。一方で、mds 自身を mds で開発するという目的に対して、手書き Rust workspace が開発上の中心として残り続ける。正本と派生物の境界が曖昧になるため不採用候補とする。

### `src-md/project/specs/` を作る

既存 `docs/project/specs` の移行先として分かりやすい。一方で、mds の規定では階層全体の設計は `index.md`、1 機能の契約やテストは implementation md に置くため、仕様専用ディレクトリを作ると規定と重複する。全体設計は `index.md` に寄せる方針とする。

### 生成コードを Git 管理する

bootstrap やレビューは容易になる。一方で、`.build/` を生成物置き場として統一する方針と衝突し、正本と派生物の差分管理が複雑になる。移行中の比較対象として既存 `crates/` を残すが、最終形では `.build/` を Git 管理しない方針を優先する。

### `components/` などの中間ディレクトリを作る

Rust 以外の package を含めた総称としては自然である。一方で、mds 正本の編集入口を明確にするには `src-md/` 直下に package を置く方が単純である。必要になるまでは中間ディレクトリを作らない。

## 利点

- mds 自身の開発が mds の規定に従うため、dogfooding によって仕様の不備を発見しやすくなる。
- 正本である `src-md/` と生成物である `.build/` の境界が明確になる。
- `crates/` という Rust crate 前提の名前を廃止でき、Rust workspace は生成物として扱える。
- 全体設計を `index.md` に寄せることで、mds のドキュメントモデルと実プロジェクト構成が一致する。
- `.build/` 配下に生成物を集約することで、Git 管理、clean、CI、release の扱いを単純化できる。

## リスク

- `.build/rust/` を生成してから Cargo を実行するため、既存の Rust tooling や IDE 設定の見直しが必要になる。
- `crates/` 削除までの移行期間は、正本と比較対象が併存し、差分管理が複雑になる。
- 生成コードを Git 管理しない場合、bootstrap 手順と CI の初期生成手順が失敗すると開発不能になる。
- 既存 `docs/project/specs` の内容を `index.md`、implementation md、validation、ADR に分割する際、情報の重複や移し漏れが起きやすい。
- `docs/wiki`、`CONTRIBUTING.md`、`.github`、`.agents` など、参照している旧 `crates/` パスの更新範囲が広い。

## 未確定事項

- `.build/rust/Cargo.toml` と `Cargo.lock` を常に生成するか、初回生成後に固定するか。
- bootstrap 用の最小 mds binary をどこから取得するか。
- `mds build` の既定出力先をプロジェクト全体で `.build/` に変更するか、このプロジェクト固有の設定にするか。
- `docs/project/specs` の既存個票をいつ archive するか、または移行完了まで参照資料として残すか。
- IDE / LSP が `.build/` の生成コードと `src-md/` の正本をどう関連付けるか。
- release package に `.build/` 内の生成コードを含めるか、release pipeline 内で都度生成するか。

## 正式化先候補

- `docs/project/architecture.md`: 正本と生成物の責務分離、workspace 構成、`crates/` 廃止方針
- `docs/project/adr/active/*.md`: `.build/` 生成物集約、生成コードを Git 管理しない判断、self-hosting 方針
- `docs/project/patterns/*.md`: mds 自身を mds で管理する package / index / implementation md 配置パターン
- `docs/project/validation.md`: `.build/` 生成、差分確認、Cargo / Node / release の検証項目
- `docs/project/specs/index.md`: 既存 specs を `index.md` / implementation md / validation / ADR へ移行する方針
- `CONTRIBUTING.md`: 新しい開発手順、ディレクトリ構成、ビルド手順

## 関連資料

- `../index.md`
- `../../architecture.md`
- `../../validation.md`
- `../../specs/index.md`
- `../../glossary/core.md`
