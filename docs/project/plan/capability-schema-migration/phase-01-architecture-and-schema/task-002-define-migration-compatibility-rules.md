# Task 002: Define Migration Compatibility Rules

## 目的

旧 built-in registry から config/schema runtime へ移る移行期間の互換方針を定義する。

## 前提条件

- schema surface が定義済みである

## 作業内容

- 互換期間の fallback 挙動を決める
- 旧 registry 廃止までの段階を整理する
- plan / proposal / spec に移行前提を追記する

## 完了条件

- fallback と削除順序が明文化されている

## 検証方法

- proposal と plan が移行方針で矛盾しないことを確認する

## 依存関係

- `task-001-freeze-capability-schema-surface.md`

## 成果物

- `docs/project/proposals/active/proposal-capability-schema-runtime.md`
- `docs/project/plan/capability-schema-migration/`
