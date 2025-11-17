use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

const WARCRAFT_LOGS_API: &str = "https://www.warcraftlogs.com/zone-sidebar/v2/";

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscoveredContent {
    pub raid_bosses: Vec<String>,
    pub dungeons: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ZoneSidebarResponse {
    #[serde(default)]
    #[allow(dead_code)]
    title: String,
    id: String,
    #[serde(default)]
    expansions: Vec<Expansion>,
}

#[derive(Debug, Deserialize)]
struct Expansion {
    #[serde(default)]
    #[allow(dead_code)]
    title: String,
    #[serde(default)]
    #[allow(dead_code)]
    id: String,
    #[serde(default)]
    panel: Option<Panel>,
}

#[derive(Debug, Deserialize)]
struct Panel {
    #[serde(default)]
    sections: Vec<Section>,
}

#[derive(Debug, Deserialize)]
struct Section {
    #[serde(default)]
    header: Option<Header>,
    #[serde(default)]
    children: Vec<Child>,
}

#[derive(Debug, Deserialize)]
struct Header {
    #[serde(rename = "contentTypeName", default)]
    content_type_name: String,
}

#[derive(Debug, Deserialize)]
struct Child {
    #[serde(default)]
    title: String,
    #[serde(rename = "type", default)]
    child_type: String,
}

pub struct WarcraftLogsService;

impl WarcraftLogsService {
    pub async fn discover_current_content() -> Result<DiscoveredContent> {
        // Fetch data from Warcraft Logs API
        let response = reqwest::get(WARCRAFT_LOGS_API)
            .await
            .context("Failed to fetch from Warcraft Logs API")?;

        let data: Vec<ZoneSidebarResponse> = response
            .json()
            .await
            .context("Failed to parse Warcraft Logs response")?;

        let mut raid_bosses = Vec::new();
        let mut dungeons = Vec::new();

        // Get raid bosses
        if let Some(raid_section) = data.iter().find(|x| x.id == "raid-content") {
            if let Some(current_expansion) = raid_section.expansions.first() {
                if let Some(panel) = &current_expansion.panel {
                    for section in &panel.sections {
                        if let Some(header) = &section.header {
                            if header.content_type_name == "zones" {
                                for child in &section.children {
                                    if child.child_type == "boss" && !child.title.is_empty() {
                                        let boss_slug = Self::to_slug(&child.title);
                                        raid_bosses.push(boss_slug);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Get dungeons (M+ season)
        if let Some(dungeon_section) = data.iter().find(|x| x.id == "dungeons-content") {
            if let Some(current_expansion) = dungeon_section.expansions.first() {
                if let Some(panel) = &current_expansion.panel {
                    // Get the first section (current season)
                    if let Some(current_season) = panel.sections.first() {
                        for child in &current_season.children {
                            if child.child_type == "boss" && !child.title.is_empty() {
                                let dungeon_slug = Self::to_slug(&child.title);
                                dungeons.push(dungeon_slug);
                            }
                        }
                    }
                }
            }
        }

        Ok(DiscoveredContent {
            raid_bosses,
            dungeons,
        })
    }

    /// Convert a name to a URL-friendly slug (lowercase with hyphens)
    /// Matches the C# implementation in ConvertToUrlFriendlyName
    fn to_slug(name: &str) -> String {
        name.replace("'", "")
            .replace(",", "")
            .replace(":", "")
            .replace("\"", "")
            .replace("(", "")
            .replace(")", "")
            .replace(".", "")
            .replace("!", "")
            .replace("&", "")
            .trim()
            .to_lowercase()
            .replace(" ", "-")
            .replace("--", "-")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_slug() {
        assert_eq!(
            WarcraftLogsService::to_slug("Queen Ansurek"),
            "queen-ansurek"
        );
        assert_eq!(
            WarcraftLogsService::to_slug("The Dawnbreaker"),
            "the-dawnbreaker"
        );
        assert_eq!(
            WarcraftLogsService::to_slug("Mists of Tirna Scithe"),
            "mists-of-tirna-scithe"
        );
    }
}
