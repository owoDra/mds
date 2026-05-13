# app.greet

名前を受け取り、挨拶メッセージを返す。

## 仕様

- 入力は `GreetOptions`。
- 出力は `string`。
- 例外は投げない。
- `name` が未指定または空白のみの場合は `Anonymous` を使う。

## API

`greet` は外部公開API。

`GreetOptions` は挨拶生成に必要な入力だけを持つ。

## 実装

```ts
export interface GreetOptions {
  name?: string;
}
```

```ts
export function greet(options: GreetOptions): string {
  const name = options.name?.trim() || "Anonymous";
  return `Hello, ${name}!`;
}
```

## 検証

テストは [[greet.test]] に分離する。
