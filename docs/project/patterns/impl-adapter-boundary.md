---
status: 採用
related:
  - docs/project/requirements/REQ-adapter-required-language-adapters.md
  - mds/core/.mds/source/quality.rs.md
---

# Adapter 境界

## 目的

言語固有の import 生成、lint、lint --fix、test runner 接続、出力規則を language adapter に閉じ込める。

## 適用範囲

- TypeScript adapter
- Python adapter
- Rust adapter
- md 状態の lint / lint --fix / test
- Source / Types / Test の出力規則

## 適用しない範囲

- Markdown 文書種別の共通構造
- package 境界検出
- config の共通解決規則

## パターン

- core は言語横断の Markdown model と設定解決を担う。
- adapter は言語固有の toolchain と file pattern を担う。
- CLI は core と adapter を接続し、利用者に一貫したコマンド面を提供する。

## 適用条件

- 言語ごとに toolchain、import 形式、ファイル命名が異なる。
- core の概念を変えずに言語固有処理だけ差し替えたい。

## 例外 / 逸脱条件

- core の不変条件を adapter が上書きしてはいけない。
- adapter 固有の例外を共通仕様として扱う場合は spec または ADR を更新する。

## 根拠

多言語対応を維持しながら core を言語固有事情で肥大化させないため。

## 関連資料

- `../requirements/REQ-adapter-required-language-adapters.md`
- `../../../mds/core/.mds/source/descriptor.rs.md`
- `../../../mds/core/.mds/source/quality.rs.md`
- `../architecture.md`
