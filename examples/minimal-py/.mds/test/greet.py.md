# Greet test

## Purpose

`greet` の基本挙動を確認します。

## Covers

- [[greet]]

## Cases

- `greet` returns the expected greeting for the provided name.

## Test

```py
from greet import GreetOptions, greet


def test_greet_returns_expected_message() -> None:
    options = GreetOptions(name="World")
    assert greet(options) == "Hello, World!"
```