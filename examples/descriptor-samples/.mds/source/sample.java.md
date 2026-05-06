# Java Descriptor Sample

## Purpose

Verify that the Java language descriptor can render a source file.

## Contract

- `DescriptorSample.value` returns a stable string.

## Source

```java
public final class DescriptorSample {
    private DescriptorSample() {}

    public static String value() {
        return "ok";
    }
}
```
