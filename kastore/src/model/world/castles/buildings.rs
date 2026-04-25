use std::{collections::BTreeSet, fmt::Display};

/// One concrete castle building or dwelling/upgrade bit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CastleBuilding {
    ThievesGuild,
    Tavern,
    Shipyard,
    Well,
    Statue,
    LeftTurret,
    RightTurret,
    Marketplace,
    Well2,
    Moat,
    Special,
    Castle,
    CaptainQuarters,
    Shrine,
    MageGuild1,
    MageGuild2,
    MageGuild3,
    MageGuild4,
    MageGuild5,
    Tent,
    Dwelling1,
    Dwelling2,
    Dwelling3,
    Dwelling4,
    Dwelling5,
    Dwelling6,
    Upgrade2,
    Upgrade3,
    Upgrade4,
    Upgrade5,
    Upgrade6,
    Upgrade7,
}

impl CastleBuilding {
    pub const ALL: [Self; 32] = [
        Self::ThievesGuild,
        Self::Tavern,
        Self::Shipyard,
        Self::Well,
        Self::Statue,
        Self::LeftTurret,
        Self::RightTurret,
        Self::Marketplace,
        Self::Well2,
        Self::Moat,
        Self::Special,
        Self::Castle,
        Self::CaptainQuarters,
        Self::Shrine,
        Self::MageGuild1,
        Self::MageGuild2,
        Self::MageGuild3,
        Self::MageGuild4,
        Self::MageGuild5,
        Self::Tent,
        Self::Dwelling1,
        Self::Dwelling2,
        Self::Dwelling3,
        Self::Dwelling4,
        Self::Dwelling5,
        Self::Dwelling6,
        Self::Upgrade2,
        Self::Upgrade3,
        Self::Upgrade4,
        Self::Upgrade5,
        Self::Upgrade6,
        Self::Upgrade7,
    ];

    pub const fn bits(self) -> u32 {
        match self {
            Self::ThievesGuild => 0x0000_0001,
            Self::Tavern => 0x0000_0002,
            Self::Shipyard => 0x0000_0004,
            Self::Well => 0x0000_0008,
            Self::Statue => 0x0000_0010,
            Self::LeftTurret => 0x0000_0020,
            Self::RightTurret => 0x0000_0040,
            Self::Marketplace => 0x0000_0080,
            Self::Well2 => 0x0000_0100,
            Self::Moat => 0x0000_0200,
            Self::Special => 0x0000_0400,
            Self::Castle => 0x0000_0800,
            Self::CaptainQuarters => 0x0000_1000,
            Self::Shrine => 0x0000_2000,
            Self::MageGuild1 => 0x0000_4000,
            Self::MageGuild2 => 0x0000_8000,
            Self::MageGuild3 => 0x0001_0000,
            Self::MageGuild4 => 0x0002_0000,
            Self::MageGuild5 => 0x0004_0000,
            Self::Tent => 0x0008_0000,
            Self::Dwelling1 => 0x0010_0000,
            Self::Dwelling2 => 0x0020_0000,
            Self::Dwelling3 => 0x0040_0000,
            Self::Dwelling4 => 0x0080_0000,
            Self::Dwelling5 => 0x0100_0000,
            Self::Dwelling6 => 0x0200_0000,
            Self::Upgrade2 => 0x0400_0000,
            Self::Upgrade3 => 0x0800_0000,
            Self::Upgrade4 => 0x1000_0000,
            Self::Upgrade5 => 0x2000_0000,
            Self::Upgrade6 => 0x4000_0000,
            Self::Upgrade7 => 0x8000_0000,
        }
    }

    pub const fn dwelling_tier(self) -> Option<CastleDwellingTier> {
        match self {
            Self::Dwelling1 => Some(CastleDwellingTier::Tier1),
            Self::Dwelling2 | Self::Upgrade2 => Some(CastleDwellingTier::Tier2),
            Self::Dwelling3 | Self::Upgrade3 => Some(CastleDwellingTier::Tier3),
            Self::Dwelling4 | Self::Upgrade4 => Some(CastleDwellingTier::Tier4),
            Self::Dwelling5 | Self::Upgrade5 => Some(CastleDwellingTier::Tier5),
            Self::Dwelling6 | Self::Upgrade6 | Self::Upgrade7 => Some(CastleDwellingTier::Tier6),
            _ => None,
        }
    }

    pub const fn is_dwelling(self) -> bool {
        self.dwelling_tier().is_some()
    }

    pub const fn is_upgrade(self) -> bool {
        matches!(
            self,
            Self::Upgrade2
                | Self::Upgrade3
                | Self::Upgrade4
                | Self::Upgrade5
                | Self::Upgrade6
                | Self::Upgrade7
        )
    }

    pub const fn is_mage_guild_level(self) -> bool {
        matches!(
            self,
            Self::MageGuild1
                | Self::MageGuild2
                | Self::MageGuild3
                | Self::MageGuild4
                | Self::MageGuild5
        )
    }
}

/// A semantic set of castle buildings backed by a deterministic `BTreeSet`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CastleBuildingSet(BTreeSet<CastleBuilding>);

impl CastleBuildingSet {
    /// Decode a serialized castle building bitmask into a set of buildings.
    pub fn from_mask(mask: u32) -> Self {
        Self(
            CastleBuilding::ALL
                .into_iter()
                .filter(|building| (mask & building.bits()) != 0)
                .collect(),
        )
    }

    /// Encode the building set back into the fheroes2 bitmask representation.
    pub fn to_mask(&self) -> u32 {
        self.0
            .iter()
            .fold(0, |mask, building| mask | building.bits())
    }

    pub fn contains(&self, building: CastleBuilding) -> bool {
        self.0.contains(&building)
    }
}

impl Display for CastleBuilding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::ThievesGuild => "Thieves' Guild",
            Self::Tavern => "Tavern",
            Self::Shipyard => "Shipyard",
            Self::Well => "Well",
            Self::Statue => "Statue",
            Self::LeftTurret => "Left Turret",
            Self::RightTurret => "Right Turret",
            Self::Marketplace => "Marketplace",
            Self::Well2 => "Second Growth Building",
            Self::Moat => "Moat",
            Self::Special => "Special Building",
            Self::Castle => "Castle",
            Self::CaptainQuarters => "Captain's Quarters",
            Self::Shrine => "Shrine",
            Self::MageGuild1 => "Mage Guild Level 1",
            Self::MageGuild2 => "Mage Guild Level 2",
            Self::MageGuild3 => "Mage Guild Level 3",
            Self::MageGuild4 => "Mage Guild Level 4",
            Self::MageGuild5 => "Mage Guild Level 5",
            Self::Tent => "Tent",
            Self::Dwelling1 => "Dwelling 1",
            Self::Dwelling2 => "Dwelling 2",
            Self::Dwelling3 => "Dwelling 3",
            Self::Dwelling4 => "Dwelling 4",
            Self::Dwelling5 => "Dwelling 5",
            Self::Dwelling6 => "Dwelling 6",
            Self::Upgrade2 => "Dwelling Upgrade 2",
            Self::Upgrade3 => "Dwelling Upgrade 3",
            Self::Upgrade4 => "Dwelling Upgrade 4",
            Self::Upgrade5 => "Dwelling Upgrade 5",
            Self::Upgrade6 => "Dwelling Upgrade 6",
            Self::Upgrade7 => "Dwelling Upgrade 7",
        })
    }
}

/// The six serialized castle dwelling population slots.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CastleDwellings {
    pub tier_1: u32,
    pub tier_2: u32,
    pub tier_3: u32,
    pub tier_4: u32,
    pub tier_5: u32,
    pub tier_6: u32,
}

impl CastleDwellings {
    pub const COUNT: usize = 6;

    pub const fn from_counts(counts: [u32; Self::COUNT]) -> Self {
        Self {
            tier_1: counts[0],
            tier_2: counts[1],
            tier_3: counts[2],
            tier_4: counts[3],
            tier_5: counts[4],
            tier_6: counts[5],
        }
    }

    pub const fn counts(self) -> [u32; Self::COUNT] {
        [
            self.tier_1,
            self.tier_2,
            self.tier_3,
            self.tier_4,
            self.tier_5,
            self.tier_6,
        ]
    }

    pub const fn count(self, tier: CastleDwellingTier) -> u32 {
        match tier {
            CastleDwellingTier::Tier1 => self.tier_1,
            CastleDwellingTier::Tier2 => self.tier_2,
            CastleDwellingTier::Tier3 => self.tier_3,
            CastleDwellingTier::Tier4 => self.tier_4,
            CastleDwellingTier::Tier5 => self.tier_5,
            CastleDwellingTier::Tier6 => self.tier_6,
        }
    }

    pub fn set_count(&mut self, tier: CastleDwellingTier, count: u32) {
        match tier {
            CastleDwellingTier::Tier1 => self.tier_1 = count,
            CastleDwellingTier::Tier2 => self.tier_2 = count,
            CastleDwellingTier::Tier3 => self.tier_3 = count,
            CastleDwellingTier::Tier4 => self.tier_4 = count,
            CastleDwellingTier::Tier5 => self.tier_5 = count,
            CastleDwellingTier::Tier6 => self.tier_6 = count,
        }
    }
}

impl Display for CastleDwellings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "T1={} T2={} T3={} T4={} T5={} T6={}",
            self.tier_1, self.tier_2, self.tier_3, self.tier_4, self.tier_5, self.tier_6
        )
    }
}

/// One of the six serialized dwelling population tiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CastleDwellingTier {
    Tier1,
    Tier2,
    Tier3,
    Tier4,
    Tier5,
    Tier6,
}

impl CastleDwellingTier {
    pub const fn index(self) -> usize {
        match self {
            Self::Tier1 => 0,
            Self::Tier2 => 1,
            Self::Tier3 => 2,
            Self::Tier4 => 3,
            Self::Tier5 => 4,
            Self::Tier6 => 5,
        }
    }
}

impl Display for CastleDwellingTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tier {}", self.index() + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::{CastleBuilding, CastleBuildingSet, CastleDwellingTier, CastleDwellings};

    #[test]
    fn building_set_from_mask_round_trips_known_bits() {
        let mask = CastleBuilding::Well.bits()
            | CastleBuilding::Special.bits()
            | CastleBuilding::Upgrade4.bits();
        let buildings = CastleBuildingSet::from_mask(mask);

        assert!(buildings.contains(CastleBuilding::Well));
        assert!(buildings.contains(CastleBuilding::Special));
        assert!(buildings.contains(CastleBuilding::Upgrade4));
        assert!(!buildings.contains(CastleBuilding::Castle));
        assert_eq!(buildings.to_mask(), mask);
    }

    #[test]
    fn dwellings_round_trip_fixed_six_slot_layout() {
        let dwellings = CastleDwellings::from_counts([11, 22, 33, 44, 55, 66]);

        assert_eq!(dwellings.count(CastleDwellingTier::Tier4), 44);
        assert_eq!(dwellings.counts(), [11, 22, 33, 44, 55, 66]);
    }
}
