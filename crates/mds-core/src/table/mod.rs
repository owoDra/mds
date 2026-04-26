use std::collections::HashMap;
use std::path::Path;

use crate::diagnostics::{Diagnostic, RunState};

pub(crate) fn parse_table(
    section: &str,
    required: &[&str],
    path: &Path,
    state: &mut RunState,
) -> Option<Vec<HashMap<String, String>>> {
    let lines = section.lines().collect::<Vec<_>>();
    let mut saw_table = false;
    for idx in 0..lines.len().saturating_sub(1) {
        if !lines[idx].trim_start().starts_with('|') || !lines[idx + 1].contains("---") {
            continue;
        }
        saw_table = true;
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
    if saw_table {
        state.diagnostics.push(Diagnostic::error(
            Some(path.to_path_buf()),
            format!("table is missing required columns: {}", required.join(", ")),
        ));
    }
    None
}

pub(crate) fn split_table_row(line: &str) -> Vec<String> {
    line.trim()
        .trim_matches('|')
        .split('|')
        .map(|cell| cell.trim().to_string())
        .collect()
}
