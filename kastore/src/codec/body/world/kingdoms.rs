use crate::Error;
use crate::internal::reader::Reader;
use crate::internal::writer::Writer;
use crate::model::header::player::PlayerColor;
use crate::model::world::IndexObject;
use crate::model::world::heroes::id::HeroID;
use crate::model::world::kingdoms::{
    KINGDOM_SLOT_COUNT, Kingdom, KingdomModeSet, KingdomPuzzle, KingdomRecruit, KingdomRecruits,
    PUZZLE_REVEALED_TILES_COUNT, PUZZLE_ZONE_COUNTS,
};

const EXPECTED_KINGDOMS_COUNT: u32 = KINGDOM_SLOT_COUNT as u32;

pub(super) fn decode(reader: &mut Reader<'_>) -> std::result::Result<Vec<Kingdom>, Error> {
    let count_offset = reader.position();
    let count = reader.read_u32_be("kingdoms count")?;
    if count != EXPECTED_KINGDOMS_COUNT {
        return Err(reader.invalid_value(
            "kingdoms count",
            count_offset,
            "unexpected kingdoms table size, expected 7",
        ));
    }

    let mut kingdoms = Vec::with_capacity(KINGDOM_SLOT_COUNT);
    for _ in 0..count {
        kingdoms.push(decode_kingdom(reader)?);
    }

    Ok(kingdoms)
}

pub(super) fn encode(writer: &mut Writer, kingdoms: &[Kingdom]) -> std::result::Result<(), Error> {
    writer.write_u32_be(EXPECTED_KINGDOMS_COUNT);
    for kingdom in kingdoms {
        encode_kingdom(writer, kingdom)?;
    }

    Ok(())
}

fn decode_kingdom(reader: &mut Reader<'_>) -> std::result::Result<Kingdom, Error> {
    let mode = KingdomModeSet::from_bits(reader.read_u32_be("kingdom mode")?);
    let color = PlayerColor::from_bits(reader.read_u8("kingdom color")?);

    let funds = super::decode_funds(reader)?;
    let lost_town_days = reader.read_u32_be("kingdom lost town days")?;

    let castle_indexes = reader.read_vec_u32("kingdom castles count", |reader| {
        reader.read_i32_be("kingdom castle index")
    })?;

    let hero_ids = reader.read_vec_u32("kingdom heroes count", |reader| {
        Ok(HeroID::from_i32(reader.read_i32_be("kingdom hero id")?))
    })?;

    let recruits = KingdomRecruits {
        first: KingdomRecruit {
            hero_id: HeroID::from_i32(reader.read_i32_be("kingdom recruit hero id")?),
            surrender_day: reader.read_u32_be("kingdom recruit surrender day")?,
        },
        second: KingdomRecruit {
            hero_id: HeroID::from_i32(reader.read_i32_be("kingdom recruit hero id")?),
            surrender_day: reader.read_u32_be("kingdom recruit surrender day")?,
        },
    };

    let visited_objects = reader.read_vec_u32("kingdom visited objects count", |reader| {
        Ok(IndexObject {
            tile_index: reader.read_i32_be("kingdom visited object tile index")?,
            object_type: reader.read_u16_be("kingdom visited object type")?,
        })
    })?;

    let puzzle = KingdomPuzzle {
        revealed_tiles: {
            let revealed_tiles_offset = reader.position();
            let len = reader.read_len_u32_with_max(
                "kingdom puzzle revealed tiles",
                PUZZLE_REVEALED_TILES_COUNT,
            )?;
            let revealed_tiles = reader
                .read_bytes(len, "kingdom puzzle revealed tiles")?
                .to_vec();
            if !revealed_tiles_are_valid(&revealed_tiles) {
                return Err(reader.invalid_value(
                    "kingdom puzzle revealed tiles",
                    revealed_tiles_offset,
                    "revealed tiles must be 48 ASCII '0'/'1' bytes",
                ));
            }
            revealed_tiles
        },
        zone1_order: decode_puzzle_zone(
            reader,
            "kingdom puzzle zone1 count",
            "kingdom puzzle zone1 tile",
            PUZZLE_ZONE_COUNTS[0],
            "kingdom puzzle zone1 must contain exactly 24 tiles",
        )?,
        zone2_order: decode_puzzle_zone(
            reader,
            "kingdom puzzle zone2 count",
            "kingdom puzzle zone2 tile",
            PUZZLE_ZONE_COUNTS[1],
            "kingdom puzzle zone2 must contain exactly 16 tiles",
        )?,
        zone3_order: decode_puzzle_zone(
            reader,
            "kingdom puzzle zone3 count",
            "kingdom puzzle zone3 tile",
            PUZZLE_ZONE_COUNTS[2],
            "kingdom puzzle zone3 must contain exactly 4 tiles",
        )?,
        zone4_order: decode_puzzle_zone(
            reader,
            "kingdom puzzle zone4 count",
            "kingdom puzzle zone4 tile",
            PUZZLE_ZONE_COUNTS[3],
            "kingdom puzzle zone4 must contain exactly 4 tiles",
        )?,
    };

    Ok(Kingdom {
        mode,
        color,
        funds,
        lost_town_days,
        castle_indexes,
        hero_ids,
        recruits,
        visited_objects,
        puzzle,
        visited_tents_colors: reader.read_i32_be("kingdom visited tents colors")?,
        top_castle_in_kingdom_view: reader.read_i32_be("kingdom top castle in kingdom view")?,
        top_hero_in_kingdom_view: reader.read_i32_be("kingdom top hero in kingdom view")?,
    })
}

fn encode_kingdom(writer: &mut Writer, kingdom: &Kingdom) -> std::result::Result<(), Error> {
    writer.write_u32_be(kingdom.mode.bits());
    writer.write_u8(kingdom.color.bits());
    super::encode_funds(writer, &kingdom.funds);
    writer.write_u32_be(kingdom.lost_town_days);
    writer.write_len_u32(kingdom.castle_indexes.len(), "kingdom castles")?;
    for castle_index in &kingdom.castle_indexes {
        writer.write_i32_be(*castle_index);
    }
    writer.write_len_u32(kingdom.hero_ids.len(), "kingdom heroes")?;
    for hero_id in &kingdom.hero_ids {
        writer.write_i32_be(hero_id.to_i32());
    }
    writer.write_i32_be(kingdom.recruits.first.hero_id.to_i32());
    writer.write_u32_be(kingdom.recruits.first.surrender_day);
    writer.write_i32_be(kingdom.recruits.second.hero_id.to_i32());
    writer.write_u32_be(kingdom.recruits.second.surrender_day);
    writer.write_len_u32(kingdom.visited_objects.len(), "kingdom visited objects")?;
    for object in &kingdom.visited_objects {
        writer.write_i32_be(object.tile_index);
        writer.write_u16_be(object.object_type);
    }
    if !revealed_tiles_are_valid(&kingdom.puzzle.revealed_tiles) {
        return Err(Error::InvalidModel {
            field: "kingdom puzzle revealed tiles",
            message: "revealed tiles must be 48 ASCII '0'/'1' bytes",
        });
    }
    writer.write_len_u32(
        kingdom.puzzle.revealed_tiles.len(),
        "kingdom puzzle revealed tiles",
    )?;
    writer.write_bytes(&kingdom.puzzle.revealed_tiles);
    encode_puzzle_zone(
        writer,
        &kingdom.puzzle.zone1_order,
        "kingdom puzzle zone1",
        PUZZLE_ZONE_COUNTS[0],
        "zone must contain exactly 24 tiles",
    )?;
    encode_puzzle_zone(
        writer,
        &kingdom.puzzle.zone2_order,
        "kingdom puzzle zone2",
        PUZZLE_ZONE_COUNTS[1],
        "zone must contain exactly 16 tiles",
    )?;
    encode_puzzle_zone(
        writer,
        &kingdom.puzzle.zone3_order,
        "kingdom puzzle zone3",
        PUZZLE_ZONE_COUNTS[2],
        "zone must contain exactly 4 tiles",
    )?;
    encode_puzzle_zone(
        writer,
        &kingdom.puzzle.zone4_order,
        "kingdom puzzle zone4",
        PUZZLE_ZONE_COUNTS[3],
        "zone must contain exactly 4 tiles",
    )?;
    writer.write_i32_be(kingdom.visited_tents_colors);
    writer.write_i32_be(kingdom.top_castle_in_kingdom_view);
    writer.write_i32_be(kingdom.top_hero_in_kingdom_view);

    Ok(())
}

fn decode_puzzle_zone(
    reader: &mut Reader<'_>,
    count_field: &'static str,
    tile_field: &'static str,
    expected_count: usize,
    invalid_count_message: &'static str,
) -> std::result::Result<Vec<u8>, Error> {
    let count_offset = reader.position();
    let count = reader.read_u8(count_field)?;
    if usize::from(count) != expected_count {
        return Err(reader.invalid_value(count_field, count_offset, invalid_count_message));
    }
    let mut zone = Vec::with_capacity(usize::from(count));
    for _ in 0..count {
        zone.push(reader.read_u8(tile_field)?);
    }

    Ok(zone)
}

fn encode_puzzle_zone(
    writer: &mut Writer,
    zone: &[u8],
    field: &'static str,
    expected_count: usize,
    invalid_count_message: &'static str,
) -> std::result::Result<(), Error> {
    if zone.len() != expected_count {
        return Err(Error::InvalidModel {
            field,
            message: invalid_count_message,
        });
    }

    writer.write_u8(u8::try_from(zone.len()).map_err(|_| Error::InvalidModel {
        field,
        message: "puzzle zone tile count must fit in u8",
    })?);
    for tile in zone {
        writer.write_u8(*tile);
    }

    Ok(())
}

fn revealed_tiles_are_valid(revealed_tiles: &[u8]) -> bool {
    revealed_tiles.len() == PUZZLE_REVEALED_TILES_COUNT
        && revealed_tiles
            .iter()
            .all(|byte| matches!(byte, b'0' | b'1'))
}
