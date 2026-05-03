# src/labels.rs

## Purpose

Migrated implementation source for `src/labels.rs`.

## Contract

- Preserve the behavior of the pre-migration Rust source.
- This file is synchronized into `.build/rust/mds/lsp/src/labels.rs`.

## Source

````rs
use mds_core::model::Config;
````

Resolve a canonical label key to its display form using label overrides.

````rs
pub fn resolve_label(key: &str, config: &Config) -> String {
    if let Some(override_label) = config.label_overrides.get(key) {
        return override_label.clone();
    }
    // Default: capitalize first letter
    let mut chars = key.chars();
    match chars.next() {
        Some(first) => {
            let upper: String = first.to_uppercase().collect();
            format!("{upper}{}", chars.as_str())
        }
        None => key.to_string(),
    }
}
````