use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Represents a discovered WoW character
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscoveredCharacter {
    pub name: String,
    pub realm: String,
    pub class: String,
    pub account_id: String,
    /// Unix timestamp of last login (from config-cache.wtf mtime), 0 if unknown
    pub last_played: u64,
}

/// Scanner for finding WoW installation and characters
pub struct WowScanner {
    wow_path: PathBuf,
}

impl WowScanner {
    /// Create a new scanner with the given WoW installation path
    pub fn new(wow_path: impl Into<PathBuf>) -> Self {
        Self {
            wow_path: wow_path.into(),
        }
    }

    /// Find the default WoW installation path based on the platform
    pub fn find_default_wow_path() -> Option<PathBuf> {
        #[cfg(target_os = "macos")]
        {
            let path = PathBuf::from("/Applications/World of Warcraft/_retail_");
            if path.exists() {
                return Some(path);
            }
        }

        #[cfg(target_os = "windows")]
        {
            let paths = vec![
                PathBuf::from("C:\\Program Files (x86)\\World of Warcraft\\_retail_"),
                PathBuf::from("C:\\Program Files\\World of Warcraft\\_retail_"),
            ];
            for path in paths {
                if path.exists() {
                    return Some(path);
                }
            }
        }

        #[cfg(target_os = "linux")]
        {
            if let Ok(home) = std::env::var("HOME") {
                let candidates = [
                    PathBuf::from(&home).join("Games/World of Warcraft/_retail_"),
                    PathBuf::from(&home).join(".wine/drive_c/Program Files (x86)/World of Warcraft/_retail_"),
                    PathBuf::from(&home).join("Games/battlenet/World of Warcraft/_retail_"),
                    PathBuf::from("/opt/games/World of Warcraft/_retail_"),
                ];
                for path in &candidates {
                    if path.exists() {
                        return Some(path.clone());
                    }
                }
            }
        }

        None
    }

    /// Get the path to TalentLoadoutsEx.lua for a specific account
    #[allow(dead_code)]
    pub fn get_talent_loadouts_path(&self, account_id: &str) -> PathBuf {
        self.wow_path
            .join("WTF")
            .join("Account")
            .join(account_id)
            .join("SavedVariables")
            .join("TalentLoadoutsEx.lua")
    }

    /// Scan for all characters in the WoW installation, sorted by most recently played
    pub fn scan_characters(&self) -> Result<Vec<DiscoveredCharacter>> {
        let wtf_path = self.wow_path.join("WTF").join("Account");

        if !wtf_path.exists() {
            anyhow::bail!("WTF/Account directory not found at {:?}", wtf_path);
        }

        let mut characters = Vec::new();

        for account_entry in fs::read_dir(&wtf_path)? {
            let account_entry = account_entry?;
            let account_path = account_entry.path();

            if !account_path.is_dir() {
                continue;
            }

            let account_id = account_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();

            if account_id == "SavedVariables" {
                continue;
            }

            if let Ok(realm_entries) = fs::read_dir(&account_path) {
                for realm_entry in realm_entries.flatten() {
                    let realm_path = realm_entry.path();

                    if !realm_path.is_dir() {
                        continue;
                    }

                    let realm_name = realm_path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("")
                        .to_string();

                    if realm_name == "SavedVariables" {
                        continue;
                    }

                    if let Ok(char_entries) = fs::read_dir(&realm_path) {
                        for char_entry in char_entries.flatten() {
                            let char_path = char_entry.path();

                            if !char_path.is_dir() {
                                continue;
                            }

                            let char_name = char_path
                                .file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("")
                                .to_string();

                            if let Ok((class, last_played)) = self.detect_character_info(&char_path) {
                                characters.push(DiscoveredCharacter {
                                    name: char_name,
                                    realm: realm_name.clone(),
                                    class,
                                    account_id: account_id.clone(),
                                    last_played,
                                });
                            }
                        }
                    }
                }
            }
        }

        // Sort by most recently played first
        characters.sort_by(|a, b| b.last_played.cmp(&a.last_played));

        Ok(characters)
    }

    /// Detect class and last-played time from config-cache.wtf.
    /// Returns an error if the class cannot be determined (skips the character).
    fn detect_character_info(&self, char_path: &Path) -> Result<(String, u64)> {
        let config_path = char_path.join("config-cache.wtf");
        let contents = fs::read_to_string(&config_path)?;

        // Get last-modified time as a proxy for last played
        let last_played = fs::metadata(&config_path)
            .ok()
            .and_then(|m| m.modified().ok())
            .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
            .map(|d| d.as_secs())
            .unwrap_or(0);

        for line in contents.lines() {
            let line = line.trim();
            if let Some(rest) = line.strip_prefix("SET EJLootClass \"") {
                let id_str = rest.trim_end_matches('"').trim();
                if let Ok(id) = id_str.parse::<u8>() {
                    // ID 0 = character slot exists but no class (deleted/placeholder)
                    if id == 0 {
                        anyhow::bail!("character has no class (id=0)");
                    }
                    let class = Self::class_id_to_name(id);
                    if class == "Unknown" {
                        anyhow::bail!("unrecognized class id: {}", id);
                    }
                    return Ok((class.to_string(), last_played));
                }
            }
        }

        anyhow::bail!("EJLootClass not found in config-cache.wtf")
    }

    fn class_id_to_name(id: u8) -> &'static str {
        match id {
            1 => "Warrior",
            2 => "Paladin",
            3 => "Hunter",
            4 => "Rogue",
            5 => "Priest",
            6 => "DeathKnight",
            7 => "Shaman",
            8 => "Mage",
            9 => "Warlock",
            10 => "Monk",
            11 => "Druid",
            12 => "DemonHunter",
            13 => "Evoker",
            _ => "Unknown",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_default_wow_path() {
        let path = WowScanner::find_default_wow_path();
        println!("Default WoW path: {:?}", path);
    }
}
