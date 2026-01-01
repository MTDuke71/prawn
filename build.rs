// Build script to embed compile-time metadata
use std::process::Command;

fn main() {
    // Generate build timestamp (cross-platform)
    let timestamp = get_timestamp();
    
    println!("cargo:rustc-env=BUILT_TIMESTAMP={}", timestamp);
    
    // Rerun if build.rs changes
    println!("cargo:rerun-if-changed=build.rs");
}

fn get_timestamp() -> String {
    // Try Unix date command first (Linux/macOS)
    if let Some(ts) = try_unix_date() {
        return ts;
    }
    
    // Try PowerShell (Windows)
    if let Some(ts) = try_powershell() {
        return ts;
    }
    
    // Fallback
    "unknown".to_string()
}

fn try_unix_date() -> Option<String> {
    Command::new("date")
        .args(["-u", "+%Y-%m-%d %H:%M:%S UTC"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn try_powershell() -> Option<String> {
    Command::new("powershell")
        .args(["-Command", "(Get-Date).ToUniversalTime().ToString('yyyy-MM-dd HH:mm:ss') + ' UTC'"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}
