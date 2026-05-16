# lib

## Purpose

`minimal-rs` の公開 module を束ねます。

## Contract

- `greet` module を crate root から利用できるようにする。

## API

crate root は `minimal_rs::greet` として greeting module を公開する。

## Source

##### greet

crate root から参照するための module export です。

````rs
pub mod greet;
````

## Cases

- crate root から `minimal_rs::greet` を参照できる。
