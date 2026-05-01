# build.rs

## Purpose

Migrated implementation source for `build.rs`.

## Contract

- Preserve the behavior of the pre-migration Rust source.
- This file is synchronized into `.build/rust/mds-core/build.rs`.

## Source

````rs
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let templates_dir = Path::new("src/init/templates");
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("template_registry.rs");

    println!("cargo:rerun-if-changed=src/init/templates");

    let mut code = String::new();
    code.push_str("/// Auto-generated template registry from manifest.toml files.\n");
    code.push_str("pub(crate) struct TemplateEntry {\n");
    code.push_str("    pub output_path: &'static str,\n");
    code.push_str("    pub category: &'static str,\n");
    code.push_str("    pub content: &'static str,\n");
    code.push_str("}\n\n");

    let mut targets: Vec<String> = Vec::new();

    if let Ok(entries) = fs::read_dir(templates_dir) {
        let mut dirs: Vec<_> = entries.filter_map(|e| e.ok()).collect();
        dirs.sort_by_key(|e| e.file_name());

        for entry in dirs {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let manifest_path = path.join("manifest.toml");
            if !manifest_path.exists() {
                continue;
            }

            let target_name = path.file_name().unwrap().to_str().unwrap().to_string();
            let const_name = target_name.replace('-', "_").to_uppercase();

            println!("cargo:rerun-if-changed={}", manifest_path.display());

            let manifest_content = fs::read_to_string(&manifest_path)
                .unwrap_or_else(|e| panic!("failed to read {}: {e}", manifest_path.display()));

            let manifest: toml::Value = manifest_content
                .parse()
                .unwrap_or_else(|e| panic!("failed to parse {}: {e}", manifest_path.display()));

            let files = manifest
                .get("file")
                .and_then(|v| v.as_array())
                .unwrap_or_else(|| panic!("no [[file]] entries in {}", manifest_path.display()));

            code.push_str(&format!(
                "pub(crate) const {const_name}: &[TemplateEntry] = &[\n"
            ));

            for file_entry in files {
                let template = file_entry.get("template").and_then(|v| v.as_str()).unwrap();
                let output_path = file_entry
                    .get("output_path")
                    .and_then(|v| v.as_str())
                    .unwrap();
                let category = file_entry.get("category").and_then(|v| v.as_str()).unwrap();

                let template_path = path.join(template);
                println!("cargo:rerun-if-changed={}", template_path.display());

                let content = fs::read_to_string(&template_path)
                    .unwrap_or_else(|e| panic!("failed to read {}: {e}", template_path.display()));

                code.push_str("    TemplateEntry {\n");
                code.push_str(&format!("        output_path: {:?},\n", output_path));
                code.push_str(&format!("        category: {:?},\n", category));
                code.push_str(&format!("        content: {:?},\n", content));
                code.push_str("    },\n");
            }

            code.push_str("];\n\n");
            targets.push(target_name);
        }
    }

    // Generate a lookup function
    code.push_str(
        "pub(crate) fn templates_for_target(target: &str) -> &'static [TemplateEntry] {\n",
    );
    code.push_str("    match target {\n");
    for target in &targets {
        let const_name = target.replace('-', "_").to_uppercase();
        code.push_str(&format!("        {:?} => {const_name},\n", target));
    }
    code.push_str("        _ => &[],\n");
    code.push_str("    }\n");
    code.push_str("}\n");

    fs::write(&dest_path, code).unwrap();
}
````
