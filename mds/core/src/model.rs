use std::collections::{HashMap};
use std::path::{Path};
use std::path::{PathBuf};
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BuildMode {
    DryRun,
    Write,
}

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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InitQualityCommands {
    pub lang: Lang,
    pub type_check: Option<String>,
    pub lint: Option<String>,
    pub test: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InitTargetCategories {
    pub target: AiTarget,
    pub categories: Vec<AgentKitCategory>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum LabelPreset {
    English,
    Japanese,
}

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
                ("source", "実装"),
                ("cases", "ケース"),
                ("test", "テスト"),
                ("covers", "対象確認"),
                ("imports", "依存"),
                ("exports", "公開"),
                ("expose", "公開"),
                ("exposes", "公開面"),
                ("from", "取得元"),
                ("target", "対象"),
                ("symbols", "識別子"),
                ("via", "経由"),
                ("summary", "概要"),
                ("name", "名前"),
                ("visibility", "公開範囲"),
                ("reference", "参照"),
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NewOptions {
    pub name: String,
    pub force: bool,
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

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
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
}

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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DoctorFormat {
    Text,
    Json,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DocKind {
    Source,
    Test,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DocProfile {
    Overview,
    Spec,
    Impl,
    Test,
}

impl DocKind {
    pub fn key(self) -> &'static str {
        match self {
            Self::Source => "source",
            Self::Test => "test",
        }
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum Lang {
    Other(String),
}

impl Lang {
    pub fn from_path(path: &Path) -> Option<Self> {
        let name = path.file_name()?.to_string_lossy();
        let without_md = name.strip_suffix(".md")?;
        let dot_pos = without_md.rfind('.')?;
        let ext = &without_md[dot_pos + 1..];
        if !ext.is_empty() && ext.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
            Some(Self::Other(ext.to_string()))
        } else {
            None
        }
    }

    pub fn key(&self) -> &str {
        match self {
            Self::Other(ext) => ext.as_str(),
        }
    }

    pub fn header_prefix(&self) -> &str {
        match self {
            Self::Other(ext) => match ext.as_str() {
                "py" | "rb" | "sh" | "bash" | "zsh" | "pl" | "pm" => "#",
                "hs" | "lua" => "--",
                "html" | "xml" => "<!--",
                _ => "//",
            },
        }
    }

    pub fn file_ext(&self) -> &str {
        match self {
            Self::Other(ext) => ext.as_str(),
        }
    }

    pub fn builtins() -> Vec<Lang> {
        Vec::new()
    }
}

#[derive(Debug, Clone)]
pub struct Roots {
    pub markdown: PathBuf,
    pub source: PathBuf,
    pub test: PathBuf,
}

impl Default for Roots {
    fn default() -> Self {
        Self {
            markdown: PathBuf::from("src-md"),
            source: PathBuf::from("src"),
            test: PathBuf::from("tests"),
        }
    }
}

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

impl Default for Config {
    fn default() -> Self {
        Self {
            enabled: false,
            allow_raw_source: false,
            copy_source_assets: true,
            check: CheckConfig::default(),
            mds_version: None,
            roots: Roots::default(),
            adapters: HashMap::new(),
            quality: HashMap::new(),
            excludes: Vec::new(),
            package_sync_hook_enabled: false,
            package_sync_hook: None,
            label_overrides: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CheckConfig {
    pub code_blocks_required: bool,
    pub code_fence_integrity: bool,
    pub duplicate_h2_sections: bool,
    pub markdown_links: bool,
    pub import_with_implementation: bool,
    pub top_level_fence_required: bool,
    pub doc_comments_outside_code: bool,
    pub documented_sections: bool,
    pub documented_exports: bool,
}

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
            documented_sections: true,
            documented_exports: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct QualityConfig {
    pub type_check: Option<String>,
    pub lint: Option<String>,
    pub fix: Option<String>,
    pub test: Option<String>,
    pub required: Vec<String>,
    pub optional: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Package {
    pub root: PathBuf,
    pub config: Config,
    pub package_manager_id: String,
}

#[derive(Debug, Clone)]
pub struct PackageMetadata {
    pub name: String,
    pub version: String,
    pub dependencies: HashMap<String, String>,
    pub dev_dependencies: HashMap<String, String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CodeFenceBlock {
    pub fence_index: usize,
    pub content_start_line: usize,
    pub content_end_line: usize,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct ImplDoc {
    pub doc_kind: DocKind,
    pub lang: Lang,
    pub path: PathBuf,
    pub package_relative_path: PathBuf,
    pub markdown_relative_path: PathBuf,
    pub code: String,
    pub source_code: String,
    pub test_code: String,
    pub source_blocks: Vec<CodeFenceBlock>,
    pub test_blocks: Vec<CodeFenceBlock>,
    pub covers: Vec<String>,
    pub normalized_input: String,
}

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OutputKind {
    Source,
    Test,
}

impl OutputKind {
    pub fn manifest_kind(self) -> &'static str {
        match self {
            Self::Source => "source",
            Self::Test => "test",
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SourceSpan {
    pub markdown_path: PathBuf,
    pub markdown_start_line: usize,
    pub markdown_end_line: usize,
    pub generated_path: PathBuf,
    pub generated_start_line: usize,
    pub generated_end_line: usize,
    pub output_kind: OutputKind,
    pub extension_key: String,
    pub fence_index: usize,
}

impl SourceSpan {
    pub fn contains_markdown_line(&self, line: usize) -> bool {
        (self.markdown_start_line..=self.markdown_end_line).contains(&line)
    }

    pub fn contains_generated_line(&self, line: usize) -> bool {
        (self.generated_start_line..=self.generated_end_line).contains(&line)
    }

    pub fn markdown_line_for_generated(&self, line: usize) -> Option<usize> {
        self.contains_generated_line(line)
            .then_some(self.markdown_start_line + (line - self.generated_start_line))
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct SourceMap {
    pub(crate) spans: Vec<SourceSpan>,
}

impl SourceMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.spans.is_empty()
    }

    pub fn spans(&self) -> &[SourceSpan] {
        &self.spans
    }

    pub fn extend<I>(&mut self, spans: I)
    where
        I: IntoIterator<Item = SourceSpan>,
    {
        self.spans.extend(spans);
    }

    pub fn find_markdown(&self, path: &Path, line: usize) -> Option<&SourceSpan> {
        self.spans
            .iter()
            .find(|span| span.markdown_path.as_path() == path && span.contains_markdown_line(line))
    }

    pub fn find_generated(&self, path: &Path, line: usize) -> Option<&SourceSpan> {
        self.spans
            .iter()
            .find(|span| span.generated_path.as_path() == path && span.contains_generated_line(line))
    }

    pub fn remap_generated_line(&self, path: &Path, line: usize) -> Option<(&Path, usize)> {
        let span = self.find_generated(path, line)?;
        Some((
            span.markdown_path.as_path(),
            span.markdown_line_for_generated(line)?,
        ))
    }
}

#[derive(Debug, Clone)]
pub struct GeneratedFile {
    pub path: PathBuf,
    pub content: String,
    pub kind: GeneratedKind,
    pub source_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum GeneratedKind {
    Output(OutputKind),
    Asset,
    Manifest,
}
