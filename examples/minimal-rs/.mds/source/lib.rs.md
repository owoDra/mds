# lib

## Purpose

`minimal-rs` の公開 module を束ねます。

## Contract

- `greet` module を crate root から利用できるようにする。

## Exports

| Name | Visibility | Summary |
| --- | --- | --- |
| greet | public | Greeting module |

##### greet

crate root から参照するための module export です。

## Imports

| From | Target | Symbols | Via | Summary | Reference |
| --- | --- | --- | --- | --- | --- |
| - | - | - | - | - | - |

## Source

````rs
pub mod greet;
````

## Cases

- crate root から `minimal_rs::greet` を参照できる。