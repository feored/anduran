use crate::Error;
use crate::model::{SaveGame, SaveHeader};
use crate::version::{SaveVersion, profile_for};

#[derive(Debug, Clone, PartialEq, Eq)]
enum GameType {
    Standard,
    Campaign,
    Hotseat,
    Unknown(String),
}

impl GameType {
    fn extension(&self) -> &str {
        match self {
            GameType::Standard => "sav",
            GameType::Campaign => "savc",
            GameType::Hotseat => "savh",
            GameType::Unknown(_) => "savm",
        }
    }

    fn from_extension(extension: &str) -> GameType {
        match extension {
            "sav" => GameType::Standard,
            "savc" => GameType::Campaign,
            "savh" => GameType::Hotseat,
            _ => GameType::Unknown(extension.to_string()),
        }
    }
}

pub fn load(bytes: &[u8]) -> std::result::Result<SaveGame, Error> {
    let profile = crate::version::LATEST_PROFILE;
    let container = crate::container::decode_container(profile.container_revision, bytes)?;

    Ok(SaveGame {
        source_version: container.save_version,
        header: SaveHeader {
            requires_pol: container.header.requires_pol,
        },
        map_info: container.header.map_info,
    })
}

pub fn save(save_game: &SaveGame) -> std::result::Result<Vec<u8>, Error> {
    save_as(save_game, SaveVersion::FORMAT_VERSION_1111_RELEASE)
}

pub fn save_as(save_game: &SaveGame, target: SaveVersion) -> std::result::Result<Vec<u8>, Error> {
    let Some(profile) = profile_for(target) else {
        return Err(Error::UnsupportedSaveVersion {
            version: target.as_u16(),
        });
    };
    let _ = (save_game, profile);
    Err(Error::NotImplemented {
        feature: "save encode",
    })
}

#[cfg(test)]
mod tests;
