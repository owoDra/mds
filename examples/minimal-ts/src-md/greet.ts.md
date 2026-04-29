# Greet

## Purpose

名前を受け取り、挨拶メッセージを返す関数を提供します。

## Contract

- `greet` は `GreetOptions` を受け取り、`"Hello, <name>!"` 形式の文字列を返す。
- `name` が空文字列の場合でもエラーにせず `"Hello, !"` を返す。

## Expose

| Kind | Name | Summary |
| --- | --- | --- |
| function | greet | 挨拶メッセージを返します。 |

## Types

| From | Target | Expose | Summary |
| --- | --- | --- | --- |

```ts
/** greet 関数の引数 */
export interface GreetOptions {
  name: string;
}
```

## Source

| From | Target | Expose | Summary |
| --- | --- | --- | --- |

```ts
/**
 * 名前を受け取り、挨拶メッセージを返します。
 */
export function greet(options: GreetOptions): string {
  return `Hello, ${options.name}!`;
}
```

## Cases

| # | Input | Expected | Notes |
| --- | --- | --- | --- |
| 1 | `{ name: "World" }` | `"Hello, World!"` | 基本ケース |
| 2 | `{ name: "" }` | `"Hello, !"` | 空文字列 |

## Test

| From | Target | Expose | Summary |
| --- | --- | --- | --- |
| package | vitest | { describe, it, expect } | テストフレームワーク |
| internal | greet | { greet } | テスト対象関数 |

```ts
describe("greet", () => {
  it("returns greeting message", () => {
    expect(greet({ name: "World" })).toBe("Hello, World!");
  });

  it("handles empty name", () => {
    expect(greet({ name: "" })).toBe("Hello, !");
  });
});
```
