# Greet

## Purpose

名前を受け取り、挨拶メッセージを返す関数を提供します。

## Contract

- `greet` は `GreetOptions` を受け取り、`"Hello, <name>!"` 形式の文字列を返す。
- `name` が空文字列の場合でもエラーにせず `"Hello, !"` を返す。

`GreetOptions` は `greet` 関数の入力です。

`greet` は名前を受け取り、挨拶メッセージを返します。

## Source

````rs
pub struct GreetOptions {
    pub name: String,
}
````

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
