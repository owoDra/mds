# C# Descriptor Sample

## Purpose

Verify that the C# language descriptor can render a source file.

## Contract

- `DescriptorSample.Value` returns a stable integer.

## Source

```cs
namespace DescriptorSamples;

public static class DescriptorSample
{
    public static int Value() => 1;
}
```
