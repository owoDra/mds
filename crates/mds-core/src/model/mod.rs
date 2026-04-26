use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BuildMode {
    DryRun,
    Write,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Command {
    Check,
    Build { mode: BuildMode },
}

#[derive(Debug)]
pub struct CliRequest {
    pub cwd: PathBuf,
    pub package: Option<PathBuf>,
    pub verbose: bool,
    pub command: Command,
}

#[derive(Debug, Default)]
pub struct CliResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub(crate) enum Lang {
    TypeScript,
    Python,
    Rust,
}

impl Lang {
    pub(crate) fn from_path(path: &Path) -> Option<Self> {
        let name = path.file_name()?.to_string_lossy();
        if name.ends_with(".ts.md") {
            Some(Self::TypeScript)
        } else if name.ends_with(".py.md") {
            Some(Self::Python)
        } else if name.ends_with(".rs.md") {
            Some(Self::Rust)
        } else {
            None
        }
    }

    pub(crate) fn key(self) -> &'static str {
        match self {
            Self::TypeScript => "ts",
            Self::Python => "py",
            Self::Rust => "rs",
        }
    }

    pub(crate) fn header_prefix(self) -> &'static str {
        match self {
            Self::Python => "#",
            Self::TypeScript | Self::Rust => "//",
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Roots {
    pub(crate) markdown: PathBuf,
    pub(crate) source: PathBuf,
    pub(crate) types: PathBuf,
    pub(crate) test: PathBuf,
}

impl Default for Roots {
    fn default() -> Self {
        Self {
            markdown: PathBuf::from("src-md"),
            source: PathBuf::from("src"),
            types: PathBuf::from("src"),
            test: PathBuf::from("tests"),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Config {
    pub(crate) enabled: bool,
    pub(crate) allow_raw_source: bool,
    pub(crate) roots: Roots,
    pub(crate) adapters: HashMap<Lang, bool>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            enabled: false,
            allow_raw_source: false,
            roots: Roots::default(),
            adapters: HashMap::from([
                (Lang::TypeScript, true),
                (Lang::Python, true),
                (Lang::Rust, true),
            ]),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Package {
    pub(crate) root: PathBuf,
    pub(crate) config: Config,
    pub(crate) metadata_kind: MetadataKind,
}

#[derive(Debug, Clone)]
pub(crate) struct PackageMetadata {
    pub(crate) name: String,
    pub(crate) version: String,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum MetadataKind {
    Node,
    Python,
    Rust,
}

#[derive(Debug, Clone)]
pub(crate) struct ImplDoc {
    pub(crate) lang: Lang,
    pub(crate) path: PathBuf,
    pub(crate) package_relative_path: PathBuf,
    pub(crate) markdown_relative_path: PathBuf,
    pub(crate) uses: HashMap<OutputKind, Vec<UseRow>>,
    pub(crate) code: HashMap<OutputKind, String>,
    pub(crate) normalized_input: String,
}

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum OutputKind {
    Types,
    Source,
    Test,
}

impl OutputKind {
    pub(crate) fn section(self) -> &'static str {
        match self {
            Self::Types => "Types",
            Self::Source => "Source",
            Self::Test => "Test",
        }
    }

    pub(crate) fn manifest_kind(self) -> &'static str {
        match self {
            Self::Types => "types",
            Self::Source => "source",
            Self::Test => "test",
        }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub(crate) struct UseRow {
    pub(crate) from: UseFrom,
    pub(crate) target: String,
    pub(crate) exposes: Vec<String>,
}

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum UseFrom {
    Builtin,
    Package,
    Workspace,
    Internal,
}

impl UseFrom {
    pub(crate) fn parse(value: &str) -> Option<Self> {
        match value {
            "builtin" => Some(Self::Builtin),
            "package" => Some(Self::Package),
            "workspace" => Some(Self::Workspace),
            "internal" => Some(Self::Internal),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct GeneratedFile {
    pub(crate) path: PathBuf,
    pub(crate) content: String,
    pub(crate) kind: GeneratedKind,
    pub(crate) source_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum GeneratedKind {
    Output(OutputKind),
    Manifest,
    RustModule,
}
