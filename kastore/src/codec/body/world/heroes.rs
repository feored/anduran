use crate::Error;
use crate::SaveString;
use crate::internal::reader::Reader;
use crate::internal::writer::Writer;
use crate::model::header::player::{PlayerColor, Race};
use crate::model::world::heroes::army::{Army, MonsterType, Troop};
use crate::model::world::heroes::artifact::{Artifact, ArtifactID};
use crate::model::world::heroes::id::HeroID;
use crate::model::world::heroes::modes::HeroModeSet;
use crate::model::world::heroes::path::{Direction, Path, RouteStep};
use crate::model::world::heroes::skills::{SecondarySkill, Skill, SkillLevel};
use crate::model::world::heroes::spells::Spell;
use crate::model::world::heroes::{Hero, HeroBase, PrimarySkills};
use crate::model::world::{IndexObject, MapPosition, Point};

const EXPECTED_HEROES_COUNT: u32 = 73;

pub(super) fn decode(reader: &mut Reader<'_>) -> std::result::Result<Vec<Hero>, Error> {
    let count_offset = reader.position();
    let count = reader.read_u32_be("heroes count")?;
    if count != EXPECTED_HEROES_COUNT {
        return Err(reader.invalid_value(
            "heroes count",
            count_offset,
            "unexpected heroes table size, expected 73",
        ));
    }
    let mut heroes = Vec::with_capacity(EXPECTED_HEROES_COUNT as usize);
    for _ in 0..count {
        let hero = decode_hero(reader)?;
        if hero.is_meaningful() {
            heroes.push(hero);
        }
    }
    Ok(heroes)
}

pub(super) fn encode(writer: &mut Writer, heroes: &[Hero]) -> std::result::Result<(), Error> {
    let slots = hero_slots(heroes)?;

    writer.write_u32_be(EXPECTED_HEROES_COUNT);
    for slot in slots {
        match slot {
            Some(hero) => encode_hero(writer, hero)?,
            None => encode_placeholder_hero(writer)?,
        }
    }

    Ok(())
}

fn decode_hero(reader: &mut Reader<'_>) -> std::result::Result<Hero, Error> {
    let base = decode_hero_base(reader)?;
    let name = reader.read_save_string("hero name")?;
    let color_base = PlayerColor::from_bits(reader.read_u8("hero color base")?);
    let experience = reader.read_u32_be("hero experience")?;
    let secondary_skills = decode_secondary_skills(reader)?;
    let army = decode_army(reader)?;
    let id = HeroID::from_i32(reader.read_i32_be("hero id")?);
    let portrait = reader.read_i32_be("hero portrait")?;
    let race = Race::from_i32(reader.read_i32_be("hero race")?);
    let object_type_under_hero = reader.read_u16_be("hero object type under hero")?;
    let path = decode_path(reader)?;
    let direction = Direction::from_i32(reader.read_i32_be("hero direction")?);
    let sprite_index = reader.read_i32_be("hero sprite index")?;
    let patrol_center = Point {
        x: reader.read_i32_be("hero patrol center x")?,
        y: reader.read_i32_be("hero patrol center y")?,
    };
    let patrol_distance = reader.read_u32_be("hero patrol distance")?;
    let visited_objects = decode_visited_objects(reader)?;
    let last_ground_region = reader.read_u32_be("hero last ground region")?;

    Ok(Hero {
        base,
        name,
        color_base,
        experience,
        secondary_skills,
        army,
        id,
        portrait,
        race,
        object_type_under_hero,
        path,
        direction,
        sprite_index,
        patrol_center,
        patrol_distance,
        visited_objects,
        last_ground_region,
    })
}

fn encode_hero(writer: &mut Writer, hero: &Hero) -> std::result::Result<(), Error> {
    encode_hero_base(writer, &hero.base)?;
    writer.write_save_string(&hero.name, "hero name")?;
    writer.write_u8(hero.color_base.bits());
    writer.write_u32_be(hero.experience);
    encode_secondary_skills(writer, &hero.secondary_skills)?;
    encode_army(writer, &hero.army)?;
    writer.write_i32_be(hero.id.to_i32());
    writer.write_i32_be(hero.portrait);
    writer.write_i32_be(hero.race.to_i32());
    writer.write_u16_be(hero.object_type_under_hero);
    encode_path(writer, &hero.path)?;
    writer.write_i32_be(hero.direction.to_i32());
    writer.write_i32_be(hero.sprite_index);
    writer.write_i32_be(hero.patrol_center.x);
    writer.write_i32_be(hero.patrol_center.y);
    writer.write_u32_be(hero.patrol_distance);
    encode_visited_objects(writer, &hero.visited_objects)?;
    writer.write_u32_be(hero.last_ground_region);

    Ok(())
}

pub(crate) fn decode_hero_base(reader: &mut Reader<'_>) -> std::result::Result<HeroBase, Error> {
    let primary_skills = PrimarySkills {
        attack: reader.read_i32_be("hero primary skill attack")?,
        defense: reader.read_i32_be("hero primary skill defense")?,
        knowledge: reader.read_i32_be("hero primary skill knowledge")?,
        power: reader.read_i32_be("hero primary skill power")?,
    };
    let map_position = MapPosition {
        x: reader.read_i16_be("hero map position x")?,
        y: reader.read_i16_be("hero map position y")?,
    };
    let modes = HeroModeSet::from_bits(reader.read_u32_be("hero modes")?);
    let spell_points = reader.read_u32_be("hero spell points")?;
    let move_points = reader.read_u32_be("hero move points")?;
    let spell_book = reader.read_vec_u32("hero spell book count", |reader| {
        Ok(Spell::from_i32(
            reader.read_i32_be("hero spell book spell")?,
        ))
    })?;
    let bag_artifacts = reader.read_vec_u32("hero bag artifacts count", |reader| {
        Ok(Artifact {
            id: ArtifactID::from_i32(reader.read_i32_be("hero bag artifact")?),
            ext: reader.read_i32_be("hero bag artifact")?,
        })
    })?;
    Ok(HeroBase {
        primary_skills,
        map_position,
        modes,
        spell_points,
        move_points,
        spell_book,
        bag_artifacts,
    })
}

pub(crate) fn encode_hero_base(
    writer: &mut Writer,
    base: &HeroBase,
) -> std::result::Result<(), Error> {
    writer.write_i32_be(base.primary_skills.attack);
    writer.write_i32_be(base.primary_skills.defense);
    writer.write_i32_be(base.primary_skills.knowledge);
    writer.write_i32_be(base.primary_skills.power);
    writer.write_i16_be(base.map_position.x);
    writer.write_i16_be(base.map_position.y);
    writer.write_u32_be(base.modes.bits());
    writer.write_u32_be(base.spell_points);
    writer.write_u32_be(base.move_points);
    writer.write_len_u32(base.spell_book.len(), "hero spell book")?;
    for spell in &base.spell_book {
        writer.write_i32_be(spell.to_i32());
    }
    writer.write_len_u32(base.bag_artifacts.len(), "hero bag artifacts")?;
    for artifact in &base.bag_artifacts {
        writer.write_i32_be(artifact.id.to_i32());
        writer.write_i32_be(artifact.ext);
    }

    Ok(())
}

fn decode_secondary_skills(
    reader: &mut Reader<'_>,
) -> std::result::Result<Vec<SecondarySkill>, Error> {
    let secondary_skills_count_offset = reader.position();
    let secondary_skills_count = reader.read_u32_be("hero secondary skills count")?;
    if secondary_skills_count > 8 {
        return Err(reader.invalid_value(
            "hero secondary skills count",
            secondary_skills_count_offset,
            "too many hero secondary skills",
        ));
    }
    let mut secondary_skills = Vec::with_capacity(secondary_skills_count as usize);
    for _ in 0..secondary_skills_count {
        secondary_skills.push(SecondarySkill {
            id: Skill::from_i32(reader.read_i32_be("hero secondary skill")?),
            level: SkillLevel::from_i32(reader.read_i32_be("hero secondary skill level")?),
        });
    }
    Ok(secondary_skills)
}

fn encode_secondary_skills(
    writer: &mut Writer,
    secondary_skills: &[SecondarySkill],
) -> std::result::Result<(), Error> {
    if secondary_skills.len() > 8 {
        return Err(Error::InvalidModel {
            field: "hero secondary skills",
            message: "secondary skill count must be at most 8",
        });
    }

    writer.write_len_u32(secondary_skills.len(), "hero secondary skills")?;
    for skill in secondary_skills {
        writer.write_i32_be(skill.id.to_i32());
        writer.write_i32_be(skill.level.to_i32());
    }

    Ok(())
}

pub(crate) fn decode_army(reader: &mut Reader<'_>) -> std::result::Result<Army, Error> {
    let troops_count_offset = reader.position();
    let troops_count = reader.read_u32_be("hero army troops count")?;
    if troops_count != 5 {
        return Err(reader.invalid_value(
            "hero army troops count",
            troops_count_offset,
            "unexpected hero army troops count, expected 5",
        ));
    }
    let mut troops = Vec::with_capacity(5);
    for _ in 0..troops_count {
        troops.push(Troop {
            monster: MonsterType::from_i32(reader.read_i32_be("hero army troop monster type")?),
            count: reader.read_u32_be("hero army troop count")?,
        });
    }
    let army: Army = Army {
        troops,
        spread_combat_formation: reader.read_byte_as_bool("spread combat formation")?,
        player_color: PlayerColor::from_bits(reader.read_u8("army player color")?),
    };
    Ok(army)
}

pub(crate) fn encode_army(writer: &mut Writer, army: &Army) -> std::result::Result<(), Error> {
    if army.troops.len() != 5 {
        return Err(Error::InvalidModel {
            field: "hero army troops",
            message: "hero army must contain exactly 5 troop slots",
        });
    }

    writer.write_u32_be(5);
    for troop in &army.troops {
        encode_troop(writer, troop);
    }
    writer.write_byte_from_bool(army.spread_combat_formation);
    writer.write_u8(army.player_color.bits());

    Ok(())
}
pub(crate) fn encode_troop(writer: &mut Writer, troop: &Troop) {
    writer.write_i32_be(troop.monster.to_i32());
    writer.write_u32_be(troop.count);
}

fn decode_path(reader: &mut Reader<'_>) -> std::result::Result<Path, Error> {
    let hidden = reader.read_byte_as_bool("hero path hidden")?;
    let steps = reader.read_vec_u32("hero path steps count", |reader| {
        Ok(RouteStep {
            from_index: reader.read_i32_be("hero path step from index")?,
            direction: Direction::from_i32(reader.read_i32_be("hero path step direction")?),
            movement_cost: reader.read_u32_be("hero path step movement cost")?,
        })
    })?;
    Ok(Path { hidden, steps })
}

fn encode_path(writer: &mut Writer, path: &Path) -> std::result::Result<(), Error> {
    writer.write_byte_from_bool(path.hidden);
    writer.write_len_u32(path.steps.len(), "hero path steps")?;
    for step in &path.steps {
        writer.write_i32_be(step.from_index);
        writer.write_i32_be(step.direction.to_i32());
        writer.write_u32_be(step.movement_cost);
    }

    Ok(())
}

fn decode_visited_objects(reader: &mut Reader<'_>) -> std::result::Result<Vec<IndexObject>, Error> {
    reader.read_vec_u32("hero visited objects count", |reader| {
        Ok(IndexObject {
            tile_index: reader.read_i32_be("hero visited object tile index")?,
            object_type: reader.read_u16_be("hero visited object type")?,
        })
    })
}

fn encode_visited_objects(
    writer: &mut Writer,
    visited_objects: &[IndexObject],
) -> std::result::Result<(), Error> {
    writer.write_len_u32(visited_objects.len(), "hero visited objects")?;
    for object in visited_objects {
        writer.write_i32_be(object.tile_index);
        writer.write_u16_be(object.object_type);
    }

    Ok(())
}

fn encode_placeholder_hero(writer: &mut Writer) -> std::result::Result<(), Error> {
    let placeholder = Hero {
        base: HeroBase {
            primary_skills: PrimarySkills::default(),
            map_position: MapPosition::default(),
            modes: HeroModeSet::EMPTY,
            spell_points: 0,
            move_points: 0,
            spell_book: Vec::new(),
            bag_artifacts: Vec::new(),
        },
        name: SaveString::default(),
        color_base: PlayerColor::None,
        experience: 0,
        secondary_skills: Vec::new(),
        army: Army {
            troops: vec![Troop::default(); 5],
            spread_combat_formation: false,
            player_color: PlayerColor::None,
        },
        id: HeroID::Unknown(0),
        portrait: 0,
        race: Race::None,
        object_type_under_hero: 0,
        path: Path::default(),
        direction: Direction::Unknown,
        sprite_index: 0,
        patrol_center: Point::default(),
        patrol_distance: 0,
        visited_objects: Vec::new(),
        last_ground_region: 0,
    };

    encode_hero(writer, &placeholder)
}

fn hero_slots(heroes: &[Hero]) -> std::result::Result<Vec<Option<&Hero>>, Error> {
    let mut slots = vec![None; EXPECTED_HEROES_COUNT as usize];

    for hero in heroes {
        let slot_index = semantic_slot_index(hero)?;
        if slots[slot_index].is_some() {
            return Err(Error::InvalidModel {
                field: "world heroes",
                message: "hero ids must be unique",
            });
        }
        slots[slot_index] = Some(hero);
    }

    Ok(slots)
}

fn semantic_slot_index(hero: &Hero) -> std::result::Result<usize, Error> {
    if !hero.is_meaningful() {
        return Err(Error::InvalidModel {
            field: "world heroes",
            message: "hero list must not contain placeholder or debug heroes",
        });
    }

    let raw_id = hero.id.to_i32();
    let slot_index = usize::try_from(raw_id).map_err(|_| Error::InvalidModel {
        field: "world heroes",
        message: "hero id must fit in serialized hero table",
    })?;

    if slot_index >= EXPECTED_HEROES_COUNT as usize {
        return Err(Error::InvalidModel {
            field: "world heroes",
            message: "hero id must fit in serialized hero table",
        });
    }

    Ok(slot_index)
}
