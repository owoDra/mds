# Greet

## Purpose

名前を受け取り、挨拶メッセージを返す関数を提供します。

## Contract

- `greet` は `GreetOptions` を受け取り、`"Hello, <name>!"` 形式の文字列を返す。
- `name` が空文字列の場合でもエラーにせず `"Hello, !"` を返す。

## API

`GreetOptions` は `greet` 関数の入力で、`greet` はテストと呼び出し側が使う公開関数です。

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

## Cases

- `GreetOptions { name: "World" }` では `"Hello, World!"` を返す。
- `GreetOptions { name: "" }` では `"Hello, !"` を返す。
