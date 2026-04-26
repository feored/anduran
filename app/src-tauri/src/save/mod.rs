use std::path::Path;

use kastore::{load_with_options, LoadOptions};

use crate::bridge::opened_save_from_report;
use crate::dto::OpenedSaveDto;

pub fn open_save_file(path: &Path) -> Result<OpenedSaveDto, String> {
    let bytes = std::fs::read(path).map_err(|error| {
        format!("Could not read {}: {error}", path.display())
    })?;

    let report = load_with_options(&bytes, &LoadOptions::permissive()).map_err(|error| {
        format!("Could not parse {}: {error}", path.display())
    })?;

    Ok(opened_save_from_report(path, report))
}
