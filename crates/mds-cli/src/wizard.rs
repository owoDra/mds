use std::io::{self, stdout};
use std::path::Path;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use mds_core::{
    AgentKitCategory, AiTarget, InitOptions, InitQualityCommands, InitTargetCategories,
    LabelPreset, Lang,
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Padding, Paragraph, Wrap},
};

#[derive(Clone)]
enum Step {
    LabelPreset,
    ToolchainCommand {
        toolchain: usize,
        field: QualityField,
    },
    AiNeeded,
    AiTargets,
    AiCategories {
        target: usize,
    },
    Confirm,
}

impl Step {
    fn title(&self, state: &WizardState) -> String {
        let japanese = state.is_japanese();
        match self {
            Step::LabelPreset => localized(japanese, "Label Language", "ラベル言語").into(),
            Step::ToolchainCommand { toolchain, field } => {
                if japanese {
                    format!(
                        "{} の{}コマンド",
                        state.toolchains[*toolchain].profile.name,
                        field.title(japanese)
                    )
                } else {
                    format!(
                        "{} {} Command",
                        state.toolchains[*toolchain].profile.name,
                        field.title(japanese)
                    )
                }
            }
            Step::AiNeeded => localized(japanese, "AI Kit", "AI キット").into(),
            Step::AiTargets => localized(japanese, "AI CLI Targets", "AI CLI 対象").into(),
            Step::AiCategories { target } => {
                if japanese {
                    format!("{} の生成項目", ai_target_label(AiTarget::all()[*target]))
                } else {
                    format!("{} Items", ai_target_label(AiTarget::all()[*target]))
                }
            }
            Step::Confirm => localized(japanese, "Confirm", "確認").into(),
        }
    }
}

#[derive(Clone, Copy)]
enum QualityField {
    TypeCheck,
    Lint,
    Test,
}

impl QualityField {
    fn title(self, japanese: bool) -> &'static str {
        match (self, japanese) {
            (Self::TypeCheck, false) => "Type Check",
            (Self::Lint, false) => "Lint Check",
            (Self::Test, false) => "Test Check",
            (Self::TypeCheck, true) => "型チェック",
            (Self::Lint, true) => "Lint チェック",
            (Self::Test, true) => "テスト",
        }
    }

    fn suggestion(self, profile: &ToolchainProfile) -> &'static str {
        match self {
            Self::TypeCheck => profile.type_check,
            Self::Lint => profile.lint,
            Self::Test => profile.test,
        }
    }
}

struct ToolchainProfile {
    lang: Lang,
    name: &'static str,
    metadata: &'static str,
    type_check: &'static str,
    lint: &'static str,
    test: &'static str,
}

struct ToolchainState {
    profile: ToolchainProfile,
    type_check: String,
    lint: String,
    test: String,
}

struct SelectItem {
    label: String,
    description: String,
    selected: bool,
}

struct WizardState {
    steps: Vec<Step>,
    current_step: usize,
    label_preset: LabelPreset,
    toolchains: Vec<ToolchainState>,
    ai_enabled: bool,
    ai_targets: Vec<bool>,
    ai_categories: Vec<[bool; 3]>,
    confirmed: bool,
    list_state: ListState,
    cancelled: bool,
}

impl WizardState {
    fn new(root: &Path) -> Self {
        let toolchains = detect_toolchains(root);
        let mut steps = vec![Step::LabelPreset];
        for index in 0..toolchains.len() {
            steps.push(Step::ToolchainCommand {
                toolchain: index,
                field: QualityField::TypeCheck,
            });
            steps.push(Step::ToolchainCommand {
                toolchain: index,
                field: QualityField::Lint,
            });
            steps.push(Step::ToolchainCommand {
                toolchain: index,
                field: QualityField::Test,
            });
        }
        steps.push(Step::AiNeeded);
        steps.push(Step::AiTargets);
        for target in 0..AiTarget::all().len() {
            steps.push(Step::AiCategories { target });
        }
        steps.push(Step::Confirm);

        let mut state = Self {
            steps,
            current_step: 0,
            label_preset: LabelPreset::English,
            toolchains,
            ai_enabled: true,
            ai_targets: vec![true, true, true, true],
            ai_categories: vec![[true, true, true]; 4],
            confirmed: false,
            list_state: ListState::default(),
            cancelled: false,
        };
        state.list_state.select(Some(0));
        state
    }

    fn current_items(&self) -> Vec<SelectItem> {
        let japanese = self.is_japanese();
        match &self.steps[self.current_step] {
            Step::LabelPreset => vec![
                SelectItem {
                    label: "English".into(),
                    description: localized(
                        japanese,
                        "Use English section labels such as Purpose, Types, Source, and Test.",
                        "Purpose、Types、Source、Test などの英語ラベルを使います。",
                    )
                    .into(),
                    selected: self.label_preset == LabelPreset::English,
                },
                SelectItem {
                    label: "日本語".into(),
                    description: localized(
                        japanese,
                        "Use Japanese section labels such as 目的, 型定義, 実装, and テスト.",
                        "目的、型定義、実装、テスト などの日本語ラベルを使います。",
                    )
                    .into(),
                    selected: self.label_preset == LabelPreset::Japanese,
                },
            ],
            Step::AiNeeded => vec![
                SelectItem {
                    label: localized(japanese, "Generate AI kit", "AI キットを生成").into(),
                    description: localized(
                        japanese,
                        "Create agent instructions, skills, or commands for selected AI CLIs.",
                        "選択した AI CLI 向けの指示、スキル、コマンドを生成します。",
                    )
                    .into(),
                    selected: self.ai_enabled,
                },
                SelectItem {
                    label: localized(japanese, "Skip AI kit", "AI キットをスキップ").into(),
                    description: localized(
                        japanese,
                        "Only initialize the mds project files and skip AI CLI files.",
                        "mds プロジェクトファイルだけを初期化し、AI CLI ファイルは生成しません。",
                    )
                    .into(),
                    selected: !self.ai_enabled,
                },
            ],
            Step::AiTargets => AiTarget::all()
                .iter()
                .enumerate()
                .map(|(i, target)| SelectItem {
                    label: ai_target_label(*target).into(),
                    description: ai_target_description(*target, japanese).into(),
                    selected: self.ai_targets[i],
                })
                .collect(),
            Step::AiCategories { target } => AgentKitCategory::all()
                .iter()
                .enumerate()
                .map(|(i, category)| SelectItem {
                    label: category_label(*category, japanese).into(),
                    description: category_description(*category, japanese).into(),
                    selected: self.ai_categories[*target][i],
                })
                .collect(),
            Step::Confirm => vec![
                SelectItem {
                    label: localized(japanese, "Apply initialization", "初期化を実行").into(),
                    description: localized(
                        japanese,
                        "Write the generated project files using this configuration.",
                        "この設定で生成したプロジェクトファイルを書き込みます。",
                    )
                    .into(),
                    selected: true,
                },
                SelectItem {
                    label: localized(japanese, "Cancel", "キャンセル").into(),
                    description: localized(
                        japanese,
                        "Exit the wizard without writing any files.",
                        "ファイルを書き込まずにウィザードを終了します。",
                    )
                    .into(),
                    selected: false,
                },
            ],
            Step::ToolchainCommand { .. } => Vec::new(),
        }
    }

    fn is_text_input(&self) -> bool {
        matches!(self.steps[self.current_step], Step::ToolchainCommand { .. })
    }

    fn is_japanese(&self) -> bool {
        self.label_preset == LabelPreset::Japanese
    }

    fn is_multi_select(&self) -> bool {
        matches!(
            self.steps[self.current_step],
            Step::AiTargets | Step::AiCategories { .. }
        )
    }

    fn should_skip_step(&self) -> bool {
        match &self.steps[self.current_step] {
            Step::AiTargets => !self.ai_enabled,
            Step::AiCategories { target } => !self.ai_enabled || !self.ai_targets[*target],
            _ => false,
        }
    }

    fn text_value_mut(&mut self) -> Option<&mut String> {
        let Step::ToolchainCommand { toolchain, field } = self.steps[self.current_step] else {
            return None;
        };
        let state = &mut self.toolchains[toolchain];
        Some(match field {
            QualityField::TypeCheck => &mut state.type_check,
            QualityField::Lint => &mut state.lint,
            QualityField::Test => &mut state.test,
        })
    }

    fn text_value(&self) -> Option<&str> {
        let Step::ToolchainCommand { toolchain, field } = self.steps[self.current_step] else {
            return None;
        };
        let state = &self.toolchains[toolchain];
        Some(match field {
            QualityField::TypeCheck => &state.type_check,
            QualityField::Lint => &state.lint,
            QualityField::Test => &state.test,
        })
    }

    fn text_suggestion(&self) -> Option<&str> {
        let Step::ToolchainCommand { toolchain, field } = self.steps[self.current_step] else {
            return None;
        };
        Some(field.suggestion(&self.toolchains[toolchain].profile))
    }

    fn text_metadata(&self) -> Option<&str> {
        let Step::ToolchainCommand { toolchain, .. } = self.steps[self.current_step] else {
            return None;
        };
        Some(self.toolchains[toolchain].profile.metadata)
    }

    fn text_description(&self) -> Option<&'static str> {
        let Step::ToolchainCommand { field, .. } = self.steps[self.current_step] else {
            return None;
        };
        Some(match (field, self.is_japanese()) {
            (QualityField::TypeCheck, false) => {
                "Command used by mds to verify generated source types without running tests."
            }
            (QualityField::Lint, false) => {
                "Command used by mds to check formatting, style, and static analysis issues."
            }
            (QualityField::Test, false) => {
                "Command used by mds to run the project test suite after generation."
            }
            (QualityField::TypeCheck, true) => {
                "生成されたソースの型を、テスト実行なしで検証するためのコマンドです。"
            }
            (QualityField::Lint, true) => {
                "フォーマット、スタイル、静的解析の問題を確認するためのコマンドです。"
            }
            (QualityField::Test, true) => {
                "生成後にプロジェクトのテストスイートを実行するためのコマンドです。"
            }
        })
    }

    fn toggle_at_cursor(&mut self) {
        let idx = self.list_state.selected().unwrap_or(0);
        match &self.steps[self.current_step] {
            Step::LabelPreset => {
                self.label_preset = if idx == 1 {
                    LabelPreset::Japanese
                } else {
                    LabelPreset::English
                };
            }
            Step::AiNeeded => self.ai_enabled = idx == 0,
            Step::AiTargets => self.ai_targets[idx] = !self.ai_targets[idx],
            Step::AiCategories { target } => {
                self.ai_categories[*target][idx] = !self.ai_categories[*target][idx]
            }
            Step::Confirm => self.confirmed = idx == 0,
            Step::ToolchainCommand { .. } => {}
        }
    }

    fn advance(&mut self) {
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
        let quality_commands = self
            .toolchains
            .iter()
            .map(|toolchain| InitQualityCommands {
                lang: toolchain.profile.lang.clone(),
                type_check: non_empty_command(&toolchain.type_check),
                lint: non_empty_command(&toolchain.lint),
                test: non_empty_command(&toolchain.test),
            })
            .collect();

        let targets: Vec<AiTarget> = if self.ai_enabled {
            self.ai_targets
                .iter()
                .enumerate()
                .filter(|(_, selected)| **selected)
                .map(|(i, _)| AiTarget::all()[i])
                .collect()
        } else {
            Vec::new()
        };
        let target_categories = targets
            .iter()
            .map(|target| {
                let target_index = AiTarget::all()
                    .iter()
                    .position(|known| known == target)
                    .unwrap_or(0);
                InitTargetCategories {
                    target: *target,
                    categories: AgentKitCategory::all()
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| self.ai_categories[target_index][*i])
                        .map(|(_, category)| *category)
                        .collect(),
                }
            })
            .collect();

        InitOptions {
            ai_only: false,
            yes: true,
            force: false,
            targets,
            categories: Vec::new(),
            ts_tools: Vec::new(),
            py_tools: Vec::new(),
            rs_tools: Vec::new(),
            install_project_deps: false,
            install_toolchains: false,
            install_ai_cli: false,
            label_preset: self.label_preset,
            quality_commands,
            target_categories,
        }
    }
}

fn render(frame: &mut Frame, state: &mut WizardState) {
    let area = frame.size();
    let shell = centered_rect(92, 86, area);
    frame.render_widget(Clear, shell);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(12),
            Constraint::Length(4),
        ])
        .split(shell);

    let progress_bar: String = state
        .steps
        .iter()
        .enumerate()
        .map(|(i, _)| match i.cmp(&state.current_step) {
            std::cmp::Ordering::Less => '*',
            std::cmp::Ordering::Equal => '>',
            std::cmp::Ordering::Greater => '.',
        })
        .collect();
    let title = state.steps[state.current_step].title(state);
    let japanese = state.is_japanese();
    let header = Paragraph::new(vec![
        Line::from(vec![
            Span::styled(
                " mds init ",
                Style::default().fg(Color::Black).bg(Color::Cyan).bold(),
            ),
            Span::raw("  "),
            Span::styled(
                if japanese {
                    format!("ステップ {}/{}", state.current_step + 1, state.steps.len())
                } else {
                    format!("Step {} of {}", state.current_step + 1, state.steps.len())
                },
                Style::default().fg(Color::Gray),
            ),
        ]),
        Line::from(vec![
            Span::styled(title, Style::default().fg(Color::White).bold()),
            Span::raw("  "),
            Span::styled(progress_bar, Style::default().fg(Color::DarkGray)),
        ]),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .padding(Padding::horizontal(2)),
    );
    frame.render_widget(header, chunks[0]);

    if state.is_text_input() {
        render_text_input(frame, state, chunks[1]);
    } else {
        render_selection(frame, state, chunks[1]);
    }

    let footer_text = if state.is_text_input() {
        localized(
            japanese,
            " Type command  Enter/Right Next  Left Back  Esc/q Quit ",
            " コマンド入力  Enter/Right 次へ  Left 戻る  Esc/q 終了 ",
        )
    } else if state.is_multi_select() {
        localized(
            japanese,
            " Up/Down Move  Enter Toggle  Right Next  Left Back  Esc/q Quit ",
            " Up/Down 移動  Enter 切替  Right 次へ  Left 戻る  Esc/q 終了 ",
        )
    } else {
        localized(
            japanese,
            " Up/Down Move  Enter Select  Right Next  Left Back  Esc/q Quit ",
            " Up/Down 移動  Enter 選択  Right 次へ  Left 戻る  Esc/q 終了 ",
        )
    };
    let footer = Paragraph::new(Line::from(vec![
        Span::styled(
            localized(japanese, "Keys", "キー"),
            Style::default().fg(Color::Cyan).bold(),
        ),
        Span::styled(footer_text, Style::default().fg(Color::Gray)),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray))
            .padding(Padding::horizontal(2)),
    );
    frame.render_widget(footer, chunks[2]);
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1])[1]
}

fn render_selection(frame: &mut Frame, state: &mut WizardState, area: Rect) {
    let inner = centered_rect(86, 90, area);
    frame.render_widget(Clear, inner);
    let japanese = state.is_japanese();

    let items: Vec<ListItem> = state
        .current_items()
        .into_iter()
        .enumerate()
        .map(|(i, item)| {
            let marker = if item.selected { "✅" } else { "⬜" };
            let is_cursor = state.list_state.selected() == Some(i);
            let card_style = if is_cursor {
                Style::default().fg(Color::Black).bg(Color::Cyan).bold()
            } else if item.selected {
                Style::default().fg(Color::Green).bold()
            } else {
                Style::default().fg(Color::Gray)
            };
            let cursor = if is_cursor { ">" } else { " " };
            let is_recommended = !state.is_multi_select()
                && matches!(
                    state.steps[state.current_step],
                    Step::AiNeeded | Step::Confirm
                )
                && i == 0;
            let status: String = if is_recommended {
                localized(japanese, "[recommended]", "[推奨]").into()
            } else {
                "".into()
            };
            let detail = if item.description.is_empty() {
                "".into()
            } else {
                format!("    {}", item.description)
            };
            let status_style = if is_cursor {
                Style::default().fg(Color::Black).bg(Color::Cyan).bold()
            } else {
                Style::default().fg(Color::Yellow).bold()
            };
            let mut title_spans = vec![Span::styled(format!("{cursor} "), card_style)];
            title_spans.push(Span::styled(marker, status_style));
            title_spans.push(Span::raw(" "));
            title_spans.push(Span::styled(item.label, card_style));
            if !status.is_empty() {
                if !state.is_multi_select() {
                    title_spans.push(Span::raw(" "));
                    title_spans.push(Span::styled(status, status_style));
                }
            }
            ListItem::new(vec![
                Line::from(title_spans),
                Line::from(Span::styled(detail, Style::default().fg(Color::DarkGray))),
                Line::raw(""),
            ])
        })
        .collect();
    let title = state.steps[state.current_step].title(state);
    let list = List::new(items).block(
        Block::default()
            .title(Span::styled(
                if japanese {
                    format!(" {title} - 選択してください ")
                } else {
                    format!(" {title} - choose an option ")
                },
                Style::default().fg(Color::Cyan).bold(),
            ))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .padding(Padding::new(2, 2, 1, 1)),
    );
    frame.render_stateful_widget(list, inner, &mut state.list_state);
}

fn render_text_input(frame: &mut Frame, state: &WizardState, area: Rect) {
    let inner = centered_rect(86, 90, area);
    frame.render_widget(Clear, inner);
    let japanese = state.is_japanese();

    let title = state.steps[state.current_step].title(state);
    let value = state.text_value().unwrap_or_default();
    let suggestion = state.text_suggestion().unwrap_or_default();
    let metadata = state.text_metadata().unwrap_or_default();
    let description = state.text_description().unwrap_or_default();

    let outer = Block::default()
        .title(Span::styled(
            if japanese {
                format!(" {title} - コマンド編集 ")
            } else {
                format!(" {title} - edit command ")
            },
            Style::default().fg(Color::Cyan).bold(),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow))
        .padding(Padding::new(2, 2, 1, 1));
    let content = Rect {
        x: inner.x.saturating_add(3),
        y: inner.y.saturating_add(2),
        width: inner.width.saturating_sub(6),
        height: inner.height.saturating_sub(4),
    };
    frame.render_widget(outer, inner);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(content);

    let description_lines = vec![
        Line::from(Span::styled(
            localized(japanese, "Description", "説明"),
            Style::default().fg(Color::Cyan).bold(),
        )),
        Line::from(Span::styled(description, Style::default().fg(Color::Gray))),
    ];
    frame.render_widget(
        Paragraph::new(description_lines).wrap(Wrap { trim: false }),
        chunks[0],
    );

    let metadata_lines = vec![
        Line::from(vec![
            Span::styled(
                localized(japanese, "Detected ", "検出済み "),
                Style::default().fg(Color::DarkGray),
            ),
            Span::styled(metadata, Style::default().fg(Color::White).bold()),
        ]),
        Line::from(vec![
            Span::styled(
                localized(japanese, "Suggestion ", "推奨コマンド "),
                Style::default().fg(Color::DarkGray),
            ),
            Span::styled(suggestion, Style::default().fg(Color::Green)),
        ]),
    ];
    frame.render_widget(Paragraph::new(metadata_lines), chunks[1]);

    let field = Paragraph::new(Span::styled(
        value,
        Style::default().fg(Color::White).bg(Color::Blue).bold(),
    ))
    .block(
        Block::default()
            .title(Span::styled(
                localized(japanese, " Command ", " コマンド "),
                Style::default().fg(Color::Black).bg(Color::Yellow).bold(),
            ))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow))
            .padding(Padding::horizontal(1)),
    );
    frame.render_widget(field, chunks[3]);

    let help = Paragraph::new(Line::from(vec![
        Span::styled(
            localized(japanese, "Saved value: ", "保存される値: "),
            Style::default().fg(Color::DarkGray),
        ),
        Span::styled(
            localized(japanese, "the blue field text", "青いフィールド内の文字"),
            Style::default().fg(Color::Blue).bold(),
        ),
        Span::styled(
            localized(
                japanese,
                ". Leave it empty to disable this command.",
                "です。空にするとこのコマンドを無効化します。",
            ),
            Style::default().fg(Color::Gray),
        ),
    ]))
    .wrap(Wrap { trim: false });
    frame.render_widget(help, chunks[4]);

    let cursor_x = chunks[3]
        .x
        .saturating_add(2)
        .saturating_add(value.chars().count() as u16);
    let cursor_y = chunks[3].y.saturating_add(1);
    if cursor_x < chunks[3].x + chunks[3].width.saturating_sub(1)
        && cursor_y < chunks[3].y + chunks[3].height.saturating_sub(1)
    {
        frame.set_cursor(cursor_x, cursor_y);
    }
}

pub fn run_interactive_init(cwd: &Path, package: Option<&Path>) -> Result<InitOptions, String> {
    enable_raw_mode().map_err(|e| format!("failed to enable raw mode: {e}"))?;
    stdout()
        .execute(EnterAlternateScreen)
        .map_err(|e| format!("failed to enter alternate screen: {e}"))?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))
        .map_err(|e| format!("failed to create terminal: {e}"))?;

    let root = package.map_or_else(|| cwd.to_path_buf(), |path| cwd.join(path));
    let mut state = WizardState::new(&root);
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
                KeyCode::Esc => {
                    if state.current_step == 0 {
                        state.cancelled = true;
                        return Ok(());
                    }
                    state.go_back();
                }
                KeyCode::Left => state.go_back(),
                KeyCode::Right => {
                    if state.is_final_step() {
                        state.toggle_at_cursor();
                        return Ok(());
                    }
                    state.advance();
                }
                KeyCode::Up if !state.is_text_input() => move_cursor(state, -1),
                KeyCode::Down if !state.is_text_input() => move_cursor(state, 1),
                KeyCode::Backspace if state.is_text_input() => {
                    if let Some(value) = state.text_value_mut() {
                        value.pop();
                    }
                }
                KeyCode::Char(ch) if state.is_text_input() => {
                    if let Some(value) = state.text_value_mut() {
                        value.push(ch);
                    }
                }
                KeyCode::Enter => {
                    if state.is_text_input() {
                        state.advance();
                    } else if state.is_final_step() {
                        state.toggle_at_cursor();
                        return Ok(());
                    } else {
                        state.toggle_at_cursor();
                    }
                }
                _ => {}
            }
        }
    }
}

fn move_cursor(state: &mut WizardState, direction: isize) {
    let items = state.current_items();
    if items.is_empty() {
        return;
    }
    let current = state.list_state.selected().unwrap_or(0) as isize;
    let len = items.len() as isize;
    let next = (current + direction).rem_euclid(len) as usize;
    state.list_state.select(Some(next));
}

fn detect_toolchains(root: &Path) -> Vec<ToolchainState> {
    let mut toolchains = Vec::new();
    for profile in toolchain_profiles() {
        if root.join(profile.metadata).exists() {
            toolchains.push(ToolchainState {
                type_check: profile.type_check.into(),
                lint: profile.lint.into(),
                test: profile.test.into(),
                profile,
            });
        }
    }
    toolchains
}

fn toolchain_profiles() -> Vec<ToolchainProfile> {
    vec![
        ToolchainProfile {
            lang: Lang::TypeScript,
            name: "Node.js",
            metadata: "package.json",
            type_check: "npm run typecheck",
            lint: "npm run lint",
            test: "npm test",
        },
        ToolchainProfile {
            lang: Lang::Python,
            name: "Python",
            metadata: "pyproject.toml",
            type_check: "uv run pyright",
            lint: "uv run ruff check",
            test: "uv run pytest",
        },
        ToolchainProfile {
            lang: Lang::Rust,
            name: "Rust",
            metadata: "Cargo.toml",
            type_check: "cargo check",
            lint: "cargo clippy",
            test: "cargo test",
        },
    ]
}

fn non_empty_command(command: &str) -> Option<String> {
    let trimmed = command.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn localized<'a>(japanese: bool, english: &'a str, japanese_text: &'a str) -> &'a str {
    if japanese {
        japanese_text
    } else {
        english
    }
}

fn ai_target_label(target: AiTarget) -> &'static str {
    match target {
        AiTarget::ClaudeCode => "Claude Code",
        AiTarget::CodexCli => "Codex CLI",
        AiTarget::Opencode => "Opencode",
        AiTarget::GithubCopilotCli => "GitHub Copilot",
    }
}

fn ai_target_description(target: AiTarget, japanese: bool) -> &'static str {
    match (target, japanese) {
        (AiTarget::ClaudeCode, false) => "Generate project instructions for Claude Code.",
        (AiTarget::CodexCli, false) => "Generate project instructions for Codex CLI.",
        (AiTarget::Opencode, false) => "Generate project instructions for Opencode.",
        (AiTarget::GithubCopilotCli, false) => {
            "Generate project instructions for GitHub Copilot CLI."
        }
        (AiTarget::ClaudeCode, true) => "Claude Code 向けのプロジェクト指示を生成します。",
        (AiTarget::CodexCli, true) => "Codex CLI 向けのプロジェクト指示を生成します。",
        (AiTarget::Opencode, true) => "Opencode 向けのプロジェクト指示を生成します。",
        (AiTarget::GithubCopilotCli, true) => {
            "GitHub Copilot CLI 向けのプロジェクト指示を生成します。"
        }
    }
}

fn category_label(category: AgentKitCategory, japanese: bool) -> &'static str {
    match (category, japanese) {
        (AgentKitCategory::Instructions, false) => "Instructions",
        (AgentKitCategory::Skills, false) => "Skills",
        (AgentKitCategory::Commands, false) => "Commands",
        (AgentKitCategory::Instructions, true) => "指示",
        (AgentKitCategory::Skills, true) => "スキル",
        (AgentKitCategory::Commands, true) => "コマンド",
    }
}

fn category_description(category: AgentKitCategory, japanese: bool) -> &'static str {
    match (category, japanese) {
        (AgentKitCategory::Instructions, false) => "Always-on project rules and coding guidance.",
        (AgentKitCategory::Skills, false) => "Reusable workflows for common project tasks.",
        (AgentKitCategory::Commands, false) => {
            "Executable prompts or commands for repeatable actions."
        }
        (AgentKitCategory::Instructions, true) => {
            "常に参照されるプロジェクトルールとコーディング指針です。"
        }
        (AgentKitCategory::Skills, true) => "共通作業で再利用できるワークフローです。",
        (AgentKitCategory::Commands, true) => {
            "繰り返し実行する作業向けのプロンプトまたはコマンドです。"
        }
    }
}
