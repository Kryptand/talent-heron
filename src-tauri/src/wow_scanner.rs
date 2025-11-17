use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Represents a discovered WoW character
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscoveredCharacter {
    pub name: String,
    pub realm: String,
    pub class: String,
    pub account_id: String,
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
                let path = PathBuf::from(home)
                    .join(".wine/drive_c/Program Files (x86)/World of Warcraft/_retail_");
                if path.exists() {
                    return Some(path);
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

    /// Scan for all characters in the WoW installation
    pub fn scan_characters(&self) -> Result<Vec<DiscoveredCharacter>> {
        let wtf_path = self.wow_path.join("WTF").join("Account");

        if !wtf_path.exists() {
            anyhow::bail!("WTF/Account directory not found at {:?}", wtf_path);
        }

        let mut characters = Vec::new();

        // Iterate through each account folder
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

            // Skip SavedVariables folder (it's at account level, not a character)
            if account_id == "SavedVariables" {
                continue;
            }

            // Scan realms within this account
            let realms_path = account_path.clone();
            if let Ok(realm_entries) = fs::read_dir(&realms_path) {
                for realm_entry in realm_entries {
                    let realm_entry = realm_entry.ok();
                    if realm_entry.is_none() {
                        continue;
                    }
                    let realm_entry = realm_entry.unwrap();
                    let realm_path = realm_entry.path();

                    if !realm_path.is_dir() {
                        continue;
                    }

                    let realm_name = realm_path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("")
                        .to_string();

                    // Skip SavedVariables at this level too
                    if realm_name == "SavedVariables" {
                        continue;
                    }

                    // Scan characters within this realm
                    if let Ok(char_entries) = fs::read_dir(&realm_path) {
                        for char_entry in char_entries {
                            let char_entry = char_entry.ok();
                            if char_entry.is_none() {
                                continue;
                            }
                            let char_entry = char_entry.unwrap();
                            let char_path = char_entry.path();

                            if !char_path.is_dir() {
                                continue;
                            }

                            let char_name = char_path
                                .file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("")
                                .to_string();

                            // Try to determine class from SavedVariables
                            if let Ok(class) = self.detect_character_class(&char_path) {
                                characters.push(DiscoveredCharacter {
                                    name: char_name,
                                    realm: realm_name.clone(),
                                    class,
                                    account_id: account_id.clone(),
                                });
                            }
                        }
                    }
                }
            }
        }

        Ok(characters)
    }

    /// Try to detect character class from SavedVariables
    fn detect_character_class(&self, _char_path: &Path) -> Result<String> {
        // For now, we'll return Unknown - in a full implementation,
        // we'd parse character-specific SavedVariables files to determine class
        // This would require parsing specific addon data files

        // A simple heuristic: check if certain class-specific files exist
        // For now, just return Unknown and let user select
        Ok("Unknown".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_default_wow_path() {
        // This test will pass on systems with WoW installed
        let path = WowScanner::find_default_wow_path();
        // We can't assert it exists since it depends on the system
        println!("Default WoW path: {:?}", path);
    }
}
