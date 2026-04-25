use crate::Error;
use crate::internal::reader::Reader;
use crate::internal::writer::Writer;
use crate::model::header::player::{PlayerColor, PlayerColorsSet};
use crate::model::world::tile::direction::DirectionSet;
use crate::model::world::tile::{LayerType, ObjectPart, Tile};

pub(super) fn decode(reader: &mut Reader<'_>) -> std::result::Result<Tile, Error> {
    let index: i32 = reader.read_i32_be("tile index")?;
    let terrain_image_index: u16 = reader.read_u16_be("tile terrain image index")?;
    let terrain_flags: u8 = reader.read_u8("tile terrain flags")?;
    let tile_passability_directions =
        DirectionSet::from_bits(reader.read_u16_be("tile passability directions")?);
    let main_object_part = decode_object_part(reader)?;
    let main_object_type: u16 = reader.read_u16_be("tile main object type")?;
    let fog_colors: PlayerColorsSet =
        PlayerColorsSet::from_bits(reader.read_u8("tile fog colors")?);
    let metadata = reader.read_vec_u32("tile metadata count", |reader| {
        reader.read_u32_be("tile metadata")
    })?;
    let occupant_hero_id: u8 = reader.read_u8("tile occupant hero id")?;
    let is_tile_marked_as_road: bool = reader.read_byte_as_bool("tile is marked as road")?;
    let ground_object_parts =
        reader.read_vec_u32("tile ground object parts count", decode_object_part)?;
    let top_object_parts =
        reader.read_vec_u32("tile top object parts count", decode_object_part)?;
    let boat_owner_color: PlayerColor =
        PlayerColor::from_bits(reader.read_u8("tile boat owner color")?);

    Ok(Tile {
        index,
        terrain_image_index,
        terrain_flags,
        tile_passability_directions,
        main_object_part,
        main_object_type,
        fog_colors,
        metadata,
        occupant_hero_id,
        is_tile_marked_as_road,
        ground_object_parts,
        top_object_parts,
        boat_owner_color,
    })
}

pub(super) fn encode(writer: &mut Writer, tile: &Tile) -> std::result::Result<(), Error> {
    writer.write_i32_be(tile.index);
    writer.write_u16_be(tile.terrain_image_index);
    writer.write_u8(tile.terrain_flags);
    writer.write_u16_be(tile.tile_passability_directions.bits());
    encode_object_part(writer, tile.main_object_part);
    writer.write_u16_be(tile.main_object_type);
    writer.write_u8(tile.fog_colors.bits());
    writer.write_len_u32(tile.metadata.len(), "tile metadata")?;
    for value in &tile.metadata {
        writer.write_u32_be(*value);
    }
    writer.write_u8(tile.occupant_hero_id);
    writer.write_byte_from_bool(tile.is_tile_marked_as_road);
    writer.write_len_u32(tile.ground_object_parts.len(), "tile ground object parts")?;
    for part in &tile.ground_object_parts {
        encode_object_part(writer, *part);
    }
    writer.write_len_u32(tile.top_object_parts.len(), "tile top object parts")?;
    for part in &tile.top_object_parts {
        encode_object_part(writer, *part);
    }
    writer.write_u8(tile.boat_owner_color.bits());

    Ok(())
}

fn decode_object_part(reader: &mut Reader<'_>) -> std::result::Result<ObjectPart, Error> {
    let layer_type = LayerType::from_byte(reader.read_u8("object part layer type")?);
    let uid = reader.read_u32_be("object part uid")?;
    let icn_type = reader.read_u8("object part icn type")?;
    let icn_index = reader.read_u8("object part icn index")?;

    Ok(ObjectPart {
        layer_type,
        uid,
        icn_type,
        icn_index,
    })
}

fn encode_object_part(writer: &mut Writer, part: ObjectPart) {
    writer.write_u8(part.layer_type.to_byte());
    writer.write_u32_be(part.uid);
    writer.write_u8(part.icn_type);
    writer.write_u8(part.icn_index);
}
