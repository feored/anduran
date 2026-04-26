use std::path::Path;

use crate::dto::OpenedSaveDto;
use crate::save::open_save_file;

#[tauri::command]
pub fn open_save(path: String) -> Result<OpenedSaveDto, String> {
    open_save_file(Path::new(&path))
}
