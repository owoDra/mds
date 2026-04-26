use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use mds_core::{execute, BuildMode, CliRequest, Command};

static TEMP_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[test]
fn builds_three_language_fixture() {
    let temp = TestDir::new();
    write_fixture(temp.path());

    let check = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Check,
    });
    assert_eq!(check.exit_code, 0, "{}", check.stderr);
    assert!(check.stdout.contains("check ok"));

    let dry_run = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Build {
            mode: BuildMode::DryRun,
        },
    });
    assert_eq!(dry_run.exit_code, 0, "{}", dry_run.stderr);
    assert!(dry_run.stdout.contains("Build plan:"));
    assert!(dry_run.stdout.contains("bar.types.ts"));
    assert!(dry_run.stdout.contains(".mds/manifest.toml"));
    assert!(dry_run.stdout.contains("src/lib.rs"));
    assert!(dry_run.stdout.contains("--- /dev/null"));
    assert!(!temp.path().join("pkg/src/foo/bar.ts").exists());

    let build = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Build {
            mode: BuildMode::Write,
        },
    });
    assert_eq!(build.exit_code, 0, "{}", build.stderr);
    assert!(temp.path().join("pkg/src/foo/bar.ts").exists());
    assert!(temp.path().join("pkg/src/pkg/foo.py").exists());
    assert!(temp.path().join("pkg/src/foo/bar.rs").exists());
    assert!(temp.path().join("pkg/.mds/manifest.toml").exists());
    assert!(fs::read_to_string(temp.path().join("pkg/src/lib.rs"))
        .unwrap()
        .contains("pub mod foo"));
}

#[test]
fn merges_root_and_package_config() {
    let temp = TestDir::new();
    write_fixture(temp.path());
    fs::write(
        temp.path().join("mds.config.toml"),
        "[roots]\nsource = \"generated\"\ntypes = \"generated\"\n",
    )
    .unwrap();

    let build = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Build {
            mode: BuildMode::Write,
        },
    });
    assert_eq!(build.exit_code, 0, "{}", build.stderr);
    assert!(temp.path().join("pkg/generated/foo/bar.ts").exists());
    assert!(temp.path().join("pkg/generated/foo/bar.types.ts").exists());
}

#[test]
fn checks_dependency_versions_against_metadata() {
    let temp = TestDir::new();
    write_fixture(temp.path());
    fs::write(
        temp.path().join("pkg/package.json"),
        "{\"name\":\"fixture\",\"version\":\"0.1.0\",\"dependencies\":{\"left-pad\":\"1.3.0\"}}\n",
    )
    .unwrap();

    let check = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Check,
    });
    assert_eq!(check.exit_code, 1);
    assert!(check.stderr.contains("missing dependency `left-pad`"));
}

#[test]
fn package_sync_updates_managed_sections_and_preserves_rules() {
    let temp = TestDir::new();
    write_fixture(temp.path());
    fs::write(
        temp.path().join("pkg/package.json"),
        "{\"name\":\"fixture\",\"version\":\"0.2.0\",\"dependencies\":{\"left-pad\":\"1.3.0\"},\"devDependencies\":{\"vitest\":\"2.0.0\"}}\n",
    )
    .unwrap();

    let check = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::PackageSync { check: true },
    });
    assert_eq!(check.exit_code, 1);
    assert!(check.stdout.contains("left-pad"));
    assert!(!fs::read_to_string(temp.path().join("pkg/package.md"))
        .unwrap()
        .contains("0.2.0"));

    let sync = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::PackageSync { check: false },
    });
    assert_eq!(sync.exit_code, 0, "{}", sync.stderr);
    let package_md = fs::read_to_string(temp.path().join("pkg/package.md")).unwrap();
    assert!(package_md.contains("| fixture | 0.2.0 |"));
    assert!(package_md.contains("| left-pad | 1.3.0 |  |"));
    assert!(package_md.contains("| vitest | 2.0.0 |  |"));
    assert!(package_md.contains("## Rules\n\n- test fixture"));
}

#[test]
fn rejects_table_missing_required_columns() {
    let temp = TestDir::new();
    write_fixture(temp.path());
    let doc = temp.path().join("pkg/src-md/foo/bar.ts.md");
    let text = fs::read_to_string(&doc).unwrap().replace(
        "| From | Target | Expose | Summary |",
        "| From | Expose | Summary |",
    );
    fs::write(doc, text).unwrap();

    let check = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Check,
    });
    assert_eq!(check.exit_code, 1);
    assert!(check.stderr.contains("missing required columns"));
}

#[test]
fn rust_module_block_includes_index_exposes() {
    let temp = TestDir::new();
    write_fixture(temp.path());
    fs::write(
        temp.path().join("pkg/src-md/index.md"),
        "# Index\n\n## Purpose\n\nFixture.\n\n## Architecture\n\nFixture.\n\n## Exposes\n\n| Kind | Name | Target | Summary |\n| --- | --- | --- | --- |\n| module | Extra | extra/baz | extra module |\n\n## Rules\n\n- Fixture.\n",
    )
    .unwrap();

    let build = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Build {
            mode: BuildMode::Write,
        },
    });
    assert_eq!(build.exit_code, 0, "{}", build.stderr);
    let lib = fs::read_to_string(temp.path().join("pkg/src/lib.rs")).unwrap();
    assert!(lib.contains("pub mod extra"));
    assert!(lib.contains("pub mod baz;"));
}

#[test]
fn rejects_invalid_internal_target() {
    let temp = TestDir::new();
    write_fixture(temp.path());
    let doc = temp.path().join("pkg/src-md/foo/bar.ts.md");
    let text = fs::read_to_string(&doc).unwrap().replace(
        "| internal | foo/util | Util | helper |",
        "| internal | ./foo/util.ts | Util | helper |",
    );
    fs::write(doc, text).unwrap();

    let check = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Check,
    });
    assert_eq!(check.exit_code, 1);
    assert!(check.stderr.contains("internal Uses.Target must be"));
}

#[test]
fn supports_typescript_extended_uses_expose_tokens() {
    let temp = TestDir::new();
    write_fixture(temp.path());
    let doc = temp.path().join("pkg/src-md/foo/bar.ts.md");
    let text = fs::read_to_string(&doc).unwrap().replace(
        "| internal | foo/util | Util | helper |",
        "| package | fixture-lib | default: Fixture, Util as Renamed | helper |",
    );
    fs::write(doc, text).unwrap();

    let build = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Build {
            mode: BuildMode::Write,
        },
    });
    assert_eq!(build.exit_code, 0, "{}", build.stderr);
    let types = fs::read_to_string(temp.path().join("pkg/src/foo/bar.types.ts")).unwrap();
    assert!(types.contains("import type Fixture, { Util as Renamed } from \"fixture-lib\";"));
}

#[test]
fn rejects_invalid_default_namespace_combination() {
    let temp = TestDir::new();
    write_fixture(temp.path());
    let doc = temp.path().join("pkg/src-md/foo/bar.ts.md");
    let text = fs::read_to_string(&doc).unwrap().replace(
        "| internal | foo/util | Util | helper |",
        "| package | fixture-lib | default: Fixture, * as ns | helper |",
    );
    fs::write(doc, text).unwrap();

    let check = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Check,
    });
    assert_eq!(check.exit_code, 1);
    assert!(check.stderr.contains("default and namespace"));
}

#[test]
fn reports_adapter_unsupported_import_tokens() {
    let temp = TestDir::new();
    write_fixture(temp.path());
    let doc = temp.path().join("pkg/src-md/pkg/foo.py.md");
    let text = fs::read_to_string(&doc).unwrap().replace(
        "## Types\n\n| From | Target | Expose | Summary |\n| --- | --- | --- | --- |",
        "## Types\n\n| From | Target | Expose | Summary |\n| --- | --- | --- | --- |\n| package | fixture_py | default: Fixture | helper |",
    );
    fs::write(doc, text).unwrap();

    let check = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Check,
    });
    assert_eq!(check.exit_code, 1);
    assert!(check.stderr.contains("Python adapter does not support"));
}

#[test]
fn reports_unsupported_config_key_as_warning() {
    let temp = TestDir::new();
    write_fixture(temp.path());
    fs::write(
        temp.path().join("pkg/mds.config.toml"),
        "[package]\nenabled = true\nunknown = true\n",
    )
    .unwrap();

    let check = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Check,
    });
    assert_eq!(check.exit_code, 0, "{}", check.stderr);
    assert!(check.stderr.contains("warning:"));
}

#[test]
fn rejects_package_metadata_mismatch() {
    let temp = TestDir::new();
    write_fixture(temp.path());
    let package_md = temp.path().join("pkg/package.md");
    let text = fs::read_to_string(&package_md)
        .unwrap()
        .replace("| fixture | 0.1.0 |", "| other | 0.1.0 |");
    fs::write(package_md, text).unwrap();

    let check = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Check,
    });
    assert_eq!(check.exit_code, 1);
    assert!(check.stderr.contains("does not match metadata"));
}

#[test]
fn rejects_broken_manifest_before_building() {
    let temp = TestDir::new();
    write_fixture(temp.path());
    fs::create_dir_all(temp.path().join("pkg/.mds")).unwrap();
    fs::write(temp.path().join("pkg/.mds/manifest.toml"), "not manifest\n").unwrap();

    let build = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Build {
            mode: BuildMode::Write,
        },
    });
    assert_eq!(build.exit_code, 1);
    assert!(build.stderr.contains("manifest schema requires"));
    assert!(!temp.path().join("pkg/src/foo/bar.ts").exists());
}

#[test]
fn lint_fix_check_reports_diff_without_writing_and_fix_writes_code_blocks_only() {
    let temp = TestDir::new();
    write_fixture(temp.path());
    let fixer = write_tool(
        temp.path(),
        "fixer",
        "#!/bin/sh\nprintf 'formatted_code()\n' > \"$1\"\n",
    );
    fs::write(
        temp.path().join("pkg/mds.config.toml"),
        format!(
            "[package]\nenabled = true\nallow_raw_source = false\n\n[quality.ts]\nfixer = \"{}\"\nrequired = []\noptional = []\n\n[quality.py]\nfixer = false\nrequired = []\noptional = []\n\n[quality.rs]\nfixer = false\nrequired = []\noptional = []\n",
            fixer.display()
        ),
    )
    .unwrap();
    let doc = temp.path().join("pkg/src-md/foo/bar.ts.md");
    let original = fs::read_to_string(&doc).unwrap();

    let check = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Lint {
            fix: true,
            check: true,
        },
    });
    assert_eq!(check.exit_code, 1);
    assert!(check.stdout.contains("formatted_code"));
    assert_eq!(fs::read_to_string(&doc).unwrap(), original);

    let fix = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Lint {
            fix: true,
            check: false,
        },
    });
    assert_eq!(fix.exit_code, 0, "{}", fix.stderr);
    let fixed = fs::read_to_string(&doc).unwrap();
    assert!(fixed.contains("formatted_code()"));
    assert!(fixed.contains("## Purpose"));
    assert!(fixed.contains("| From | Target | Expose | Summary |"));
}

#[test]
fn lint_and_test_use_configured_toolchain_commands() {
    let temp = TestDir::new();
    write_fixture(temp.path());
    let tool = write_tool(temp.path(), "ok-tool", "#!/bin/sh\ntest -f \"$1\"\n");
    fs::write(
        temp.path().join("pkg/mds.config.toml"),
        format!(
            "[package]\nenabled = true\nallow_raw_source = false\n\n[quality.ts]\nlinter = \"{}\"\ntest_runner = \"{}\"\nrequired = []\noptional = []\n\n[quality.py]\nlinter = false\ntest_runner = false\nrequired = []\noptional = []\n\n[quality.rs]\nlinter = false\ntest_runner = false\nrequired = []\noptional = []\n",
            tool.display(),
            tool.display()
        ),
    )
    .unwrap();

    let lint = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Lint {
            fix: false,
            check: false,
        },
    });
    assert_eq!(lint.exit_code, 0, "{}", lint.stderr);
    assert!(lint.stdout.contains("lint ok"));

    let test = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Test,
    });
    assert_eq!(test.exit_code, 0, "{}", test.stderr);
    assert!(test.stdout.contains("test ok"));
}

#[test]
fn lint_reports_environment_missing_as_exit_code_four() {
    let temp = TestDir::new();
    write_fixture(temp.path());
    fs::write(
        temp.path().join("pkg/mds.config.toml"),
        "[package]\nenabled = true\n\n[quality.ts]\nlinter = \"/missing/mds-tool\"\nrequired = []\noptional = []\n\n[quality.py]\nlinter = false\nrequired = []\noptional = []\n\n[quality.rs]\nlinter = false\nrequired = []\noptional = []\n",
    )
    .unwrap();

    let lint = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Lint {
            fix: false,
            check: false,
        },
    });
    assert_eq!(lint.exit_code, 4);
    assert!(lint.stderr.contains("required toolchain"));
}

#[test]
fn doctor_outputs_json_and_uses_exit_code_four_for_missing_required_tools() {
    let temp = TestDir::new();
    write_fixture(temp.path());
    fs::write(
        temp.path().join("pkg/mds.config.toml"),
        "[package]\nenabled = true\n\n[adapters.py]\nenabled = false\n\n[adapters.rs]\nenabled = false\n\n[quality.ts]\nrequired = [\"/missing/mds-doctor-tool\"]\noptional = []\n",
    )
    .unwrap();

    let doctor = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Doctor {
            format: mds_core::DoctorFormat::Json,
        },
    });
    assert_eq!(doctor.exit_code, 4);
    assert!(doctor.stdout.starts_with("{\"checks\":"));
    assert!(doctor.stdout.contains("/missing/mds-doctor-tool"));
}

#[test]
fn doctor_rejects_runtime_versions_below_minimum() {
    let temp = TestDir::new();
    write_fixture(temp.path());
    let node = write_tool(temp.path(), "node", "#!/bin/sh\nprintf 'v23.0.0\n'\n");
    fs::write(
        temp.path().join("pkg/mds.config.toml"),
        format!(
            "[package]\nenabled = true\n\n[adapters.py]\nenabled = false\n\n[adapters.rs]\nenabled = false\n\n[quality.ts]\nrequired = [\"{}\"]\noptional = []\n",
            node.display()
        ),
    )
    .unwrap();

    let doctor = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Doctor {
            format: mds_core::DoctorFormat::Text,
        },
    });
    assert_eq!(doctor.exit_code, 4);
    assert!(doctor.stderr.contains("DOCTOR002_VERSION_TOO_OLD"));
}

#[test]
fn exclude_skips_markdown_discovery_and_generation_outputs() {
    let temp = TestDir::new();
    write_fixture(temp.path());
    fs::write(
        temp.path().join("pkg/mds.config.toml"),
        "[package]\nenabled = true\nallow_raw_source = false\n\n[roots]\nexclude = [\"src-md/foo/bar.rs.md\", \"src/foo/bar.rs\"]\n",
    )
    .unwrap();

    let build = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Build {
            mode: BuildMode::Write,
        },
    });
    assert_eq!(build.exit_code, 0, "{}", build.stderr);
    assert!(!temp.path().join("pkg/src/foo/bar.rs").exists());
    assert!(temp.path().join("pkg/src/foo/bar.ts").exists());
}

#[test]
fn label_overrides_preserve_canonical_table_and_section_meaning() {
    let temp = TestDir::new();
    write_fixture(temp.path());
    fs::write(
        temp.path().join("pkg/mds.config.toml"),
        "[package]\nenabled = true\nallow_raw_source = false\n\n[labels]\ntypes = \"Type Definitions\"\nfrom = \"Origin\"\ntarget = \"Module\"\nexpose = \"Symbols\"\nsummary = \"Notes\"\n",
    )
    .unwrap();
    let doc = temp.path().join("pkg/src-md/foo/bar.ts.md");
    let text = fs::read_to_string(&doc)
        .unwrap()
        .replace("## Types", "## Type Definitions")
        .replace(
            "| From | Target | Expose | Summary |\n| --- | --- | --- | --- |\n| internal | foo/util | Util | helper |",
            "| Origin | Module | Symbols | Notes |\n| --- | --- | --- | --- |\n| internal | foo/util | Util | helper |",
        );
    fs::write(doc, text).unwrap();

    let check = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Check,
    });
    assert_eq!(check.exit_code, 0, "{}", check.stderr);
}

#[test]
fn validates_local_markdown_links() {
    let temp = TestDir::new();
    write_fixture(temp.path());
    let doc = temp.path().join("pkg/src-md/foo/bar.ts.md");
    let text = fs::read_to_string(&doc)
        .unwrap()
        .replace("Fixture.", "Fixture. [Missing](missing.md)");
    fs::write(doc, text).unwrap();

    let check = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Check,
    });
    assert_eq!(check.exit_code, 1);
    assert!(check.stderr.contains("Markdown link target does not exist"));
}

#[test]
fn table_parser_keeps_pipes_inside_code_spans() {
    let temp = TestDir::new();
    write_fixture(temp.path());
    let doc = temp.path().join("pkg/src-md/foo/bar.ts.md");
    let text = fs::read_to_string(&doc)
        .unwrap()
        .replace(
            "| internal | foo/util | Util | helper |",
            "| internal | foo/util | Util | helper `a | b` |",
        )
        .replace("## Contract", "[Util](util.ts.md)\n\n## Contract");
    fs::write(doc, text).unwrap();

    let check = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Check,
    });
    assert_eq!(check.exit_code, 0, "{}", check.stderr);
}

#[test]
fn metadata_parser_handles_common_json_toml_dependency_shapes() {
    let temp = TestDir::new();
    write_fixture(temp.path());
    fs::write(
        temp.path().join("pkg/package.json"),
        "{\n  \"name\": \"fixture\",\n  \"version\": \"0.1.0\",\n  \"dependencies\": {\n    \"simple\": \"1.0.0\",\n    \"detailed\": { \"version\": \"2.0.0\" }\n  }\n}\n",
    )
    .unwrap();
    let check = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Check,
    });
    assert_eq!(check.exit_code, 1);
    assert!(check.stderr.contains("missing dependency `simple`"));
    assert!(check.stderr.contains("missing dependency `detailed`"));

    let rust_pkg = temp.path().join("rust-pkg");
    fs::create_dir_all(&rust_pkg).unwrap();
    fs::write(
        rust_pkg.join("mds.config.toml"),
        "[package]\nenabled = true\nallow_raw_source = false\n",
    )
    .unwrap();
    fs::write(
        rust_pkg.join("Cargo.toml"),
        "[package]\nname = \"rust-fixture\"\nversion = \"0.1.0\"\n\n[dependencies]\nserde = { version = \"1.0\", features = [\"derive\"] }\n",
    )
    .unwrap();
    fs::write(
        rust_pkg.join("package.md"),
        "# Package\n\n## Package\n\n| Name | Version |\n| --- | --- |\n| rust-fixture | 0.1.0 |\n\n## Dependencies\n\n| Name | Version | Summary |\n| --- | --- | --- |\n\n## Dev Dependencies\n\n| Name | Version | Summary |\n| --- | --- | --- |\n\n## Rules\n\n- test fixture\n",
    )
    .unwrap();

    let rust_check = execute(CliRequest {
        cwd: rust_pkg.clone(),
        package: None,
        verbose: false,
        command: Command::Check,
    });
    assert_eq!(rust_check.exit_code, 1);
    assert!(rust_check.stderr.contains("missing dependency `serde`"));
}

#[test]
fn package_sync_rejects_handwritten_content_inside_managed_sections() {
    let temp = TestDir::new();
    write_fixture(temp.path());
    let package_md = temp.path().join("pkg/package.md");
    let text = fs::read_to_string(&package_md)
        .unwrap()
        .replace("## Dependencies\n\n", "## Dependencies\n\nManual note.\n");
    fs::write(package_md, text).unwrap();

    let sync = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::PackageSync { check: false },
    });
    assert_eq!(sync.exit_code, 1);
    assert!(sync.stderr.contains("hand-written content"));
}

#[test]
fn package_sync_hook_enabled_uses_default_check_command() {
    let temp = TestDir::new();
    write_fixture(temp.path());
    fs::write(
        temp.path().join("pkg/mds.config.toml"),
        "[package]\nenabled = true\nallow_raw_source = false\n\n[package_sync]\nhook_enabled = true\n",
    )
    .unwrap();

    let sync = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::PackageSync { check: true },
    });
    assert_eq!(sync.exit_code, 0, "{}", sync.stderr);
    assert!(sync
        .stdout
        .contains("package sync hook command: mds package sync --check"));
}

#[test]
fn lint_fix_updates_successful_quality_blocks_only() {
    let temp = TestDir::new();
    write_fixture(temp.path());
    let fixer = write_tool(
        temp.path(),
        "partial-fixer",
        "#!/bin/sh\nif grep -q DO_NOT_FIX \"$1\"; then exit 1; fi\nprintf 'fixed_code()\n' > \"$1\"\n",
    );
    fs::write(
        temp.path().join("pkg/mds.config.toml"),
        format!(
            "[package]\nenabled = true\nallow_raw_source = false\n\n[quality.ts]\nfixer = \"{}\"\nrequired = []\noptional = []\n\n[quality.py]\nfixer = false\nrequired = []\noptional = []\n\n[quality.rs]\nfixer = false\nrequired = []\noptional = []\n",
            fixer.display()
        ),
    )
    .unwrap();
    let doc = temp.path().join("pkg/src-md/foo/bar.ts.md");
    let text = fs::read_to_string(&doc)
        .unwrap()
        .replace(
            "Fixture.\n\n## Contract",
            "Fixture.\n\n```text\nnot_quality_block\n```\n\n## Contract",
        )
        .replace("expect(bar).toBe(\"ok\");", "DO_NOT_FIX");
    fs::write(&doc, text).unwrap();

    let fix = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Lint {
            fix: true,
            check: false,
        },
    });
    assert_eq!(fix.exit_code, 1);
    let fixed = fs::read_to_string(&doc).unwrap();
    assert!(fixed.contains("fixed_code()"));
    assert!(fixed.contains("DO_NOT_FIX"));
    assert!(fixed.contains("not_quality_block"));
}

#[test]
fn refuses_to_overwrite_unmanaged_file() {
    let temp = TestDir::new();
    write_fixture(temp.path());
    fs::create_dir_all(temp.path().join("pkg/src/foo")).unwrap();
    fs::write(temp.path().join("pkg/src/foo/bar.ts"), "manual\n").unwrap();

    let build = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Build {
            mode: BuildMode::Write,
        },
    });
    assert_eq!(build.exit_code, 1);
    assert!(build.stderr.contains("refusing to overwrite"));
    assert_eq!(
        fs::read_to_string(temp.path().join("pkg/src/foo/bar.ts")).unwrap(),
        "manual\n"
    );
}

struct TestDir {
    path: PathBuf,
}

impl TestDir {
    fn new() -> Self {
        let id = TEMP_COUNTER.fetch_add(1, Ordering::SeqCst);
        let path = std::env::temp_dir().join(format!("mds-core-test-{}-{id}", std::process::id()));
        if path.exists() {
            fs::remove_dir_all(&path).unwrap();
        }
        fs::create_dir_all(&path).unwrap();
        Self { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TestDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

fn write_fixture(root: &Path) {
    let package = root.join("pkg");
    fs::create_dir_all(package.join("src-md/foo")).unwrap();
    fs::create_dir_all(package.join("src-md/pkg")).unwrap();
    fs::write(
        package.join("package.json"),
        "{\"name\":\"fixture\",\"version\":\"0.1.0\"}\n",
    )
    .unwrap();
    fs::write(
        package.join("mds.config.toml"),
        "[package]\nenabled = true\nallow_raw_source = false\n",
    )
    .unwrap();
    fs::write(
        package.join("package.md"),
        "# Package\n\n## Package\n\n| Name | Version |\n| --- | --- |\n| fixture | 0.1.0 |\n\n## Dependencies\n\n| Name | Version | Summary |\n| --- | --- | --- |\n\n## Dev Dependencies\n\n| Name | Version | Summary |\n| --- | --- | --- |\n\n## Rules\n\n- test fixture\n",
    )
    .unwrap();
    fs::write(
        package.join("src-md/foo/util.ts.md"),
        impl_doc(
            "ts",
            "Util",
            "export type Util = string;",
            "export const util = \"ok\";",
            "expect(util).toBe(\"ok\");",
            "",
        ),
    )
    .unwrap();
    fs::write(
        package.join("src-md/foo/bar.ts.md"),
        impl_doc(
            "ts",
            "Bar",
            "export type Bar = Util;",
            "export const bar: Bar = util;",
            "expect(bar).toBe(\"ok\");",
            "| internal | foo/util | Util | helper |",
        ),
    )
    .unwrap();
    fs::write(
        package.join("src-md/pkg/foo.py.md"),
        impl_doc(
            "py",
            "Foo",
            "class Foo: ...",
            "VALUE = 1",
            "assert VALUE == 1",
            "",
        ),
    )
    .unwrap();
    fs::write(
        package.join("src-md/foo/bar.rs.md"),
        impl_doc(
            "rs",
            "bar",
            "pub type Bar = String;",
            "pub fn bar() -> Bar { String::from(\"ok\") }",
            "#[test]\nfn works() { assert_eq!(bar(), \"ok\"); }",
            "",
        ),
    )
    .unwrap();
}

fn write_tool(root: &Path, name: &str, script: &str) -> PathBuf {
    let path = root.join("bin").join(name);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(&path, script).unwrap();
    let mut permissions = fs::metadata(&path).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&path, permissions).unwrap();
    path
}

fn impl_doc(
    lang: &str,
    name: &str,
    types: &str,
    source: &str,
    test: &str,
    uses_row: &str,
) -> String {
    let uses = if uses_row.is_empty() {
        "| From | Target | Expose | Summary |\n| --- | --- | --- | --- |\n".to_string()
    } else {
        format!("| From | Target | Expose | Summary |\n| --- | --- | --- | --- |\n{uses_row}\n")
    };
    format!(
        "# {name}\n\n## Purpose\n\nFixture.\n\n## Contract\n\nStable.\n\n## Types\n\n{uses}```{lang}\n{types}\n```\n\n## Source\n\n| From | Target | Expose | Summary |\n| --- | --- | --- | --- |\n\n```{lang}\n{source}\n```\n\n## Cases\n\n- Works.\n\n## Test\n\n| From | Target | Expose | Summary |\n| --- | --- | --- | --- |\n\n```{lang}\n{test}\n```\n"
    )
}
