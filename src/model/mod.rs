//! Public save model types.

mod header;
mod save_game;
mod world;

pub use header::{
    Difficulty, GameType, GameVersion, LossConditionData, LossConditionKind, MapInfo, PlayerColor,
    PlayerColorsSet, PlayerSlotInfo, PlayerSlotView, Race, SaveHeader, SupportedLanguage,
    VictoryConditionData, VictoryConditionKind, WorldDate,
};
pub use save_game::{BodyCompressionHeader, SaveGame};
pub use world::{
    Army, Artifact, ArtifactID, Castle, CastleBuilding, CastleBuildingSet, CastleDwellingTier,
    CastleDwellings, CastleModeSet, Direction, DirectionSet, Hero, HeroBase, HeroID, HeroModeSet,
    IndexObject, LayerType, MageGuild, MapPosition, MonsterType, ObjectPart, Path, Point,
    PrimarySkills, RouteStep, SecondarySkill, Skill, SkillLevel, Spell, Tile, Troop, World,
};
