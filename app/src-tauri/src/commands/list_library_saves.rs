use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use kastore::{load_summary_with_options, LoadOptions};

use crate::bridge::{parser_diagnostics, save_summary_dto};
use crate::dto::{DiagnosticDto, LibraryDto, LibrarySaveEntryDto, SaveSummaryDto};

#[tauri::command]
pub fn list_library_saves(path: Option<String>) -> Result<LibraryDto, String> {
    let library_path = library_path(path)?;
    let read_dir =
        fs::read_dir(&library_path).map_err(|error| format!("Could not read save folder: {error}"))?;

    let mut entries: Vec<_> = read_dir
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| is_save_file(path))
        .map(library_entry)
        .collect();

    entries.sort_by(|left, right| {
        right
            .modified_timestamp
            .cmp(&left.modified_timestamp)
            .then_with(|| left.file_name.cmp(&right.file_name))
    });

    Ok(LibraryDto { entries })
}

fn library_path(path: Option<String>) -> Result<PathBuf, String> {
    path.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| PathBuf::from(trimmed))
    })
    .or_else(default_library_path)
    .ok_or_else(|| "No default save folder is available on this platform.".to_string())
}

fn library_entry(path: PathBuf) -> LibrarySaveEntryDto {
    let mut diagnostics = Vec::new();
    let metadata = fs::metadata(&path).ok();
    let size_bytes = metadata.as_ref().map_or(0, fs::Metadata::len);
    let modified_timestamp = metadata
        .and_then(|metadata| metadata.modified().ok())
        .and_then(system_time_timestamp);
    let (status, summary) = save_summary(&path, &mut diagnostics);

    LibrarySaveEntryDto {
        path: path_to_string(&path),
        file_name: path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default(),
        size_bytes,
        modified_timestamp,
        status,
        summary,
        diagnostics,
    }
}

fn save_summary(
    path: &Path,
    diagnostics: &mut Vec<DiagnosticDto>,
) -> (String, Option<SaveSummaryDto>) {
    let bytes = match fs::read(path) {
        Ok(bytes) => bytes,
        Err(error) => {
            diagnostics.push(library_diagnostic(
                "error",
                format!("Could not read save file: {error}"),
            ));
            return ("unreadable".to_string(), None);
        }
    };

    match load_summary_with_options(&bytes, &LoadOptions::permissive()) {
        Ok(report) => {
            diagnostics.extend(parser_diagnostics(&report.diagnostics));
            ("ready".to_string(), Some(save_summary_dto(&report.value)))
        }
        Err(error) => {
            diagnostics.push(library_diagnostic(
                "error",
                format!("Could not parse save summary: {error}"),
            ));
            ("parseError".to_string(), None)
        }
    }
}

fn is_save_file(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }

    path.extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| matches!(extension.to_ascii_lowercase().as_str(), "sav" | "savc"))
        .unwrap_or(false)
}

fn system_time_timestamp(value: SystemTime) -> Option<u64> {
    value
        .duration_since(UNIX_EPOCH)
        .ok()
        .and_then(|duration| u64::try_from(duration.as_millis()).ok())
}

fn path_to_string(path: &PathBuf) -> String {
    path.to_string_lossy().into_owned()
}

fn library_diagnostic(
    severity: impl Into<String>,
    message: impl Into<String>,
) -> DiagnosticDto {
    DiagnosticDto {
        severity: severity.into(),
        kind: "library".to_string(),
        section: "library".to_string(),
        field: None,
        offset: None,
        message: message.into(),
    }
}

#[cfg(windows)]
fn default_library_path() -> Option<PathBuf> {
    std::env::var_os("APPDATA")
        .map(PathBuf::from)
        .map(|path| path.join("fheroes2").join("files").join("save"))
}

#[cfg(not(windows))]
fn default_library_path() -> Option<PathBuf> {
    None
}
