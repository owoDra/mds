# Greet

## Purpose

名前を受け取り、挨拶メッセージを返す関数を提供します。

## Contract

- `greet` は `GreetOptions` を受け取り、`"Hello, <name>!"` 形式の文字列を返す。
- `name` が空文字列の場合でもエラーにせず `"Hello, !"` を返す。

`GreetOptions` は `greet` 関数の引数です。

`greet` は名前を受け取り、挨拶メッセージを返します。

## Imports

| From | Target | Symbols | Via | Summary | Reference |
| --- | --- | --- | --- | --- | --- |
| builtin | dataclasses | dataclass | - | - | - |

## Source

```py
@dataclass
class GreetOptions:
    name: str
```

```py
def greet(options: GreetOptions) -> str:
    return f"Hello, {options.name}!"
```

### Dependencies

| Target | Summary |
| --- | --- |

## Cases

| # | Input | Expected | Notes |
| --- | --- | --- | --- |
| 1 | `GreetOptions(name="World")` | `"Hello, World!"` | 基本ケース |
| 2 | `GreetOptions(name="")` | `"Hello, !"` | 空文字列 |
