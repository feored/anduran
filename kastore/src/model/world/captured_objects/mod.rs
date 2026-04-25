use std::fmt::Display;

use crate::model::header::player::PlayerColor;
use crate::model::world::heroes::army::{MonsterType, Troop};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CapturedObject {
    pub object_type: u16,
    pub color: PlayerColor,
    pub guardians: Troop,
}

impl Display for CapturedObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let guardians = if is_unguarded(&self.guardians) {
            "Unguarded".to_string()
        } else {
            self.guardians.to_string()
        };

        write!(
            f,
            "object=0x{:04X}, color={}, guardians={}",
            self.object_type, self.color, guardians
        )
    }
}

fn is_unguarded(troop: &Troop) -> bool {
    troop.count == 0 || matches!(troop.monster, MonsterType::Unknown(_))
}
