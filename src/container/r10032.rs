use super::{ContainerHeader, DecodedContainer};
use crate::internal::error::ParseSection;
use crate::internal::reader::Reader;
use crate::model::{
    Difficulty, LossConditionData, LossConditionKind, MapInfo, PlayerColor, PlayerColorsSet,
    PlayerSlotInfo, Race, VictoryConditionData, VictoryConditionKind,
};
use crate::version::SaveVersion;
use crate::{Error, SaveString};

pub(crate) const MAGIC_NUMBER: u16 = 0xFF03;
pub(crate) const REQUIRES_POL: u16 = 0x4000;

pub(crate) fn decode(bytes: &[u8]) -> std::result::Result<DecodedContainer, Error> {
    let mut reader = Reader::with_context(bytes, ParseSection::Container);
    let magic_offset = reader.position();
    let magic_number = reader.read_u16_be("magic number")?;

    if magic_number != MAGIC_NUMBER {
        return Err(reader.unexpected_value(
            "magic number",
            magic_offset,
            "0xFF03",
            format!("0x{magic_number:04X}"),
        ));
    }

    let _save_version_string = reader.read_string_bytes("save version string")?;
    let save_version_number = reader.read_u16_be("save version")?;
    let save_version = SaveVersion::from_u16(save_version_number);

    if save_version != SaveVersion::FORMAT_VERSION_1111_RELEASE {
        return Err(Error::UnsupportedSaveVersion {
            version: save_version_number,
        });
    }

    reader.set_section(ParseSection::Header);
    let requires_pol = (reader.read_u16_be("flags")? & REQUIRES_POL) != 0;

    reader.set_section(ParseSection::MapInfo);
    let filename = SaveString::from_bytes(reader.read_string_bytes("map filename")?);
    let name = SaveString::from_bytes(reader.read_string_bytes("map name")?);
    let description = SaveString::from_bytes(reader.read_string_bytes("map description")?);
    let width = reader.read_u16_be("map width")?;
    let height = reader.read_u16_be("map height")?;
    let difficulty = Difficulty::from(reader.read_u8("map difficulty")?);
    let player_entry_count = reader.read_u8("player entry count")?;
    let mut player_slots = Vec::with_capacity(usize::from(player_entry_count));

    for slot_index in 0..player_entry_count {
        player_slots.push(PlayerSlotInfo {
            slot_index,
            color: PlayerColor::from_index(slot_index),
            race: Race::from(reader.read_u8("player race")?),
            allies: PlayerColorsSet::from_bits(reader.read_u8("player allies")?),
        });
    }

    let map_info = MapInfo {
        filename,
        name,
        description,
        width,
        height,
        difficulty,
        player_slots,
        kingdom_colors: PlayerColorsSet::from_bits(reader.read_u8("kingdom colors")?),
        colors_available_for_humans: PlayerColorsSet::from_bits(
            reader.read_u8("colors available for humans")?,
        ),
        colors_available_for_comp: PlayerColorsSet::from_bits(
            reader.read_u8("colors available for computer")?,
        ),
        colors_of_random_races: PlayerColorsSet::from_bits(
            reader.read_u8("colors of random races")?,
        ),
        victory_condition: VictoryConditionData {
            kind: VictoryConditionKind::from(reader.read_u8("victory condition type")?),
            comp_also_wins: reader.read_byte_as_bool("computer also wins")?,
            allow_normal_victory: reader.read_byte_as_bool("allow normal victory")?,
            params: [
                reader.read_u16_be("victory condition param 0")?,
                reader.read_u16_be("victory condition param 1")?,
            ],
        },
        loss_condition: LossConditionData {
            kind: LossConditionKind::from(reader.read_u8("loss condition type")?),
            params: [
                reader.read_u16_be("loss condition param 0")?,
                reader.read_u16_be("loss condition param 1")?,
            ],
        },
        timestamp: reader.read_u32_be("timestamp")?,
    };

    Ok(DecodedContainer {
        save_version,
        header: ContainerHeader {
            requires_pol,
            map_info,
        },
        payload: reader.remaining().to_vec(),
    })
}
