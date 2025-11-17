use std::collections::HashMap;

/// WoW class representation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WowClass {
    Warrior,
    Paladin,
    Hunter,
    Rogue,
    Priest,
    DeathKnight,
    Shaman,
    Mage,
    Warlock,
    Monk,
    Druid,
    DemonHunter,
    Evoker,
}

impl WowClass {
    /// Parse class from string (PascalCase)
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Warrior" => Some(Self::Warrior),
            "Paladin" => Some(Self::Paladin),
            "Hunter" => Some(Self::Hunter),
            "Rogue" => Some(Self::Rogue),
            "Priest" => Some(Self::Priest),
            "DeathKnight" => Some(Self::DeathKnight),
            "Shaman" => Some(Self::Shaman),
            "Mage" => Some(Self::Mage),
            "Warlock" => Some(Self::Warlock),
            "Monk" => Some(Self::Monk),
            "Druid" => Some(Self::Druid),
            "DemonHunter" => Some(Self::DemonHunter),
            "Evoker" => Some(Self::Evoker),
            _ => None,
        }
    }

    /// Convert class to URL-safe format for Archon.gg
    /// Most classes use lowercase, but DeathKnight and DemonHunter use hyphens
    pub fn to_url_format(&self) -> &'static str {
        match self {
            Self::Warrior => "warrior",
            Self::Paladin => "paladin",
            Self::Hunter => "hunter",
            Self::Rogue => "rogue",
            Self::Priest => "priest",
            Self::DeathKnight => "death-knight",
            Self::Shaman => "shaman",
            Self::Mage => "mage",
            Self::Warlock => "warlock",
            Self::Monk => "monk",
            Self::Druid => "druid",
            Self::DemonHunter => "demon-hunter",
            Self::Evoker => "evoker",
        }
    }

    /// Convert class to uppercase format for Lua files (e.g., "WARRIOR")
    pub fn to_lua_format(&self) -> &'static str {
        match self {
            Self::Warrior => "WARRIOR",
            Self::Paladin => "PALADIN",
            Self::Hunter => "HUNTER",
            Self::Rogue => "ROGUE",
            Self::Priest => "PRIEST",
            Self::DeathKnight => "DEATHKNIGHT",
            Self::Shaman => "SHAMAN",
            Self::Mage => "MAGE",
            Self::Warlock => "WARLOCK",
            Self::Monk => "MONK",
            Self::Druid => "DRUID",
            Self::DemonHunter => "DEMONHUNTER",
            Self::Evoker => "EVOKER",
        }
    }

    /// Get the specialization index for a given spec name
    pub fn spec_index(&self, spec_name: &str) -> Option<u8> {
        let spec_map = self.get_spec_map();
        spec_map.get(spec_name).copied()
    }

    /// Get all valid specializations for this class
    #[allow(dead_code)]
    pub fn valid_specs(&self) -> Vec<&'static str> {
        self.get_spec_map().keys().copied().collect()
    }

    /// Internal helper to get the spec name -> index mapping
    fn get_spec_map(&self) -> HashMap<&'static str, u8> {
        match self {
            Self::Warrior => {
                let mut map = HashMap::new();
                map.insert("arms", 1);
                map.insert("fury", 2);
                map.insert("protection", 3);
                map
            }
            Self::Paladin => {
                let mut map = HashMap::new();
                map.insert("holy", 1);
                map.insert("protection", 2);
                map.insert("retribution", 3);
                map
            }
            Self::Hunter => {
                let mut map = HashMap::new();
                map.insert("beast-mastery", 1);
                map.insert("marksmanship", 2);
                map.insert("survival", 3);
                map
            }
            Self::Rogue => {
                let mut map = HashMap::new();
                map.insert("assassination", 1);
                map.insert("combat", 2);
                map.insert("subtlety", 3);
                map
            }
            Self::Priest => {
                let mut map = HashMap::new();
                map.insert("discipline", 1);
                map.insert("holy", 2);
                map.insert("shadow", 3);
                map
            }
            Self::DeathKnight => {
                let mut map = HashMap::new();
                map.insert("blood", 1);
                map.insert("frost", 2);
                map.insert("unholy", 3);
                map
            }
            Self::Shaman => {
                let mut map = HashMap::new();
                map.insert("elemental", 1);
                map.insert("enhancement", 2);
                map.insert("restoration", 3);
                map
            }
            Self::Mage => {
                let mut map = HashMap::new();
                map.insert("arcane", 1);
                map.insert("fire", 2);
                map.insert("frost", 3);
                map
            }
            Self::Warlock => {
                let mut map = HashMap::new();
                map.insert("affliction", 1);
                map.insert("demonology", 2);
                map.insert("destruction", 3);
                map
            }
            Self::Monk => {
                let mut map = HashMap::new();
                map.insert("brewmaster", 1);
                map.insert("mistweaver", 2);
                map.insert("windwalker", 3);
                map
            }
            Self::Druid => {
                let mut map = HashMap::new();
                map.insert("balance", 1);
                map.insert("feral", 2);
                map.insert("guardian", 3);
                map.insert("restoration", 4);
                map
            }
            Self::DemonHunter => {
                let mut map = HashMap::new();
                map.insert("havoc", 1);
                map.insert("vengeance", 2);
                map
            }
            Self::Evoker => {
                let mut map = HashMap::new();
                map.insert("devastation", 1);
                map.insert("preservation", 2);
                map
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_class_from_string() {
        assert_eq!(WowClass::from_str("Warrior"), Some(WowClass::Warrior));
        assert_eq!(
            WowClass::from_str("DeathKnight"),
            Some(WowClass::DeathKnight)
        );
        assert_eq!(
            WowClass::from_str("DemonHunter"),
            Some(WowClass::DemonHunter)
        );
        assert_eq!(WowClass::from_str("InvalidClass"), None);
    }

    #[test]
    fn test_class_to_url_format() {
        assert_eq!(WowClass::Warrior.to_url_format(), "warrior");
        assert_eq!(WowClass::DeathKnight.to_url_format(), "death-knight");
        assert_eq!(WowClass::DemonHunter.to_url_format(), "demon-hunter");
    }

    #[test]
    fn test_class_to_lua_format() {
        assert_eq!(WowClass::Warrior.to_lua_format(), "WARRIOR");
        assert_eq!(WowClass::DeathKnight.to_lua_format(), "DEATHKNIGHT");
        assert_eq!(WowClass::DemonHunter.to_lua_format(), "DEMONHUNTER");
    }

    #[test]
    fn test_spec_indices() {
        assert_eq!(WowClass::Warrior.spec_index("arms"), Some(1));
        assert_eq!(WowClass::Warrior.spec_index("fury"), Some(2));
        assert_eq!(WowClass::Warrior.spec_index("protection"), Some(3));
        assert_eq!(WowClass::Warrior.spec_index("invalid"), None);

        assert_eq!(WowClass::DeathKnight.spec_index("blood"), Some(1));
        assert_eq!(WowClass::DeathKnight.spec_index("frost"), Some(2));
        assert_eq!(WowClass::DeathKnight.spec_index("unholy"), Some(3));

        assert_eq!(WowClass::Mage.spec_index("arcane"), Some(1));
        assert_eq!(WowClass::Mage.spec_index("fire"), Some(2));
        assert_eq!(WowClass::Mage.spec_index("frost"), Some(3));

        // Druid has 4 specs
        assert_eq!(WowClass::Druid.spec_index("balance"), Some(1));
        assert_eq!(WowClass::Druid.spec_index("feral"), Some(2));
        assert_eq!(WowClass::Druid.spec_index("guardian"), Some(3));
        assert_eq!(WowClass::Druid.spec_index("restoration"), Some(4));
    }

    #[test]
    fn test_valid_specs() {
        let warrior_specs = WowClass::Warrior.valid_specs();
        assert!(warrior_specs.contains(&"arms"));
        assert!(warrior_specs.contains(&"fury"));
        assert!(warrior_specs.contains(&"protection"));
        assert_eq!(warrior_specs.len(), 3);

        let druid_specs = WowClass::Druid.valid_specs();
        assert_eq!(druid_specs.len(), 4);
    }
}
