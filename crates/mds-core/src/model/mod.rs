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
    Lint { fix: bool, check: bool },
    Test,
    Doctor { format: DoctorFormat },
    PackageSync { check: bool },
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DoctorFormat {
    Text,
    Json,
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
    pub(crate) quality: HashMap<Lang, QualityConfig>,
    pub(crate) excludes: Vec<String>,
    pub(crate) package_sync_hook: Option<String>,
    pub(crate) label_overrides: HashMap<String, String>,
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
            quality: HashMap::from([
                (Lang::TypeScript, QualityConfig::for_lang(Lang::TypeScript)),
                (Lang::Python, QualityConfig::for_lang(Lang::Python)),
                (Lang::Rust, QualityConfig::for_lang(Lang::Rust)),
            ]),
            excludes: Vec::new(),
            package_sync_hook: None,
            label_overrides: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct QualityConfig {
    pub(crate) lint: Option<String>,
    pub(crate) fix: Option<String>,
    pub(crate) test: Option<String>,
    pub(crate) required: Vec<String>,
    pub(crate) optional: Vec<String>,
}

impl QualityConfig {
    fn for_lang(lang: Lang) -> Self {
        match lang {
            Lang::TypeScript => Self {
                lint: Some("eslint".to_string()),
                fix: Some("prettier --write".to_string()),
                test: Some("vitest run".to_string()),
                required: vec![
                    "node".to_string(),
                    "eslint".to_string(),
                    "prettier".to_string(),
                    "vitest".to_string(),
                ],
                optional: Vec::new(),
            },
            Lang::Python => Self {
                lint: Some("ruff check".to_string()),
                fix: Some("ruff format".to_string()),
                test: Some("pytest".to_string()),
                required: vec![
                    "python3".to_string(),
                    "ruff".to_string(),
                    "pytest".to_string(),
                ],
                optional: Vec::new(),
            },
            Lang::Rust => Self {
                lint: Some("cargo clippy".to_string()),
                fix: Some("rustfmt".to_string()),
                test: Some("cargo test".to_string()),
                required: vec![
                    "rustc".to_string(),
                    "cargo".to_string(),
                    "rustfmt".to_string(),
                ],
                optional: vec!["clippy-driver".to_string()],
            },
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
    pub(crate) dependencies: HashMap<String, String>,
    pub(crate) dev_dependencies: HashMap<String, String>,
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
    pub(crate) exposes: Vec<UseExpose>,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub(crate) enum UseExpose {
    Named { name: String, alias: Option<String> },
    Default { local: String },
    Namespace { local: String },
}

impl UseExpose {
    pub(crate) fn render_key(&self) -> String {
        match self {
            Self::Named { name, alias: None } => name.clone(),
            Self::Named {
                name,
                alias: Some(alias),
            } => format!("{name} as {alias}"),
            Self::Default { local } => format!("default: {local}"),
            Self::Namespace { local } => format!("* as {local}"),
        }
    }
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
