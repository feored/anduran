fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fixtures = [
        "tests/saves/10032/Guardian_War_0009.sav",
        "tests/saves/10032/Good_5_Complete.savc",
        "tests/saves/10032/Evil_1_0018.savc",
    ];

    for fixture in fixtures {
        let bytes = std::fs::read(fixture)?;
        let save_game = kastore::load(&bytes)?;

        println!("{fixture}");
        println!("{save_game}");
    }

    Ok(())
}
