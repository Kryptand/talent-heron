use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

const RAID_DISCOVERY_URL: &str =
    "https://www.archon.gg/wow/builds/frost/mage/raid/overview/heroic/imperator";
const MYTHIC_PLUS_DISCOVERY_URL: &str =
    "https://www.archon.gg/wow/builds/frost/mage/mythic-plus/overview/10/maisara-caverns/this-week";

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscoveredContent {
    pub raid_bosses: Vec<String>,
    pub dungeons: Vec<String>,
}

pub struct WarcraftLogsService;

impl WarcraftLogsService {
    pub async fn discover_current_content() -> Result<DiscoveredContent> {
        let client = Client::builder()
            .user_agent("ArchonConfigUpdater/1.0")
            .build()
            .context("Failed to create HTTP client")?;

        let (raid_html, mp_html) = tokio::try_join!(
            Self::fetch(&client, RAID_DISCOVERY_URL),
            Self::fetch(&client, MYTHIC_PLUS_DISCOVERY_URL),
        )?;

        let raid_bosses = Self::extract_raid_bosses(&raid_html)?;
        let dungeons = Self::extract_dungeons(&mp_html)?;

        Ok(DiscoveredContent {
            raid_bosses,
            dungeons,
        })
    }

    async fn fetch(client: &Client, url: &str) -> Result<String> {
        client
            .get(url)
            .send()
            .await
            .context("Failed to fetch page")?
            .text()
            .await
            .context("Failed to read response body")
    }

    /// Extract boss slugs from the JSON data embedded in the Archon.gg raid page.
    /// Looks for: "url":"/wow/builds/frost/mage/raid/overview/heroic/{slug}"
    fn extract_raid_bosses(html: &str) -> Result<Vec<String>> {
        const PREFIX: &str = "\"/wow/builds/frost/mage/raid/overview/heroic/";
        let mut seen = HashSet::new();
        let mut bosses = Vec::new();

        let mut remaining = html;
        while let Some(pos) = remaining.find(PREFIX) {
            remaining = &remaining[pos + PREFIX.len()..];
            if let Some(end) = remaining.find('"') {
                let slug = &remaining[..end];
                if !slug.is_empty() && !slug.contains('#') && slug != "all-bosses" {
                    if seen.insert(slug.to_string()) {
                        bosses.push(slug.to_string());
                    }
                }
                remaining = &remaining[end..];
            }
        }

        Ok(bosses)
    }

    /// Extract dungeon slugs from the JSON data embedded in the Archon.gg M+ page.
    /// Looks for: "url":"/wow/builds/frost/mage/mythic-plus/overview/10//{slug}/this-week"
    fn extract_dungeons(html: &str) -> Result<Vec<String>> {
        const PREFIX: &str = "\"/wow/builds/frost/mage/mythic-plus/overview/10/";
        let mut seen = HashSet::new();
        let mut dungeons = Vec::new();

        let mut remaining = html;
        while let Some(pos) = remaining.find(PREFIX) {
            remaining = &remaining[pos + PREFIX.len()..];
            if let Some(end) = remaining.find('"') {
                let full = &remaining[..end];
                // full is like "maisara-caverns/this-week" — take only the slug part
                let slug = full.split('/').next().unwrap_or("");
                if !slug.is_empty() && slug != "all-dungeons" {
                    if seen.insert(slug.to_string()) {
                        dungeons.push(slug.to_string());
                    }
                }
                remaining = &remaining[end..];
            }
        }

        Ok(dungeons)
    }
}
