# Phase 01: Architecture And Schema

## 目的

config/schema 中心 runtime の architecture と capability schema 契約を確定する。

## 前提条件

- proposal と現行 spec の差分が把握されている

## 完了条件

- schema が担う責務と kernel が担う責務が明文化されている
- 最低限の schema surface が spec と plan task に落ちている

## 検証方法

- architecture / shared spec / proposal の参照整合確認

## task 一覧

- `task-001-freeze-capability-schema-surface.md`: schema の最小 surface を定義する
- `task-002-define-migration-compatibility-rules.md`: 旧 registry からの移行互換方針を定義する

## 依存関係

- `../index.md`
- `../../../proposals/active/proposal-capability-schema-runtime.md`

## 参照

- `task-001-freeze-capability-schema-surface.md`: schema の最小 surface 定義
- `task-002-define-migration-compatibility-rules.md`: 移行互換方針の定義
