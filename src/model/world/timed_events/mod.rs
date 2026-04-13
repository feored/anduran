use std::fmt::Display;

use crate::SaveString;
use crate::model::header::player::PlayerColorsSet;
use crate::model::world::Funds;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TimedEvent {
    pub resources: Funds,
    pub is_applicable_for_ai_players: bool,
    pub first_occurrence_day: u32,
    pub repeat_period_in_days: u32,
    pub colors: PlayerColorsSet,
    pub message: SaveString,
    pub title: SaveString,
}

impl Display for TimedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "title={}, day={}, repeat={}, ai={}, colors={}, resources={}",
            brief_save_string(&self.title, 48),
            self.first_occurrence_day,
            self.repeat_period_in_days,
            self.is_applicable_for_ai_players,
            self.colors,
            brief_funds(&self.resources)
        )?;

        if !self.message.is_empty() {
            write!(f, ", message={}", brief_save_string(&self.message, 72))?;
        }

        Ok(())
    }
}

fn brief_funds(funds: &Funds) -> String {
    let mut parts = Vec::new();

    for (value, name) in [
        (funds.wood, "wood"),
        (funds.mercury, "mercury"),
        (funds.ore, "ore"),
        (funds.sulfur, "sulfur"),
        (funds.crystal, "crystal"),
        (funds.gems, "gems"),
        (funds.gold, "gold"),
    ] {
        if value != 0 {
            parts.push(format!("{value} {name}"));
        }
    }

    if parts.is_empty() {
        "none".to_string()
    } else {
        parts.join(", ")
    }
}

fn brief_save_string(value: &SaveString, max_chars: usize) -> String {
    let single_line = value.to_string_lossy().replace(['\r', '\n'], " ");
    let total_chars = single_line.chars().count();
    let mut shortened: String = single_line.chars().take(max_chars).collect();
    if total_chars > max_chars {
        shortened.push_str("...");
    }

    format!("{shortened:?}")
}
