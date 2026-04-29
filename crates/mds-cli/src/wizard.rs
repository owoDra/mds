use dialoguer::{Confirm, MultiSelect, Select};
use mds_core::{
    AgentKitCategory, AiTarget, InitOptions, PythonTool, RustTool, TypeScriptTool,
};

pub fn run_interactive_init() -> Result<InitOptions, String> {
    println!("mds init — interactive setup\n");

    let ai_only = Select::new()
        .with_prompt("What do you want to initialize?")
        .items(&["Project files + AI agent kit", "AI agent kit only"])
        .default(0)
        .interact()
        .map_err(|e| format!("prompt error: {e}"))?
        == 1;

    let ts_tools = if !ai_only {
        select_ts_tools()?
    } else {
        Vec::new()
    };

    let py_tools = if !ai_only {
        select_py_tools()?
    } else {
        Vec::new()
    };

    let rs_tools = if !ai_only {
        select_rs_tools()?
    } else {
        Vec::new()
    };

    let targets = select_ai_targets()?;

    let categories = if !targets.is_empty() {
        select_categories()?
    } else {
        Vec::new()
    };

    let install_project_deps = if !ai_only {
        Confirm::new()
            .with_prompt("Install project dependencies (npm install / cargo fetch / uv sync)?")
            .default(false)
            .interact()
            .map_err(|e| format!("prompt error: {e}"))?
    } else {
        false
    };

    let install_toolchains = if !ai_only {
        Confirm::new()
            .with_prompt("Check and install required toolchains?")
            .default(false)
            .interact()
            .map_err(|e| format!("prompt error: {e}"))?
    } else {
        false
    };

    let install_ai_cli = if !targets.is_empty() {
        Confirm::new()
            .with_prompt("Check and install AI CLI tools?")
            .default(false)
            .interact()
            .map_err(|e| format!("prompt error: {e}"))?
    } else {
        false
    };

    let options = InitOptions {
        ai_only,
        yes: false,
        force: false,
        targets,
        categories,
        ts_tools,
        py_tools,
        rs_tools,
        install_project_deps,
        install_toolchains,
        install_ai_cli,
    };

    println!();
    print_summary(&options);

    let confirmed = Confirm::new()
        .with_prompt("Apply this plan?")
        .default(true)
        .interact()
        .map_err(|e| format!("prompt error: {e}"))?;

    if confirmed {
        Ok(InitOptions {
            yes: true,
            ..options
        })
    } else {
        Err("init cancelled by user".to_string())
    }
}

fn select_ts_tools() -> Result<Vec<TypeScriptTool>, String> {
    let items = &[
        ("eslint", "Linter"),
        ("prettier", "Formatter"),
        ("biome", "Linter + Formatter"),
        ("vitest", "Test runner"),
        ("jest", "Test runner"),
    ];
    let defaults = &[true, true, false, true, false];

    let selections = MultiSelect::new()
        .with_prompt("TypeScript tools (Space to toggle, Enter to confirm)")
        .items(
            &items
                .iter()
                .map(|(name, desc)| format!("{name} — {desc}"))
                .collect::<Vec<_>>(),
        )
        .defaults(defaults)
        .interact()
        .map_err(|e| format!("prompt error: {e}"))?;

    let all_tools = [
        TypeScriptTool::Eslint,
        TypeScriptTool::Prettier,
        TypeScriptTool::Biome,
        TypeScriptTool::Vitest,
        TypeScriptTool::Jest,
    ];

    let selected: Vec<TypeScriptTool> = selections.iter().map(|&i| all_tools[i]).collect();
    if selected.contains(&TypeScriptTool::Vitest) && selected.contains(&TypeScriptTool::Jest) {
        return Err("Cannot select both vitest and jest. Please choose one.".to_string());
    }
    Ok(selected)
}

fn select_py_tools() -> Result<Vec<PythonTool>, String> {
    let items = &[
        ("ruff", "Linter + Formatter"),
        ("black", "Formatter"),
        ("pytest", "Test runner"),
        ("unittest", "Test runner (stdlib)"),
    ];
    let defaults = &[true, false, true, false];

    let selections = MultiSelect::new()
        .with_prompt("Python tools (Space to toggle, Enter to confirm)")
        .items(
            &items
                .iter()
                .map(|(name, desc)| format!("{name} — {desc}"))
                .collect::<Vec<_>>(),
        )
        .defaults(defaults)
        .interact()
        .map_err(|e| format!("prompt error: {e}"))?;

    let all_tools = [
        PythonTool::Ruff,
        PythonTool::Black,
        PythonTool::Pytest,
        PythonTool::Unittest,
    ];

    let selected: Vec<PythonTool> = selections.iter().map(|&i| all_tools[i]).collect();
    if selected.contains(&PythonTool::Pytest) && selected.contains(&PythonTool::Unittest) {
        return Err("Cannot select both pytest and unittest. Please choose one.".to_string());
    }
    Ok(selected)
}

fn select_rs_tools() -> Result<Vec<RustTool>, String> {
    let items = &[
        ("rustfmt", "Formatter"),
        ("clippy", "Linter"),
        ("cargo test", "Test runner"),
        ("nextest", "Test runner (faster)"),
    ];
    let defaults = &[true, true, true, false];

    let selections = MultiSelect::new()
        .with_prompt("Rust tools (Space to toggle, Enter to confirm)")
        .items(
            &items
                .iter()
                .map(|(name, desc)| format!("{name} — {desc}"))
                .collect::<Vec<_>>(),
        )
        .defaults(defaults)
        .interact()
        .map_err(|e| format!("prompt error: {e}"))?;

    let all_tools = [
        RustTool::Rustfmt,
        RustTool::Clippy,
        RustTool::CargoTest,
        RustTool::Nextest,
    ];

    let selected: Vec<RustTool> = selections.iter().map(|&i| all_tools[i]).collect();
    if selected.contains(&RustTool::CargoTest) && selected.contains(&RustTool::Nextest) {
        return Err("Cannot select both cargo-test and nextest. Please choose one.".to_string());
    }
    Ok(selected)
}

fn select_ai_targets() -> Result<Vec<AiTarget>, String> {
    let items = &[
        ("Claude Code", "Anthropic Claude Code"),
        ("Codex CLI", "OpenAI Codex CLI"),
        ("Opencode", "Opencode AI"),
        ("GitHub Copilot", "GitHub Copilot CLI"),
    ];
    let defaults = &[true, true, true, true];

    let selections = MultiSelect::new()
        .with_prompt("AI agent targets (Space to toggle, Enter to confirm)")
        .items(
            &items
                .iter()
                .map(|(name, desc)| format!("{name} — {desc}"))
                .collect::<Vec<_>>(),
        )
        .defaults(defaults)
        .interact()
        .map_err(|e| format!("prompt error: {e}"))?;

    let all_targets = [
        AiTarget::ClaudeCode,
        AiTarget::CodexCli,
        AiTarget::Opencode,
        AiTarget::GithubCopilotCli,
    ];

    Ok(selections.iter().map(|&i| all_targets[i]).collect())
}

fn select_categories() -> Result<Vec<AgentKitCategory>, String> {
    let items = &[
        ("instructions", "Agent instructions and rules"),
        ("skills", "Agent skill definitions"),
        ("commands", "Agent command shortcuts"),
    ];
    let defaults = &[true, true, true];

    let selections = MultiSelect::new()
        .with_prompt("Agent kit categories (Space to toggle, Enter to confirm)")
        .items(
            &items
                .iter()
                .map(|(name, desc)| format!("{name} — {desc}"))
                .collect::<Vec<_>>(),
        )
        .defaults(defaults)
        .interact()
        .map_err(|e| format!("prompt error: {e}"))?;

    let all_categories = [
        AgentKitCategory::Instructions,
        AgentKitCategory::Skills,
        AgentKitCategory::Commands,
    ];

    Ok(selections.iter().map(|&i| all_categories[i]).collect())
}

fn print_summary(options: &InitOptions) {
    println!("--- Init Plan ---");

    if options.ai_only {
        println!("Mode: AI agent kit only");
    } else {
        println!("Mode: Full project initialization");
    }

    if !options.ai_only {
        println!(
            "TypeScript tools: {}",
            if options.ts_tools.is_empty() {
                "none".to_string()
            } else {
                options
                    .ts_tools
                    .iter()
                    .map(|t| format!("{t:?}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            }
        );
        println!(
            "Python tools: {}",
            if options.py_tools.is_empty() {
                "none".to_string()
            } else {
                options
                    .py_tools
                    .iter()
                    .map(|t| format!("{t:?}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            }
        );
        println!(
            "Rust tools: {}",
            if options.rs_tools.is_empty() {
                "none".to_string()
            } else {
                options
                    .rs_tools
                    .iter()
                    .map(|t| format!("{t:?}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            }
        );
    }

    if !options.targets.is_empty() {
        println!(
            "AI targets: {}",
            options
                .targets
                .iter()
                .map(|t| t.key())
                .collect::<Vec<_>>()
                .join(", ")
        );
        println!(
            "Agent kit categories: {}",
            options
                .categories
                .iter()
                .map(|c| c.key())
                .collect::<Vec<_>>()
                .join(", ")
        );
    } else {
        println!("AI targets: none");
    }

    if options.install_project_deps {
        println!("Install project deps: yes");
    }
    if options.install_toolchains {
        println!("Install toolchains: yes");
    }
    if options.install_ai_cli {
        println!("Install AI CLIs: yes");
    }

    println!("-----------------");
}
