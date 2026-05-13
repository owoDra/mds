# app.greet.test

[[greet]] の挙動を検証する。

## 対象

- [[greet]]
- [[greet#greet]]

## ケース

- 通常名
  - input: `{ name: "World" }`
  - expect: `"Hello, World!"`

- 未指定
  - input: `{}`
  - expect: `"Hello, Anonymous!"`

## 実装

```ts
import { describe, expect, it } from "vitest";
import { greet } from "../src/greet";

describe("greet", () => {
  it("returns greeting with name", () => {
    expect(greet({ name: "World" })).toBe("Hello, World!");
  });

  it("uses Anonymous when name is missing", () => {
    expect(greet({})).toBe("Hello, Anonymous!");
  });
});
```
