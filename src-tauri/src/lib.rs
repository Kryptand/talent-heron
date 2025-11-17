// Modules
mod archon;
mod config;
mod fetcher;
mod lua_talent;
mod orchestrator;
mod warcraft_logs;
mod wow;
mod wow_scanner;

use config::Config;
use orchestrator::{TalentOrchestrator, UpdateSummary};
use warcraft_logs::{DiscoveredContent, WarcraftLogsService};
use wow_scanner::{DiscoveredCharacter, WowScanner};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Tauri command to read a file
#[tauri::command]
fn read_file(path: String) -> Result<String, String> {
    std::fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read file: {}", e))
}

/// Tauri command to find the default WoW installation path
#[tauri::command]
fn find_wow_path() -> Result<String, String> {
    WowScanner::find_default_wow_path()
        .and_then(|p| p.to_str().map(|s| s.to_string()))
        .ok_or_else(|| "Could not find WoW installation".to_string())
}

/// Tauri command to scan for characters in WoW installation
#[tauri::command]
fn scan_characters(wow_path: String) -> Result<Vec<DiscoveredCharacter>, String> {
    let scanner = WowScanner::new(wow_path);
    scanner
        .scan_characters()
        .map_err(|e| format!("Failed to scan characters: {}", e))
}

/// Tauri command to update talents from Archon.gg
#[tauri::command]
async fn update_talents_from_config(config: Config) -> Result<UpdateSummary, String> {
    // Create orchestrator and run
    let orchestrator = TalentOrchestrator::new(config);
    orchestrator
        .run()
        .await
        .map_err(|e| format!("Failed to update talents: {}", e))
}

/// Tauri command to update talents from a config file (kept for backwards compatibility)
#[tauri::command]
async fn update_talents(config_path: String) -> Result<String, String> {
    // Load configuration
    let config = Config::from_file(&config_path).map_err(|e| format!("Failed to load config: {}", e))?;

    // Create orchestrator and run
    let orchestrator = TalentOrchestrator::new(config);
    orchestrator
        .run()
        .await
        .map_err(|e| format!("Failed to update talents: {}", e))?;

    Ok("Talents updated successfully!".to_string())
}

/// Tauri command to auto-discover current raids and dungeons from Warcraft Logs
#[tauri::command]
async fn discover_content() -> Result<DiscoveredContent, String> {
    WarcraftLogsService::discover_current_content()
        .await
        .map_err(|e| format!("Failed to discover content: {}", e))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            greet,
            read_file,
            find_wow_path,
            scan_characters,
            update_talents_from_config,
            update_talents,
            discover_content
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
