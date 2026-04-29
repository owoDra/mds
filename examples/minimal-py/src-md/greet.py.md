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

```py
@dataclass
class GreetOptions:
    """greet 関数の引数"""
    name: str
```

## Source

| From | Target | Expose | Summary |
| --- | --- | --- | --- |

```py
def greet(options: GreetOptions) -> str:
    """名前を受け取り、挨拶メッセージを返します。"""
    return f"Hello, {options.name}!"
```

## Cases

| # | Input | Expected | Notes |
| --- | --- | --- | --- |
| 1 | `GreetOptions(name="World")` | `"Hello, World!"` | 基本ケース |
| 2 | `GreetOptions(name="")` | `"Hello, !"` | 空文字列 |

## Test

| From | Target | Expose | Summary |
| --- | --- | --- | --- |

```py
def test_greet_returns_message():
    assert greet(GreetOptions(name="World")) == "Hello, World!"


def test_greet_handles_empty_name():
    assert greet(GreetOptions(name="")) == "Hello, !"
```
