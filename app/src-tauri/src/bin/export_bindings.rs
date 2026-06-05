#[path = "../dto/mod.rs"]
mod dto;

use dto::{
    LibraryDto, OpenedSaveDto, ScenarioMutationDto, ScenarioMutationResultDto, ValidationResultDto,
};
use ts_rs::{Config, TS};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env();

    LibraryDto::export_all(&config)?;
    OpenedSaveDto::export_all(&config)?;
    ScenarioMutationDto::export_all(&config)?;
    ScenarioMutationResultDto::export_all(&config)?;
    ValidationResultDto::export_all(&config)?;

    Ok(())
}
