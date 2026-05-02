# tests/parser_generation_mvp.rs

## Purpose

Migrated implementation source for `tests/parser_generation_mvp.rs`.

## Contract

- Preserve the behavior of the pre-migration Rust source.
- This file is synchronized into `.build/rust/mds-core/tests/parser_generation_mvp.rs`.

## Covers

- src/adapter
- src/config
- src/diagnostics
- src/diff
- src/doctor
- src/fs_utils
- src/generation
- src/hash
- src/init/mod
- src/manifest
- src/markdown
- src/model
- src/new
- src/package
- src/package_sync
- src/quality
- src/release_quality
- src/runner
- src/table

## Test

````rs
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use mds_core::{
    execute, AgentKitCategory, AiTarget, BuildMode, CliRequest, Command, InitOptions,
    InitQualityCommands, InitTargetCategories, Lang, PythonTool, ReleaseQualityOptions, RustTool,
    TypeScriptTool,
};
````

````rs
static TEMP_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[test]
````

````rs
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
    assert!(dry_run.stdout.contains(".mds/manifest.toml"));
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
}

#[test]
````

````rs
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
}

#[test]
````

````rs
fn package_metadata_dependencies_do_not_require_markdown_mirror() {
    let temp = TestDir::new();
    write_fixture(temp.path());
    fs::write(
        temp.path().join("pkg/package.json"),
        "{\"name\":\"fixture\",\"version\":\"0.1.0\",\"dependencies\":{\"left-pad\":\"1.3.0\"}}\n",
    )
    .unwrap();

    let sync = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::PackageSync { check: false },
    });
    assert_eq!(sync.exit_code, 0, "{}", sync.stderr);

    let check = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Check,
    });
    assert_eq!(check.exit_code, 0, "{}", check.stderr);
}

#[test]
````

````rs
fn package_sync_skips_markdown_package_metadata() {
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
    assert!(check.stderr.contains("dependency snapshot is not synchronized"));

    let sync = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::PackageSync { check: false },
    });
    assert_eq!(sync.exit_code, 0, "{}", sync.stderr);
    assert!(sync.stdout.contains("package sync ok"));
    let overview = fs::read_to_string(temp.path().join("pkg/.mds/source/overview.md")).unwrap();
    assert!(overview.contains("left-pad"));
    assert!(overview.contains("vitest"));
}

#[test]
````

````rs
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
````

````rs
#[test]
fn rejects_imports_mixed_with_implementation_code_blocks() {
    let temp = TestDir::new();
    write_fixture(temp.path());
    fs::write(
        temp.path().join("pkg/src-md/foo/mixed.ts.md"),
        "# Mixed\n\n## Purpose\n\nMixed imports.\n\n## Source\n\n```ts\nimport { util } from './util';\nexport const mixed = util;\n```\n",
    )
    .unwrap();

    let check = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Check,
    });
    assert_eq!(check.exit_code, 1);
    assert!(check.stderr.contains("mixes imports with implementation"));
}
````

````rs
#[test]
fn rejects_multiple_top_level_implementations_in_one_code_block() {
    let temp = TestDir::new();
    write_fixture(temp.path());
    fs::write(
        temp.path().join("pkg/src-md/foo/multiple.ts.md"),
        "# Multiple\n\n## Purpose\n\nMultiple declarations.\n\n## Source\n\n```ts\nexport const first = 1;\nexport const second = 2;\n```\n",
    )
    .unwrap();

    let check = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Check,
    });
    assert_eq!(check.exit_code, 1);
    assert!(check.stderr.contains("multiple top-level implementations"));
}

#[test]
fn rejects_unterminated_code_fence() {
    let temp = TestDir::new();
    write_fixture(temp.path());
    fs::write(
        temp.path().join("pkg/src-md/foo/unterminated.ts.md"),
        "# Unterminated\n\n## Purpose\n\nBroken fence.\n\n## Source\n\n```ts\nexport const broken = 1;\n",
    )
    .unwrap();

    let check = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Check,
    });
    assert_eq!(check.exit_code, 1);
    assert!(check.stderr.contains("unterminated code fence"));
}

#[test]
fn rejects_new_fence_opener_before_previous_fence_closes() {
    let temp = TestDir::new();
    write_fixture(temp.path());
    fs::write(
        temp.path().join("pkg/src-md/foo/reopened.ts.md"),
        "# Reopened\n\n## Purpose\n\nBroken fence.\n\n## Source\n\n````ts\nexport const first = 1;\n````ts\nexport const second = 2;\n````\n",
    )
    .unwrap();

    let check = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Check,
    });
    assert_eq!(check.exit_code, 1);
    assert!(check.stderr.contains("before a new fence opener"));
}

#[test]
fn rejects_duplicate_h2_sections() {
    let temp = TestDir::new();
    write_fixture(temp.path());
    fs::write(
        temp.path().join("pkg/src-md/foo/duplicate-sections.ts.md"),
        "# Duplicate\n\n## Purpose\n\nFirst.\n\n## Source\n\n```ts\nexport const first = 1;\n```\n\n## Source\n\n```ts\nexport const second = 2;\n```\n",
    )
    .unwrap();

    let check = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Check,
    });
    assert_eq!(check.exit_code, 1);
    assert!(check.stderr.contains("duplicate H2 section"));
}
````

````rs
#[test]
fn package_check_uses_language_metadata_without_markdown_mirror() {
    let temp = TestDir::new();
    write_fixture(temp.path());

    let check = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Check,
    });
    assert_eq!(check.exit_code, 0, "{}", check.stderr);
}

#[test]
````

````rs
fn rejects_broken_manifest_before_building() {
    let temp = TestDir::new();
    write_fixture(temp.path());
    fs::create_dir_all(temp.path().join("pkg/.mds")).unwrap();
    fs::write(
        temp.path().join("pkg/.mds/manifest.toml"),
        "not manifest\n",
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
    assert_eq!(build.exit_code, 1);
    assert!(build.stderr.contains("manifest schema requires"));
    assert!(!temp.path().join("pkg/src/foo/bar.ts").exists());
}

#[test]
fn builds_types_and_test_outputs_from_fixed_authoring_roots() {
    let temp = TestDir::new();
    let package = temp.path().join("pkg");
    fs::create_dir_all(package.join(".mds/source/foo")).unwrap();
    fs::create_dir_all(package.join(".mds/test/foo")).unwrap();
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
        package.join(".mds/source/overview.md"),
        "# Overview\n\n## Purpose\n\nFixture package.\n\n## Architecture\n\nFixture architecture.\n\n<!-- mds:begin package-summary -->\n| Name | Version |\n| --- | --- |\n| fixture | 0.1.0 |\n<!-- mds:end package-summary -->\n\n## Exposes\n\n| Kind | Name | Target | Summary |\n| --- | --- | --- | --- |\n\n<!-- mds:begin dependencies -->\n| Name | Version | Summary |\n| --- | --- | --- |\n<!-- mds:end dependencies -->\n\n<!-- mds:begin dev-dependencies -->\n| Name | Version | Summary |\n| --- | --- | --- |\n<!-- mds:end dev-dependencies -->\n\n## Rules\n\n- Fixture rules.\n",
    )
    .unwrap();
    fs::write(
        package.join(".mds/source/foo/bar.ts.md"),
        "# Bar\n\n## Purpose\n\nFixture.\n\n## Types\n\n```ts\nexport type Bar = string;\n```\n\n## Source\n\n```ts\nexport const bar: Bar = 'ok';\n```\n",
    )
    .unwrap();
    fs::write(
        package.join(".mds/test/foo/bar.md"),
        "# Bar test\n\n## Purpose\n\nVerify bar.\n\n## Covers\n\n- foo/bar\n\n## Cases\n\n- returns ok\n\n## Test\n\n```ts\nexpect(bar).toBe('ok');\n```\n",
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
    assert!(package.join("src/foo/bar.ts").exists());
    assert!(package.join("src/foo/bar.types.ts").exists());
    assert!(package.join("tests/foo/bar.test.ts").exists());
}

#[test]
fn rejects_test_doc_without_covers() {
    let temp = TestDir::new();
    let package = temp.path().join("pkg");
    fs::create_dir_all(package.join(".mds/source/foo")).unwrap();
    fs::create_dir_all(package.join(".mds/test/foo")).unwrap();
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
        package.join(".mds/source/overview.md"),
        "# Overview\n\n## Purpose\n\nFixture package.\n\n## Architecture\n\nFixture architecture.\n\n<!-- mds:begin package-summary -->\n| Name | Version |\n| --- | --- |\n| fixture | 0.1.0 |\n<!-- mds:end package-summary -->\n\n## Exposes\n\n| Kind | Name | Target | Summary |\n| --- | --- | --- | --- |\n\n<!-- mds:begin dependencies -->\n| Name | Version | Summary |\n| --- | --- | --- |\n<!-- mds:end dependencies -->\n\n<!-- mds:begin dev-dependencies -->\n| Name | Version | Summary |\n| --- | --- | --- |\n<!-- mds:end dev-dependencies -->\n\n## Rules\n\n- Fixture rules.\n",
    )
    .unwrap();
    fs::write(
        package.join(".mds/source/foo/bar.ts.md"),
        "# Bar\n\n## Purpose\n\nFixture.\n\n## Source\n\n```ts\nexport const bar = 'ok';\n```\n",
    )
    .unwrap();
    fs::write(
        package.join(".mds/test/foo/bar.md"),
        "# Bar test\n\n## Purpose\n\nVerify bar.\n\n## Test\n\n```ts\nexpect(bar).toBe('ok');\n```\n",
    )
    .unwrap();

    let check = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Check,
    });
    assert_eq!(check.exit_code, 1);
    assert!(check.stderr.contains("test md requires at least one Covers entry"));
}

#[test]
````

````rs
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
    assert!(fixed.contains("## Source"));
}

#[test]
````

````rs
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
````

````rs
fn lint_reports_markdown_path_and_preserved_line_numbers() {
    let temp = TestDir::new();
    write_fixture(temp.path());
    let tool = write_tool(
        temp.path(),
        "line-reporting-linter",
        "#!/bin/sh\nprintf '%s:9:1: lint failed\n' \"$1\" >&2\nexit 1\n",
    );
    fs::write(
        temp.path().join("pkg/mds.config.toml"),
        format!(
            "[package]\nenabled = true\nallow_raw_source = false\n\n[quality.ts]\nlinter = \"{}\"\nrequired = []\noptional = []\n\n[quality.py]\nlinter = false\nrequired = []\noptional = []\n\n[quality.rs]\nlinter = false\nrequired = []\noptional = []\n",
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
    let md_path = temp.path().join("pkg/src-md/foo/bar.ts.md");
    assert_eq!(lint.exit_code, 1);
    assert!(lint.stderr.contains(&format!("{}:9:1", md_path.display())));
    assert!(!lint.stderr.contains(".build/mds/tmp/source.ts"));
}

#[test]
````

````rs
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
````

````rs
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
````

````rs
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
````

````rs
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
````

````rs
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
````

````rs
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
````

````rs
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
````

````rs
fn metadata_parser_accepts_common_json_toml_dependency_shapes() {
    let temp = TestDir::new();
    write_fixture(temp.path());
    fs::write(
        temp.path().join("pkg/package.json"),
        "{\n  \"name\": \"fixture\",\n  \"version\": \"0.1.0\",\n  \"dependencies\": {\n    \"simple\": \"1.0.0\",\n    \"detailed\": { \"version\": \"2.0.0\" }\n  }\n}\n",
    )
    .unwrap();
    let sync = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::PackageSync { check: false },
    });
    assert_eq!(sync.exit_code, 0, "{}", sync.stderr);
    let check = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Check,
    });
    assert_eq!(check.exit_code, 0, "{}", check.stderr);

    let rust_pkg = temp.path().join("rust-pkg");
    fs::create_dir_all(rust_pkg.join("src-md")).unwrap();
    fs::create_dir_all(rust_pkg.join(".mds/source")).unwrap();
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
        rust_pkg.join("src-md/overview.md"),
        "# Overview\n\n## Purpose\n\nRust fixture.\n\n## Architecture\n\nFixture source.\n\n## Exposes\n\n| Kind | Name | Target | Summary |\n| --- | --- | --- | --- |\n\n## Rules\n\n- Test fixture.\n",
    )
    .unwrap();
    fs::write(
        rust_pkg.join(".mds/source/overview.md"),
        "# Overview\n\n## Purpose\n\nRust fixture.\n\n## Architecture\n\nFixture source.\n\n<!-- mds:begin package-summary -->\n| Name | Version |\n| --- | --- |\n| rust-fixture | 0.1.0 |\n<!-- mds:end package-summary -->\n\n## Exposes\n\n| Kind | Name | Target | Summary |\n| --- | --- | --- | --- |\n\n<!-- mds:begin dependencies -->\n| Name | Version | Summary |\n| --- | --- | --- |\n| serde | 1.0 |  |\n<!-- mds:end dependencies -->\n\n<!-- mds:begin dev-dependencies -->\n| Name | Version | Summary |\n| --- | --- | --- |\n<!-- mds:end dev-dependencies -->\n\n## Rules\n\n- Test fixture.\n",
    )
    .unwrap();
    let rust_check = execute(CliRequest {
        cwd: rust_pkg.clone(),
        package: None,
        verbose: false,
        command: Command::Check,
    });
    assert_eq!(rust_check.exit_code, 0, "{}", rust_check.stderr);
}

#[test]
````

````rs
fn package_sync_requires_source_overview() {
    let temp = TestDir::new();
    write_fixture(temp.path());
    fs::remove_file(temp.path().join("pkg/.mds/source/overview.md")).unwrap();

    let sync = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::PackageSync { check: false },
    });
    assert_eq!(sync.exit_code, 1);
    assert!(sync.stderr.contains("failed to read source overview"));
}

#[test]
fn check_and_build_reject_stale_dependency_snapshot() {
    let temp = TestDir::new();
    write_fixture(temp.path());
    fs::write(
        temp.path().join("pkg/package.json"),
        "{\"name\":\"fixture\",\"version\":\"0.2.0\",\"dependencies\":{\"left-pad\":\"1.3.0\"}}\n",
    )
    .unwrap();

    let check = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Check,
    });
    assert_eq!(check.exit_code, 1);
    assert!(check.stderr.contains("dependency snapshot is not synchronized"));

    let build = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Build {
            mode: BuildMode::Write,
        },
    });
    assert_eq!(build.exit_code, 1);
    assert!(build.stderr.contains("dependency snapshot is not synchronized"));
}

#[test]
````

````rs
fn package_sync_hook_enabled_uses_default_sync_command() {
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
        .contains("package sync hook command: mds package sync"));
}

#[test]
````

````rs
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
            "[package]\nenabled = true\n\n[quality.ts]\nfixer = \"{}\"\nrequired = []\noptional = []\n\n[quality.py]\nfixer = false\nrequired = []\noptional = []\n\n[quality.rs]\nfixer = false\nrequired = []\noptional = []\n",
            fixer.display()
        ),
    )
    .unwrap();
    let doc = temp.path().join("pkg/src-md/foo/bar.ts.md");
    // Replace the last code block content with DO_NOT_FIX to simulate partial failure
    let text = fs::read_to_string(&doc)
        .unwrap()
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
    // Successful blocks get fixed, failing block stays unchanged
    assert!(fixed.contains("fixed_code()"));
    assert!(fixed.contains("DO_NOT_FIX"));
}

#[test]
````

````rs
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

#[test]
````

````rs
fn init_ai_plan_does_not_write_without_yes() {
    let temp = TestDir::new();
    let result = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Init {
            options: InitOptions {
                ai_only: true,
                targets: vec![AiTarget::ClaudeCode],
                categories: vec![AgentKitCategory::Instructions],
                ..InitOptions::default()
            },
        },
    });
    assert_eq!(result.exit_code, 0, "{}", result.stderr);
    assert!(result.stdout.contains("Init plan:"));
    assert!(result.stdout.contains("No changes written"));
    assert!(!temp.path().join("CLAUDE.md").exists());
}

#[test]
````

````rs
fn init_generates_selected_ai_agent_kit_and_project_skeleton() {
    let temp = TestDir::new();
    let result = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Init {
            options: InitOptions {
                yes: true,
                targets: vec![AiTarget::ClaudeCode, AiTarget::Opencode],
                categories: vec![AgentKitCategory::Instructions, AgentKitCategory::Commands],
                ..InitOptions::default()
            },
        },
    });
    assert_eq!(result.exit_code, 0, "{}", result.stderr);
    assert!(temp.path().join("mds.config.toml").exists());
    assert!(!temp.path().join("index.md").exists());
    assert!(temp.path().join(".mds/source/overview.md").exists());
    assert!(temp.path().join(".mds/test/overview.md").exists());
    assert!(temp.path().join(".claude/rules/mds.md").exists());
    assert!(temp.path().join(".claude/commands/mds-check.md").exists());
    assert!(temp.path().join(".claude/commands/mds-build.md").exists());
    assert!(temp.path().join(".claude/commands/mds-lint.md").exists());
    assert!(temp.path().join(".opencode/agents/mds-build.md").exists());
    assert!(temp.path().join(".opencode/agents/mds-check.md").exists());
    assert!(!temp.path().join(".claude/skills/mds/SKILL.md").exists());
    let rules = fs::read_to_string(temp.path().join(".claude/rules/mds.md")).unwrap();
    assert!(rules.contains("mds-managed: true"));
    assert!(rules.contains("mds check"));
    let config = fs::read_to_string(temp.path().join("mds.config.toml")).unwrap();
    assert!(config.contains("linter = \"eslint\""));
    assert!(config.contains("fixer = \"prettier --write\""));
    assert!(config.contains("test_runner = \"vitest run\""));
}

#[test]
````

````rs
fn init_writes_selected_quality_tool_config() {
    let temp = TestDir::new();
    let result = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Init {
            options: InitOptions {
                yes: true,
                targets: Vec::new(),
                categories: Vec::new(),
                ts_tools: vec![TypeScriptTool::Biome, TypeScriptTool::Jest],
                py_tools: vec![PythonTool::Ruff, PythonTool::Black, PythonTool::Unittest],
                rs_tools: vec![RustTool::Clippy, RustTool::Nextest],
                ..InitOptions::default()
            },
        },
    });
    assert_eq!(result.exit_code, 0, "{}", result.stderr);
    let config = fs::read_to_string(temp.path().join("mds.config.toml")).unwrap();
    assert!(config.contains("[quality.ts]"));
    assert!(config.contains("linter = \"biome lint\""));
    assert!(config.contains("fixer = \"biome format --write\""));
    assert!(config.contains("test_runner = \"jest\""));
    assert!(config.contains("required = [\"node\", \"biome\", \"jest\"]"));
    assert!(config.contains("[quality.py]"));
    assert!(config.contains("linter = \"ruff check\""));
    assert!(config.contains("fixer = \"black\""));
    assert!(config.contains("test_runner = \"python3 -m unittest\""));
    assert!(config.contains("[quality.rs]"));
    assert!(config.contains("linter = \"cargo clippy\""));
    assert!(config.contains("fixer = false"));
    assert!(config.contains("test_runner = \"cargo nextest run\""));
    assert!(config.contains("optional = [\"clippy-driver\", \"cargo-nextest\"]"));
}

#[test]
````

````rs
fn init_writes_custom_quality_commands() {
    let temp = TestDir::new();
    let result = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Init {
            options: InitOptions {
                yes: true,
                targets: Vec::new(),
                categories: Vec::new(),
                quality_commands: vec![InitQualityCommands {
                    lang: Lang::TypeScript,
                    type_check: Some("npm run typecheck".to_string()),
                    lint: Some("npm run lint".to_string()),
                    test: Some("npm test".to_string()),
                }],
                ..InitOptions::default()
            },
        },
    });
    assert_eq!(result.exit_code, 0, "{}", result.stderr);
    let config = fs::read_to_string(temp.path().join("mds.config.toml")).unwrap();
    assert!(config.contains("type_checker = \"npm run typecheck\""));
    assert!(config.contains("linter = \"npm run lint\""));
    assert!(config.contains("test_runner = \"npm test\""));
}

#[test]
````

````rs
fn init_generates_ai_categories_per_target() {
    let temp = TestDir::new();
    let result = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Init {
            options: InitOptions {
                yes: true,
                targets: vec![AiTarget::ClaudeCode, AiTarget::Opencode],
                categories: Vec::new(),
                target_categories: vec![
                    InitTargetCategories {
                        target: AiTarget::ClaudeCode,
                        categories: vec![AgentKitCategory::Instructions],
                    },
                    InitTargetCategories {
                        target: AiTarget::Opencode,
                        categories: vec![AgentKitCategory::Skills],
                    },
                ],
                ..InitOptions::default()
            },
        },
    });
    assert_eq!(result.exit_code, 0, "{}", result.stderr);
    assert!(temp.path().join(".claude/rules/mds.md").exists());
    assert!(!temp.path().join(".claude/skills/mds/SKILL.md").exists());
    assert!(temp.path().join(".opencode/skills/mds/SKILL.md").exists());
    assert!(!temp.path().join(".opencode/agents/mds-check.md").exists());
}

#[test]
````

````rs
fn init_can_disable_language_quality_tools() {
    let temp = TestDir::new();
    let result = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Init {
            options: InitOptions {
                yes: true,
                targets: Vec::new(),
                categories: Vec::new(),
                ts_tools: Vec::new(),
                py_tools: Vec::new(),
                rs_tools: Vec::new(),
                ..InitOptions::default()
            },
        },
    });
    assert_eq!(result.exit_code, 0, "{}", result.stderr);
    let config = fs::read_to_string(temp.path().join("mds.config.toml")).unwrap();
    assert!(config.contains(
        "[quality.ts]\nlinter = false\nfixer = false\ntest_runner = false\nrequired = []"
    ));
    assert!(config.contains(
        "[quality.py]\nlinter = false\nfixer = false\ntest_runner = false\nrequired = []"
    ));
    assert!(config.contains(
        "[quality.rs]\nlinter = false\nfixer = false\ntest_runner = false\nrequired = []"
    ));
}

#[test]
````

````rs
fn init_setup_plan_uses_selected_quality_tools() {
    let temp = TestDir::new();
    let result = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Init {
            options: InitOptions {
                install_toolchains: true,
                targets: Vec::new(),
                categories: Vec::new(),
                ts_tools: vec![TypeScriptTool::Biome, TypeScriptTool::Jest],
                py_tools: vec![PythonTool::Black, PythonTool::Pytest],
                rs_tools: vec![RustTool::Nextest],
                ..InitOptions::default()
            },
        },
    });
    assert_eq!(result.exit_code, 0, "{}", result.stderr);
    assert!(result.stdout.contains("Setup plan:"));
    assert!(result.stdout.contains("biome --version"));
    assert!(result.stdout.contains("jest --version"));
    assert!(result.stdout.contains("black --version"));
    assert!(result.stdout.contains("pytest --version"));
    assert!(result.stdout.contains("cargo-nextest --version"));
    assert!(!temp.path().join("mds.config.toml").exists());
}

#[test]
````

````rs
fn init_refuses_nonmanaged_overwrite_without_force() {
    let temp = TestDir::new();
    fs::create_dir_all(temp.path().join(".claude/rules")).unwrap();
    fs::write(temp.path().join(".claude/rules/mds.md"), "manual\n").unwrap();
    let result = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Init {
            options: InitOptions {
                ai_only: true,
                yes: true,
                targets: vec![AiTarget::ClaudeCode],
                categories: vec![AgentKitCategory::Instructions],
                ..InitOptions::default()
            },
        },
    });
    assert_eq!(result.exit_code, 1);
    assert!(result.stderr.contains("refusing to overwrite non-managed"));
    assert_eq!(
        fs::read_to_string(temp.path().join(".claude/rules/mds.md")).unwrap(),
        "manual\n"
    );
}

#[test]
````

````rs
fn init_reports_setup_partial_failures() {
    let temp = TestDir::new();
    let result = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::Init {
            options: InitOptions {
                yes: true,
                install_ai_cli: true,
                ..InitOptions::default()
            },
        },
    });
    assert_ne!(result.exit_code, 0);
    assert!(result.stdout.contains("Setup plan:"));
    assert!(result.stderr.contains("setup action"));
    assert!(temp.path().join("mds.config.toml").exists());
}

#[test]
fn new_creates_source_doc_under_fixed_authoring_root() {
    let temp = TestDir::new();
    fs::write(
        temp.path().join("mds.config.toml"),
        "[package]\nenabled = true\nallow_raw_source = false\n",
    )
    .unwrap();

    let result = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::New {
            options: mds_core::NewOptions {
                name: "greet.ts.md".to_string(),
                force: false,
            },
        },
    });
    assert_eq!(result.exit_code, 0, "{}", result.stderr);
    assert!(temp.path().join(".mds/source/greet.ts.md").exists());
    assert!(!temp.path().join("src-md/greet.ts.md").exists());
}

#[test]
fn new_creates_test_doc_under_fixed_test_root() {
    let temp = TestDir::new();
    fs::write(
        temp.path().join("mds.config.toml"),
        "[package]\nenabled = true\nallow_raw_source = false\n",
    )
    .unwrap();

    let result = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::New {
            options: mds_core::NewOptions {
                name: "greet.md".to_string(),
                force: false,
            },
        },
    });
    assert_eq!(result.exit_code, 0, "{}", result.stderr);
    let path = temp.path().join(".mds/test/greet.md");
    assert!(path.exists());
    assert!(fs::read_to_string(path).unwrap().contains("## Covers"));
}

#[test]
````

````rs
fn release_quality_gate_requires_supply_chain_artifacts() {
    let temp = TestDir::new();
    fs::create_dir_all(temp.path().join("dist")).unwrap();
    for file in ["mds", "mds.sig", "mds.spdx.json", "mds.intoto.jsonl"] {
        fs::write(temp.path().join("dist").join(file), "ok\n").unwrap();
    }
    fs::write(
        temp.path().join("dist/mds.sha256"),
        "dc51b8c96c2d745df3bd5590d990230a482fd247123599548e0632fdbf97fc22  mds\n",
    )
    .unwrap();
    fs::write(
        temp.path().join("dist/mds.spdx.json"),
        "{\"spdxVersion\":\"SPDX-2.3\",\"SPDXID\":\"SPDXRef-DOCUMENT\"}\n",
    )
    .unwrap();
    fs::write(
        temp.path().join("dist/mds.intoto.jsonl"),
        "{\"_type\":\"https://in-toto.io/Statement/v1\"}\n",
    )
    .unwrap();
    fs::write(
        temp.path().join("release.mds.toml"),
        "[[artifacts]]\nname = \"native-linux-x64\"\nchannel = \"native\"\npath = \"dist/mds\"\nchecksum = \"dist/mds.sha256\"\nsignature = \"dist/mds.sig\"\nsbom = \"dist/mds.spdx.json\"\nprovenance = \"dist/mds.intoto.jsonl\"\nsmoke = true\n",
    )
    .unwrap();
    let ok = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::ReleaseCheck {
            options: ReleaseQualityOptions {
                manifest: PathBuf::from("release.mds.toml"),
            },
        },
    });
    assert_eq!(ok.exit_code, 0, "{}", ok.stderr);
    assert!(ok.stdout.contains("release quality ok"));

    fs::remove_file(temp.path().join("dist/mds.sig")).unwrap();
    let missing = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::ReleaseCheck {
            options: ReleaseQualityOptions {
                manifest: PathBuf::from("release.mds.toml"),
            },
        },
    });
    assert_eq!(missing.exit_code, 1);
    assert!(missing.stderr.contains("signature"));

    fs::write(temp.path().join("dist/mds.sig"), "ok\n").unwrap();
    fs::write(temp.path().join("dist/mds.sha256"), "bad\n").unwrap();
    let mismatch = execute(CliRequest {
        cwd: temp.path().to_path_buf(),
        package: None,
        verbose: false,
        command: Command::ReleaseCheck {
            options: ReleaseQualityOptions {
                manifest: PathBuf::from("release.mds.toml"),
            },
        },
    });
    assert_eq!(mismatch.exit_code, 1);
    assert!(mismatch.stderr.contains("SHA-256"));
}
````

````rs
struct TestDir {
    path: PathBuf,
}
````

````rs
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
````

````rs
impl Drop for TestDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}
````

````rs
fn write_fixture(root: &Path) {
    let package = root.join("pkg");
    fs::create_dir_all(package.join("src-md/foo")).unwrap();
    fs::create_dir_all(package.join("src-md/pkg")).unwrap();
    fs::create_dir_all(package.join(".mds/source")).unwrap();
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
        package.join(".mds/source/overview.md"),
        "# Overview\n\n## Purpose\n\nFixture package.\n\n## Architecture\n\nFixture architecture.\n\n<!-- mds:begin package-summary -->\n| Name | Version |\n| --- | --- |\n| fixture | 0.1.0 |\n<!-- mds:end package-summary -->\n\n## Exposes\n\n| Kind | Name | Target | Summary |\n| --- | --- | --- | --- |\n\n<!-- mds:begin dependencies -->\n| Name | Version | Summary |\n| --- | --- | --- |\n<!-- mds:end dependencies -->\n\n<!-- mds:begin dev-dependencies -->\n| Name | Version | Summary |\n| --- | --- | --- |\n<!-- mds:end dev-dependencies -->\n\n## Rules\n\n- Fixture rules.\n",
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
````

````rs
fn write_tool(root: &Path, name: &str, script: &str) -> PathBuf {
    let path = root.join("bin").join(name);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(&path, script).unwrap();
    let mut permissions = fs::metadata(&path).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&path, permissions).unwrap();
    path
}
````

````rs
fn impl_doc(
    lang: &str,
    name: &str,
    types: &str,
    source: &str,
    test: &str,
    _uses_row: &str,
) -> String {
    format!(
        "# {name}\n\n## Purpose\n\nFixture.\n\n## Source\n\n```{lang}\n{types}\n```\n\n```{lang}\n{source}\n```\n\n```{lang}\n{test}\n```\n"
    )
}
````
