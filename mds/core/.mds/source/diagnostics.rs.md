# src/diagnostics.rs

## Purpose

Migrated implementation source for `src/diagnostics.rs`.

## Contract

- Preserve the behavior of the pre-migration Rust source.
- This file is synchronized into `.build/rust/mds/core/src/diagnostics.rs`.

## Source

````rs
use std::path::PathBuf;
````

````rs
#[derive(Debug, Clone, Copy, Eq, PartialEq)]

pub enum Severity {
    Warning,
    Error,
}

#[derive(Debug, Clone)]
````

````rs
pub struct Diagnostic {
    pub severity: Severity,
    pub path: Option<PathBuf>,
    pub line: Option<usize>,
    pub message: String,
}
````

````rs
impl Diagnostic {
    pub fn warning(path: Option<PathBuf>, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warning,
            path,
            line: None,
            message: message.into(),
        }
    }

    pub fn error(path: Option<PathBuf>, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            path,
            line: None,
            message: message.into(),
        }
    }

    pub fn at_line(mut self, line: usize) -> Self {
        self.line = Some(line);
        self
    }

    pub fn render(&self) -> String {
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
````

````rs
pub struct RunState {
    pub stdout: String,
    pub diagnostics: Vec<Diagnostic>,
    pub generated: Vec<PathBuf>,
    pub environment_missing: bool,
}
````

````rs
impl RunState {
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == Severity::Error)
    }
}
````