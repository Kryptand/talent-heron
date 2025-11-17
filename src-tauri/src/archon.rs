use crate::wow::WowClass;
use chrono::Datelike;

/// Content type for Archon.gg builds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ContentType {
    Raid,
    MythicPlus,
}

/// Raid difficulty levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RaidDifficulty {
    Normal,
    Heroic,
    Mythic,
}

impl RaidDifficulty {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "normal" => Some(Self::Normal),
            "heroic" => Some(Self::Heroic),
            "mythic" => Some(Self::Mythic),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Heroic => "heroic",
            Self::Mythic => "mythic",
        }
    }
}

/// Mythic+ time period for build data
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MythicPlusTimespan {
    ThisWeek,
    LastWeek,
}

impl MythicPlusTimespan {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ThisWeek => "this-week",
            Self::LastWeek => "last-week",
        }
    }

    /// Determine the primary timespan based on the current day
    /// On Wednesday (reset day), prefer last-week first
    /// On other days, prefer this-week first
    pub fn primary_for_today() -> Self {
        let now = chrono::Local::now();
        if now.weekday() == chrono::Weekday::Wed {
            Self::LastWeek
        } else {
            Self::ThisWeek
        }
    }

    /// Get the fallback timespan (the opposite of the current one)
    pub fn fallback(&self) -> Self {
        match self {
            Self::ThisWeek => Self::LastWeek,
            Self::LastWeek => Self::ThisWeek,
        }
    }
}

/// Talent identifier for generated builds
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TalentIdentifier {
    Raid {
        difficulty: RaidDifficulty,
        boss: String,
    },
    MythicPlus {
        dungeon: String,
    },
}

impl TalentIdentifier {
    /// Generate the identifier string (e.g., "R-heroic-sikran" or "M+-ara-kara")
    pub fn as_identifier(&self) -> String {
        match self {
            Self::Raid { difficulty, boss } => {
                format!("R-{}-{}", difficulty.as_str(), boss)
            }
            Self::MythicPlus { dungeon } => {
                format!("M+-{}", dungeon)
            }
        }
    }

    /// Generate the full name with _ARCT suffix for auto-generated talents
    pub fn as_talent_name(&self) -> String {
        format!("{}_ARCT", self.as_identifier())
    }
}

/// URL builder for Archon.gg talent builds
pub struct ArchonUrlBuilder {
    base_url: String,
}

impl Default for ArchonUrlBuilder {
    fn default() -> Self {
        Self {
            base_url: "https://www.archon.gg/wow/builds".to_string(),
        }
    }
}

impl ArchonUrlBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Build URL for a raid boss talent build
    /// Format: https://www.archon.gg/wow/builds/{spec}/{class}/raid/overview/{difficulty}/{encounter}
    pub fn build_raid_url(
        &self,
        class: WowClass,
        spec: &str,
        difficulty: RaidDifficulty,
        boss: &str,
    ) -> String {
        format!(
            "{}/{}/{}/raid/overview/{}/{}",
            self.base_url,
            spec.to_lowercase(),
            class.to_url_format(),
            difficulty.as_str(),
            boss.to_lowercase()
        )
    }

    /// Build URL for a Mythic+ dungeon talent build
    /// Format: https://www.archon.gg/wow/builds/{spec}/{class}/mythic-plus/overview/10//{dungeon}/{timespan}
    /// Note the double slash (//) where difficulty would be for raids
    pub fn build_mythic_plus_url(
        &self,
        class: WowClass,
        spec: &str,
        dungeon: &str,
        timespan: MythicPlusTimespan,
    ) -> String {
        format!(
            "{}/{}/{}/mythic-plus/overview/10//{}/{}",
            self.base_url,
            spec.to_lowercase(),
            class.to_url_format(),
            dungeon.to_lowercase(),
            timespan.as_str()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raid_difficulty_parsing() {
        assert_eq!(RaidDifficulty::from_str("normal"), Some(RaidDifficulty::Normal));
        assert_eq!(RaidDifficulty::from_str("heroic"), Some(RaidDifficulty::Heroic));
        assert_eq!(RaidDifficulty::from_str("mythic"), Some(RaidDifficulty::Mythic));
        assert_eq!(RaidDifficulty::from_str("HEROIC"), Some(RaidDifficulty::Heroic));
        assert_eq!(RaidDifficulty::from_str("invalid"), None);
    }

    #[test]
    fn test_mythic_plus_timespan_fallback() {
        assert_eq!(
            MythicPlusTimespan::ThisWeek.fallback(),
            MythicPlusTimespan::LastWeek
        );
        assert_eq!(
            MythicPlusTimespan::LastWeek.fallback(),
            MythicPlusTimespan::ThisWeek
        );
    }

    #[test]
    fn test_talent_identifier_raid() {
        let id = TalentIdentifier::Raid {
            difficulty: RaidDifficulty::Heroic,
            boss: "sikran".to_string(),
        };
        assert_eq!(id.as_identifier(), "R-heroic-sikran");
        assert_eq!(id.as_talent_name(), "R-heroic-sikran_ARCT");
    }

    #[test]
    fn test_talent_identifier_mythic_plus() {
        let id = TalentIdentifier::MythicPlus {
            dungeon: "ara-kara".to_string(),
        };
        assert_eq!(id.as_identifier(), "M+-ara-kara");
        assert_eq!(id.as_talent_name(), "M+-ara-kara_ARCT");
    }

    #[test]
    fn test_build_raid_url() {
        let builder = ArchonUrlBuilder::new();
        let url = builder.build_raid_url(
            WowClass::Mage,
            "frost",
            RaidDifficulty::Heroic,
            "broodtwister",
        );
        assert_eq!(
            url,
            "https://www.archon.gg/wow/builds/frost/mage/raid/overview/heroic/broodtwister"
        );
    }

    #[test]
    fn test_build_raid_url_death_knight() {
        let builder = ArchonUrlBuilder::new();
        let url = builder.build_raid_url(
            WowClass::DeathKnight,
            "unholy",
            RaidDifficulty::Heroic,
            "sikran",
        );
        assert_eq!(
            url,
            "https://www.archon.gg/wow/builds/unholy/death-knight/raid/overview/heroic/sikran"
        );
    }

    #[test]
    fn test_build_mythic_plus_url() {
        let builder = ArchonUrlBuilder::new();
        let url = builder.build_mythic_plus_url(
            WowClass::Warrior,
            "protection",
            "ara-kara",
            MythicPlusTimespan::ThisWeek,
        );
        assert_eq!(
            url,
            "https://www.archon.gg/wow/builds/protection/warrior/mythic-plus/overview/10//ara-kara/this-week"
        );
    }

    #[test]
    fn test_build_mythic_plus_url_with_last_week() {
        let builder = ArchonUrlBuilder::new();
        let url = builder.build_mythic_plus_url(
            WowClass::DeathKnight,
            "unholy",
            "mists-of-tirna-scithe",
            MythicPlusTimespan::LastWeek,
        );
        assert_eq!(
            url,
            "https://www.archon.gg/wow/builds/unholy/death-knight/mythic-plus/overview/10//mists-of-tirna-scithe/last-week"
        );
    }

    #[test]
    fn test_url_double_slash_for_mythic_plus() {
        // Verify that M+ URLs have the double slash where difficulty would be
        let builder = ArchonUrlBuilder::new();
        let url = builder.build_mythic_plus_url(
            WowClass::Mage,
            "fire",
            "city-of-threads",
            MythicPlusTimespan::ThisWeek,
        );
        assert!(url.contains("overview/10//city-of-threads"));
    }
}
