# Greet test

## Purpose

`greet` の基本挙動を確認します。

## Covers

- greet

## Imports

| From | Target | Symbols | Via | Summary | Reference |
| --- | --- | --- | --- | --- | --- |
| internal | greet | GreetOptions, greet | - | Function under test | [../source/greet.py.md#source](../source/greet.py.md#source) |

## Test

```py
def test_greet_returns_expected_message() -> None:
    options = GreetOptions(name="World")
    assert greet(options) == "Hello, World!"
```