# src/new.rs

## Purpose

Migrated implementation source for `src/new.rs`.

## Contract

- Preserve the behavior of the pre-migration Rust source.
- This file is synchronized into `.build/rust/mds-core/src/new.rs`.

## Source

````rs
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::diagnostics::{Diagnostic, RunState};
use crate::fs_utils::is_mds_managed_file;
use crate::model::NewOptions;
````

````rs
pub(crate) fn run_new(
    cwd: &Path,
    package: Option<&Path>,
    options: &NewOptions,
    _verbose: bool,
    state: &mut RunState,
) -> Result<(), String> {
    let root = package.map_or_else(|| cwd.to_path_buf(), |path| cwd.join(path));
    let name = &options.name;
    let doc_kind = detect_doc_kind(name);

    let is_index = name == "overview.md" || name.ends_with("/overview.md");

    if matches!(doc_kind, DocKind::Source) && !is_index {
        let lang = detect_lang(name);
        if lang.is_none() {
            return Err(format!(
                "cannot detect language from `{name}`; expected overview.md or a name ending in .{{lang}}.md (e.g. greet.ts.md, utils.go.md)"
            ));
        }
    }

    let target = markdown_target_path(&root, name, doc_kind);
    if target.exists() && !is_mds_managed_file(&target) && !options.force {
        state.diagnostics.push(Diagnostic::error(
            Some(target.clone()),
            "file already exists and is not mds-managed; use --force to overwrite",
        ));
        return Ok(());
    }

    let labels = read_label_overrides(&root);

    let feature_name = extract_feature_name(name);
    let content = if is_index {
        generate_index_template(&feature_name, &labels, doc_kind)
    } else if matches!(doc_kind, DocKind::Test) {
        generate_test_template(&feature_name, &labels)
    } else {
        generate_impl_template(name, &feature_name, detect_lang(name).unwrap(), &labels)
    };

    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("failed to create {}: {e}", parent.display()))?;
    }
    fs::write(&target, &content)
        .map_err(|e| format!("failed to write {}: {e}", target.display()))?;

    state.generated.push(target.clone());
    state
        .stdout
        .push_str(&format!("created {}\n", target.display()));
    Ok(())
}
````

````rs
#[derive(Clone, Copy, Eq, PartialEq)]
enum DocKind {
    Source,
    Test,
}
````

````rs
fn detect_doc_kind(name: &str) -> DocKind {
    if name == "overview.md" || name.ends_with("/overview.md") || detect_lang(name).is_some() {
        DocKind::Source
    } else {
        DocKind::Test
    }
}
````

````rs
fn markdown_target_path(root: &Path, name: &str, doc_kind: DocKind) -> PathBuf {
    match doc_kind {
        DocKind::Source => root.join(".mds/source").join(name),
        DocKind::Test => root.join(".mds/test").join(name),
    }
}
````

````rs
fn read_label_overrides(root: &Path) -> HashMap<String, String> {
    let config_path = root.join("mds.config.toml");
    let mut labels = HashMap::new();
    if let Ok(content) = fs::read_to_string(&config_path) {
        if let Ok(table) = content.parse::<toml::Value>() {
            for table_name in ["labels", "label_overrides", "label-overrides"] {
                if let Some(label_table) = table.get(table_name).and_then(toml::Value::as_table) {
                    for (key, value) in label_table {
                        if let Some(s) = value.as_str() {
                            labels.insert(key.to_ascii_lowercase(), s.to_string());
                        }
                    }
                }
            }
        }
    }
    labels
}
````

````rs
fn label<'a>(labels: &'a HashMap<String, String>, canonical: &str, default: &'a str) -> &'a str {
    labels.get(canonical).map(|s| s.as_str()).unwrap_or(default)
}
````

````rs
fn detect_lang(name: &str) -> Option<&str> {
    let without_md = name.strip_suffix(".md")?;
    let dot_pos = without_md.rfind('.')?;
    let ext = &without_md[dot_pos + 1..];
    if !ext.is_empty() && ext.chars().all(|c| c.is_ascii_alphanumeric()) {
        Some(ext)
    } else {
        None
    }
}
````

````rs
fn extract_feature_name(name: &str) -> String {
    let base = name
        .trim_end_matches(".md")
        .trim_end_matches(".ts")
        .trim_end_matches(".py")
        .trim_end_matches(".rs");
    let base = if base == "index"
        || base.ends_with("/index")
        || base == "overview"
        || base.ends_with("/overview")
    {
        if let Some(pos) = base.rfind('/') {
            let dir = &base[..pos];
            if let Some(pos2) = dir.rfind('/') {
                &dir[pos2 + 1..]
            } else {
                dir
            }
        } else {
            "Index"
        }
    } else if let Some(pos) = base.rfind('/') {
        &base[pos + 1..]
    } else {
        base
    };
    to_title_case(base)
}
````

````rs
fn to_title_case(s: &str) -> String {
    s.split(['-', '_'])
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => {
                    let upper: String = first.to_uppercase().collect();
                    upper + chars.as_str()
                }
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
````

````rs
fn generate_index_template(
    feature_name: &str,
    labels: &HashMap<String, String>,
    doc_kind: DocKind,
) -> String {
    let l_purpose = label(labels, "purpose", "Purpose");
    let l_exposes = label(labels, "exposes", "Exposes");
    let l_kind = label(labels, "kind", "Kind");
    let l_name = label(labels, "name", "Name");
    let l_target = label(labels, "target", "Target");
    let l_summary = label(labels, "summary", "Summary");
    let architecture = match doc_kind {
        DocKind::Source => "Markdown files in this directory are the source of truth.",
        DocKind::Test => "Markdown files in this directory define executable verification intent.",
    };
    let rule = match doc_kind {
        DocKind::Source => "- Keep one source md per feature.",
        DocKind::Test => "- Keep one test md per verification target.",
    };
    format!(
        "<!-- Generated by mds new. -->\n\
         # {feature_name}\n\
         \n\
         ## {l_purpose}\n\
         \n\
         Describe the purpose of this directory.\n\
         \n\
         ## Architecture\n\
         \n\
         {architecture}\n\
         \n\
         ## {l_exposes}\n\
         \n\
         | {l_kind} | {l_name} | {l_target} | {l_summary} |\n\
         | --- | --- | --- | --- |\n\
         \n\
         ## Rules\n\
         \n\
         {rule}\n"
    )
}
````

````rs
fn generate_impl_template(
    name: &str,
    feature_name: &str,
    lang: &str,
    labels: &HashMap<String, String>,
) -> String {
    let source_block = match lang {
        "ts" => ts_source_template(name, feature_name),
        "py" => py_source_template(feature_name),
        "rs" => rs_source_template(feature_name),
        _ => generic_source_template(lang, feature_name),
    };

    let l_purpose = label(labels, "purpose", "Purpose");
    let l_source = label(labels, "source", "Source");

    format!(
        "<!-- Generated by mds new. -->\n\
         # {feature_name}\n\
         \n\
         ## {l_purpose}\n\
         \n\
         Describe the purpose of this feature.\n\
         \n\
         ## {l_source}\n\
         \n\
         {source_block}\n"
    )
}
````

````rs
fn generate_test_template(feature_name: &str, labels: &HashMap<String, String>) -> String {
    let l_purpose = label(labels, "purpose", "Purpose");
    let l_from = label(labels, "from", "From");
    let l_target = label(labels, "target", "Target");
    let l_expose = label(labels, "expose", "Expose");
    let l_summary = label(labels, "summary", "Summary");
    let l_cases = label(labels, "cases", "Cases");
    let l_test = label(labels, "test", "Test");

    format!(
        "<!-- Generated by mds new. -->\n\
         # {feature_name} test\n\
         \n\
         ## {l_purpose}\n\
         \n\
         Describe the behavior being verified.\n\
         \n\
         ## Covers\n\
         \n\
         - {feature_name}\n\
         \n\
         ## Uses\n\
         \n\
         | {l_from} | {l_target} | {l_expose} | {l_summary} |\n\
         | --- | --- | --- | --- |\n\
         | internal | {feature_name} | {feature_name} | Function under test |\n\
         \n\
         ## {l_cases}\n\
         \n\
         - Describe the expected behavior.\n\
         \n\
         ## {l_test}\n\
         \n\
         ```ts\n// Implement your test here.\n```\n"
    )
}
````

````rs
fn ts_source_template(_name: &str, _feature_name: &str) -> String {
    "```ts\n// Implement your feature here.\n```".to_string()
}
````

````rs
fn py_source_template(_feature_name: &str) -> String {
    "```py\n# Implement your feature here.\n```".to_string()
}
````

````rs
fn rs_source_template(_feature_name: &str) -> String {
    "```rs\n// Implement your feature here.\n```".to_string()
}
````

````rs
fn generic_source_template(lang: &str, _feature_name: &str) -> String {
    format!("```{lang}\n// Implement your feature here.\n```")
}
````
