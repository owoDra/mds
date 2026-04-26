use std::path::PathBuf;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum Severity {
    Warning,
    Error,
}

#[derive(Debug, Clone)]
pub(crate) struct Diagnostic {
    pub(crate) severity: Severity,
    path: Option<PathBuf>,
    line: Option<usize>,
    message: String,
}

impl Diagnostic {
    pub(crate) fn warning(path: Option<PathBuf>, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warning,
            path,
            line: None,
            message: message.into(),
        }
    }

    pub(crate) fn error(path: Option<PathBuf>, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            path,
            line: None,
            message: message.into(),
        }
    }

    pub(crate) fn at_line(mut self, line: usize) -> Self {
        self.line = Some(line);
        self
    }

    pub(crate) fn render(&self) -> String {
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
pub(crate) struct RunState {
    pub(crate) stdout: String,
    pub(crate) diagnostics: Vec<Diagnostic>,
    pub(crate) generated: Vec<PathBuf>,
}

impl RunState {
    pub(crate) fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == Severity::Error)
    }
}
