use anyhow::{Context, Result};
use futures_util::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::io::Write;
use tauri::{AppHandle, Emitter, Manager};

const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
const RELEASES_API: &str = "https://api.github.com/repos/Kryptand/talent-heron/releases/latest";

#[cfg(target_os = "linux")]
const ASSET_NAME: &str = "talent-heron-linux-x86_64";
#[cfg(target_os = "macos")]
const ASSET_NAME: &str = "talent-heron-macos-aarch64";
#[cfg(target_os = "windows")]
const ASSET_NAME: &str = "talent-heron-windows-x86_64.exe";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateInfo {
    pub available: bool,
    pub current_version: String,
    pub latest_version: String,
    pub release_url: String,
    pub release_notes: String,
    pub download_url: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GitHubAsset {
    name: String,
    browser_download_url: String,
}

#[derive(Debug, Deserialize)]
struct GitHubRelease {
    tag_name: String,
    html_url: String,
    #[serde(default)]
    body: String,
    #[serde(default)]
    assets: Vec<GitHubAsset>,
}

pub async fn check_for_updates() -> Result<UpdateInfo> {
    let client = Client::builder()
        .user_agent("talent-heron-updater")
        .build()
        .context("Failed to build HTTP client")?;

    let release: GitHubRelease = client
        .get(RELEASES_API)
        .send()
        .await
        .context("Failed to reach GitHub releases API")?
        .json()
        .await
        .context("Failed to parse GitHub release response")?;

    let latest = release.tag_name.trim_start_matches('v');
    let available = is_newer(latest, CURRENT_VERSION);

    let download_url = release
        .assets
        .iter()
        .find(|a| a.name == ASSET_NAME)
        .map(|a| a.browser_download_url.clone());

    Ok(UpdateInfo {
        available,
        current_version: CURRENT_VERSION.to_string(),
        latest_version: latest.to_string(),
        release_url: release.html_url,
        release_notes: release.body,
        download_url,
    })
}

pub async fn download_and_install(app: AppHandle, url: String) -> Result<()> {
    let client = Client::builder()
        .user_agent("talent-heron-updater")
        .build()?;

    let response = client.get(&url).send().await?;
    let total = response.content_length().unwrap_or(0);
    let mut downloaded = 0u64;

    // Stage in the same directory as the running binary (same filesystem = atomic rename)
    let current_exe = std::env::current_exe()?;
    let staging = current_exe.with_extension("update-staging");

    let mut file = std::fs::File::create(&staging)?;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk)?;
        downloaded += chunk.len() as u64;

        if total > 0 {
            let pct = (downloaded * 100 / total) as u8;
            app.emit("update-progress", pct).ok();
        }
    }
    drop(file);

    // Make executable on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&staging, std::fs::Permissions::from_mode(0o755))?;
    }

    // Atomic replace
    std::fs::rename(&staging, &current_exe)?;

    // Signal 100% done before restarting
    app.emit("update-progress", 100u8).ok();
    tokio::time::sleep(std::time::Duration::from_millis(800)).await;

    // Relaunch the new binary and exit
    std::process::Command::new(&current_exe).spawn().ok();
    std::process::exit(0);
}

/// Returns true if `candidate` is a higher semver than `current`.
fn is_newer(candidate: &str, current: &str) -> bool {
    let parse = |v: &str| -> Vec<u64> {
        v.split('.').filter_map(|p| p.parse().ok()).collect()
    };
    let a = parse(candidate);
    let b = parse(current);
    for i in 0..a.len().max(b.len()) {
        let av = a.get(i).copied().unwrap_or(0);
        let bv = b.get(i).copied().unwrap_or(0);
        if av != bv {
            return av > bv;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_newer() {
        assert!(is_newer("0.2.0", "0.1.0"));
        assert!(is_newer("1.0.0", "0.9.9"));
        assert!(!is_newer("0.1.0", "0.1.0"));
        assert!(!is_newer("0.1.0", "0.2.0"));
        assert!(is_newer("0.1.1", "0.1.0"));
    }
}
