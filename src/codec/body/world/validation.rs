use crate::model::header::player::PlayerColor;
use crate::model::world::World;
use crate::model::world::castles::Castle;
use crate::model::world::kingdoms::KINGDOM_SLOT_COUNT;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct WorldValidationIssue {
    pub(super) field: &'static str,
    pub(super) message: &'static str,
}

pub(super) fn validate_kingdoms(world: &World) -> std::result::Result<(), WorldValidationIssue> {
    if world.kingdoms.len() != KINGDOM_SLOT_COUNT {
        return Err(WorldValidationIssue {
            field: "world kingdoms",
            message: "kingdom table must contain exactly 7 slots",
        });
    }

    for (slot_index, kingdom) in world.kingdoms.iter().enumerate() {
        if !kingdom_slot_accepts_color(slot_index, kingdom.color) {
            return Err(WorldValidationIssue {
                field: "kingdom colors",
                message: "kingdom slot colors must match fheroes2 slot order or be None for inactive slots",
            });
        }
    }

    let mut seen_hero_ids = Vec::new();
    for kingdom in &world.kingdoms {
        for hero_id in &kingdom.hero_ids {
            let Some(hero) = world.heroes.iter().find(|hero| hero.id == *hero_id) else {
                return Err(WorldValidationIssue {
                    field: "kingdom heroes",
                    message: "kingdom hero references must point to decoded heroes",
                });
            };

            if hero.color_base != kingdom.color {
                return Err(WorldValidationIssue {
                    field: "kingdom heroes",
                    message: "kingdom hero references must match the referenced hero color",
                });
            }

            seen_hero_ids.push(*hero_id);
        }
    }

    for hero in &world.heroes {
        if hero.color_base == PlayerColor::None {
            continue;
        }

        if !seen_hero_ids.contains(&hero.id) {
            return Err(WorldValidationIssue {
                field: "kingdom heroes",
                message: "every non-neutral hero must appear in exactly one kingdom hero list",
            });
        }
    }

    for kingdom in &world.kingdoms {
        for castle_index in &kingdom.castle_indexes {
            let Some(castle) = find_castle_by_index(world, *castle_index) else {
                return Err(WorldValidationIssue {
                    field: "kingdom castles",
                    message: "kingdom castle references must point to decoded castles",
                });
            };

            if castle.color_base != kingdom.color {
                return Err(WorldValidationIssue {
                    field: "kingdom castles",
                    message: "kingdom castle references must match the referenced castle color",
                });
            }
        }
    }

    Ok(())
}

fn kingdom_slot_accepts_color(slot_index: usize, color: PlayerColor) -> bool {
    if slot_index + 1 == KINGDOM_SLOT_COUNT {
        return color == PlayerColor::None;
    }

    color == PlayerColor::None
        || u8::try_from(slot_index)
            .ok()
            .and_then(PlayerColor::from_index)
            == Some(color)
}

fn find_castle_by_index(world: &World, castle_index: i32) -> Option<&Castle> {
    world.castles.iter().find(|castle| {
        castle_index_from_map_position(world.width, world.height, castle) == Some(castle_index)
    })
}

pub(super) fn castle_index_from_map_position(
    width: i32,
    height: i32,
    castle: &Castle,
) -> Option<i32> {
    let x = i32::from(castle.map_position.x);
    let y = i32::from(castle.map_position.y);

    if width <= 0 || height <= 0 || x < 0 || y < 0 || x >= width || y >= height {
        return None;
    }

    y.checked_mul(width)?.checked_add(x)
}
