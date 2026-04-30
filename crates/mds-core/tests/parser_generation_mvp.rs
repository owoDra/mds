use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use mds_core::{
    execute, AgentKitCategory, AiTarget, BuildMode, CliRequest, Command, InitOptions, PythonTool,
    ReleaseQualityOptions, RustTool, TypeScriptTool,
};

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
    assert!(fixed.contains("## Source"));
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
    assert!(temp.path().join("package.md").exists());
    assert!(temp.path().join("src-md/index.md").exists());
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
    _uses_row: &str,
) -> String {
    format!(
        "# {name}\n\n## Purpose\n\nFixture.\n\n## Source\n\n```{lang}\n{types}\n```\n\n```{lang}\n{source}\n```\n\n```{lang}\n{test}\n```\n"
    )
}
