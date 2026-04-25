use crate::Error;
use crate::internal::error::ParseSection;
use crate::internal::reader::Reader;
use crate::internal::writer::Writer;
use crate::model::campaign_save_data::{CampaignSaveData, ScenarioInfoId};
use crate::model::world::heroes::army::{MonsterType, Troop};

pub(crate) fn decode(reader: &mut Reader<'_>) -> std::result::Result<CampaignSaveData, Error> {
    reader.set_section(ParseSection::Campaign);
    let current_scenario_info = ScenarioInfoId {
        campaign_id: reader.read_i32_be("current scenario campaign id")?,
        scenario_id: reader.read_i32_be("current scenario scenario id")?,
    };
    let current_scenario_bonus_id = reader.read_i32_be("current scenario bonus id")?;

    let finished_maps = reader.read_vec_u32("finished maps", |reader| {
        Ok(ScenarioInfoId {
            campaign_id: reader.read_i32_be("finished map campaign id")?,
            scenario_id: reader.read_i32_be("finished map scenario id")?,
        })
    })?;

    let bonuses_for_finished_maps = reader.read_vec_u32("finished map bonuses", |reader| {
        reader.read_i32_be("finished map bonus")
    })?;

    let days_passed_per_finished_map = reader.read_vec_u32("finished map days", |reader| {
        reader.read_u32_be("finished map days entry")
    })?;

    let obtained_campaign_awards = reader.read_vec_u32("obtained campaign awards", |reader| {
        reader.read_i32_be("obtained campaign award")
    })?;

    let carry_over_troops = reader.read_vec_u32("carry-over troops", |reader| {
        Ok(Troop {
            monster: MonsterType::from_i32(reader.read_i32_be("campaign troop monster")?),
            count: reader.read_u32_be("campaign troop count")?,
        })
    })?;

    let difficulty = reader.read_i32_be("campaign difficulty")?;
    let min_difficulty = reader.read_i32_be("campaign min difficulty")?;

    Ok(CampaignSaveData {
        current_scenario_info,
        current_scenario_bonus_id,
        finished_maps,
        bonuses_for_finished_maps,
        days_passed_per_finished_map,
        obtained_campaign_awards,
        carry_over_troops,
        difficulty,
        min_difficulty,
    })
}

pub(crate) fn encode(
    writer: &mut Writer,
    campaign_save_data: &CampaignSaveData,
) -> std::result::Result<(), Error> {
    if campaign_save_data.bonuses_for_finished_maps.len() != campaign_save_data.finished_maps.len()
    {
        return Err(Error::InvalidModel {
            field: "campaign bonuses_for_finished_maps",
            message: "bonuses_for_finished_maps length must match finished_maps length",
        });
    }
    if campaign_save_data.days_passed_per_finished_map.len()
        != campaign_save_data.finished_maps.len()
    {
        return Err(Error::InvalidModel {
            field: "campaign days_passed_per_finished_map",
            message: "days_passed_per_finished_map length must match finished_maps length",
        });
    }

    writer.write_i32_be(campaign_save_data.current_scenario_info.campaign_id);
    writer.write_i32_be(campaign_save_data.current_scenario_info.scenario_id);
    writer.write_i32_be(campaign_save_data.current_scenario_bonus_id);

    writer.write_len_u32(
        campaign_save_data.finished_maps.len(),
        "campaign finished maps",
    )?;
    for entry in &campaign_save_data.finished_maps {
        writer.write_i32_be(entry.campaign_id);
        writer.write_i32_be(entry.scenario_id);
    }

    writer.write_len_u32(
        campaign_save_data.bonuses_for_finished_maps.len(),
        "campaign finished map bonuses",
    )?;
    for bonus in &campaign_save_data.bonuses_for_finished_maps {
        writer.write_i32_be(*bonus);
    }

    writer.write_len_u32(
        campaign_save_data.days_passed_per_finished_map.len(),
        "campaign finished map days",
    )?;
    for days in &campaign_save_data.days_passed_per_finished_map {
        writer.write_u32_be(*days);
    }

    writer.write_len_u32(
        campaign_save_data.obtained_campaign_awards.len(),
        "campaign obtained awards",
    )?;
    for award in &campaign_save_data.obtained_campaign_awards {
        writer.write_i32_be(*award);
    }

    writer.write_len_u32(
        campaign_save_data.carry_over_troops.len(),
        "campaign carry-over troops",
    )?;
    for troop in &campaign_save_data.carry_over_troops {
        writer.write_i32_be(troop.monster.to_i32());
        writer.write_u32_be(troop.count);
    }

    writer.write_i32_be(campaign_save_data.difficulty);
    writer.write_i32_be(campaign_save_data.min_difficulty);

    Ok(())
}
