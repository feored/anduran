use std::fmt::Display;

use crate::SaveString;
use crate::model::{PlayerColorsSet, PlayerSlotInfo, SupportedLanguage};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum GameVersion {
    #[default]
    SuccessionWars,
    PriceOfLoyalty,
    Resurrection,
}

impl From<u32> for GameVersion {
    fn from(value: u32) -> Self {
        match value {
            0 => GameVersion::SuccessionWars,
            1 => GameVersion::PriceOfLoyalty,
            2 => GameVersion::Resurrection,
            other => panic!("Unknown game version: {other}"),
        }
    }
}

impl From<GameVersion> for u32 {
    fn from(value: GameVersion) -> Self {
        match value {
            GameVersion::SuccessionWars => 0,
            GameVersion::PriceOfLoyalty => 1,
            GameVersion::Resurrection => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct WorldDate {
    pub day: u32,
    pub week: u32,
    pub month: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Difficulty {
    #[default]
    Easy,
    Normal,
    Hard,
    Expert,
    Impossible,
    Unknown(u8),
}

impl From<u8> for Difficulty {
    fn from(value: u8) -> Self {
        match value {
            0 => Difficulty::Easy,
            1 => Difficulty::Normal,
            2 => Difficulty::Hard,
            3 => Difficulty::Expert,
            4 => Difficulty::Impossible,
            other => Difficulty::Unknown(other),
        }
    }
}

impl From<Difficulty> for u8 {
    fn from(value: Difficulty) -> Self {
        match value {
            Difficulty::Easy => 0,
            Difficulty::Normal => 1,
            Difficulty::Hard => 2,
            Difficulty::Expert => 3,
            Difficulty::Impossible => 4,
            Difficulty::Unknown(other) => other,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VictoryConditionKind {
    #[default]
    DefeatEveryone,
    CaptureTown,
    KillHero,
    ObtainArtifact,
    DefeatOtherSide,
    CollectEnoughGold,
    Unknown(u8),
}

impl From<u8> for VictoryConditionKind {
    fn from(value: u8) -> Self {
        match value {
            0 => VictoryConditionKind::DefeatEveryone,
            1 => VictoryConditionKind::CaptureTown,
            2 => VictoryConditionKind::KillHero,
            3 => VictoryConditionKind::ObtainArtifact,
            4 => VictoryConditionKind::DefeatOtherSide,
            5 => VictoryConditionKind::CollectEnoughGold,
            other => VictoryConditionKind::Unknown(other),
        }
    }
}

impl From<VictoryConditionKind> for u8 {
    fn from(value: VictoryConditionKind) -> Self {
        match value {
            VictoryConditionKind::DefeatEveryone => 0,
            VictoryConditionKind::CaptureTown => 1,
            VictoryConditionKind::KillHero => 2,
            VictoryConditionKind::ObtainArtifact => 3,
            VictoryConditionKind::DefeatOtherSide => 4,
            VictoryConditionKind::CollectEnoughGold => 5,
            VictoryConditionKind::Unknown(other) => other,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VictoryConditionData {
    pub kind: VictoryConditionKind,
    pub comp_also_wins: bool,
    pub allow_normal_victory: bool,
    pub params: [u16; 2],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LossConditionKind {
    #[default]
    LossEverything,
    LossTown,
    LossHero,
    LossOutOfTime,
    Unknown(u8),
}

impl From<u8> for LossConditionKind {
    fn from(value: u8) -> Self {
        match value {
            0 => LossConditionKind::LossEverything,
            1 => LossConditionKind::LossTown,
            2 => LossConditionKind::LossHero,
            3 => LossConditionKind::LossOutOfTime,
            other => LossConditionKind::Unknown(other),
        }
    }
}

impl From<LossConditionKind> for u8 {
    fn from(value: LossConditionKind) -> Self {
        match value {
            LossConditionKind::LossEverything => 0,
            LossConditionKind::LossTown => 1,
            LossConditionKind::LossHero => 2,
            LossConditionKind::LossOutOfTime => 3,
            LossConditionKind::Unknown(other) => other,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LossConditionData {
    pub kind: LossConditionKind,
    pub params: [u16; 2],
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MapInfo {
    pub filename: SaveString,
    pub name: SaveString,
    pub description: SaveString,
    pub width: u16,
    pub height: u16,
    pub difficulty: Difficulty,
    pub player_slots: Vec<PlayerSlotInfo>,
    pub kingdom_colors: PlayerColorsSet,
    pub colors_available_for_humans: PlayerColorsSet,
    pub colors_available_for_comp: PlayerColorsSet,
    pub colors_of_random_races: PlayerColorsSet,
    pub victory_condition: VictoryConditionData,
    pub loss_condition: LossConditionData,
    pub timestamp: u32,
    pub start_with_hero_in_first_castle: bool,
    pub version: GameVersion,
    pub world_date: WorldDate,
    pub main_language: SupportedLanguage,
}

impl Display for MapInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "map filename: {}", self.filename)?;
        writeln!(f, "map name: {}", self.name)?;
        writeln!(f, "description: {}", self.description)?;
        writeln!(f, "width: {}", self.width)?;
        writeln!(f, "height: {}", self.height)?;
        writeln!(f, "difficulty: {:?}", self.difficulty)?;

        for slot in &self.player_slots {
            writeln!(f, "{slot}")?;
        }

        writeln!(f, "kingdom colors: {}", self.kingdom_colors)?;
        writeln!(
            f,
            "colors available for humans: {}",
            self.colors_available_for_humans
        )?;
        writeln!(
            f,
            "colors available for computer: {}",
            self.colors_available_for_comp
        )?;
        writeln!(f, "colors of random races: {}", self.colors_of_random_races)?;
        writeln!(f, "victory condition: {:?}", self.victory_condition)?;
        writeln!(f, "loss condition: {:?}", self.loss_condition)?;
        writeln!(f, "timestamp: {}", self.timestamp)?;
        writeln!(
            f,
            "start with hero in first castle: {}",
            self.start_with_hero_in_first_castle
        )?;
        writeln!(f, "version: {:?}", self.version)?;
        writeln!(
            f,
            "world date: month {}, week {}, day {}",
            self.world_date.month, self.world_date.week, self.world_date.day
        )?;
        writeln!(f, "main language: {}", self.main_language)?;
        Ok(())
    }
}
