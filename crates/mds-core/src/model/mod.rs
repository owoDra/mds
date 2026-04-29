use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BuildMode {
    DryRun,
    Write,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Command {
    Check,
    Build { mode: BuildMode },
    Lint { fix: bool, check: bool },
    Test,
    Doctor { format: DoctorFormat },
    PackageSync { check: bool },
    Init { options: InitOptions },
    ReleaseCheck { options: ReleaseQualityOptions },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InitOptions {
    pub ai_only: bool,
    pub yes: bool,
    pub force: bool,
    pub targets: Vec<AiTarget>,
    pub categories: Vec<AgentKitCategory>,
    pub ts_tools: Vec<TypeScriptTool>,
    pub py_tools: Vec<PythonTool>,
    pub rs_tools: Vec<RustTool>,
    pub install_project_deps: bool,
    pub install_toolchains: bool,
    pub install_ai_cli: bool,
}

impl Default for InitOptions {
    fn default() -> Self {
        Self {
            ai_only: false,
            yes: false,
            force: false,
            targets: AiTarget::all().to_vec(),
            categories: AgentKitCategory::all().to_vec(),
            ts_tools: TypeScriptTool::defaults().to_vec(),
            py_tools: PythonTool::defaults().to_vec(),
            rs_tools: RustTool::defaults().to_vec(),
            install_project_deps: false,
            install_toolchains: false,
            install_ai_cli: false,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TypeScriptTool {
    Eslint,
    Prettier,
    Biome,
    Vitest,
    Jest,
}

impl TypeScriptTool {
    pub fn defaults() -> &'static [Self] {
        &[Self::Eslint, Self::Prettier, Self::Vitest]
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "eslint" => Some(Self::Eslint),
            "prettier" => Some(Self::Prettier),
            "biome" => Some(Self::Biome),
            "vitest" => Some(Self::Vitest),
            "jest" => Some(Self::Jest),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PythonTool {
    Ruff,
    Black,
    Pytest,
    Unittest,
}

impl PythonTool {
    pub fn defaults() -> &'static [Self] {
        &[Self::Ruff, Self::Pytest]
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "ruff" => Some(Self::Ruff),
            "black" => Some(Self::Black),
            "pytest" => Some(Self::Pytest),
            "unittest" | "python-unittest" => Some(Self::Unittest),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RustTool {
    Rustfmt,
    Clippy,
    CargoTest,
    Nextest,
}

impl RustTool {
    pub fn defaults() -> &'static [Self] {
        &[Self::Rustfmt, Self::Clippy, Self::CargoTest]
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "rustfmt" => Some(Self::Rustfmt),
            "clippy" | "cargo-clippy" => Some(Self::Clippy),
            "cargo-test" | "test" => Some(Self::CargoTest),
            "nextest" | "cargo-nextest" => Some(Self::Nextest),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum AiTarget {
    ClaudeCode,
    CodexCli,
    Opencode,
    GithubCopilotCli,
}

impl AiTarget {
    pub fn all() -> &'static [Self] {
        &[
            Self::ClaudeCode,
            Self::CodexCli,
            Self::Opencode,
            Self::GithubCopilotCli,
        ]
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "all" => None,
            "claude" | "claude-code" => Some(Self::ClaudeCode),
            "codex" | "codex-cli" => Some(Self::CodexCli),
            "opencode" => Some(Self::Opencode),
            "copilot" | "github-copilot" | "github-copilot-cli" => Some(Self::GithubCopilotCli),
            _ => None,
        }
    }

    pub fn key(self) -> &'static str {
        match self {
            Self::ClaudeCode => "claude-code",
            Self::CodexCli => "codex-cli",
            Self::Opencode => "opencode",
            Self::GithubCopilotCli => "github-copilot-cli",
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum AgentKitCategory {
    Instructions,
    Skills,
    Commands,
    Workflows,
    Docs,
}

impl AgentKitCategory {
    pub fn all() -> &'static [Self] {
        &[
            Self::Instructions,
            Self::Skills,
            Self::Commands,
            Self::Workflows,
            Self::Docs,
        ]
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "instructions" => Some(Self::Instructions),
            "skills" => Some(Self::Skills),
            "commands" => Some(Self::Commands),
            "workflows" => Some(Self::Workflows),
            "docs" => Some(Self::Docs),
            _ => None,
        }
    }

    pub fn key(self) -> &'static str {
        match self {
            Self::Instructions => "instructions",
            Self::Skills => "skills",
            Self::Commands => "commands",
            Self::Workflows => "workflows",
            Self::Docs => "docs",
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ReleaseQualityOptions {
    pub manifest: PathBuf,
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
    pub(crate) package_sync_hook_enabled: bool,
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
            package_sync_hook_enabled: false,
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
                lint: None,
                fix: None,
                test: None,
                required: Vec::new(),
                optional: Vec::new(),
            },
            Lang::Python => Self {
                lint: None,
                fix: None,
                test: None,
                required: Vec::new(),
                optional: Vec::new(),
            },
            Lang::Rust => Self {
                lint: None,
                fix: None,
                test: None,
                required: Vec::new(),
                optional: Vec::new(),
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
