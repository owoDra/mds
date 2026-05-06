# Greet

## Purpose

名前を受け取り、挨拶メッセージを返す関数を提供します。

## Contract

- `greet` は `GreetOptions` を受け取り、`"Hello, <name>!"` 形式の文字列を返す。
- `name` が空文字列の場合でもエラーにせず `"Hello, !"` を返す。

`GreetOptions` は `greet` 関数の入力です。

`greet` は名前を受け取り、挨拶メッセージを返します。

## Exports

| Name | Visibility | Summary |
| --- | --- | --- |
| GreetOptions | public | Greeting input data accepted by `greet`. |
| greet | public | Returns a greeting message from the provided options. |

## Source

##### GreetOptions

`GreetOptions` carries the name used to build a greeting message.

````rs
pub struct GreetOptions {
    pub name: String,
}
````

##### greet

`greet` is the public function referenced by tests and callers that need a formatted greeting.

````rs
pub fn greet(options: &GreetOptions) -> String {
    format!("Hello, {}!", options.name)
}
````
### Dependencies

| Target | Summary |
| --- | --- |

## Cases

| # | Input | Expected | Notes |
| --- | --- | --- | --- |
| 1 | `GreetOptions { name: "World" }` | `"Hello, World!"` | 基本ケース |
| 2 | `GreetOptions { name: "" }` | `"Hello, !"` | 空文字列 |
