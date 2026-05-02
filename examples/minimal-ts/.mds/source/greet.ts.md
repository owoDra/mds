# Greet

## Purpose

名前を受け取り、挨拶メッセージを返す関数を提供します。

## Contract

- `greet` は `GreetOptions` を受け取り、`"Hello, <name>!"` 形式の文字列を返す。
- `name` が空文字列の場合でもエラーにせず `"Hello, !"` を返す。

## Source

```ts
export interface GreetOptions {
  name: string;
}
```

```ts
export function greet(options: GreetOptions): string {
  return `Hello, ${options.name}!`;
}
```

### Dependencies

| Target | Summary |
| --- | --- |

## Cases

| # | Input | Expected | Notes |
| --- | --- | --- | --- |
| 1 | `{ name: "World" }` | `"Hello, World!"` | 基本ケース |
| 2 | `{ name: "" }` | `"Hello, !"` | 空文字列 |
