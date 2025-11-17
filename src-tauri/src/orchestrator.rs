use crate::archon::{ArchonUrlBuilder, MythicPlusTimespan, RaidDifficulty, TalentIdentifier};
use crate::config::Config;
use crate::fetcher::ArchonFetcher;
use crate::lua_talent::{LuaTalentManager, TalentLoadout};
use crate::wow::WowClass;
use anyhow::{Context, Result};
use serde::Serialize;

/// Summary of the talent update operation
#[derive(Debug, Serialize)]
pub struct UpdateSummary {
    pub total_talents_updated: usize,
    pub raid_talents: usize,
    pub mythic_plus_talents: usize,
    pub characters_processed: usize,
}

/// Orchestrates the entire talent fetch and update process
pub struct TalentOrchestrator {
    config: Config,
    fetcher: ArchonFetcher,
    url_builder: ArchonUrlBuilder,
}

impl TalentOrchestrator {
    /// Create a new orchestrator with the given configuration
    pub fn new(config: Config) -> Self {
        Self {
            config,
            fetcher: ArchonFetcher::new(),
            url_builder: ArchonUrlBuilder::new(),
        }
    }

    /// Run the full talent update process
    pub async fn run(&self) -> Result<UpdateSummary> {
        println!("Starting talent fetch from Archon.gg...");

        let mut raid_talents = 0;
        let mut mythic_plus_talents = 0;

        // Load existing talents
        let mut talent_manager = if self.config.output_path.exists() {
            println!("Loading existing talents from {:?}", self.config.output_path);
            LuaTalentManager::load_from_file(&self.config.output_path)
                .context("Failed to load existing talents")?
        } else {
            println!("No existing talent file found, creating new one");
            LuaTalentManager::new()
        };

        // Clear previous auto-generated builds if requested
        if self.config.clear_previous_builds {
            println!("Clearing all previous auto-generated builds");
            talent_manager.remove_all_auto_generated();
        }

        // Process each character
        for character in &self.config.characters {
            println!("\nProcessing character: {} ({})", character.name, character.class);

            let wow_class = WowClass::from_str(&character.class)
                .ok_or_else(|| anyhow::anyhow!("Invalid class: {}", character.class))?;

            for spec in &character.specializations {
                println!("  Specialization: {}", spec);

                // Validate spec for this class
                let spec_index = wow_class
                    .spec_index(spec)
                    .ok_or_else(|| anyhow::anyhow!("Invalid spec {} for class {}", spec, character.class))?;

                // Clear auto-generated talents for this spec
                if !self.config.clear_previous_builds {
                    talent_manager.remove_auto_generated(wow_class.to_lua_format(), spec_index);
                }

                // Fetch raid builds
                if !self.config.raid_bosses.is_empty() && !self.config.raid_difficulties.is_empty() {
                    raid_talents += self.fetch_raid_builds(&mut talent_manager, wow_class, spec, spec_index)
                        .await?;
                }

                // Fetch Mythic+ builds
                if !self.config.dungeons.is_empty() {
                    mythic_plus_talents += self.fetch_mythic_plus_builds(&mut talent_manager, wow_class, spec, spec_index)
                        .await?;
                }
            }
        }

        // Write updated talents back to file
        println!("\nWriting talents to {:?}", self.config.output_path);
        talent_manager
            .write_to_file(&self.config.output_path)
            .context("Failed to write talents to file")?;

        let summary = UpdateSummary {
            total_talents_updated: raid_talents + mythic_plus_talents,
            raid_talents,
            mythic_plus_talents,
            characters_processed: self.config.characters.len(),
        };

        println!("Talent fetch complete!");
        println!("Summary: {} total talents updated ({} raid, {} M+)",
            summary.total_talents_updated, summary.raid_talents, summary.mythic_plus_talents);

        Ok(summary)
    }

    /// Fetch raid builds for a specific class/spec
    async fn fetch_raid_builds(
        &self,
        talent_manager: &mut LuaTalentManager,
        wow_class: WowClass,
        spec: &str,
        spec_index: u8,
    ) -> Result<usize> {
        let mut count = 0;

        for boss in &self.config.raid_bosses {
            for difficulty_str in &self.config.raid_difficulties {
                let difficulty = RaidDifficulty::from_str(difficulty_str)
                    .ok_or_else(|| anyhow::anyhow!("Invalid difficulty: {}", difficulty_str))?;

                let identifier = TalentIdentifier::Raid {
                    difficulty,
                    boss: boss.clone(),
                };

                let url = self.url_builder.build_raid_url(wow_class, spec, difficulty, boss);

                println!("    Fetching: {} from {}", identifier.as_identifier(), url);

                match self.fetcher.fetch_talent_build(&url).await? {
                    Some(talent_string) => {
                        let talent = TalentLoadout::new(identifier.as_talent_name(), talent_string);
                        talent_manager.add_talent(
                            wow_class.to_lua_format().to_string(),
                            spec_index,
                            talent,
                        );
                        println!("      Found talent build");
                        count += 1;
                    }
                    None => {
                        println!("      No talent build available");
                    }
                }
            }
        }

        Ok(count)
    }

    /// Fetch Mythic+ builds for a specific class/spec
    async fn fetch_mythic_plus_builds(
        &self,
        talent_manager: &mut LuaTalentManager,
        wow_class: WowClass,
        spec: &str,
        spec_index: u8,
    ) -> Result<usize> {
        let mut count = 0;

        for dungeon in &self.config.dungeons {
            let identifier = TalentIdentifier::MythicPlus {
                dungeon: dungeon.clone(),
            };

            // Try primary timespan first
            let primary_timespan = MythicPlusTimespan::primary_for_today();
            let url = self.url_builder.build_mythic_plus_url(wow_class, spec, dungeon, primary_timespan);

            println!("    Fetching: {} from {}", identifier.as_identifier(), url);

            let talent_string = match self.fetcher.fetch_talent_build(&url).await? {
                Some(talent) => {
                    println!("      Found talent build ({})", primary_timespan.as_str());
                    Some(talent)
                }
                None => {
                    // Try fallback timespan
                    let fallback_timespan = primary_timespan.fallback();
                    let fallback_url = self.url_builder.build_mythic_plus_url(
                        wow_class,
                        spec,
                        dungeon,
                        fallback_timespan,
                    );

                    println!("      Trying fallback: {}", fallback_timespan.as_str());

                    match self.fetcher.fetch_talent_build(&fallback_url).await? {
                        Some(talent) => {
                            println!("      Found talent build ({})", fallback_timespan.as_str());
                            Some(talent)
                        }
                        None => {
                            println!("      No talent build available");
                            None
                        }
                    }
                }
            };

            if let Some(talent_string) = talent_string {
                let talent = TalentLoadout::new(identifier.as_talent_name(), talent_string);
                talent_manager.add_talent(
                    wow_class.to_lua_format().to_string(),
                    spec_index,
                    talent,
                );
                count += 1;
            }
        }

        Ok(count)
    }
}
