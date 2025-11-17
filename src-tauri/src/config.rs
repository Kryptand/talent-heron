use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuration structure for the Archon talent fetcher
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    /// List of characters to fetch talents for
    pub characters: Vec<Character>,

    /// Raid difficulties to fetch (e.g., ["heroic", "normal", "mythic"])
    pub raid_difficulties: Vec<String>,

    /// List of raid boss names (lowercase, hyphenated)
    pub raid_bosses: Vec<String>,

    /// List of dungeon names (lowercase, hyphenated)
    pub dungeons: Vec<String>,

    /// Whether to clear all previous auto-generated builds before updating
    /// When false: only removes builds for classes/specs being updated
    /// When true: removes ALL auto-generated builds (with _ARCT suffix)
    pub clear_previous_builds: bool,

    /// Path to TalentLoadoutsEx.lua file
    /// Example: "/Applications/World of Warcraft/_retail_/WTF/Account/400793633#1/SavedVariables/TalentLoadoutsEx.lua"
    pub output_path: PathBuf,
}

/// Character configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Character {
    /// Character name (for identification only)
    pub name: String,

    /// Class name in PascalCase (e.g., "DeathKnight", "DemonHunter", "Warrior")
    pub class: String,

    /// List of specialization names in lowercase (e.g., ["frost", "unholy"])
    pub specializations: Vec<String>,
}

impl Config {
    /// Load configuration from a JSON file
    pub fn from_file(path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&contents)?;
        config.validate()?;
        Ok(config)
    }

    /// Validate configuration settings
    fn validate(&self) -> anyhow::Result<()> {
        if self.characters.is_empty() {
            anyhow::bail!("Configuration must include at least one character");
        }

        if self.raid_difficulties.is_empty() && self.raid_bosses.is_empty() && self.dungeons.is_empty() {
            anyhow::bail!("Configuration must include at least one of: raid difficulties/bosses or dungeons");
        }

        for character in &self.characters {
            if character.class.is_empty() {
                anyhow::bail!("Character '{}' has no class specified", character.name);
            }

            if character.specializations.is_empty() {
                anyhow::bail!("Character '{}' has no specializations specified", character.name);
            }
        }

        Ok(())
    }

    /// Create a default example configuration
    #[allow(dead_code)]
    pub fn example() -> Self {
        Config {
            characters: vec![
                Character {
                    name: "MyWarrior".to_string(),
                    class: "Warrior".to_string(),
                    specializations: vec!["arms".to_string(), "fury".to_string()],
                },
                Character {
                    name: "MyMage".to_string(),
                    class: "Mage".to_string(),
                    specializations: vec!["frost".to_string(), "fire".to_string()],
                },
            ],
            raid_difficulties: vec!["heroic".to_string(), "normal".to_string()],
            raid_bosses: vec![
                "broodtwister".to_string(),
                "sikran".to_string(),
                "queen-ansurek".to_string(),
            ],
            dungeons: vec![
                "ara-kara".to_string(),
                "city-of-threads".to_string(),
                "mists-of-tirna-scithe".to_string(),
            ],
            clear_previous_builds: false,
            output_path: PathBuf::from("/Applications/World of Warcraft/_retail_/WTF/Account/YOUR_ACCOUNT_ID/SavedVariables/TalentLoadoutsEx.lua"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_config_is_valid() {
        let config = Config::example();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_empty_characters_fails_validation() {
        let mut config = Config::example();
        config.characters.clear();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_character_without_class_fails_validation() {
        let mut config = Config::example();
        config.characters[0].class = String::new();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_character_without_specs_fails_validation() {
        let mut config = Config::example();
        config.characters[0].specializations.clear();
        assert!(config.validate().is_err());
    }
}
