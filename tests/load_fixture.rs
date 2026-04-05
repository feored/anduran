use std::fs;

use kastore::{SaveVersion, load};

#[test]
fn load_supported_fixtures_decodes_metadata() {
    let fixtures = [
        (
            "tests/saves/10032/Guardian_War_0009.sav",
            true,
            "GUARDWAR.MX2",
            "Guardian War",
        ),
        (
            "tests/saves/10032/Good_5_Complete.savc",
            false,
            "CAMPG05.H2C",
            "Good 5",
        ),
        (
            "tests/saves/10032/Evil_1_0018.savc",
            false,
            "CAMPE01.H2C",
            "Evil 1",
        ),
    ];

    for (fixture, requires_pol, filename, name) in fixtures {
        let bytes = fs::read(fixture).unwrap();
        let save_game = load(&bytes).unwrap();

        assert_eq!(save_game.source_version, SaveVersion::V10032);
        assert_eq!(save_game.header.requires_pol, requires_pol);
        assert_eq!(save_game.map_info.filename, filename);
        assert_eq!(save_game.map_info.name, name);
        assert!(!save_game.map_info.description.is_empty());
    }
}
