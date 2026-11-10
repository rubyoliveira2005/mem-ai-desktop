//! Static build info for Mem Ai Desktop.

pub const BUILD_VERSION: &str  = "v4.8.17";
pub const BUILD_CHANNEL: &str  = "lts";
pub const BUILD_COMMIT: &str   = "9f266bb79758";
pub const BUILD_TIMESTAMP: u64 = 1747287151;

pub fn version_tuple() -> (u16, u16, u16) {
    let mut parts = BUILD_VERSION.trim_start_matches('v').split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

pub fn user_agent() -> String {
    format!("{}/{} ({})", "Mem Ai Desktop", BUILD_VERSION, BUILD_CHANNEL)
}
