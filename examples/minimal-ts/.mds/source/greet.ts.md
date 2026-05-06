# Greet

## Purpose

名前を受け取り、挨拶メッセージを返す関数を提供します。

## Contract

- `greet` は `GreetOptions` を受け取り、`"Hello, <name>!"` 形式の文字列を返す。
- `name` が空文字列の場合でもエラーにせず `"Hello, !"` を返す。

## Exports

| Name | Visibility | Summary |
| --- | --- | --- |
| GreetOptions | public | Greeting input data accepted by `greet`. |
| greet | public | Returns a greeting message from the provided options. |

## Source

##### GreetOptions

`GreetOptions` carries the name used to build a greeting message.

```ts
export interface GreetOptions {
  name: string;
}
```

##### greet

`greet` is the public function referenced by tests and callers that need a formatted greeting.

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
