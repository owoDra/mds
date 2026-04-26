use std::collections::{BTreeMap, HashMap, HashSet};
use std::ffi::OsStr;
use std::fs;
use std::path::{Component, Path, PathBuf};

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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Severity {
    Warning,
    Error,
}

#[derive(Debug, Clone)]
struct Diagnostic {
    severity: Severity,
    path: Option<PathBuf>,
    line: Option<usize>,
    message: String,
}

impl Diagnostic {
    fn warning(path: Option<PathBuf>, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warning,
            path,
            line: None,
            message: message.into(),
        }
    }

    fn error(path: Option<PathBuf>, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            path,
            line: None,
            message: message.into(),
        }
    }

    fn at_line(mut self, line: usize) -> Self {
        self.line = Some(line);
        self
    }

    fn render(&self) -> String {
        let level = match self.severity {
            Severity::Warning => "warning",
            Severity::Error => "error",
        };
        match (&self.path, self.line) {
            (Some(path), Some(line)) => {
                format!("{level}: {}:{line}: {}\n", path.display(), self.message)
            }
            (Some(path), None) => format!("{level}: {}: {}\n", path.display(), self.message),
            (None, _) => format!("{level}: {}\n", self.message),
        }
    }
}

#[derive(Debug, Default)]
struct RunState {
    stdout: String,
    diagnostics: Vec<Diagnostic>,
    generated: Vec<PathBuf>,
}

impl RunState {
    fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == Severity::Error)
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
enum Lang {
    TypeScript,
    Python,
    Rust,
}

impl Lang {
    fn from_path(path: &Path) -> Option<Self> {
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

    fn key(self) -> &'static str {
        match self {
            Self::TypeScript => "ts",
            Self::Python => "py",
            Self::Rust => "rs",
        }
    }

    fn header_prefix(self) -> &'static str {
        match self {
            Self::Python => "#",
            Self::TypeScript | Self::Rust => "//",
        }
    }
}

#[derive(Debug, Clone)]
struct Roots {
    markdown: PathBuf,
    source: PathBuf,
    types: PathBuf,
    test: PathBuf,
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
struct Config {
    enabled: bool,
    allow_raw_source: bool,
    roots: Roots,
    adapters: HashMap<Lang, bool>,
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
struct Package {
    root: PathBuf,
    config: Config,
    metadata_kind: MetadataKind,
}

#[derive(Debug, Clone)]
struct PackageMetadata {
    name: String,
    version: String,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum MetadataKind {
    Node,
    Python,
    Rust,
}

#[derive(Debug, Clone)]
struct ImplDoc {
    lang: Lang,
    path: PathBuf,
    package_relative_path: PathBuf,
    markdown_relative_path: PathBuf,
    uses: HashMap<OutputKind, Vec<UseRow>>,
    code: HashMap<OutputKind, String>,
    normalized_input: String,
}

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum OutputKind {
    Types,
    Source,
    Test,
}

impl OutputKind {
    fn section(self) -> &'static str {
        match self {
            Self::Types => "Types",
            Self::Source => "Source",
            Self::Test => "Test",
        }
    }

    fn manifest_kind(self) -> &'static str {
        match self {
            Self::Types => "types",
            Self::Source => "source",
            Self::Test => "test",
        }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct UseRow {
    from: UseFrom,
    target: String,
    exposes: Vec<String>,
}

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum UseFrom {
    Builtin,
    Package,
    Workspace,
    Internal,
}

impl UseFrom {
    fn parse(value: &str) -> Option<Self> {
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
struct GeneratedFile {
    path: PathBuf,
    content: String,
    kind: GeneratedKind,
    source_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum GeneratedKind {
    Output(OutputKind),
    Manifest,
    RustModule,
}

pub fn execute(request: CliRequest) -> CliResult {
    match execute_inner(request) {
        Ok(state) => render_result(state),
        Err(error) => CliResult {
            stdout: String::new(),
            stderr: format!("internal error: {error}\n"),
            exit_code: 3,
        },
    }
}

fn execute_inner(request: CliRequest) -> Result<RunState, String> {
    let mut state = RunState::default();
    let packages = discover_packages(&request.cwd, request.package.as_deref(), &mut state)?;
    if packages.is_empty() {
        state
            .diagnostics
            .push(Diagnostic::error(None, "no mds enabled packages found"));
        return Ok(state);
    }

    for package in packages {
        run_package(&package, request.command, request.verbose, &mut state)?;
    }

    Ok(state)
}

fn render_result(mut state: RunState) -> CliResult {
    if !state.generated.is_empty() {
        state.stdout.push_str("Generated files:\n");
        for path in &state.generated {
            state.stdout.push_str("- ");
            state.stdout.push_str(&path.display().to_string());
            state.stdout.push('\n');
        }
    }

    let stderr = state
        .diagnostics
        .iter()
        .map(Diagnostic::render)
        .collect::<String>();
    let exit_code = if state.has_errors() { 1 } else { 0 };
    CliResult {
        stdout: state.stdout,
        stderr,
        exit_code,
    }
}

fn discover_packages(
    cwd: &Path,
    package: Option<&Path>,
    state: &mut RunState,
) -> Result<Vec<Package>, String> {
    if let Some(package) = package {
        let root = if package.is_absolute() {
            package.to_path_buf()
        } else {
            cwd.join(package)
        };
        return Ok(load_package(&root, state).into_iter().collect());
    }

    let mut packages = Vec::new();
    for path in collect_files(cwd, true)? {
        if path.file_name() == Some(OsStr::new("mds.config.toml")) {
            let Some(root) = path.parent() else {
                continue;
            };
            if let Some(package) = load_package(root, state) {
                packages.push(package);
            }
        }
    }
    packages.sort_by(|left, right| left.root.cmp(&right.root));
    Ok(packages)
}

fn load_package(root: &Path, state: &mut RunState) -> Option<Package> {
    let config_path = root.join("mds.config.toml");
    if !config_path.exists() {
        state.diagnostics.push(Diagnostic::error(
            Some(root.to_path_buf()),
            "mds.config.toml is required for an mds package",
        ));
        return None;
    }

    let mut config = match parse_config(&config_path, state) {
        Some(config) => config,
        None => return None,
    };
    if !config.enabled {
        return None;
    }

    let package_md = root.join("package.md");
    if !package_md.exists() {
        state.diagnostics.push(Diagnostic::error(
            Some(package_md),
            "enabled package requires package.md",
        ));
        return None;
    }

    let metadata_kind = match metadata_kind(root) {
        Some(kind) => kind,
        None => {
            state.diagnostics.push(Diagnostic::error(
                Some(root.to_path_buf()),
                "enabled package requires package.json, pyproject.toml, or Cargo.toml",
            ));
            return None;
        }
    };

    if metadata_kind == MetadataKind::Python && config.roots.source == PathBuf::from("src") {
        config.roots.source = PathBuf::from("src");
    }

    Some(Package {
        root: root.to_path_buf(),
        config,
        metadata_kind,
    })
}

fn parse_config(path: &Path, state: &mut RunState) -> Option<Config> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) => {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                format!("failed to read config: {error}"),
            ));
            return None;
        }
    };
    let mut config = Config::default();
    let mut section = String::new();
    for (idx, raw_line) in text.lines().enumerate() {
        let line = raw_line
            .split_once('#')
            .map(|(line, _)| line)
            .unwrap_or(raw_line)
            .trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            section = line.trim_matches(['[', ']']).to_string();
            if !matches!(
                section.as_str(),
                "package"
                    | "roots"
                    | "adapters.ts"
                    | "adapters.typescript"
                    | "adapters.py"
                    | "adapters.python"
                    | "adapters.rs"
                    | "adapters.rust"
            ) {
                state.diagnostics.push(Diagnostic::warning(
                    Some(path.to_path_buf()),
                    format!("MVP ignores unsupported config table `{section}`"),
                ));
            }
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            state.diagnostics.push(
                Diagnostic::error(Some(path.to_path_buf()), "invalid config assignment")
                    .at_line(idx + 1),
            );
            continue;
        };
        let key = key.trim();
        let value = value.trim();
        match section.as_str() {
            "package" => match key {
                "enabled" => config.enabled = parse_bool(value, path, idx + 1, state),
                "allow_raw_source" => {
                    config.allow_raw_source = parse_bool(value, path, idx + 1, state)
                }
                _ => state.diagnostics.push(Diagnostic::warning(
                    Some(path.to_path_buf()),
                    format!("MVP ignores unsupported package config `{key}`"),
                )),
            },
            "roots" => match key {
                "markdown" => config.roots.markdown = PathBuf::from(parse_string(value)),
                "source" => config.roots.source = PathBuf::from(parse_string(value)),
                "types" => config.roots.types = PathBuf::from(parse_string(value)),
                "test" => config.roots.test = PathBuf::from(parse_string(value)),
                _ => state.diagnostics.push(Diagnostic::warning(
                    Some(path.to_path_buf()),
                    format!("MVP ignores unsupported roots config `{key}`"),
                )),
            },
            "adapters.ts"
            | "adapters.typescript"
            | "adapters.py"
            | "adapters.python"
            | "adapters.rs"
            | "adapters.rust" => {
                if key != "enabled" {
                    state.diagnostics.push(Diagnostic::warning(
                        Some(path.to_path_buf()),
                        format!("MVP ignores unsupported adapter config `{section}.{key}`"),
                    ));
                    continue;
                }
                let lang = match section.as_str() {
                    "adapters.ts" | "adapters.typescript" => Lang::TypeScript,
                    "adapters.py" | "adapters.python" => Lang::Python,
                    _ => Lang::Rust,
                };
                config
                    .adapters
                    .insert(lang, parse_bool(value, path, idx + 1, state));
            }
            _ => state.diagnostics.push(Diagnostic::warning(
                Some(path.to_path_buf()),
                format!("MVP ignores unsupported config key `{key}`"),
            )),
        }
    }

    Some(config)
}

fn parse_bool(value: &str, path: &Path, line: usize, state: &mut RunState) -> bool {
    match value {
        "true" => true,
        "false" => false,
        _ => {
            state.diagnostics.push(
                Diagnostic::error(
                    Some(path.to_path_buf()),
                    "boolean config value must be true or false",
                )
                .at_line(line),
            );
            false
        }
    }
}

fn parse_string(value: &str) -> String {
    value.trim().trim_matches('"').to_string()
}

fn metadata_kind(root: &Path) -> Option<MetadataKind> {
    if root.join("package.json").exists() {
        Some(MetadataKind::Node)
    } else if root.join("pyproject.toml").exists() {
        Some(MetadataKind::Python)
    } else if root.join("Cargo.toml").exists() {
        Some(MetadataKind::Rust)
    } else {
        None
    }
}

fn run_package(
    package: &Package,
    command: Command,
    verbose: bool,
    state: &mut RunState,
) -> Result<(), String> {
    if verbose {
        state
            .stdout
            .push_str(&format!("Checking package {}\n", package.root.display()));
    }
    validate_manifest(package, state);
    validate_package_md(package, state);
    validate_index_docs(package, state);

    let docs = load_implementation_docs(package, state)?;
    let generated = plan_generation(package, &docs, state);

    match command {
        Command::Check => {
            if !state.has_errors() {
                state.stdout.push_str(&format!(
                    "check ok: {} ({} implementation files)\n",
                    package.root.display(),
                    docs.len()
                ));
            }
        }
        Command::Build { mode } => {
            if state.has_errors() {
                return Ok(());
            }
            match mode {
                BuildMode::DryRun => render_dry_run(&generated, state),
                BuildMode::Write => write_generated(&generated, state)?,
            }
        }
    }
    Ok(())
}

fn validate_manifest(package: &Package, state: &mut RunState) {
    let path = package.root.join(".mds/manifest.toml");
    if !path.exists() {
        return;
    }
    let text = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(error) => {
            state.diagnostics.push(Diagnostic::error(
                Some(path),
                format!("failed to read manifest: {error}"),
            ));
            return;
        }
    };
    if !text.contains("[[sources]]") {
        state.diagnostics.push(Diagnostic::error(
            Some(path),
            "manifest schema requires [[sources]] entries",
        ));
    }
}

fn validate_package_md(package: &Package, state: &mut RunState) {
    let path = package.root.join("package.md");
    let text = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(error) => {
            state.diagnostics.push(Diagnostic::error(
                Some(path),
                format!("failed to read package.md: {error}"),
            ));
            return;
        }
    };
    let sections = sections(&text);
    for required in ["Package", "Dependencies", "Dev Dependencies", "Rules"] {
        if !sections.contains_key(required) {
            state.diagnostics.push(Diagnostic::error(
                Some(path.clone()),
                format!("package.md requires ## {required}"),
            ));
        }
    }

    if let Some(package_section) = sections.get("Package") {
        if let Some(rows) = parse_table(package_section, &["Name", "Version"], &path, state) {
            if rows.is_empty() {
                state.diagnostics.push(Diagnostic::error(
                    Some(path.clone()),
                    "package.md Package table must contain at least one row",
                ));
            }
            if let Some(metadata) = read_package_metadata(package, state) {
                if let Some(row) = rows.first() {
                    let doc_name = row.get("name").map(String::as_str).unwrap_or_default();
                    let doc_version = row.get("version").map(String::as_str).unwrap_or_default();
                    if doc_name != metadata.name {
                        state.diagnostics.push(Diagnostic::error(
                            Some(path.clone()),
                            format!(
                                "package.md Package.Name `{doc_name}` does not match metadata `{}`",
                                metadata.name
                            ),
                        ));
                    }
                    if doc_version != metadata.version {
                        state.diagnostics.push(Diagnostic::error(
                            Some(path.clone()),
                            format!(
                                "package.md Package.Version `{doc_version}` does not match metadata `{}`",
                                metadata.version
                            ),
                        ));
                    }
                }
            }
        } else {
            state.diagnostics.push(Diagnostic::error(
                Some(path),
                "package.md Package section requires Name and Version table columns",
            ));
        }
    }
}

fn read_package_metadata(package: &Package, state: &mut RunState) -> Option<PackageMetadata> {
    match package.metadata_kind {
        MetadataKind::Node => read_node_metadata(&package.root.join("package.json"), state),
        MetadataKind::Python => {
            read_toml_metadata(&package.root.join("pyproject.toml"), &["project"], state)
        }
        MetadataKind::Rust => {
            read_toml_metadata(&package.root.join("Cargo.toml"), &["package"], state)
        }
    }
}

fn read_node_metadata(path: &Path, state: &mut RunState) -> Option<PackageMetadata> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) => {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                format!("failed to read package metadata: {error}"),
            ));
            return None;
        }
    };
    let name = json_string_field(&text, "name");
    let version = json_string_field(&text, "version");
    match (name, version) {
        (Some(name), Some(version)) => Some(PackageMetadata { name, version }),
        _ => {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                "package metadata requires name and version",
            ));
            None
        }
    }
}

fn json_string_field(text: &str, key: &str) -> Option<String> {
    let pattern = format!("\"{key}\"");
    let after_key = text.split_once(&pattern)?.1;
    let after_colon = after_key.split_once(':')?.1.trim_start();
    let value = after_colon.strip_prefix('"')?;
    let end = value.find('"')?;
    Some(value[..end].to_string())
}

fn read_toml_metadata(
    path: &Path,
    table_path: &[&str],
    state: &mut RunState,
) -> Option<PackageMetadata> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) => {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                format!("failed to read package metadata: {error}"),
            ));
            return None;
        }
    };
    let section_name = table_path.join(".");
    let fields = simple_toml_section(&text, &section_name);
    let name = fields.get("name").cloned();
    let version = fields.get("version").cloned();
    match (name, version) {
        (Some(name), Some(version)) => Some(PackageMetadata { name, version }),
        _ => {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                format!("package metadata requires [{section_name}] name and version"),
            ));
            None
        }
    }
}

fn simple_toml_section(text: &str, section_name: &str) -> HashMap<String, String> {
    let mut current = String::new();
    let mut fields = HashMap::new();
    for raw_line in text.lines() {
        let line = raw_line
            .split_once('#')
            .map(|(line, _)| line)
            .unwrap_or(raw_line)
            .trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            current = line.trim_matches(['[', ']']).to_string();
            continue;
        }
        if current != section_name {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            fields.insert(key.trim().to_string(), parse_string(value.trim()));
        }
    }
    fields
}

fn validate_index_docs(package: &Package, state: &mut RunState) {
    let markdown_root = package.root.join(&package.config.roots.markdown);
    if !markdown_root.exists() {
        return;
    }
    let Ok(files) = collect_files(&markdown_root, false) else {
        return;
    };
    for path in files
        .into_iter()
        .filter(|path| path.file_name() == Some(OsStr::new("index.md")))
    {
        let text = match fs::read_to_string(&path) {
            Ok(text) => text,
            Err(error) => {
                state.diagnostics.push(Diagnostic::error(
                    Some(path),
                    format!("failed to read index.md: {error}"),
                ));
                continue;
            }
        };
        let sections = sections(&text);
        for required in ["Purpose", "Architecture", "Exposes", "Rules"] {
            if !sections.contains_key(required) {
                state.diagnostics.push(Diagnostic::error(
                    Some(path.clone()),
                    format!("index.md requires ## {required}"),
                ));
            }
        }
        if let Some(exposes_section) = sections.get("Exposes") {
            let Some(rows) = parse_table(
                exposes_section,
                &["Kind", "Name", "Target", "Summary"],
                &path,
                state,
            ) else {
                state.diagnostics.push(Diagnostic::error(
                    Some(path.clone()),
                    "index.md Exposes section requires Kind, Name, Target, and Summary table columns",
                ));
                continue;
            };
            validate_expose_rows(&rows, &path, state);
        }
    }
}

fn validate_expose_rows(rows: &[HashMap<String, String>], path: &Path, state: &mut RunState) {
    let mut seen = HashSet::new();
    for row in rows {
        let kind = row.get("kind").map(String::as_str).unwrap_or_default();
        let name = row.get("name").map(String::as_str).unwrap_or_default();
        if !matches!(kind, "type" | "value" | "function" | "class" | "module") {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                format!(
                    "Expose.Kind must be one of type, value, function, class, module: `{kind}`"
                ),
            ));
        }
        if name.is_empty() {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                "Expose.Name is required",
            ));
        }
        if !seen.insert((kind.to_string(), name.to_string())) {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                "duplicate Expose row with the same Kind and Name",
            ));
        }
    }
}

fn load_implementation_docs(
    package: &Package,
    state: &mut RunState,
) -> Result<Vec<ImplDoc>, String> {
    let markdown_root = package.root.join(&package.config.roots.markdown);
    if !markdown_root.exists() {
        state.diagnostics.push(Diagnostic::error(
            Some(markdown_root),
            "markdown root does not exist",
        ));
        return Ok(Vec::new());
    }

    let mut docs = Vec::new();
    for path in collect_files(&markdown_root, false)? {
        let Some(lang) = Lang::from_path(&path) else {
            continue;
        };
        if !package.config.adapters.get(&lang).copied().unwrap_or(true) {
            continue;
        }
        if let Some(doc) = parse_impl_doc(package, lang, &path, state) {
            docs.push(doc);
        }
    }
    docs.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(docs)
}

fn parse_impl_doc(
    package: &Package,
    lang: Lang,
    path: &Path,
    state: &mut RunState,
) -> Option<ImplDoc> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) => {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                format!("failed to read implementation md: {error}"),
            ));
            return None;
        }
    };
    for (idx, line) in text.lines().enumerate() {
        if line.starts_with("#####") {
            state.diagnostics.push(
                Diagnostic::error(
                    Some(path.to_path_buf()),
                    "implementation md only allows H3-H4 helper headings",
                )
                .at_line(idx + 1),
            );
        }
    }

    let sections = sections(&text);
    for required in ["Purpose", "Contract", "Types", "Source", "Cases", "Test"] {
        if !sections.contains_key(required) {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                format!("implementation md requires ## {required}"),
            ));
        }
    }

    let mut uses = HashMap::new();
    let mut code = HashMap::new();
    for kind in [OutputKind::Types, OutputKind::Source, OutputKind::Test] {
        if let Some(section) = sections.get(kind.section()) {
            uses.insert(kind, parse_uses(section, path, state));
            let joined = code_blocks(section, path, state);
            if joined.trim().is_empty() {
                state.diagnostics.push(Diagnostic::error(
                    Some(path.to_path_buf()),
                    format!(
                        "{} section requires at least one code block",
                        kind.section()
                    ),
                ));
            }
            code.insert(kind, joined);
        }
    }

    let package_relative_path = match path.strip_prefix(&package.root) {
        Ok(path) => path.to_path_buf(),
        Err(_) => path.to_path_buf(),
    };
    let markdown_relative_path =
        match path.strip_prefix(package.root.join(&package.config.roots.markdown)) {
            Ok(path) => path.to_path_buf(),
            Err(_) => path.to_path_buf(),
        };
    let normalized_input = normalized_input(path, &text);

    Some(ImplDoc {
        lang,
        path: path.to_path_buf(),
        package_relative_path,
        markdown_relative_path,
        uses,
        code,
        normalized_input,
    })
}

fn sections(text: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();
    let mut current: Option<String> = None;
    let mut body = String::new();
    for line in text.lines() {
        if let Some(title) = line.strip_prefix("## ") {
            if let Some(name) = current.replace(title.trim().to_string()) {
                result.insert(name, body.trim_matches('\n').to_string());
                body.clear();
            }
        } else if current.is_some() {
            body.push_str(line);
            body.push('\n');
        }
    }
    if let Some(name) = current {
        result.insert(name, body.trim_matches('\n').to_string());
    }
    result
}

fn parse_uses(section: &str, path: &Path, state: &mut RunState) -> Vec<UseRow> {
    let Some(rows) = parse_table(
        section,
        &["From", "Target", "Expose", "Summary"],
        path,
        state,
    ) else {
        state.diagnostics.push(Diagnostic::error(
            Some(path.to_path_buf()),
            "Uses table requires From, Target, Expose, and Summary columns",
        ));
        return Vec::new();
    };
    let mut seen = HashSet::new();
    let mut uses = Vec::new();
    for row in rows {
        let from_text = row
            .get("from")
            .map(String::as_str)
            .unwrap_or_default()
            .trim();
        let Some(from) = UseFrom::parse(from_text) else {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                format!(
                    "Uses.From must be one of builtin, package, workspace, internal: `{from_text}`"
                ),
            ));
            continue;
        };
        let target = row
            .get("target")
            .map(String::as_str)
            .unwrap_or_default()
            .trim()
            .to_string();
        validate_target(from, &target, path, state);
        let exposes = row
            .get("expose")
            .map(String::as_str)
            .unwrap_or_default()
            .split(',')
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();
        let key = (from, target.clone(), exposes.join(","));
        if !seen.insert(key) {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                "duplicate Uses row with the same From, Target, and Expose",
            ));
        }
        uses.push(UseRow {
            from,
            target,
            exposes,
        });
    }
    uses
}

fn parse_table(
    section: &str,
    required: &[&str],
    path: &Path,
    state: &mut RunState,
) -> Option<Vec<HashMap<String, String>>> {
    let lines = section.lines().collect::<Vec<_>>();
    for idx in 0..lines.len().saturating_sub(1) {
        if !lines[idx].trim_start().starts_with('|') || !lines[idx + 1].contains("---") {
            continue;
        }
        let headers = split_table_row(lines[idx]);
        let canonical = headers
            .iter()
            .map(|header| header.trim().to_ascii_lowercase())
            .collect::<Vec<_>>();
        let required_canonical = required
            .iter()
            .map(|header| header.to_ascii_lowercase())
            .collect::<Vec<_>>();
        if !required_canonical
            .iter()
            .all(|required| canonical.contains(required))
        {
            continue;
        }
        for header in &canonical {
            if !required_canonical.contains(header) {
                state.diagnostics.push(Diagnostic::warning(
                    Some(path.to_path_buf()),
                    format!("ignoring unsupported table column `{header}`"),
                ));
            }
        }

        let mut rows = Vec::new();
        for row_line in lines.iter().skip(idx + 2) {
            if !row_line.trim_start().starts_with('|') {
                break;
            }
            let cells = split_table_row(row_line);
            let mut row = HashMap::new();
            for (index, header) in canonical.iter().enumerate() {
                if required_canonical.contains(header) {
                    row.insert(
                        header.clone(),
                        cells
                            .get(index)
                            .map(|cell| cell.trim())
                            .unwrap_or_default()
                            .to_string(),
                    );
                }
            }
            rows.push(row);
        }
        return Some(rows);
    }
    None
}

fn split_table_row(line: &str) -> Vec<String> {
    line.trim()
        .trim_matches('|')
        .split('|')
        .map(|cell| cell.trim().to_string())
        .collect()
}

fn validate_target(from: UseFrom, target: &str, path: &Path, state: &mut RunState) {
    if target.is_empty() {
        state.diagnostics.push(Diagnostic::error(
            Some(path.to_path_buf()),
            "Uses.Target is required",
        ));
        return;
    }
    if target.contains(".md") || target.contains('\\') {
        state.diagnostics.push(Diagnostic::error(
            Some(path.to_path_buf()),
            "Uses.Target must not contain .md or backslash separators",
        ));
    }
    if from == UseFrom::Internal {
        let invalid = target.starts_with("./")
            || target.starts_with("../")
            || target.starts_with('/')
            || target.ends_with('/')
            || Path::new(target)
                .components()
                .any(|component| matches!(component, Component::ParentDir | Component::RootDir));
        if invalid
            || target
                .rsplit('/')
                .next()
                .is_some_and(|leaf| leaf.contains('.'))
        {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                "internal Uses.Target must be a markdown_root relative module path without ./, ../, extension, trailing slash, or absolute path",
            ));
        }
    }
}

fn code_blocks(section: &str, path: &Path, state: &mut RunState) -> String {
    let mut in_block = false;
    let mut current = String::new();
    let mut blocks = Vec::new();
    for (idx, line) in section.lines().enumerate() {
        if line.trim_start().starts_with("```") {
            if in_block {
                blocks.push(
                    current
                        .trim_end_matches(|ch| ch == '\r' || ch == '\n')
                        .to_string(),
                );
                current.clear();
                in_block = false;
            } else {
                in_block = true;
            }
            continue;
        }
        if in_block {
            let trimmed = line.trim_start();
            if trimmed.starts_with("import ")
                || trimmed.starts_with("from ")
                || trimmed.starts_with("use ")
                || trimmed.starts_with("require(")
                || trimmed.starts_with("const ") && trimmed.contains("require(")
            {
                state.diagnostics.push(
                    Diagnostic::error(
                        Some(path.to_path_buf()),
                        "code blocks must not contain import/use/require; use the Uses table",
                    )
                    .at_line(idx + 1),
                );
            }
            current.push_str(line);
            current.push('\n');
        }
    }
    blocks.join("\n\n") + if blocks.is_empty() { "" } else { "\n" }
}

fn normalized_input(path: &Path, text: &str) -> String {
    let mut normalized = path.display().to_string();
    normalized.push('\n');
    normalized.push_str(text.replace("\r\n", "\n").trim_end());
    normalized.push('\n');
    normalized
}

fn plan_generation(
    package: &Package,
    docs: &[ImplDoc],
    state: &mut RunState,
) -> Vec<GeneratedFile> {
    let mut generated = Vec::new();
    for doc in docs {
        let source_hash = sha256(&doc.normalized_input);
        for kind in [OutputKind::Types, OutputKind::Source, OutputKind::Test] {
            if let Some(file) = plan_output(package, doc, kind, &source_hash, state) {
                generated.push(file);
            }
        }
    }
    if docs.iter().any(|doc| doc.lang == Lang::Rust) {
        if let Some(file) = plan_rust_modules(package, docs, state) {
            generated.push(file);
        }
    }
    generated.push(plan_manifest(package, docs, &generated));
    generated
}

fn plan_output(
    package: &Package,
    doc: &ImplDoc,
    kind: OutputKind,
    source_hash: &str,
    state: &mut RunState,
) -> Option<GeneratedFile> {
    let relative = output_relative_path(doc, kind);
    let root = match kind {
        OutputKind::Types => &package.config.roots.types,
        OutputKind::Source => &package.config.roots.source,
        OutputKind::Test => &package.config.roots.test,
    };
    let path = package.root.join(root).join(relative);
    if !path_within(&package.root, &path) {
        state.diagnostics.push(Diagnostic::error(
            Some(path),
            "output path must stay inside package root",
        ));
        return None;
    }
    if path.exists() && !is_mds_managed_file(&path) {
        state.diagnostics.push(Diagnostic::error(
            Some(path),
            "refusing to overwrite file without mds generated header",
        ));
        return None;
    }

    let imports = imports_for(package, doc, kind, &path);
    let code = doc.code.get(&kind).cloned().unwrap_or_default();
    let header = format!(
        "{} Generated by mds. Do not edit. Source: {}. Source-Hash: {}.\n",
        doc.lang.header_prefix(),
        doc.package_relative_path.display(),
        source_hash
    );
    let content = if imports.is_empty() {
        format!("{header}\n{code}")
    } else {
        format!("{header}\n{imports}\n{code}")
    };
    Some(GeneratedFile {
        path,
        content,
        kind: GeneratedKind::Output(kind),
        source_path: Some(doc.package_relative_path.clone()),
    })
}

fn output_relative_path(doc: &ImplDoc, kind: OutputKind) -> PathBuf {
    let rel = strip_md_extension(&doc.markdown_relative_path);
    match (doc.lang, kind) {
        (Lang::TypeScript, OutputKind::Types) => with_suffix(&rel, ".types.ts"),
        (Lang::TypeScript, OutputKind::Test) => with_suffix(&rel, ".test.ts"),
        (Lang::TypeScript, OutputKind::Source) => rel,
        (Lang::Python, OutputKind::Types) => with_suffix(&rel, ".pyi"),
        (Lang::Python, OutputKind::Test) => prefixed_file(&rel, "test_", ".py"),
        (Lang::Python, OutputKind::Source) => rel,
        (Lang::Rust, OutputKind::Types) => with_suffix(&rel, "_types.rs"),
        (Lang::Rust, OutputKind::Test) => PathBuf::from(flattened_test_name(&rel)),
        (Lang::Rust, OutputKind::Source) => rel,
    }
}

fn strip_md_extension(path: &Path) -> PathBuf {
    let name = path.file_name().unwrap_or_default().to_string_lossy();
    let stripped = name.strip_suffix(".md").unwrap_or(&name);
    path.with_file_name(stripped)
}

fn with_suffix(path: &Path, suffix: &str) -> PathBuf {
    let stem = path.file_stem().unwrap_or_default().to_string_lossy();
    path.with_file_name(format!("{stem}{suffix}"))
}

fn prefixed_file(path: &Path, prefix: &str, suffix: &str) -> PathBuf {
    let stem = path.file_stem().unwrap_or_default().to_string_lossy();
    path.with_file_name(format!("{prefix}{stem}{suffix}"))
}

fn flattened_test_name(path: &Path) -> String {
    let mut parts = path
        .with_extension("")
        .components()
        .filter_map(|component| match component {
            Component::Normal(value) => Some(value.to_string_lossy().to_string()),
            _ => None,
        })
        .collect::<Vec<_>>();
    if parts.is_empty() {
        parts.push("generated".to_string());
    }
    format!("{}_test.rs", parts.join("_"))
}

fn imports_for(package: &Package, doc: &ImplDoc, kind: OutputKind, output_path: &Path) -> String {
    let uses = doc.uses.get(&kind).cloned().unwrap_or_default();
    let mut grouped: Vec<UseRow> = Vec::new();
    for from in [
        UseFrom::Builtin,
        UseFrom::Package,
        UseFrom::Workspace,
        UseFrom::Internal,
    ] {
        let mut named_targets: Vec<UseRow> = Vec::new();
        for row in uses.iter().filter(|row| row.from == from) {
            if row.exposes.is_empty() {
                grouped.push(row.clone());
                continue;
            }
            if let Some(existing) = named_targets
                .iter_mut()
                .find(|existing| existing.target == row.target)
            {
                for expose in &row.exposes {
                    if !existing.exposes.contains(expose) {
                        existing.exposes.push(expose.clone());
                    }
                }
            } else {
                named_targets.push(row.clone());
            }
        }
        grouped.extend(named_targets);
    }

    grouped
        .iter()
        .map(|row| match doc.lang {
            Lang::TypeScript => ts_import(package, row, kind, output_path),
            Lang::Python => py_import(row),
            Lang::Rust => rs_import(row),
        })
        .collect::<Vec<_>>()
        .join("\n")
        + if grouped.is_empty() { "" } else { "\n" }
}

fn ts_import(package: &Package, row: &UseRow, kind: OutputKind, output_path: &Path) -> String {
    let target = if row.from == UseFrom::Internal {
        let target_path = package
            .root
            .join(&package.config.roots.source)
            .join(format!("{}.ts", row.target));
        relative_module(output_path.parent().unwrap_or(&package.root), &target_path)
    } else {
        row.target.clone()
    };
    if row.exposes.is_empty() {
        format!("import \"{target}\";")
    } else if kind == OutputKind::Types {
        format!(
            "import type {{ {} }} from \"{target}\";",
            row.exposes.join(", ")
        )
    } else {
        format!("import {{ {} }} from \"{target}\";", row.exposes.join(", "))
    }
}

fn py_import(row: &UseRow) -> String {
    let target = if row.from == UseFrom::Internal {
        row.target.replace('/', ".")
    } else {
        row.target.clone()
    };
    if row.exposes.is_empty() {
        format!("import {target}")
    } else {
        format!("from {target} import {}", row.exposes.join(", "))
    }
}

fn rs_import(row: &UseRow) -> String {
    let target = if row.from == UseFrom::Internal {
        format!("crate::{}", row.target.replace('/', "::"))
    } else {
        row.target.replace('/', "::")
    };
    if row.exposes.is_empty() {
        format!("use {target};")
    } else {
        format!("use {target}::{{{}}};", row.exposes.join(", "))
    }
}

fn relative_module(from_dir: &Path, to_file: &Path) -> String {
    let to_without_extension = to_file.with_extension("");
    let from_components = from_dir.components().collect::<Vec<_>>();
    let to_components = to_without_extension.components().collect::<Vec<_>>();
    let mut common = 0;
    while common < from_components.len()
        && common < to_components.len()
        && from_components[common] == to_components[common]
    {
        common += 1;
    }
    let mut parts = Vec::new();
    for _ in common..from_components.len() {
        parts.push("..".to_string());
    }
    for component in &to_components[common..] {
        if let Component::Normal(value) = component {
            parts.push(value.to_string_lossy().to_string());
        }
    }
    if parts.is_empty() {
        ".".to_string()
    } else if parts[0] == ".." {
        parts.join("/")
    } else {
        format!("./{}", parts.join("/"))
    }
}

fn plan_rust_modules(
    package: &Package,
    docs: &[ImplDoc],
    state: &mut RunState,
) -> Option<GeneratedFile> {
    let path = package
        .root
        .join(&package.config.roots.source)
        .join("lib.rs");
    let mut modules = Vec::new();
    for doc in docs.iter().filter(|doc| doc.lang == Lang::Rust) {
        let source = output_relative_path(doc, OutputKind::Source);
        let types = output_relative_path(doc, OutputKind::Types);
        modules.push(module_path(&source));
        modules.push(module_path(&types));
    }
    modules.sort();
    modules.dedup();
    let block = rust_module_block(&modules);
    let content = if path.exists() {
        let old = match fs::read_to_string(&path) {
            Ok(old) => old,
            Err(error) => {
                state.diagnostics.push(Diagnostic::error(
                    Some(path),
                    format!("failed to read Rust module file: {error}"),
                ));
                return None;
            }
        };
        replace_or_append_module_block(&old, &block, &path, state)?
    } else {
        format!(
            "// Generated by mds. Do not edit. Source: src-md. Source-Hash: {}.\n\n{}",
            sha256(&block),
            block
        )
    };
    Some(GeneratedFile {
        path,
        content,
        kind: GeneratedKind::RustModule,
        source_path: None,
    })
}

fn module_path(path: &Path) -> Vec<String> {
    path.with_extension("")
        .components()
        .filter_map(|component| match component {
            Component::Normal(value) => Some(value.to_string_lossy().to_string()),
            _ => None,
        })
        .collect()
}

fn rust_module_block(modules: &[Vec<String>]) -> String {
    let mut tree: BTreeMap<String, Vec<Vec<String>>> = BTreeMap::new();
    for module in modules {
        if let Some((head, tail)) = module.split_first() {
            tree.entry(head.clone()).or_default().push(tail.to_vec());
        }
    }
    let mut out = String::from("// mds:begin generated modules\n");
    for (name, children) in tree {
        if children.iter().all(Vec::is_empty) {
            out.push_str(&format!("pub mod {name};\n"));
        } else {
            out.push_str(&format!("pub mod {name} {{\n"));
            for child in children.into_iter().filter(|child| !child.is_empty()) {
                write_nested_module(&mut out, &child, 1);
            }
            out.push_str("}\n");
        }
    }
    out.push_str("// mds:end generated modules\n");
    out
}

fn write_nested_module(out: &mut String, module: &[String], depth: usize) {
    let indent = "    ".repeat(depth);
    if module.len() == 1 {
        out.push_str(&format!("{indent}pub mod {};\n", module[0]));
    } else {
        out.push_str(&format!("{indent}pub mod {} {{\n", module[0]));
        write_nested_module(out, &module[1..], depth + 1);
        out.push_str(&format!("{indent}}}\n"));
    }
}

fn replace_or_append_module_block(
    old: &str,
    block: &str,
    path: &Path,
    state: &mut RunState,
) -> Option<String> {
    let begin = "// mds:begin generated modules";
    let end = "// mds:end generated modules";
    let begin_pos = old.find(begin);
    let end_pos = old.find(end);
    match (begin_pos, end_pos) {
        (Some(begin_pos), Some(end_pos)) if begin_pos < end_pos => {
            let end_after = end_pos + end.len();
            let mut content = String::new();
            content.push_str(old[..begin_pos].trim_end());
            content.push('\n');
            content.push_str(block.trim_end());
            content.push('\n');
            content.push_str(old[end_after..].trim_start_matches('\n'));
            Some(content)
        }
        (None, None) => Some(format!("{}\n\n{}", old.trim_end(), block)),
        _ => {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                "Rust module block begin/end markers are inconsistent",
            ));
            None
        }
    }
}

fn plan_manifest(
    package: &Package,
    docs: &[ImplDoc],
    generated: &[GeneratedFile],
) -> GeneratedFile {
    let mut content = String::new();
    for doc in docs {
        let source_hash = sha256(&doc.normalized_input);
        content.push_str("[[sources]]\n");
        content.push_str(&format!(
            "path = \"{}\"\n",
            toml_path(&doc.package_relative_path)
        ));
        content.push_str(&format!("adapter = \"{}\"\n", doc.lang.key()));
        content.push_str(&format!("hash = \"{source_hash}\"\n"));
        for file in generated.iter().filter(|file| {
            file.source_path.as_ref() == Some(&doc.package_relative_path)
                && matches!(file.kind, GeneratedKind::Output(_))
        }) {
            let kind = match file.kind {
                GeneratedKind::Output(kind) => kind.manifest_kind(),
                _ => continue,
            };
            let path = file.path.strip_prefix(&package.root).unwrap_or(&file.path);
            content.push_str("[[sources.outputs]]\n");
            content.push_str(&format!("kind = \"{kind}\"\n"));
            content.push_str(&format!("path = \"{}\"\n", toml_path(path)));
            content.push_str(&format!("hash = \"{}\"\n", sha256(&file.content)));
        }
        content.push('\n');
    }
    GeneratedFile {
        path: package.root.join(".mds/manifest.toml"),
        content,
        kind: GeneratedKind::Manifest,
        source_path: None,
    }
}

fn render_dry_run(generated: &[GeneratedFile], state: &mut RunState) {
    state.stdout.push_str("Build plan:\n");
    for file in generated {
        state
            .stdout
            .push_str(&format!("- {}\n", file.path.display()));
    }
    state.stdout.push('\n');
    for file in generated {
        let old = fs::read_to_string(&file.path).unwrap_or_default();
        state
            .stdout
            .push_str(&unified_diff(&file.path, &old, &file.content));
    }
}

fn write_generated(generated: &[GeneratedFile], state: &mut RunState) -> Result<(), String> {
    for file in generated {
        if let Some(parent) = file.path.parent() {
            fs::create_dir_all(parent).map_err(|error| {
                format!("failed to create directory {}: {error}", parent.display())
            })?;
        }
        fs::write(&file.path, &file.content)
            .map_err(|error| format!("failed to write {}: {error}", file.path.display()))?;
        state.generated.push(file.path.clone());
    }
    state
        .stdout
        .push_str(&format!("build ok: {} files written\n", generated.len()));
    Ok(())
}

fn unified_diff(path: &Path, old: &str, new: &str) -> String {
    if old == new {
        return String::new();
    }
    let old_label = if old.is_empty() {
        "/dev/null".to_string()
    } else {
        format!("a/{}", path.display())
    };
    let new_label = format!("b/{}", path.display());
    let old_lines = old.lines().count();
    let new_lines = new.lines().count();
    let mut out =
        format!("--- {old_label}\n+++ {new_label}\n@@ -1,{old_lines} +1,{new_lines} @@\n");
    for line in old.lines() {
        out.push('-');
        out.push_str(line);
        out.push('\n');
    }
    for line in new.lines() {
        out.push('+');
        out.push_str(line);
        out.push('\n');
    }
    out
}

fn is_mds_managed_file(path: &Path) -> bool {
    fs::read_to_string(path)
        .map(|content| {
            content
                .lines()
                .next()
                .is_some_and(|line| line.contains("Generated by mds"))
        })
        .unwrap_or(false)
}

fn path_within(root: &Path, path: &Path) -> bool {
    path.components()
        .try_fold(Vec::new(), |mut stack: Vec<_>, component| {
            match component {
                Component::ParentDir => {
                    stack.pop();
                }
                Component::Normal(value) => stack.push(value.to_os_string()),
                Component::RootDir | Component::Prefix(_) | Component::CurDir => {}
            }
            Some(stack)
        })
        .is_some()
        && path.starts_with(root)
}

fn collect_files(root: &Path, skip_workspace_private: bool) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();
    collect_files_inner(root, skip_workspace_private, &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_files_inner(
    root: &Path,
    skip_workspace_private: bool,
    files: &mut Vec<PathBuf>,
) -> Result<(), String> {
    let entries = fs::read_dir(root)
        .map_err(|error| format!("failed to read directory {}: {error}", root.display()))?;
    for entry in entries {
        let entry = entry.map_err(|error| format!("failed to read directory entry: {error}"))?;
        let path = entry.path();
        let file_type = entry
            .file_type()
            .map_err(|error| format!("failed to read file type {}: {error}", path.display()))?;
        if file_type.is_dir() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if skip_workspace_private
                && matches!(
                    name.as_ref(),
                    ".git" | ".opencode" | ".agents" | "docs" | "target" | "node_modules"
                )
            {
                continue;
            }
            collect_files_inner(&path, skip_workspace_private, files)?;
        } else if file_type.is_file() {
            files.push(path);
        }
    }
    Ok(())
}

fn sha256(content: &str) -> String {
    sha256_bytes(content.as_bytes())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn sha256_bytes(input: &[u8]) -> [u8; 32] {
    const H0: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];

    let mut data = input.to_vec();
    let bit_len = (data.len() as u64) * 8;
    data.push(0x80);
    while data.len() % 64 != 56 {
        data.push(0);
    }
    data.extend_from_slice(&bit_len.to_be_bytes());

    let mut h = H0;
    for chunk in data.chunks_exact(64) {
        let mut w = [0u32; 64];
        for (idx, word) in w.iter_mut().take(16).enumerate() {
            let start = idx * 4;
            *word = u32::from_be_bytes([
                chunk[start],
                chunk[start + 1],
                chunk[start + 2],
                chunk[start + 3],
            ]);
        }
        for idx in 16..64 {
            let s0 =
                w[idx - 15].rotate_right(7) ^ w[idx - 15].rotate_right(18) ^ (w[idx - 15] >> 3);
            let s1 = w[idx - 2].rotate_right(17) ^ w[idx - 2].rotate_right(19) ^ (w[idx - 2] >> 10);
            w[idx] = w[idx - 16]
                .wrapping_add(s0)
                .wrapping_add(w[idx - 7])
                .wrapping_add(s1);
        }

        let mut a = h[0];
        let mut b = h[1];
        let mut c = h[2];
        let mut d = h[3];
        let mut e = h[4];
        let mut f = h[5];
        let mut g = h[6];
        let mut hh = h[7];
        for idx in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[idx])
                .wrapping_add(w[idx]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);
            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }
        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);
    }

    let mut out = [0u8; 32];
    for (idx, value) in h.iter().enumerate() {
        out[idx * 4..idx * 4 + 4].copy_from_slice(&value.to_be_bytes());
    }
    out
}

fn toml_path(path: &Path) -> String {
    path.components()
        .filter_map(|component| match component {
            Component::Normal(value) => Some(value.to_string_lossy().to_string()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("/")
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

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

    #[test]
    fn computes_sha256() {
        assert_eq!(
            sha256("abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    struct TestDir {
        path: PathBuf,
    }

    impl TestDir {
        fn new() -> Self {
            let id = TEMP_COUNTER.fetch_add(1, Ordering::SeqCst);
            let path =
                std::env::temp_dir().join(format!("mds-core-test-{}-{id}", std::process::id()));
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
}
