use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

static TEMP_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[test]
fn npm_wrapper_invokes_native_cli_and_preserves_exit_code() {
    let native = native_cli();
    let wrapper = workspace_root().join("packages/cli/bin/mds.js");
    let temp = TestDir::new();

    let output = Command::new("node")
        .arg(wrapper)
        .arg("check")
        .env("MDS_NATIVE_BIN", native)
        .current_dir(temp.path())
        .output()
        .expect("node wrapper should run");

    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stderr).contains("no mds enabled packages found"));
}

#[test]
fn python_wrapper_invokes_native_cli_and_preserves_exit_code() {
    let native = native_cli();
    let python_package = workspace_root().join("python/mds_cli");
    let temp = TestDir::new();

    let output = Command::new("python3")
        .arg("-m")
        .arg("mds_cli")
        .arg("check")
        .env("MDS_NATIVE_BIN", native)
        .env("PYTHONPATH", python_package)
        .current_dir(temp.path())
        .output()
        .expect("python wrapper should run");

    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stderr).contains("no mds enabled packages found"));
}

fn native_cli() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_mds"))
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("workspace root")
        .to_path_buf()
}

struct TestDir {
    path: PathBuf,
}

impl TestDir {
    fn new() -> Self {
        let id = TEMP_COUNTER.fetch_add(1, Ordering::SeqCst);
        let path =
            std::env::temp_dir().join(format!("mds-wrapper-test-{}-{id}", std::process::id()));
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
