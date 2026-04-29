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

```rs
/// greet 関数のオプション
pub struct GreetOptions {
    pub name: String,
}
```

## Source

| From | Target | Expose | Summary |
| --- | --- | --- | --- |

```rs
/// 名前を受け取り、挨拶メッセージを返します。
pub fn greet(options: &GreetOptions) -> String {
    format!("Hello, {}!", options.name)
}
```

## Cases

| # | Input | Expected | Notes |
| --- | --- | --- | --- |
| 1 | `GreetOptions { name: "World" }` | `"Hello, World!"` | 基本ケース |
| 2 | `GreetOptions { name: "" }` | `"Hello, !"` | 空文字列 |

## Test

| From | Target | Expose | Summary |
| --- | --- | --- | --- |
| internal | greet | { GreetOptions, greet } | テスト対象 |

```rs
#[test]
fn test_greet_returns_message() {
    let options = GreetOptions {
        name: "World".to_string(),
    };
    assert_eq!(greet(&options), "Hello, World!");
}

#[test]
fn test_greet_handles_empty_name() {
    let options = GreetOptions {
        name: String::new(),
    };
    assert_eq!(greet(&options), "Hello, !");
}
```
