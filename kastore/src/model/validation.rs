use crate::Error;
use crate::model::header::game_type::GameType;
use crate::model::header::map_info::MapInfo;
use crate::model::header::player::PlayerColor;
use crate::model::save_game::SaveGame;
use crate::model::settings::{Settings, SettingsFocusKind};
use crate::model::world::castles::Castle;
use crate::model::world::kingdoms::KINGDOM_SLOT_COUNT;
use crate::model::world::{IndexObject, MapPosition, World};

const MAX_WORLD_TILES: usize = 1_000_000;

/// Model-level validation failure.
///
/// Diagnostics describe odd bytes observed while parsing. Validation issues
/// describe an incoherent in-memory model that should not be encoded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValidationIssue {
    pub field: &'static str,
    pub message: &'static str,
}

impl From<ValidationIssue> for Error {
    fn from(issue: ValidationIssue) -> Self {
        Error::InvalidModel {
            field: issue.field,
            message: issue.message,
        }
    }
}

pub fn validate_save_game(save_game: &SaveGame) -> std::result::Result<(), ValidationIssue> {
    validate_header_settings(save_game)?;
    validate_campaign_data(save_game)?;
    validate_settings(&save_game.settings)?;
    let tile_count = validate_world_shape(&save_game.world)?;
    validate_world_with_tile_count(&save_game.world, tile_count)?;
    validate_settings_focus(&save_game.settings, tile_count)
}

pub fn validate_world(world: &World) -> std::result::Result<(), ValidationIssue> {
    let tile_count = validate_world_shape(world)?;
    validate_world_with_tile_count(world, tile_count)
}

fn validate_world_shape(world: &World) -> std::result::Result<usize, ValidationIssue> {
    let tile_count = expected_tile_count(world.width, world.height).ok_or_else(|| {
        issue(
            "world dimensions",
            "world dimensions must be positive and fit parser limits",
        )
    })?;
    if world.tiles.len() != tile_count {
        return Err(issue(
            "world tiles",
            "world tile count must match width * height",
        ));
    }

    Ok(tile_count)
}

fn validate_world_with_tile_count(
    world: &World,
    tile_count: usize,
) -> std::result::Result<(), ValidationIssue> {
    for (index, tile) in world.tiles.iter().enumerate() {
        if tile.index != index as i32 {
            return Err(issue(
                "world tiles",
                "tile indexes must match serialized tile order",
            ));
        }
    }

    validate_kingdoms(world)?;
    validate_world_references(world, tile_count)
}

fn validate_header_settings(save_game: &SaveGame) -> std::result::Result<(), ValidationIssue> {
    validate_map_info(&save_game.header.file_info)?;
    validate_map_info(&save_game.settings.current_map_info)?;

    if save_game.header.game_type != save_game.settings.game_type {
        return Err(issue(
            "settings game type",
            "header game type must match settings game type",
        ));
    }

    if i32::from(save_game.header.file_info.width) != save_game.world.width
        || i32::from(save_game.header.file_info.height) != save_game.world.height
    {
        return Err(issue(
            "header map info",
            "header map dimensions must match world dimensions",
        ));
    }

    if i32::from(save_game.settings.current_map_info.width) != save_game.world.width
        || i32::from(save_game.settings.current_map_info.height) != save_game.world.height
    {
        return Err(issue(
            "settings map info",
            "settings map dimensions must match world dimensions",
        ));
    }

    Ok(())
}

fn validate_map_info(map_info: &MapInfo) -> std::result::Result<(), ValidationIssue> {
    if map_info.player_slots.len() > usize::from(u8::MAX) {
        return Err(issue("player slots", "player slot count must fit in u8"));
    }

    Ok(())
}

fn validate_campaign_data(save_game: &SaveGame) -> std::result::Result<(), ValidationIssue> {
    let is_campaign = save_game.header.game_type.contains(GameType::CAMPAIGN);
    match (is_campaign, save_game.campaign_save_data.is_some()) {
        (true, false) => Err(issue(
            "campaign save data",
            "campaign game type requires campaign save data",
        )),
        (false, true) => Err(issue(
            "campaign save data",
            "non-campaign game type must not include campaign save data",
        )),
        _ => Ok(()),
    }
}

fn validate_settings(settings: &Settings) -> std::result::Result<(), ValidationIssue> {
    let expected_players_count = settings.players.colors.bits().count_ones() as usize;
    if settings.players.entries.len() != expected_players_count {
        return Err(issue(
            "settings players",
            "player entry count must match colors bit count",
        ));
    }

    if settings.players.current_player_color != PlayerColor::None
        && !settings
            .players
            .colors
            .contains(settings.players.current_player_color)
    {
        return Err(issue(
            "settings current player color",
            "current player color must be active or None",
        ));
    }

    for player in &settings.players.entries {
        if !settings.players.colors.contains(player.color) {
            return Err(issue(
                "settings players",
                "player entry color must be active in settings colors",
            ));
        }
    }

    Ok(())
}

fn validate_world_references(
    world: &World,
    tile_count: usize,
) -> std::result::Result<(), ValidationIssue> {
    for castle in &world.castles {
        if !position_in_bounds(&castle.map_position, world.width, world.height) {
            return Err(issue(
                "world castles",
                "castle positions must be inside the map",
            ));
        }
    }

    for hero in &world.heroes {
        if hero.is_in_play()
            && !position_in_bounds(&hero.base.map_position, world.width, world.height)
        {
            return Err(issue(
                "world heroes",
                "in-play hero positions must be inside the map",
            ));
        }

        validate_index_objects(&hero.visited_objects, tile_count, "hero visited objects")?;
    }

    for kingdom in &world.kingdoms {
        validate_index_objects(
            &kingdom.visited_objects,
            tile_count,
            "kingdom visited objects",
        )?;
    }

    for tile_index in world.captured_objects.keys() {
        if !tile_index_in_bounds(*tile_index, tile_count) {
            return Err(issue(
                "world captured objects",
                "captured object tile indexes must be inside the map",
            ));
        }
    }

    for map_object in world.map_objects.values() {
        if !position_in_bounds(&map_object.base().map_position, world.width, world.height) {
            return Err(issue(
                "world map objects",
                "map object positions must be inside the map",
            ));
        }
    }

    if world.ultimate_artifact.index >= 0
        && !tile_index_in_bounds(world.ultimate_artifact.index, tile_count)
    {
        return Err(issue(
            "ultimate artifact",
            "ultimate artifact tile index must be inside the map",
        ));
    }

    Ok(())
}

fn validate_settings_focus(
    settings: &Settings,
    tile_count: usize,
) -> std::result::Result<(), ValidationIssue> {
    for player in &settings.players.entries {
        if matches!(
            player.focus.kind,
            SettingsFocusKind::Hero | SettingsFocusKind::Castle
        ) && !tile_index_in_bounds(player.focus.tile_index, tile_count)
        {
            return Err(issue(
                "settings player focus",
                "player focus tile index must be inside the map",
            ));
        }
    }

    Ok(())
}

fn validate_index_objects(
    objects: &[IndexObject],
    tile_count: usize,
    field: &'static str,
) -> std::result::Result<(), ValidationIssue> {
    for object in objects {
        if !tile_index_in_bounds(object.tile_index, tile_count) {
            return Err(issue(field, "object tile indexes must be inside the map"));
        }
    }

    Ok(())
}

pub(crate) fn validate_kingdoms(world: &World) -> std::result::Result<(), ValidationIssue> {
    if world.kingdoms.len() != KINGDOM_SLOT_COUNT {
        return Err(issue(
            "world kingdoms",
            "kingdom table must contain exactly 7 slots",
        ));
    }

    for (slot_index, kingdom) in world.kingdoms.iter().enumerate() {
        if !kingdom_slot_accepts_color(slot_index, kingdom.color) {
            return Err(issue(
                "kingdom colors",
                "kingdom slot colors must match fheroes2 slot order or be None for inactive slots",
            ));
        }
    }

    let mut seen_hero_ids = Vec::new();
    for kingdom in &world.kingdoms {
        for hero_id in &kingdom.hero_ids {
            if seen_hero_ids.contains(hero_id) {
                return Err(issue(
                    "kingdom heroes",
                    "kingdom hero references must be unique",
                ));
            }

            let Some(hero) = world.heroes.iter().find(|hero| hero.id == *hero_id) else {
                return Err(issue(
                    "kingdom heroes",
                    "kingdom hero references must point to decoded heroes",
                ));
            };

            if hero.color_base != kingdom.color {
                return Err(issue(
                    "kingdom heroes",
                    "kingdom hero references must match the referenced hero color",
                ));
            }

            seen_hero_ids.push(*hero_id);
        }
    }

    for hero in &world.heroes {
        if hero.color_base == PlayerColor::None {
            continue;
        }

        if !seen_hero_ids.contains(&hero.id) {
            return Err(issue(
                "kingdom heroes",
                "every non-neutral hero must appear in exactly one kingdom hero list",
            ));
        }
    }

    for kingdom in &world.kingdoms {
        for castle_index in &kingdom.castle_indexes {
            let Some(castle) = find_castle_by_index(world, *castle_index) else {
                return Err(issue(
                    "kingdom castles",
                    "kingdom castle references must point to decoded castles",
                ));
            };

            if castle.color_base != kingdom.color {
                return Err(issue(
                    "kingdom castles",
                    "kingdom castle references must match the referenced castle color",
                ));
            }
        }
    }

    validate_unique_hero_ids(world)
}

fn validate_unique_hero_ids(world: &World) -> std::result::Result<(), ValidationIssue> {
    let mut seen = Vec::new();
    for hero in &world.heroes {
        if !hero.is_meaningful() {
            return Err(issue(
                "world heroes",
                "hero list must not contain placeholder or debug heroes",
            ));
        }

        let id = hero.id.to_i32();
        if id < 0 || id as usize >= 73 {
            return Err(issue(
                "world heroes",
                "hero id must fit in serialized hero table",
            ));
        }

        if seen.contains(&hero.id) {
            return Err(issue("world heroes", "hero ids must be unique"));
        }

        seen.push(hero.id);
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

pub(crate) fn castle_index_from_map_position(
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

fn expected_tile_count(width: i32, height: i32) -> Option<usize> {
    if width <= 0 || height <= 0 {
        return None;
    }

    let count = i64::from(width).checked_mul(i64::from(height))?;
    usize::try_from(count)
        .ok()
        .filter(|count| *count <= MAX_WORLD_TILES && u32::try_from(*count).is_ok())
}

fn position_in_bounds(position: &MapPosition, width: i32, height: i32) -> bool {
    let x = i32::from(position.x);
    let y = i32::from(position.y);
    width > 0 && height > 0 && x >= 0 && y >= 0 && x < width && y < height
}

fn tile_index_in_bounds(tile_index: i32, tile_count: usize) -> bool {
    usize::try_from(tile_index)
        .ok()
        .is_some_and(|tile_index| tile_index < tile_count)
}

fn issue(field: &'static str, message: &'static str) -> ValidationIssue {
    ValidationIssue { field, message }
}
