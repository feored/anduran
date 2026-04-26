use anduran_lib::dto::OpenedSaveDto;
use ts_rs::{Config, TS};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env();

    OpenedSaveDto::export_all(&config)?;

    Ok(())
}
