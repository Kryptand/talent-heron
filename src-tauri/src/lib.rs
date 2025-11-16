// Modules
mod archon;
mod config;
mod fetcher;
mod lua_talent;
mod orchestrator;
mod wow;

use config::Config;
use orchestrator::TalentOrchestrator;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Tauri command to update talents from Archon.gg
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, update_talents])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
