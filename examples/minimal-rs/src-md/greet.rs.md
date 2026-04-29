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

```rs
/// greet 関数のオプション
pub struct GreetOptions {
    pub name: String,
}
```

## Source

```rs
use crate::greet::GreetOptions;

/// 名前を受け取り、挨拶メッセージを返します。
pub fn greet(options: &GreetOptions) -> String {
    format!("Hello, {}!", options.name)
}
```

## Test

```rs
#[cfg(test)]
mod tests {
    use super::*;

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
}
```
