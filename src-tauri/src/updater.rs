use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
const RELEASES_API: &str = "https://api.github.com/repos/Kryptand/talent-heron/releases/latest";

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateInfo {
    pub available: bool,
    pub current_version: String,
    pub latest_version: String,
    pub release_url: String,
    pub release_notes: String,
}

#[derive(Debug, Deserialize)]
struct GitHubRelease {
    tag_name: String,
    html_url: String,
    #[serde(default)]
    body: String,
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

    Ok(UpdateInfo {
        available,
        current_version: CURRENT_VERSION.to_string(),
        latest_version: latest.to_string(),
        release_url: release.html_url,
        release_notes: release.body,
    })
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
