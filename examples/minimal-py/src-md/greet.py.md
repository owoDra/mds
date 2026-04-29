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

```py
from dataclasses import dataclass


@dataclass
class GreetOptions:
    """greet 関数の引数"""
    name: str
```

## Source

```py
from .greet import GreetOptions


def greet(options: GreetOptions) -> str:
    """名前を受け取り、挨拶メッセージを返します。"""
    return f"Hello, {options.name}!"
```

## Test

```py
from src.greet import GreetOptions, greet


def test_greet_returns_message():
    assert greet(GreetOptions(name="World")) == "Hello, World!"


def test_greet_handles_empty_name():
    assert greet(GreetOptions(name="")) == "Hello, !"
```
