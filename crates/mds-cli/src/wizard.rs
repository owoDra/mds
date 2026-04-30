use std::io::{self, stdout};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use mds_core::{
    AgentKitCategory, AiTarget, InitOptions, LabelPreset, PythonTool, RustTool, TypeScriptTool,
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

// ============================================================
// Wizard State Machine
// ============================================================

#[derive(Clone)]
enum Step {
    InitMode,
    LabelPreset,
    TsTools,
    PyTools,
    RsTools,
    AiTargets,
    AiCategories,
    InstallDeps,
    InstallToolchains,
    InstallAiCli,
    Confirm,
}

impl Step {
    fn all() -> Vec<Step> {
        vec![
            Step::InitMode,
            Step::LabelPreset,
            Step::TsTools,
            Step::PyTools,
            Step::RsTools,
            Step::AiTargets,
            Step::AiCategories,
            Step::InstallDeps,
            Step::InstallToolchains,
            Step::InstallAiCli,
            Step::Confirm,
        ]
    }

    fn title(&self) -> &str {
        match self {
            Step::InitMode => "Initialize Mode",
            Step::LabelPreset => "Label Language",
            Step::TsTools => "TypeScript Tools",
            Step::PyTools => "Python Tools",
            Step::RsTools => "Rust Tools",
            Step::AiTargets => "AI Targets",
            Step::AiCategories => "AI Categories",
            Step::InstallDeps => "Install Dependencies",
            Step::InstallToolchains => "Install Toolchains",
            Step::InstallAiCli => "Install AI CLI",
            Step::Confirm => "Confirm",
        }
    }
}

struct SelectItem {
    label: String,
    description: String,
    selected: bool,
}

struct WizardState {
    steps: Vec<Step>,
    current_step: usize,
    ai_only: bool,
    label_preset: LabelPreset,
    ts_tools: Vec<bool>,
    py_tools: Vec<bool>,
    rs_tools: Vec<bool>,
    ai_targets: Vec<bool>,
    ai_categories: Vec<bool>,
    install_deps: bool,
    install_toolchains: bool,
    install_ai_cli: bool,
    confirmed: bool,
    list_state: ListState,
    cancelled: bool,
}

impl WizardState {
    fn new() -> Self {
        let mut state = Self {
            steps: Step::all(),
            current_step: 0,
            ai_only: false,
            label_preset: LabelPreset::English,
            ts_tools: vec![true, true, false, true, false],
            py_tools: vec![true, false, true, false],
            rs_tools: vec![true, true, true, false],
            ai_targets: vec![true, true, true, true],
            ai_categories: vec![true, true, true],
            install_deps: false,
            install_toolchains: false,
            install_ai_cli: false,
            confirmed: false,
            list_state: ListState::default(),
            cancelled: false,
        };
        state.list_state.select(Some(0));
        state
    }

    fn current_items(&self) -> Vec<SelectItem> {
        match &self.steps[self.current_step] {
            Step::InitMode => vec![
                SelectItem {
                    label: "Project files + AI agent kit".into(),
                    description: "Full initialization".into(),
                    selected: !self.ai_only,
                },
                SelectItem {
                    label: "AI agent kit only".into(),
                    description: "Only generate AI integration files".into(),
                    selected: self.ai_only,
                },
            ],
            Step::LabelPreset => vec![
                SelectItem {
                    label: "English (Purpose, Types, Source, Test, ...)".into(),
                    description: "".into(),
                    selected: self.label_preset == LabelPreset::English,
                },
                SelectItem {
                    label: "日本語 (目的, 型定義, 実装, テスト, ...)".into(),
                    description: "".into(),
                    selected: self.label_preset == LabelPreset::Japanese,
                },
            ],
            Step::TsTools => vec![
                SelectItem {
                    label: "eslint".into(),
                    description: "Linter".into(),
                    selected: self.ts_tools[0],
                },
                SelectItem {
                    label: "prettier".into(),
                    description: "Formatter".into(),
                    selected: self.ts_tools[1],
                },
                SelectItem {
                    label: "biome".into(),
                    description: "Linter + Formatter".into(),
                    selected: self.ts_tools[2],
                },
                SelectItem {
                    label: "vitest".into(),
                    description: "Test runner".into(),
                    selected: self.ts_tools[3],
                },
                SelectItem {
                    label: "jest".into(),
                    description: "Test runner".into(),
                    selected: self.ts_tools[4],
                },
            ],
            Step::PyTools => vec![
                SelectItem {
                    label: "ruff".into(),
                    description: "Linter + Formatter".into(),
                    selected: self.py_tools[0],
                },
                SelectItem {
                    label: "black".into(),
                    description: "Formatter".into(),
                    selected: self.py_tools[1],
                },
                SelectItem {
                    label: "pytest".into(),
                    description: "Test runner".into(),
                    selected: self.py_tools[2],
                },
                SelectItem {
                    label: "unittest".into(),
                    description: "Test runner (stdlib)".into(),
                    selected: self.py_tools[3],
                },
            ],
            Step::RsTools => vec![
                SelectItem {
                    label: "rustfmt".into(),
                    description: "Formatter".into(),
                    selected: self.rs_tools[0],
                },
                SelectItem {
                    label: "clippy".into(),
                    description: "Linter".into(),
                    selected: self.rs_tools[1],
                },
                SelectItem {
                    label: "cargo test".into(),
                    description: "Test runner".into(),
                    selected: self.rs_tools[2],
                },
                SelectItem {
                    label: "nextest".into(),
                    description: "Test runner (fast)".into(),
                    selected: self.rs_tools[3],
                },
            ],
            Step::AiTargets => vec![
                SelectItem {
                    label: "Claude Code".into(),
                    description: "".into(),
                    selected: self.ai_targets[0],
                },
                SelectItem {
                    label: "Codex CLI".into(),
                    description: "".into(),
                    selected: self.ai_targets[1],
                },
                SelectItem {
                    label: "Opencode".into(),
                    description: "".into(),
                    selected: self.ai_targets[2],
                },
                SelectItem {
                    label: "GitHub Copilot".into(),
                    description: "".into(),
                    selected: self.ai_targets[3],
                },
            ],
            Step::AiCategories => vec![
                SelectItem {
                    label: "Instructions".into(),
                    description: "Rules and guidelines".into(),
                    selected: self.ai_categories[0],
                },
                SelectItem {
                    label: "Skills".into(),
                    description: "Reusable capabilities".into(),
                    selected: self.ai_categories[1],
                },
                SelectItem {
                    label: "Commands".into(),
                    description: "Custom commands".into(),
                    selected: self.ai_categories[2],
                },
            ],
            Step::InstallDeps => vec![
                SelectItem {
                    label: "Yes".into(),
                    description: "Run npm install / cargo fetch / uv sync".into(),
                    selected: self.install_deps,
                },
                SelectItem {
                    label: "No".into(),
                    description: "Skip".into(),
                    selected: !self.install_deps,
                },
            ],
            Step::InstallToolchains => vec![
                SelectItem {
                    label: "Yes".into(),
                    description: "Check and install required toolchains".into(),
                    selected: self.install_toolchains,
                },
                SelectItem {
                    label: "No".into(),
                    description: "Skip".into(),
                    selected: !self.install_toolchains,
                },
            ],
            Step::InstallAiCli => vec![
                SelectItem {
                    label: "Yes".into(),
                    description: "Check and install AI CLI tools".into(),
                    selected: self.install_ai_cli,
                },
                SelectItem {
                    label: "No".into(),
                    description: "Skip".into(),
                    selected: !self.install_ai_cli,
                },
            ],
            Step::Confirm => vec![
                SelectItem {
                    label: "Apply this plan".into(),
                    description: "".into(),
                    selected: true,
                },
                SelectItem {
                    label: "Cancel".into(),
                    description: "".into(),
                    selected: false,
                },
            ],
        }
    }

    fn is_multi_select(&self) -> bool {
        matches!(
            self.steps[self.current_step],
            Step::TsTools | Step::PyTools | Step::RsTools | Step::AiTargets | Step::AiCategories
        )
    }

    fn should_skip_step(&self) -> bool {
        match &self.steps[self.current_step] {
            Step::LabelPreset | Step::TsTools | Step::PyTools | Step::RsTools => self.ai_only,
            Step::InstallDeps | Step::InstallToolchains => self.ai_only,
            Step::AiCategories => self.ai_targets.iter().all(|&t| !t),
            Step::InstallAiCli => self.ai_targets.iter().all(|&t| !t),
            _ => false,
        }
    }

    fn toggle_at_cursor(&mut self) {
        let idx = self.list_state.selected().unwrap_or(0);
        match &self.steps[self.current_step] {
            Step::InitMode => self.ai_only = idx == 1,
            Step::LabelPreset => {
                self.label_preset = if idx == 1 {
                    LabelPreset::Japanese
                } else {
                    LabelPreset::English
                };
            }
            Step::TsTools => self.ts_tools[idx] = !self.ts_tools[idx],
            Step::PyTools => self.py_tools[idx] = !self.py_tools[idx],
            Step::RsTools => self.rs_tools[idx] = !self.rs_tools[idx],
            Step::AiTargets => self.ai_targets[idx] = !self.ai_targets[idx],
            Step::AiCategories => self.ai_categories[idx] = !self.ai_categories[idx],
            Step::InstallDeps => self.install_deps = idx == 0,
            Step::InstallToolchains => self.install_toolchains = idx == 0,
            Step::InstallAiCli => self.install_ai_cli = idx == 0,
            Step::Confirm => self.confirmed = idx == 0,
        }
    }

    fn advance(&mut self) {
        if !self.is_multi_select() {
            self.toggle_at_cursor();
        }
        loop {
            if self.current_step >= self.steps.len() - 1 {
                break;
            }
            self.current_step += 1;
            if !self.should_skip_step() {
                break;
            }
        }
        self.list_state.select(Some(0));
    }

    fn go_back(&mut self) {
        loop {
            if self.current_step == 0 {
                break;
            }
            self.current_step -= 1;
            if !self.should_skip_step() {
                break;
            }
        }
        self.list_state.select(Some(0));
    }

    fn is_final_step(&self) -> bool {
        self.current_step == self.steps.len() - 1
    }

    fn to_options(&self) -> InitOptions {
        let ts_tools = self
            .ts_tools
            .iter()
            .enumerate()
            .filter(|(_, &s)| s)
            .map(|(i, _)| {
                [
                    TypeScriptTool::Eslint,
                    TypeScriptTool::Prettier,
                    TypeScriptTool::Biome,
                    TypeScriptTool::Vitest,
                    TypeScriptTool::Jest,
                ][i]
            })
            .collect();
        let py_tools = self
            .py_tools
            .iter()
            .enumerate()
            .filter(|(_, &s)| s)
            .map(|(i, _)| {
                [
                    PythonTool::Ruff,
                    PythonTool::Black,
                    PythonTool::Pytest,
                    PythonTool::Unittest,
                ][i]
            })
            .collect();
        let rs_tools = self
            .rs_tools
            .iter()
            .enumerate()
            .filter(|(_, &s)| s)
            .map(|(i, _)| {
                [
                    RustTool::Rustfmt,
                    RustTool::Clippy,
                    RustTool::CargoTest,
                    RustTool::Nextest,
                ][i]
            })
            .collect();
        let targets = self
            .ai_targets
            .iter()
            .enumerate()
            .filter(|(_, &s)| s)
            .map(|(i, _)| AiTarget::all()[i])
            .collect();
        let categories = self
            .ai_categories
            .iter()
            .enumerate()
            .filter(|(_, &s)| s)
            .map(|(i, _)| AgentKitCategory::all()[i])
            .collect();

        InitOptions {
            ai_only: self.ai_only,
            yes: true,
            force: false,
            targets,
            categories,
            ts_tools,
            py_tools,
            rs_tools,
            install_project_deps: self.install_deps,
            install_toolchains: self.install_toolchains,
            install_ai_cli: self.install_ai_cli,
            label_preset: self.label_preset,
        }
    }
}

// ============================================================
// TUI Rendering
// ============================================================

fn render(frame: &mut Frame, state: &mut WizardState) {
    let area = frame.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(area);

    // Header with progress
    let progress_bar: String = state
        .steps
        .iter()
        .enumerate()
        .map(|(i, _)| match i.cmp(&state.current_step) {
            std::cmp::Ordering::Less => '●',
            std::cmp::Ordering::Equal => '◉',
            std::cmp::Ordering::Greater => '○',
        })
        .collect();
    let header = Paragraph::new(Line::from(vec![
        Span::styled(
            format!(
                " mds init — Step {}/{} ",
                state.current_step + 1,
                state.steps.len()
            ),
            Style::default().fg(Color::Cyan).bold(),
        ),
        Span::raw("  "),
        Span::styled(progress_bar, Style::default().fg(Color::DarkGray)),
    ]))
    .block(
        Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(Color::DarkGray)),
    );
    frame.render_widget(header, chunks[0]);

    // Content
    let step = &state.steps[state.current_step];
    let items: Vec<ListItem> = state
        .current_items()
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let marker = if state.is_multi_select() {
                if item.selected {
                    "[×]"
                } else {
                    "[ ]"
                }
            } else if item.selected {
                " ● "
            } else {
                " ○ "
            };
            let is_cursor = state.list_state.selected() == Some(i);
            let style = if is_cursor {
                Style::default().fg(Color::White).bg(Color::Blue)
            } else if item.selected {
                Style::default().fg(Color::Green)
            } else {
                Style::default()
            };
            let text = if item.description.is_empty() {
                format!("{marker} {}", item.label)
            } else {
                format!("{marker} {} — {}", item.label, item.description)
            };
            ListItem::new(text).style(style)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .title(Span::styled(
                format!(" {} ", step.title()),
                Style::default().fg(Color::Yellow).bold(),
            ))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    );

    frame.render_stateful_widget(list, chunks[1], &mut state.list_state);

    // Footer
    let footer_text = if state.is_multi_select() {
        " ↑↓ Move  Space Toggle  Enter Confirm  Esc/← Back  q Quit "
    } else {
        " ↑↓ Move  Enter Select  Esc/← Back  q Quit "
    };
    let footer = Paragraph::new(Span::styled(
        footer_text,
        Style::default().fg(Color::DarkGray),
    ))
    .block(
        Block::default()
            .borders(Borders::TOP)
            .border_style(Style::default().fg(Color::DarkGray)),
    );
    frame.render_widget(footer, chunks[2]);
}

// ============================================================
// Public Entry Point
// ============================================================

pub fn run_interactive_init() -> Result<InitOptions, String> {
    enable_raw_mode().map_err(|e| format!("failed to enable raw mode: {e}"))?;
    stdout()
        .execute(EnterAlternateScreen)
        .map_err(|e| format!("failed to enter alternate screen: {e}"))?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))
        .map_err(|e| format!("failed to create terminal: {e}"))?;

    let mut state = WizardState::new();
    let result = run_loop(&mut terminal, &mut state);

    disable_raw_mode().ok();
    stdout().execute(LeaveAlternateScreen).ok();

    match result {
        Ok(()) if state.cancelled => Err("init cancelled by user".to_string()),
        Ok(()) if !state.confirmed => Err("init cancelled by user".to_string()),
        Ok(()) => Ok(state.to_options()),
        Err(e) => Err(e),
    }
}

fn run_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    state: &mut WizardState,
) -> Result<(), String> {
    loop {
        terminal
            .draw(|frame| render(frame, state))
            .map_err(|e| format!("draw error: {e}"))?;

        if let Event::Key(key) = event::read().map_err(|e| format!("event error: {e}"))? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            match key.code {
                KeyCode::Char('q') => {
                    state.cancelled = true;
                    return Ok(());
                }
                KeyCode::Esc | KeyCode::Left => {
                    if state.current_step == 0 {
                        state.cancelled = true;
                        return Ok(());
                    }
                    state.go_back();
                }
                KeyCode::Up => {
                    let items = state.current_items();
                    let current = state.list_state.selected().unwrap_or(0);
                    let next = if current == 0 {
                        items.len() - 1
                    } else {
                        current - 1
                    };
                    state.list_state.select(Some(next));
                }
                KeyCode::Down => {
                    let items = state.current_items();
                    let current = state.list_state.selected().unwrap_or(0);
                    let next = if current >= items.len() - 1 {
                        0
                    } else {
                        current + 1
                    };
                    state.list_state.select(Some(next));
                }
                KeyCode::Char(' ') if state.is_multi_select() => {
                    state.toggle_at_cursor();
                }
                KeyCode::Enter => {
                    if state.is_final_step() {
                        state.toggle_at_cursor();
                        return Ok(());
                    }
                    state.advance();
                }
                _ => {}
            }
        }
    }
}
