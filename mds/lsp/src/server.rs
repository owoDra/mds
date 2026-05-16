use std::collections::{HashMap, HashSet};
use std::path::{PathBuf};
use mds_core::descriptor::{lang_for_markdown_path};
use mds_core::diagnostics::{RunState};
use mds_core::markdown::{load_implementation_docs};
use mds_core::markdown::{source_markdown_root};
use mds_core::markdown::{test_markdown_root};
use mds_core::package::{discover_packages};
use serde::de::DeserializeOwned;
use serde::{Deserialize};
use serde_json::Value;
use tower_lsp::jsonrpc::{Error, Result};
use tower_lsp::lsp_types::{*};
use tower_lsp::{Client};
use tower_lsp::{LanguageServer};
use tracing::{error};
use tracing::{info};
use crate::{capabilities};
use crate::state::{OpenFile};
use crate::state::{PackageState};
use crate::state::{SharedState};
use crate::state::{WorkspaceIndex};
use crate::state::{WorkspaceState};

const RESOLVE_GENERATED_POSITION_COMMAND: &str = "mds.resolveGeneratedPosition";
const REMAP_GENERATED_LOCATIONS_COMMAND: &str = "mds.remapGeneratedLocations";
const REMAP_GENERATED_RANGE_COMMAND: &str = "mds.remapGeneratedRange";

#[derive(Debug, Deserialize)]
struct ResolveGeneratedPositionParams {
    markdown_uri: Url,
    position: Position,
}

#[derive(Debug, Deserialize)]
struct RemapGeneratedRangeParams {
    uri: Url,
    range: Range,
}

#[derive(Debug, Deserialize)]
struct RemapGeneratedLocationsParams {
    locations: Vec<Location>,
}

pub struct MdsLanguageServer {
    pub client: Client,
    pub state: SharedState,
}

fn bridge_commands() -> Vec<String> {
    vec![
        RESOLVE_GENERATED_POSITION_COMMAND.to_string(),
        REMAP_GENERATED_LOCATIONS_COMMAND.to_string(),
        REMAP_GENERATED_RANGE_COMMAND.to_string(),
    ]
}

fn invalid_params(message: impl Into<String>) -> Error {
    Error::invalid_params(message.into())
}

fn parse_single_argument<T>(arguments: Vec<Value>) -> Result<T>
where
    T: DeserializeOwned,
{
    let mut arguments = arguments.into_iter();
    let value = arguments
        .next()
        .ok_or_else(|| invalid_params("expected a single command argument"))?;
    if arguments.next().is_some() {
        return Err(invalid_params("expected exactly one command argument"));
    }
    serde_json::from_value(value)
        .map_err(|err| invalid_params(format!("failed to decode command arguments: {err}")))
}

fn execute_bridge_command(state: &WorkspaceState, params: ExecuteCommandParams) -> Result<Option<Value>> {
    let result = match params.command.as_str() {
        RESOLVE_GENERATED_POSITION_COMMAND => {
            let command = parse_single_argument::<ResolveGeneratedPositionParams>(params.arguments)?;
            serde_json::to_value(state.resolve_generated_position(&command.markdown_uri, command.position))
        }
        REMAP_GENERATED_LOCATIONS_COMMAND => {
            let command = parse_single_argument::<RemapGeneratedLocationsParams>(params.arguments)?;
            serde_json::to_value(state.remap_generated_locations(&command.locations))
        }
        REMAP_GENERATED_RANGE_COMMAND => {
            let command = parse_single_argument::<RemapGeneratedRangeParams>(params.arguments)?;
            serde_json::to_value(state.remap_generated_range(&command.uri, command.range))
        }
        _ => {
            return Err(invalid_params(format!(
                "unsupported executeCommand `{}`",
                params.command
            )));
        }
    }
    .map_err(|err| invalid_params(format!("failed to encode command result: {err}")))?;

    Ok(Some(result))
}

impl MdsLanguageServer {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            state: Default::default(),
        }
    }
}

impl MdsLanguageServer {
    pub async fn reindex_workspace(&self) {
        let mut state = self.state.write().await;
        state.packages.clear();

        let folders = state.workspace_folders.clone();
        for folder in &folders {
            let mut run_state = RunState::default();
            match discover_packages(folder, None, &mut run_state) {
                Ok(packages) => {
                    for package in packages {
                        let index = build_workspace_index(&package);
                        state.packages.push(PackageState { package, index });
                    }
                }
                Err(e) => {
                    error!("failed to discover packages in {}: {}", folder.display(), e);
                }
            }
        }
        info!("indexed {} packages", state.packages.len());
    }
}

impl MdsLanguageServer {
    async fn is_in_authoring_root(&self, path: &std::path::Path) -> bool {
        let state = self.state.read().await;
        for pkg_state in &state.packages {
            let source_root = source_markdown_root(&pkg_state.package);
            let test_root = test_markdown_root(&pkg_state.package);
            if path.starts_with(&source_root) || path.starts_with(&test_root) {
                return true;
            }
        }
        false
    }
}

impl MdsLanguageServer {
    pub async fn validate_document(&self, uri: &Url, text: &str) {
        let path = match uri.to_file_path() {
            Ok(p) => p,
            Err(_) => return,
        };

        let state = self.state.read().await;
        let pkg_state = state.package_for_path(&path);
        let config = pkg_state
            .map(|p| p.package.config.clone())
            .unwrap_or_default();
        drop(state);

        let diagnostics = if path.ends_with("mds.config.toml") {
            capabilities::diagnostics::validate_config_text(&path, text)
        } else if path.ends_with("overview.md") {
            vec![]
        } else if lang_for_markdown_path(&path).is_some() || self.is_in_authoring_root(&path).await {
            capabilities::diagnostics::validate_impl_md_text(&path, text, &config)
        } else {
            vec![]
        };

        self.client
            .publish_diagnostics(uri.clone(), diagnostics, None)
            .await;
    }
}

fn build_workspace_index(package: &mds_core::Package) -> WorkspaceIndex {
    let mut run_state = RunState::default();
    let docs_vec = load_implementation_docs(package, &mut run_state).unwrap_or_default();
    let generation_plan = mds_core::plan_generation_with_source_map(package, &docs_vec, &mut run_state);

    let mut docs = HashMap::new();
    let mut expose_index: HashMap<String, Vec<PathBuf>> = HashMap::new();
    let mut file_exposes: HashMap<PathBuf, Vec<String>> = HashMap::new();
    let mut module_index: HashMap<String, Vec<PathBuf>> = HashMap::new();
    let mut symbol_index: HashMap<(String, String), Vec<PathBuf>> = HashMap::new();
    let generated_files: HashSet<PathBuf> = generation_plan
        .generated
        .into_iter()
        .map(|file| file.path)
        .collect();

    for doc in docs_vec {
        // Build expose index from the document's uses/exposes
        let path = doc.path.clone();
        // We index the file path itself as a potential expose target
        let md_rel = doc.markdown_relative_path.clone();
        let name = md_rel.to_string_lossy();
        // Strip any known `.<lang>.md` suffix to derive the module stem.
        // This handles current and future language extensions generically.
        let stem = if let Some(pos) = name.rfind('.') {
            let before = &name[..pos]; // strip ".md"
            if let Some(pos2) = before.rfind('.') {
                before[..pos2].to_string() // strip ".<lang>"
            } else {
                before.to_string()
            }
        } else {
            name.to_string()
        };

        let module_id = stem.replace('\\', "/").replace('/', ".");
        for module_key in [stem.clone(), module_id.clone()] {
            module_index.entry(module_key.clone()).or_default().push(path.clone());
            expose_index
                .entry(module_key.clone())
                .or_default()
                .push(path.clone());
            file_exposes.entry(path.clone()).or_default().push(module_key);
        }

        expose_index
            .entry(stem.clone())
            .or_default()
            .push(path.clone());
        file_exposes.entry(path.clone()).or_default().push(stem.clone());

        if let Ok(text) = std::fs::read_to_string(&path) {
            let exported_names = exported_names_from_text(&text);
            for exported in &exported_names {
                symbol_index
                    .entry((module_id.clone(), exported.clone()))
                    .or_default()
                    .push(path.clone());
            }
            for exposed in exported_names {
                expose_index
                    .entry(exposed.clone())
                    .or_default()
                    .push(path.clone());
                file_exposes.entry(path.clone()).or_default().push(exposed);
            }
        }

        docs.insert(path, doc);
    }

    WorkspaceIndex {
        docs,
        expose_index,
        file_exposes,
        module_index,
        symbol_index,
        source_map: generation_plan.source_map,
        generated_files,
    }
}

fn exported_names_from_text(text: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut in_exports = false;

    for line in text.lines() {
        if let Some(title) = line.strip_prefix("## ") {
            let title = title.trim();
            in_exports = matches!(title, "Exports" | "Expose" | "Exposes" | "公開" | "公開面");
            continue;
        }

        if let Some(title) = line.strip_prefix("##### ") {
            let name = title.trim();
            if !name.is_empty() {
                names.push(name.to_string());
            }
            continue;
        }

        if in_exports && line.trim_start().starts_with('|') {
            let cells: Vec<&str> = line.trim().trim_matches('|').split('|').map(str::trim).collect();
            if let Some(name) = cells.first() {
                if !name.is_empty()
                    && *name != "Name"
                    && *name != "名前"
                    && !name.chars().all(|c| c == '-')
                {
                    names.push((*name).to_string());
                }
            }
        }
    }

    names.sort();
    names.dedup();
    names
}

#[tower_lsp::async_trait]

impl LanguageServer for MdsLanguageServer {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        info!("mds-lsp initializing");

        if let Some(folders) = params.workspace_folders {
            let mut state = self.state.write().await;
            for folder in folders {
                if let Ok(path) = folder.uri.to_file_path() {
                    state.workspace_folders.push(path);
                }
            }
        } else if let Some(root_uri) = params.root_uri {
            if let Ok(path) = root_uri.to_file_path() {
                let mut state = self.state.write().await;
                state.workspace_folders.push(path);
            }
        }

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        open_close: Some(true),
                        change: Some(TextDocumentSyncKind::FULL),
                        save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
                            include_text: Some(true),
                        })),
                        ..Default::default()
                    },
                )),
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec![
                        "#".to_string(),
                        "|".to_string(),
                        "`".to_string(),
                        "[".to_string(),
                    ]),
                    resolve_provider: Some(false),
                    ..Default::default()
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
                references_provider: Some(OneOf::Left(true)),
                document_symbol_provider: Some(OneOf::Left(true)),
                workspace_symbol_provider: Some(OneOf::Left(true)),
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                execute_command_provider: Some(ExecuteCommandOptions {
                    commands: bridge_commands(),
                    work_done_progress_options: WorkDoneProgressOptions::default(),
                }),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "mds-lsp".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        info!("mds-lsp initialized");
        self.reindex_workspace().await;
    }

    async fn shutdown(&self) -> Result<()> {
        info!("mds-lsp shutting down");
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        let text = params.text_document.text.clone();
        let version = params.text_document.version;

        if let Ok(path) = uri.to_file_path() {
            let lang = lang_for_markdown_path(&path);
            let mut state = self.state.write().await;
            state.open_files.insert(
                uri.to_string(),
                OpenFile {
                    uri: uri.to_string(),
                    path,
                    text: text.clone(),
                    version,
                    lang,
                },
            );
        }

        self.validate_document(&uri, &text).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        if let Some(change) = params.content_changes.into_iter().next_back() {
            let text = change.text.clone();
            {
                let mut state = self.state.write().await;
                if let Some(file) = state.open_files.get_mut(&uri.to_string()) {
                    file.text = text.clone();
                    file.version = params.text_document.version;
                }
            }
            self.validate_document(&uri, &text).await;
        }
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        let uri = params.text_document.uri;
        if let Ok(path) = uri.to_file_path() {
            // Use provided text or fall back to reading from disk
            let text = if let Some(text) = params.text {
                text
            } else {
                match std::fs::read_to_string(&path) {
                    Ok(t) => t,
                    Err(_) => return,
                }
            };
            self.validate_document(&uri, &text).await;

            // Reindex if config changed
            if path.ends_with("mds.config.toml") {
                self.reindex_workspace().await;
            }
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;
        let mut state = self.state.write().await;
        state.open_files.remove(&uri.to_string());

        // Clear diagnostics for closed file
        drop(state);
        self.client.publish_diagnostics(uri, vec![], None).await;
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;

        let state = self.state.read().await;
        let text = state
            .open_files
            .get(&uri.to_string())
            .map(|f| f.text.clone());
        let path = uri.to_file_path().ok();
        let config = path
            .as_ref()
            .and_then(|p| state.package_for_path(p))
            .map(|p| p.package.config.clone())
            .unwrap_or_default();
        if let Some(text) = text {
            let items = capabilities::completion::provide_completions(
                &text,
                position,
                path.as_deref(),
                &config,
                Some(&state),
            );
            Ok(Some(CompletionResponse::Array(items)))
        } else {
            Ok(None)
        }
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        let state = self.state.read().await;
        let text = state
            .open_files
            .get(&uri.to_string())
            .map(|f| f.text.clone());
        let path = uri.to_file_path().ok();
        drop(state);

        if let (Some(text), Some(path)) = (text, path) {
            let state = self.state.read().await;
            let hover = capabilities::hover::provide_hover(&text, position, &path, &state);
            Ok(hover)
        } else {
            Ok(None)
        }
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        let state = self.state.read().await;
        let text = state
            .open_files
            .get(&uri.to_string())
            .map(|f| f.text.clone());
        let path = uri.to_file_path().ok();
        drop(state);

        if let (Some(text), Some(path)) = (text, path) {
            let state = self.state.read().await;
            let result = capabilities::navigation::goto_definition(&text, position, &path, &state);
            Ok(result)
        } else {
            Ok(None)
        }
    }

    async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;

        let state = self.state.read().await;
        let text = state
            .open_files
            .get(&uri.to_string())
            .map(|f| f.text.clone());
        let path = uri.to_file_path().ok();
        drop(state);

        if let (Some(text), Some(path)) = (text, path) {
            let state = self.state.read().await;
            let result = capabilities::navigation::find_references(&text, position, &path, &state);
            Ok(result)
        } else {
            Ok(None)
        }
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let uri = params.text_document.uri;
        let state = self.state.read().await;
        let text = state
            .open_files
            .get(&uri.to_string())
            .map(|f| f.text.clone());
        drop(state);

        if let Some(text) = text {
            let symbols = capabilities::symbols::document_symbols(&text);
            Ok(Some(DocumentSymbolResponse::Flat(symbols)))
        } else {
            Ok(None)
        }
    }

    async fn symbol(
        &self,
        params: WorkspaceSymbolParams,
    ) -> Result<Option<Vec<SymbolInformation>>> {
        let query = params.query.to_lowercase();
        let state = self.state.read().await;
        let symbols = capabilities::symbols::workspace_symbols(&query, &state);
        Ok(Some(symbols))
    }

    async fn code_action(&self, params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        let uri = params.text_document.uri;
        let state = self.state.read().await;
        let text = state
            .open_files
            .get(&uri.to_string())
            .map(|f| f.text.clone());
        let path = uri.to_file_path().ok();
        let config = path
            .as_ref()
            .and_then(|p| state.package_for_path(p))
            .map(|p| p.package.config.clone())
            .unwrap_or_default();
        drop(state);

        if let Some(text) = text {
            let actions = capabilities::code_action::provide_code_actions(&uri, &text, &config);
            Ok(Some(actions))
        } else {
            Ok(None)
        }
    }

    async fn execute_command(&self, params: ExecuteCommandParams) -> Result<Option<Value>> {
        let state = self.state.read().await;
        execute_bridge_command(&state, params)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};
    use tempfile::TempDir;
    use tower_lsp::lsp_types::{ExecuteCommandParams, Location, Position, Range, Url};
    use super::build_workspace_index;
    use super::execute_bridge_command;
    use super::REMAP_GENERATED_LOCATIONS_COMMAND;
    use crate::state::{PackageState, WorkspaceState};
    use mds_core::{Config, Package};

    struct BridgeFixture {
        _temp: TempDir,
        package_state: PackageState,
        markdown_path: PathBuf,
        source_generated_path: PathBuf,
    }

    fn bridge_fixture() -> BridgeFixture {
        let temp = TempDir::new().unwrap();
        let root = temp.path().join("pkg");
        let markdown_path = root.join(".mds/source/foo/source-map.ts.md");
        std::fs::create_dir_all(markdown_path.parent().unwrap()).unwrap();
        std::fs::write(
            &markdown_path,
            r#"# Source map

## Purpose

Fixture.

## Contract

- Preserve source map spans.

## Source

```ts
export const one = 1;
```

```ts
export function two(): number {
  return one + 1;
}
```

## Test

```ts
expect(two()).toBe(2);
```
"#,
        )
        .unwrap();

        let package = Package {
            root: root.clone(),
            config: Config::default(),
            package_manager_id: "npm".to_string(),
        };
        let index = build_workspace_index(&package);

        BridgeFixture {
            _temp: temp,
            package_state: PackageState { package, index },
            markdown_path,
            source_generated_path: root.join("src/foo/source-map.ts"),
        }
    }

    #[test]
    fn build_workspace_index_uses_markdown_exports_for_symbol_index() {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "mds-lsp-exports-index-{}-{}",
            std::process::id(),
            suffix,
        ));
        let source = root.join(".mds/source/app/greet.ts.md");
        std::fs::create_dir_all(source.parent().unwrap()).unwrap();
        std::fs::write(
            &source,
            "# app.greet\n\n## Purpose\n\nA module.\n\n## Contract\n\n- Stable.\n\n## Exports\n\n##### greet\n\nShared entrypoint.\n\n## Source\n\n```ts\nexport function greet(): string { return 'hi'; }\n```\n",
        )
        .unwrap();

        let package = Package {
            root: root.clone(),
            config: Config::default(),
            package_manager_id: "npm".to_string(),
        };

        let index = build_workspace_index(&package);
        let locations = index
            .symbol_index
            .get(&("app.greet".to_string(), "greet".to_string()))
            .cloned()
            .unwrap_or_default();

        assert_eq!(locations, vec![source.clone()]);

        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn build_workspace_index_tracks_source_maps_and_generated_files() {
        let fixture = bridge_fixture();

        assert!(fixture
            .package_state
            .contains_generated_path(&fixture.source_generated_path));
        let remapped = fixture
            .package_state
            .index
            .source_map
            .remap_generated_line(&fixture.source_generated_path, 5)
            .expect("expected generated line to map back to markdown");
        assert_eq!(remapped.0, fixture.markdown_path.as_path());
        assert_eq!(remapped.1, 18);
    }

    #[test]
    fn remap_generated_range_returns_markdown_range_for_code_fence_lines() {
        let fixture = bridge_fixture();
        let markdown_uri = Url::from_file_path(&fixture.markdown_path).unwrap();

        let remapped = fixture
            .package_state
            .remap_generated_range(
                &fixture.source_generated_path,
                Range {
                    start: Position {
                        line: 4,
                        character: 1,
                    },
                    end: Position {
                        line: 5,
                        character: 10,
                    },
                },
            )
            .expect("expected generated range to remap");

        assert_eq!(remapped.uri, markdown_uri);
        assert_eq!(
            remapped.range,
            Range {
                start: Position {
                    line: 17,
                    character: 1,
                },
                end: Position {
                    line: 18,
                    character: 10,
                },
            }
        );
        assert!(fixture
            .package_state
            .remap_generated_range(
                &fixture.source_generated_path,
                Range {
                    start: Position {
                        line: 0,
                        character: 0,
                    },
                    end: Position {
                        line: 0,
                        character: 0,
                    },
                },
            )
            .is_none());
    }

    #[test]
    fn resolve_generated_position_returns_generated_location_for_markdown_code_fence() {
        let fixture = bridge_fixture();
        let markdown_uri = Url::from_file_path(&fixture.markdown_path).unwrap();
        let generated_uri = Url::from_file_path(&fixture.source_generated_path).unwrap();
        let state = WorkspaceState {
            packages: vec![fixture.package_state.clone()],
            ..WorkspaceState::default()
        };

        let resolved = state
            .resolve_generated_position(
                &markdown_uri,
                Position {
                    line: 17,
                    character: 6,
                },
            )
            .expect("expected markdown position to resolve to generated output");

        assert_eq!(resolved.uri, generated_uri);
        assert_eq!(
            resolved.range,
            Range {
                start: Position {
                    line: 4,
                    character: 6,
                },
                end: Position {
                    line: 4,
                    character: 6,
                },
            }
        );
        assert!(state
            .resolve_generated_position(
                &markdown_uri,
                Position {
                    line: 0,
                    character: 0,
                },
            )
            .is_none());
    }

    #[test]
    fn execute_command_remaps_generated_locations_via_json_bridge() {
        let fixture = bridge_fixture();
        let markdown_uri = Url::from_file_path(&fixture.markdown_path).unwrap();
        let generated_uri = Url::from_file_path(&fixture.source_generated_path).unwrap();
        let state = WorkspaceState {
            packages: vec![fixture.package_state],
            ..WorkspaceState::default()
        };

        let value = execute_bridge_command(
            &state,
            ExecuteCommandParams {
                command: REMAP_GENERATED_LOCATIONS_COMMAND.to_string(),
                arguments: vec![json!({
                    "locations": [
                        {
                            "uri": generated_uri,
                            "range": {
                                "start": { "line": 4, "character": 2 },
                                "end": { "line": 4, "character": 9 }
                            }
                        }
                    ]
                })],
                work_done_progress_params: Default::default(),
            },
        )
        .expect("expected execute command to succeed")
        .expect("expected execute command payload");

        let remapped: Vec<Option<Location>> = serde_json::from_value(value).unwrap();
        assert_eq!(remapped.len(), 1);
        assert_eq!(
            remapped[0],
            Some(Location {
                uri: markdown_uri,
                range: Range {
                    start: Position {
                        line: 17,
                        character: 2,
                    },
                    end: Position {
                        line: 17,
                        character: 9,
                    },
                },
            })
        );
    }
}
