# src/release_quality/mod.rs

## Purpose

Migrated implementation source for `src/release_quality/mod.rs`.

## Contract

- Preserve the behavior of the pre-migration Rust source.
- This file is synchronized into `.build/rust/mds-core/src/release_quality/mod.rs`.

## Source

````rs
use std::fs;
use std::path::{Path, PathBuf};

use crate::diagnostics::{Diagnostic, RunState};
use crate::hash::sha256_bytes;
use crate::model::ReleaseQualityOptions;

pub(crate) fn run_release_check(
    cwd: &Path,
    options: &ReleaseQualityOptions,
    state: &mut RunState,
) -> Result<(), String> {
    let manifest_path = resolve(cwd, &options.manifest);
    let content = fs::read_to_string(&manifest_path)
        .map_err(|error| format!("failed to read {}: {error}", manifest_path.display()))?;
    let value = content
        .parse::<toml::Value>()
        .map_err(|error| format!("failed to parse {}: {error}", manifest_path.display()))?;
    let Some(artifacts) = value.get("artifacts").and_then(toml::Value::as_array) else {
        state.diagnostics.push(Diagnostic::error(
            Some(manifest_path),
            "release quality manifest requires [[artifacts]] entries",
        ));
        return Ok(());
    };

    // Resolve artifact paths relative to the manifest's parent directory
    let base_dir = manifest_path.parent().unwrap_or(cwd).to_path_buf();

    state.stdout.push_str("Release quality gate:\n");
    for artifact in artifacts {
        check_artifact(&base_dir, artifact, state);
    }
    if !state.has_errors() {
        state.stdout.push_str("release quality ok\n");
    }
    Ok(())
}

fn check_artifact(cwd: &Path, artifact: &toml::Value, state: &mut RunState) {
    let name = field_str(artifact, "name").unwrap_or("<unnamed>");
    let channel = field_str(artifact, "channel").unwrap_or("unknown");

    // Artifacts with requires_published are deferred (depend on publish ordering)
    let requires_published = artifact
        .get("requires_published")
        .and_then(toml::Value::as_str);
    if let Some(dep) = requires_published {
        let path_str = field_str(artifact, "path").unwrap_or("");
        let resolved = resolve(cwd, &PathBuf::from(path_str));
        if !resolved.exists() {
            state.stdout.push_str(&format!(
                "- {name} ({channel}) [deferred: requires {dep} on registry]\n"
            ));
            return;
        }
    }

    state.stdout.push_str(&format!("- {name} ({channel})\n"));

    let artifact_path = required_path_or_dir(cwd, artifact, name, "path", state);
    let checksum_path = required_path(cwd, artifact, name, "checksum", state);
    let signature_path = required_path(cwd, artifact, name, "signature", state);
    let sbom_path = required_path(cwd, artifact, name, "sbom", state);
    let provenance_path = required_path(cwd, artifact, name, "provenance", state);

    if let (Some(artifact_path), Some(checksum_path)) = (&artifact_path, &checksum_path) {
        if artifact_path.is_dir() {
            verify_checksum_exists(name, checksum_path, state);
        } else {
            verify_checksum(name, artifact_path, checksum_path, state);
        }
    }
    if let Some(path) = &signature_path {
        verify_non_empty(name, "signature", path, state);
    }
    if let Some(path) = &sbom_path {
        verify_sbom(name, path, state);
    }
    if let Some(path) = &provenance_path {
        verify_provenance(name, path, state);
    }

    match artifact.get("smoke") {
        Some(value) if value.as_bool() == Some(true) => {}
        Some(value) if value.as_str() == Some("ok") => {}
        _ => state.diagnostics.push(Diagnostic::error(
            None,
            format!("release artifact `{name}` requires successful smoke = true"),
        )),
    }
}

fn required_path_or_dir(
    cwd: &Path,
    artifact: &toml::Value,
    name: &str,
    field: &str,
    state: &mut RunState,
) -> Option<PathBuf> {
    let Some(path) = field_str(artifact, field) else {
        state.diagnostics.push(Diagnostic::error(
            None,
            format!("release artifact `{name}` is missing `{field}`"),
        ));
        return None;
    };
    let resolved = resolve(cwd, &PathBuf::from(path));
    if !resolved.exists() {
        state.diagnostics.push(Diagnostic::error(
            Some(resolved),
            format!("release artifact field `{field}` does not exist for `{name}`"),
        ));
        return None;
    }
    Some(resolved)
}

fn required_path(
    cwd: &Path,
    artifact: &toml::Value,
    name: &str,
    field: &str,
    state: &mut RunState,
) -> Option<PathBuf> {
    let Some(path) = field_str(artifact, field) else {
        state.diagnostics.push(Diagnostic::error(
            None,
            format!("release artifact `{name}` is missing `{field}`"),
        ));
        return None;
    };
    let resolved = resolve(cwd, &PathBuf::from(path));
    if !resolved.exists() {
        state.diagnostics.push(Diagnostic::error(
            Some(resolved),
            format!("release artifact field `{field}` does not exist for `{name}`"),
        ));
        return None;
    }
    Some(resolved)
}

fn verify_checksum_exists(name: &str, checksum_path: &Path, state: &mut RunState) {
    let checksum = match fs::read_to_string(checksum_path) {
        Ok(content) => content,
        Err(error) => {
            state.diagnostics.push(Diagnostic::error(
                Some(checksum_path.to_path_buf()),
                format!("failed to read checksum for `{name}`: {error}"),
            ));
            return;
        }
    };
    let has_digest = checksum
        .split_whitespace()
        .any(|part| part.len() == 64 && part.chars().all(|ch| ch.is_ascii_hexdigit()));
    if !has_digest {
        state.diagnostics.push(Diagnostic::error(
            Some(checksum_path.to_path_buf()),
            format!("checksum for `{name}` must contain a SHA-256 hex digest"),
        ));
    }
}

fn verify_checksum(name: &str, artifact_path: &Path, checksum_path: &Path, state: &mut RunState) {
    let artifact = match fs::read(artifact_path) {
        Ok(content) => content,
        Err(error) => {
            state.diagnostics.push(Diagnostic::error(
                Some(artifact_path.to_path_buf()),
                format!("failed to read artifact `{name}`: {error}"),
            ));
            return;
        }
    };
    let checksum = match fs::read_to_string(checksum_path) {
        Ok(content) => content,
        Err(error) => {
            state.diagnostics.push(Diagnostic::error(
                Some(checksum_path.to_path_buf()),
                format!("failed to read checksum for `{name}`: {error}"),
            ));
            return;
        }
    };
    let expected = checksum
        .split_whitespace()
        .find(|part| part.len() == 64 && part.chars().all(|ch| ch.is_ascii_hexdigit()));
    let actual = hex_sha256(&artifact);
    match expected {
        Some(expected) if expected.eq_ignore_ascii_case(&actual) => {}
        Some(expected) => state.diagnostics.push(Diagnostic::error(
            Some(checksum_path.to_path_buf()),
            format!("checksum mismatch for `{name}`: expected {expected}, actual {actual}"),
        )),
        None => state.diagnostics.push(Diagnostic::error(
            Some(checksum_path.to_path_buf()),
            format!("checksum for `{name}` must contain a SHA-256 hex digest"),
        )),
    }
}

fn verify_non_empty(name: &str, field: &str, path: &Path, state: &mut RunState) {
    match fs::read(path) {
        Ok(content) if !content.is_empty() => {}
        Ok(_) => state.diagnostics.push(Diagnostic::error(
            Some(path.to_path_buf()),
            format!("{field} for `{name}` must not be empty"),
        )),
        Err(error) => state.diagnostics.push(Diagnostic::error(
            Some(path.to_path_buf()),
            format!("failed to read {field} for `{name}`: {error}"),
        )),
    }
}

fn verify_sbom(name: &str, path: &Path, state: &mut RunState) {
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) => {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                format!("failed to read SBOM for `{name}`: {error}"),
            ));
            return;
        }
    };
    let Ok(value) = serde_json::from_str::<serde_json::Value>(&content) else {
        state.diagnostics.push(Diagnostic::error(
            Some(path.to_path_buf()),
            format!("SBOM for `{name}` must be valid JSON"),
        ));
        return;
    };
    let has_known_shape = value.get("spdxVersion").is_some()
        || value.get("SPDXID").is_some()
        || value.get("bomFormat").is_some();
    if !has_known_shape {
        state.diagnostics.push(Diagnostic::error(
            Some(path.to_path_buf()),
            format!("SBOM for `{name}` must look like SPDX or CycloneDX JSON"),
        ));
    }
}

fn verify_provenance(name: &str, path: &Path, state: &mut RunState) {
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) => {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                format!("failed to read provenance for `{name}`: {error}"),
            ));
            return;
        }
    };
    if content.trim().is_empty() {
        state.diagnostics.push(Diagnostic::error(
            Some(path.to_path_buf()),
            format!("provenance for `{name}` must not be empty"),
        ));
        return;
    }
    let json_ok = serde_json::from_str::<serde_json::Value>(&content).is_ok();
    let jsonl_ok = content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .all(|line| serde_json::from_str::<serde_json::Value>(line).is_ok());
    if !json_ok && !jsonl_ok {
        state.diagnostics.push(Diagnostic::error(
            Some(path.to_path_buf()),
            format!("provenance for `{name}` must be valid JSON or JSONL"),
        ));
    }
}

fn hex_sha256(content: &[u8]) -> String {
    sha256_bytes(content)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn field_str<'a>(value: &'a toml::Value, field: &str) -> Option<&'a str> {
    value.get(field).and_then(toml::Value::as_str)
}

fn resolve(cwd: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        cwd.join(path)
    }
}
````
