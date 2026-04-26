use std::fs;
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
        "# Package\n\n## Package\n\n| Name | Version |\n| --- | --- |\n| fixture | 0.1.0 |\n\n## Dependencies\n\n| Name | Version |\n| --- | --- |\n\n## Dev Dependencies\n\n| Name | Version |\n| --- | --- |\n\n## Rules\n\n- test fixture\n",
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
