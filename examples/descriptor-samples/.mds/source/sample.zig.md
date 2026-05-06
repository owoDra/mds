# Zig Descriptor Sample

## Purpose

Verify that the Zig language descriptor can render a source file.

## Contract

- `descriptorSample` returns a stable string slice.

## Source

```zig
pub fn descriptorSample() []const u8 {
    return "ok";
}
```
