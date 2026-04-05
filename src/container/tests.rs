use std::fs;

use crate::version::ContainerRevision;
use crate::version::SaveVersion;

use super::decode_container;

#[test]
fn decode_container_rejects_invalid_magic() {
    let bytes = [0x00, 0x00, 0x12, 0x34];

    let error = decode_container(ContainerRevision::R10032, &bytes).unwrap_err();

    assert_eq!(error, crate::Error::InvalidContainer("unexpected magic number"));
}

#[test]
fn decode_container_allows_mismatched_version_string() {
    let bytes = [
        0xFF, 0x03, // magic
        0x00, 0x00, 0x00, 0x04, // version string length
        b'o', b'o', b'p', b's', // version string
        0x27, 0x30, // version number 10032
        0x00, 0x00, // flags
        0x00, 0x00, 0x00, 0x00, // filename length
        0x00, 0x00, 0x00, 0x00, // name length
        0x00, 0x00, 0x00, 0x00, // description length
        0x00, 0x00, // width
        0x00, 0x00, // height
        0x00, // difficulty
        0x00, // player entry count
        0x00, // kingdom colors
        0x00, // colors available for humans
        0x00, // colors available for computer
        0x00, // colors of random races
    ];

    let container = decode_container(ContainerRevision::R10032, &bytes).unwrap();

    assert_eq!(container.save_version, SaveVersion::V10032);
    assert!(!container.header.requires_pol);
    assert_eq!(container.header.map_info.width, 0);
    assert_eq!(container.header.map_info.difficulty, crate::model::Difficulty::Easy);
    assert!(container.header.map_info.player_slots.is_empty());
}

#[test]
fn decode_container_returns_error_for_truncated_map_filename() {
    let bytes = [
        0xFF, 0x03, // magic
        0x00, 0x00, 0x00, 0x05, // version string length
        b'1', b'0', b'0', b'3', b'2', // version string
        0x27, 0x30, // version number 10032
        0x00, 0x00, // flags
    ];

    let error = decode_container(ContainerRevision::R10032, &bytes).unwrap_err();

    assert_eq!(error, crate::Error::InvalidContainer("map filename"));
}

#[test]
fn decode_container_allows_non_utf8_string_bytes() {
    let bytes = [
        0xFF, 0x03, // magic
        0x00, 0x00, 0x00, 0x05, // version string length
        b'1', b'0', b'0', b'3', b'2', // version string
        0x27, 0x30, // version number 10032
        0x00, 0x00, // flags
        0x00, 0x00, 0x00, 0x02, // filename length
        0xFF, 0xFE, // filename bytes
        0x00, 0x00, 0x00, 0x03, // name length
        b'A', 0x00, b'B', // name bytes with embedded NUL
        0x00, 0x00, 0x00, 0x00, // description length
        0x00, 0x00, // width
        0x00, 0x00, // height
        0x00, // difficulty
        0x02, // player entry count
        0x01, // player 0 race
        0x05, // player 0 allies
        0x20, // player 1 race
        0x06, // player 1 allies
        0x11, // kingdom colors
        0x01, // colors available for humans
        0x10, // colors available for computer
        0x04, // colors of random races
    ];

    let container = decode_container(ContainerRevision::R10032, &bytes).unwrap();

    assert_eq!(container.header.map_info.filename.as_bytes(), &[0xFF, 0xFE]);
    assert_eq!(container.header.map_info.name.as_bytes(), b"A\0B");
    assert_eq!(container.header.map_info.player_slots.len(), 2);
    assert_eq!(
        container.header.map_info.player_slots[0],
        crate::model::PlayerSlotInfo {
            slot_index: 0,
            color: Some(crate::model::PlayerColor::Blue),
            race: crate::model::Race::Knight,
            allies: crate::model::PlayerColorsSet::from_bits(0x05),
        }
    );
    assert_eq!(
        container.header.map_info.player_slots[1],
        crate::model::PlayerSlotInfo {
            slot_index: 1,
            color: Some(crate::model::PlayerColor::Green),
            race: crate::model::Race::Necromancer,
            allies: crate::model::PlayerColorsSet::from_bits(0x06),
        }
    );
    assert_eq!(
        container.header.map_info.kingdom_colors,
        crate::model::PlayerColorsSet::from_bits(0x11)
    );
}

#[test]
fn decode_container_parses_real_fixture_header() {
    let bytes = fs::read("tests/saves/10032/Guardian_War_0009.sav").unwrap();

    let container = decode_container(ContainerRevision::R10032, &bytes).unwrap();

    assert_eq!(container.save_version, SaveVersion::V10032);
    assert!(container.header.requires_pol);
    assert_eq!(container.header.map_info.filename, "GUARDWAR.MX2");
    assert!(container.header.map_info.name.contains("Guardian"));
    assert_eq!(container.header.map_info.width, 72);
    assert_eq!(container.header.map_info.height, 72);
    assert_eq!(container.header.map_info.player_slots.len(), 6);
    assert_eq!(
        container.header.map_info.player_slots[0].color,
        Some(crate::model::PlayerColor::Blue)
    );
    assert_eq!(
        container.header.map_info.player_slots[0].race,
        crate::model::Race::Random
    );
    assert_eq!(
        container.header.map_info.player_slots[0].allies,
        crate::model::PlayerColorsSet::from_bits(0x03)
    );
    assert_eq!(
        container.header.map_info.player_slots[5].color,
        Some(crate::model::PlayerColor::Purple)
    );
    assert_eq!(
        container.header.map_info.player_slots[5].race,
        crate::model::Race::None
    );
    assert_eq!(
        container.header.map_info.player_slots[5].allies,
        crate::model::PlayerColorsSet::from_bits(0x20)
    );
    assert_eq!(
        container.header.map_info.kingdom_colors,
        crate::model::PlayerColorsSet::from_bits(0x1F)
    );
    assert!(container
        .header
        .map_info
        .description
        .starts_with("You and your ally's families"));
}
