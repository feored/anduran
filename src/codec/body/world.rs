use crate::Error;
use crate::internal::error::ParseSection;
use crate::internal::reader::Reader;
use crate::internal::writer::Writer;
use crate::model::{
    DirectionSet, LayerType, ObjectPart, PlayerColor, PlayerColorsSet, Tile, World,
};

pub(crate) fn decode(bytes: &[u8]) -> std::result::Result<World, Error> {
    let mut reader = Reader::with_context(bytes, ParseSection::World);
    let width: i32 = reader.read_i32_be("world width")?;
    let height: i32 = reader.read_i32_be("world height")?;
    let tiles_count: u32 = reader.read_u32_be("world tiles count")?;
    let mut tiles: Vec<Tile> = Vec::new();
    for _ in 0..tiles_count {
        tiles.push(decode_tile(&mut reader)?);
    }
    Ok(World {
        width,
        height,
        tiles,
    })
}

pub(crate) fn encode(world: &World) -> std::result::Result<Vec<u8>, Error> {
    let mut writer = Writer::new();
    writer.write_i32_be(world.width);
    writer.write_i32_be(world.height);
    writer.write_u32_be(
        u32::try_from(world.tiles.len()).map_err(|_| Error::InvalidModel {
            field: "world tiles",
            message: "tile count must fit in u32",
        })?,
    );

    for tile in &world.tiles {
        encode_tile(&mut writer, tile)?;
    }

    Ok(writer.into_bytes())
}

fn decode_tile(reader: &mut Reader<'_>) -> std::result::Result<Tile, Error> {
    let index: i32 = reader.read_i32_be("tile index")?;
    let terrain_image_index: u16 = reader.read_u16_be("tile terrain image index")?;
    let terrain_flags: u8 = reader.read_u8("tile terrain flags")?;
    let tile_passability_directions =
        DirectionSet::from_bits(reader.read_u16_be("tile passability directions")?);
    let main_object_part = decode_object_part(reader)?;
    let main_object_type: u16 = reader.read_u16_be("tile main object type")?;
    let fog_colors: PlayerColorsSet =
        PlayerColorsSet::from_bits(reader.read_u8("tile fog colors")?);
    let metadata_count = reader.read_u32_be("tile metadata count")?;
    let mut metadata = Vec::with_capacity(usize::try_from(metadata_count).unwrap());
    for _ in 0..metadata_count {
        metadata.push(reader.read_u32_be("tile metadata")?);
    }
    let occupant_hero_id: u8 = reader.read_u8("tile occupant hero id")?;
    let is_tile_marked_as_road: bool = reader.read_byte_as_bool("tile is marked as road")?;
    let ground_object_parts_count = reader.read_u32_be("tile ground object parts count")?;
    let mut ground_object_parts =
        Vec::with_capacity(usize::try_from(ground_object_parts_count).unwrap());
    for _ in 0..ground_object_parts_count {
        ground_object_parts.push(decode_object_part(reader)?);
    }
    let top_object_parts_count = reader.read_u32_be("tile top object parts count")?;
    let mut top_object_parts = Vec::with_capacity(usize::try_from(top_object_parts_count).unwrap());
    for _ in 0..top_object_parts_count {
        top_object_parts.push(decode_object_part(reader)?);
    }
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

fn encode_tile(writer: &mut Writer, tile: &Tile) -> std::result::Result<(), Error> {
    writer.write_i32_be(tile.index);
    writer.write_u16_be(tile.terrain_image_index);
    writer.write_u8(tile.terrain_flags);
    writer.write_u16_be(tile.tile_passability_directions.bits());
    encode_object_part(writer, tile.main_object_part);
    writer.write_u16_be(tile.main_object_type);
    writer.write_u8(tile.fog_colors.bits());
    writer.write_u32_be(
        u32::try_from(tile.metadata.len()).map_err(|_| Error::InvalidModel {
            field: "tile metadata",
            message: "metadata count must fit in u32",
        })?,
    );
    for value in &tile.metadata {
        writer.write_u32_be(*value);
    }
    writer.write_u8(tile.occupant_hero_id);
    writer.write_byte_from_bool(tile.is_tile_marked_as_road);
    writer.write_u32_be(u32::try_from(tile.ground_object_parts.len()).map_err(|_| {
        Error::InvalidModel {
            field: "tile ground object parts",
            message: "object part count must fit in u32",
        }
    })?);
    for part in &tile.ground_object_parts {
        encode_object_part(writer, *part);
    }
    writer.write_u32_be(u32::try_from(tile.top_object_parts.len()).map_err(|_| {
        Error::InvalidModel {
            field: "tile top object parts",
            message: "object part count must fit in u32",
        }
    })?);
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

#[cfg(test)]
mod tests {
    use super::*;

    fn push_i32(bytes: &mut Vec<u8>, value: i32) {
        bytes.extend_from_slice(&value.to_be_bytes());
    }

    fn push_u16(bytes: &mut Vec<u8>, value: u16) {
        bytes.extend_from_slice(&value.to_be_bytes());
    }

    fn push_u32(bytes: &mut Vec<u8>, value: u32) {
        bytes.extend_from_slice(&value.to_be_bytes());
    }

    fn push_object_part(
        bytes: &mut Vec<u8>,
        layer_type: u8,
        uid: u32,
        icn_type: u8,
        icn_index: u8,
    ) {
        bytes.push(layer_type);
        push_u32(bytes, uid);
        bytes.push(icn_type);
        bytes.push(icn_index);
    }

    fn sample_world_bytes() -> Vec<u8> {
        let mut bytes = Vec::new();
        push_i32(&mut bytes, 2);
        push_i32(&mut bytes, 1);
        push_u32(&mut bytes, 1);

        push_i32(&mut bytes, -7);
        push_u16(&mut bytes, 0x1234);
        bytes.push(0xA5);
        push_u16(
            &mut bytes,
            DirectionSet::TOP.bits() | DirectionSet::RIGHT.bits(),
        );
        push_object_part(&mut bytes, 3, 0x0102_0304, 45, 7);
        push_u16(&mut bytes, 0x00A3);
        bytes.push(0x21);
        push_u32(&mut bytes, 2);
        push_u32(&mut bytes, 0xDEAD_BEEF);
        push_u32(&mut bytes, 5);
        bytes.push(9);
        bytes.push(1);
        push_u32(&mut bytes, 2);
        push_object_part(&mut bytes, 0, 0x1111_1111, 12, 1);
        push_object_part(&mut bytes, 1, 0x2222_2222, 29, 8);
        push_u32(&mut bytes, 1);
        push_object_part(&mut bytes, 2, 0x3333_3333, 14, 2);
        bytes.push(PlayerColor::Red.bits());

        bytes
    }

    #[test]
    fn decode_world_reads_tile_fields() {
        let world = decode(&sample_world_bytes()).unwrap();

        assert_eq!(world.width, 2);
        assert_eq!(world.height, 1);
        assert_eq!(world.tiles.len(), 1);

        let tile = &world.tiles[0];
        assert_eq!(tile.index, -7);
        assert_eq!(tile.terrain_image_index, 0x1234);
        assert_eq!(tile.terrain_flags, 0xA5);
        assert_eq!(
            tile.tile_passability_directions,
            DirectionSet::from_bits(DirectionSet::TOP.bits() | DirectionSet::RIGHT.bits())
        );
        assert_eq!(
            tile.main_object_part,
            ObjectPart {
                layer_type: LayerType::TerrainLayer,
                uid: 0x0102_0304,
                icn_type: 45,
                icn_index: 7,
            }
        );
        assert_eq!(tile.main_object_type, 0x00A3);
        assert_eq!(tile.fog_colors, PlayerColorsSet::from_bits(0x21));
        assert_eq!(tile.metadata, vec![0xDEAD_BEEF, 5]);
        assert_eq!(tile.occupant_hero_id, 9);
        assert!(tile.is_tile_marked_as_road);
        assert_eq!(tile.ground_object_parts.len(), 2);
        assert_eq!(tile.top_object_parts.len(), 1);
        assert_eq!(tile.boat_owner_color, PlayerColor::Red);
    }

    #[test]
    fn encode_world_matches_decoded_bytes() {
        let bytes = sample_world_bytes();
        let world = decode(&bytes).unwrap();

        assert_eq!(encode(&world).unwrap(), bytes);
    }

    #[test]
    fn encode_world_writes_empty_tile_vector() {
        let world = World {
            width: 0,
            height: 0,
            tiles: Vec::new(),
        };

        assert_eq!(
            encode(&world).unwrap(),
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }
}
