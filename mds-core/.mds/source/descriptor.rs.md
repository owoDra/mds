# src/descriptor.rs

## Purpose

Built-in language descriptor registry for output path rules.

## Contract

- Preserve the behavior of the current built-in adapters while moving file rules into TOML descriptors.
- This file is synchronized into `.build/rust/mds-core/src/descriptor.rs`.

## Source

````rs
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::model::{Lang, OutputKind};
````

````rs
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Descriptor {
    #[allow(dead_code)]
    pub id: String,
    pub language: LanguageSection,
    pub files: FileRules,
}
````

````rs
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct LanguageSection {
    pub primary_ext: String,
}
````

````rs
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct FileRules {
    pub source: FileRule,
    pub types: FileRule,
    pub test: FileRule,
}
````

````rs
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct FileRule {
    #[serde(default)]
    pub strip_lang_ext: bool,
    #[serde(default)]
    pub prefix: String,
    #[serde(default)]
    pub suffix: String,
    pub extension: String,
}
````

````rs
impl Descriptor {
    pub fn file_rule(&self, kind: OutputKind) -> &FileRule {
        match kind {
            OutputKind::Source => &self.files.source,
            OutputKind::Types => &self.files.types,
            OutputKind::Test => &self.files.test,
        }
    }
}
````

````rs
pub(crate) fn builtin_descriptor(lang: &Lang) -> Descriptor {
    let raw = match lang {
        Lang::TypeScript => include_str!("descriptors/ts.toml"),
        Lang::Python => include_str!("descriptors/py.toml"),
        Lang::Rust => include_str!("descriptors/rs.toml"),
        Lang::Other(ext) => panic!("no built-in descriptor for `{ext}`"),
    };
    toml::from_str(raw).expect("built-in descriptor must parse")
}
````

````rs
pub(crate) fn output_relative_path(relative: &Path, lang: &Lang, kind: OutputKind) -> PathBuf {
    if matches!((lang, kind), (Lang::Rust, OutputKind::Source)) && relative == Path::new("build.rs.md") {
        return PathBuf::from("build.rs");
    }
    let descriptor = builtin_descriptor(lang);
    let rule = descriptor.file_rule(kind);
    apply_file_rule(relative, &descriptor.language.primary_ext, rule)
}
````

````rs
fn apply_file_rule(relative: &Path, primary_ext: &str, rule: &FileRule) -> PathBuf {
    let stripped = strip_md_extension(relative);
    let parent = stripped.parent().map(PathBuf::from).unwrap_or_default();
    let file_name = stripped.file_name().unwrap_or_default().to_string_lossy();
    let target_suffix = format!(".{}", rule.extension);
    let base_without_target = file_name.strip_suffix(&target_suffix).unwrap_or(&file_name);
    let base = if rule.strip_lang_ext {
        let primary_suffix = format!(".{primary_ext}");
        base_without_target
            .strip_suffix(&primary_suffix)
            .unwrap_or(base_without_target)
    } else {
        base_without_target
    };
    let output_name = format!("{}{}{}.{}", rule.prefix, base, rule.suffix, rule.extension);
    parent.join(output_name)
}
````

````rs
fn strip_md_extension(path: &Path) -> PathBuf {
    let name = path.file_name().unwrap_or_default().to_string_lossy();
    let stripped = name.strip_suffix(".md").unwrap_or(&name);
    path.with_file_name(stripped)
}
````
