mod map_info;
mod player;
mod save_game;

pub use map_info::{
    Difficulty, LossConditionData, LossConditionKind, MapInfo, VictoryConditionData,
    VictoryConditionKind,
};
pub use player::{PlayerColor, PlayerColorsSet, PlayerSlotInfo, Race};
pub use save_game::{SaveGame, SaveHeader};
