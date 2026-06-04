use tauri::State;

use crate::document::ManagedSaveState;

#[tauri::command]
pub fn close_open_save(state: State<'_, ManagedSaveState>) -> Result<(), String> {
    let mut state = state.lock()?;
    *state = None;

    Ok(())
}
