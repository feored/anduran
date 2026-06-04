use kastore::{
    set_duplicated_map_info_text, validate_save_game, DuplicatedMapInfoTextField, SaveString,
};
use tauri::State;

use crate::bridge::{scenario_dto, validation_result};
use crate::document::ManagedSaveState;
use crate::dto::{ScenarioMutationDto, ScenarioMutationResultDto};

#[tauri::command]
pub fn mutate_scenario(
    mutation: ScenarioMutationDto,
    state: State<'_, ManagedSaveState>,
) -> Result<ScenarioMutationResultDto, String> {
    let mut state = state.lock()?;
    let save_state = state
        .as_mut()
        .ok_or_else(|| "No save file is currently open.".to_string())?;

    let (field, text) = match mutation {
        ScenarioMutationDto::SetName { text } => (DuplicatedMapInfoTextField::Name, text),
        ScenarioMutationDto::SetFileName { text } => (DuplicatedMapInfoTextField::Filename, text),
        ScenarioMutationDto::SetDescription { text } => {
            (DuplicatedMapInfoTextField::Description, text)
        }
    };

    let new_value = SaveString::from(text);
    let already_current = match field {
        DuplicatedMapInfoTextField::Name => {
            save_state.save.header.file_info.name == new_value
                && save_state.save.settings.current_map_info.name == new_value
        }
        DuplicatedMapInfoTextField::Filename => {
            save_state.save.header.file_info.filename == new_value
                && save_state.save.settings.current_map_info.filename == new_value
        }
        DuplicatedMapInfoTextField::Description => {
            save_state.save.header.file_info.description == new_value
                && save_state.save.settings.current_map_info.description == new_value
        }
    };

    if already_current {
        return Ok(ScenarioMutationResultDto {
            scenario: scenario_dto(&save_state.save),
            dirty: save_state.dirty,
            revision: save_state.revision,
            validation: validation_result(
                save_state.revision,
                validate_save_game(&save_state.save).err(),
            ),
        });
    }

    set_duplicated_map_info_text(&mut save_state.save, field, new_value);
    save_state.mark_changed();

    Ok(ScenarioMutationResultDto {
        scenario: scenario_dto(&save_state.save),
        dirty: save_state.dirty,
        revision: save_state.revision,
        validation: validation_result(
            save_state.revision,
            validate_save_game(&save_state.save).err(),
        ),
    })
}
