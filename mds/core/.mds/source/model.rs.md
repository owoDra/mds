# src/model.rs

## Purpose

Migrated implementation source for `src/model.rs`.

## Contract

- Preserve the behavior of the pre-migration Rust source.
- This file is synchronized into `.build/rust/mds/core/src/model.rs`.

## Imports

| Kind | From | Target | Symbols | Via | Summary | Code |
| --- | --- | --- | --- | --- | --- | --- |
| rust-use | builtin | std::collections | HashMap | std |  | `use std::collections::HashMap;` |
| rust-use | builtin | std::path | Path, PathBuf | std |  | `use std::path::{Path, PathBuf};` |


## Source


````rs
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BuildMode {
    DryRun,
    Write,
}
````

````rs
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Command {
    Build { mode: BuildMode },
    Lint { fix: bool, check: bool },
    Typecheck,
    Test,
    Doctor { format: DoctorFormat },
    PackageSync { check: bool },
    Init { options: InitOptions },
    New { options: NewOptions },
    Update { version: Option<String> },
}
````

````rs
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
    pub label_preset: LabelPreset,
    pub quality_commands: Vec<InitQualityCommands>,
    pub target_categories: Vec<InitTargetCategories>,
}
````

````rs
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
            label_preset: LabelPreset::English,
            quality_commands: Vec::new(),
            target_categories: Vec::new(),
        }
    }
}
````

````rs
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InitQualityCommands {
    pub lang: Lang,
    pub type_check: Option<String>,
    pub lint: Option<String>,
    pub test: Option<String>,
}
````

````rs
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InitTargetCategories {
    pub target: AiTarget,
    pub categories: Vec<AgentKitCategory>,
}
````

````rs
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum LabelPreset {
    English,
    Japanese,
}
````

````rs
impl LabelPreset {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "en" | "english" => Some(Self::English),
            "ja" | "japanese" | "jp" => Some(Self::Japanese),
            _ => None,
        }
    }

    pub fn key(self) -> &'static str {
        match self {
            Self::English => "en",
            Self::Japanese => "ja",
        }
    }

    pub fn labels(self) -> &'static [(&'static str, &'static str)] {
        match self {
            Self::English => &[],
            Self::Japanese => &[
                ("purpose", "目的"),
                ("contract", "契約"),
                ("types", "型定義"),
                ("source", "実装"),
                ("cases", "ケース"),
                ("test", "テスト"),
                ("expose", "公開"),
                ("exposes", "公開面"),
                ("from", "取得元"),
                ("target", "対象"),
                ("summary", "概要"),
                ("kind", "種別"),
                ("name", "名前"),
                ("version", "バージョン"),
            ],
        }
    }

    pub fn section_label(self, canonical: &str) -> String {
        for (key, label) in self.labels() {
            if *key == canonical {
                return label.to_string();
            }
        }
        let mut chars = canonical.chars();
        match chars.next() {
            Some(first) => {
                let upper: String = first.to_uppercase().collect();
                format!("{upper}{}", chars.as_str())
            }
            None => canonical.to_string(),
        }
    }
}
````

````rs
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NewOptions {
    pub name: String,
    pub force: bool,
}
````

````rs
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TypeScriptTool {
    Eslint,
    Prettier,
    Biome,
    Vitest,
    Jest,
}
````

````rs
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
````

````rs
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PythonTool {
    Ruff,
    Black,
    Pytest,
    Unittest,
}
````

````rs
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
````

````rs
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RustTool {
    Rustfmt,
    Clippy,
    CargoTest,
    Nextest,
}
````

````rs
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
````

````rs
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum AiTarget {
    ClaudeCode,
    CodexCli,
    Opencode,
    GithubCopilotCli,
}
````

````rs
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
````

````rs
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum AgentKitCategory {
    Instructions,
    Skills,
    Commands,
}
````

````rs
impl AgentKitCategory {
    pub fn all() -> &'static [Self] {
        &[Self::Instructions, Self::Skills, Self::Commands]
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "instructions" => Some(Self::Instructions),
            "skills" => Some(Self::Skills),
            "commands" => Some(Self::Commands),
            _ => None,
        }
    }

    pub fn key(self) -> &'static str {
        match self {
            Self::Instructions => "instructions",
            Self::Skills => "skills",
            Self::Commands => "commands",
        }
    }
}
````

````rs
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DoctorFormat {
    Text,
    Json,
}
````

````rs
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DocKind {
    Source,
    Test,
}
````

````rs
impl DocKind {
    pub fn key(self) -> &'static str {
        match self {
            Self::Source => "source",
            Self::Test => "test",
        }
    }
}
````

````rs
#[derive(Debug, Clone)]
pub struct CliRequest {
    pub cwd: PathBuf,
    pub package: Option<PathBuf>,
    pub verbose: bool,
    pub command: Command,
}
````

````rs
#[derive(Debug, Default)]
pub struct CliResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}
````

````rs
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum Lang {
    TypeScript,
    Python,
    Rust,
    Other(String),
}
````

````rs
impl Lang {
    pub fn from_path(path: &Path) -> Option<Self> {
        let name = path.file_name()?.to_string_lossy();
        if name.ends_with(".ts.md") {
            Some(Self::TypeScript)
        } else if name.ends_with(".py.md") {
            Some(Self::Python)
        } else if name.ends_with(".rs.md") {
            Some(Self::Rust)
        } else {
            let name_str = name.as_ref();
            let without_md = name_str.strip_suffix(".md")?;
            let dot_pos = without_md.rfind('.')?;
            let ext = &without_md[dot_pos + 1..];
            if !ext.is_empty() && ext.chars().all(|c| c.is_ascii_alphanumeric()) {
                Some(Self::Other(ext.to_string()))
            } else {
                None
            }
        }
    }

    pub fn key(&self) -> &str {
        match self {
            Self::TypeScript => "ts",
            Self::Python => "py",
            Self::Rust => "rs",
            Self::Other(ext) => ext.as_str(),
        }
    }

    pub fn header_prefix(&self) -> &str {
        match self {
            Self::Python => "#",
            Self::TypeScript | Self::Rust => "//",
            Self::Other(ext) => match ext.as_str() {
                "rb" | "sh" | "bash" | "zsh" | "pl" | "pm" => "#",
                "hs" | "lua" => "--",
                "html" | "xml" => "<!--",
                _ => "//",
            },
        }
    }

    pub fn file_ext(&self) -> &str {
        match self {
            Self::TypeScript => "ts",
            Self::Python => "py",
            Self::Rust => "rs",
            Self::Other(ext) => ext.as_str(),
        }
    }

    pub fn builtins() -> &'static [Lang] {
        &[Self::TypeScript, Self::Python, Self::Rust]
    }
}
````

````rs
#[derive(Debug, Clone)]
pub struct Roots {
    pub markdown: PathBuf,
    pub source: PathBuf,
    pub types: PathBuf,
    pub test: PathBuf,
}
````

````rs
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
````

````rs
#[derive(Debug, Clone)]
pub struct Config {
    pub enabled: bool,
    pub allow_raw_source: bool,
    pub copy_source_assets: bool,
    pub check: CheckConfig,
    pub mds_version: Option<String>,
    pub roots: Roots,
    pub adapters: HashMap<Lang, bool>,
    pub quality: HashMap<Lang, QualityConfig>,
    pub excludes: Vec<String>,
    pub package_sync_hook_enabled: bool,
    pub package_sync_hook: Option<String>,
    pub label_overrides: HashMap<String, String>,
}
````

````rs
impl Default for Config {
    fn default() -> Self {
        Self {
            enabled: false,
            allow_raw_source: false,
            copy_source_assets: true,
            check: CheckConfig::default(),
            mds_version: None,
            roots: Roots::default(),
            adapters: HashMap::from([
                (Lang::TypeScript, true),
                (Lang::Python, true),
                (Lang::Rust, true),
            ]),
            quality: HashMap::from([
                (Lang::TypeScript, QualityConfig::for_lang(&Lang::TypeScript)),
                (Lang::Python, QualityConfig::for_lang(&Lang::Python)),
                (Lang::Rust, QualityConfig::for_lang(&Lang::Rust)),
            ]),
            excludes: Vec::new(),
            package_sync_hook_enabled: false,
            package_sync_hook: None,
            label_overrides: HashMap::new(),
        }
    }
}
````

````rs
#[derive(Debug, Clone)]
pub struct CheckConfig {
    pub code_blocks_required: bool,
    pub code_fence_integrity: bool,
    pub duplicate_h2_sections: bool,
    pub markdown_links: bool,
    pub import_with_implementation: bool,
    pub top_level_fence_required: bool,
    pub doc_comments_outside_code: bool,
}
````

````rs
impl Default for CheckConfig {
    fn default() -> Self {
        Self {
            code_blocks_required: true,
            code_fence_integrity: true,
            duplicate_h2_sections: true,
            markdown_links: true,
            import_with_implementation: true,
            top_level_fence_required: true,
            doc_comments_outside_code: true,
        }
    }
}
````

````rs
#[derive(Debug, Clone)]
pub struct QualityConfig {
    pub type_check: Option<String>,
    pub lint: Option<String>,
    pub fix: Option<String>,
    pub test: Option<String>,
    pub required: Vec<String>,
    pub optional: Vec<String>,
}
````

````rs
impl QualityConfig {
    fn for_lang(_lang: &Lang) -> Self {
        Self {
            type_check: None,
            lint: None,
            fix: None,
            test: None,
            required: Vec::new(),
            optional: Vec::new(),
        }
    }
}
````

````rs
#[derive(Debug, Clone)]
pub struct Package {
    pub root: PathBuf,
    pub config: Config,
    pub package_manager_id: String,
}
````

````rs
#[derive(Debug, Clone)]
pub struct PackageMetadata {
    pub name: String,
    pub version: String,
    pub dependencies: HashMap<String, String>,
    pub dev_dependencies: HashMap<String, String>,
}
````

````rs
#[derive(Debug, Clone)]
pub struct ImplDoc {
    pub doc_kind: DocKind,
    pub lang: Lang,
    pub path: PathBuf,
    pub package_relative_path: PathBuf,
    pub markdown_relative_path: PathBuf,
    pub code: String,
    pub types_code: String,
    pub source_code: String,
    pub test_code: String,
    pub covers: Vec<String>,
    pub normalized_input: String,
}
````

````rs
#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OutputKind {
    Source,
    Types,
    Test,
}
````

````rs
impl OutputKind {
    pub fn manifest_kind(self) -> &'static str {
        match self {
            Self::Source => "source",
            Self::Types => "types",
            Self::Test => "test",
        }
    }
}
````

````rs
#[derive(Debug, Clone)]
pub struct GeneratedFile {
    pub path: PathBuf,
    pub content: String,
    pub kind: GeneratedKind,
    pub source_path: Option<PathBuf>,
}
````

````rs
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum GeneratedKind {
    Output(OutputKind),
    Asset,
    Manifest,
}
````


