# Rust Descriptor Sample

## Purpose

Verify that the Rust language descriptor can render a source file.

## Contract

- `descriptor_sample` returns a stable string.

## Source

```rs
pub fn descriptor_sample() -> &'static str {
    "ok"
}
```
