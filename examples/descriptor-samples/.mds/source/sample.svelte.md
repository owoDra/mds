# Svelte Descriptor Sample

## Purpose

Verify that the Svelte overlay descriptor can render a source file.

## Contract

- The component renders one message.

## Source

```svelte
<script lang="ts">
  const message = 'ok';
</script>

<p>{message}</p>
```
