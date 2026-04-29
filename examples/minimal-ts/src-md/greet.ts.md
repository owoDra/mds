# Greet

## Purpose

名前を受け取り、挨拶メッセージを返す関数を提供します。

## Expose

| Kind | Name | Summary |
| --- | --- | --- |
| function | greet | 挨拶メッセージを返します。 |

## Uses

| Kind | Name | From | Summary |
| --- | --- | --- | --- |

## Types

```ts
/** greet 関数の引数 */
export interface GreetOptions {
  name: string;
}
```

## Source

```ts
import type { GreetOptions } from "./greet";

/**
 * 名前を受け取り、挨拶メッセージを返します。
 */
export function greet(options: GreetOptions): string {
  return `Hello, ${options.name}!`;
}
```

## Test

```ts
import { describe, it, expect } from "vitest";
import { greet } from "../src/greet";

describe("greet", () => {
  it("returns greeting message", () => {
    expect(greet({ name: "World" })).toBe("Hello, World!");
  });

  it("handles empty name", () => {
    expect(greet({ name: "" })).toBe("Hello, !");
  });
});
```
