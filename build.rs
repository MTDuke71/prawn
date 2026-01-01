// Build script to embed compile-time metadata
use std::process::Command;

fn main() {
    // Generate build timestamp
    let output = Command::new("powershell")
        .args(["-Command", "(Get-Date).ToUniversalTime().ToString('yyyy-MM-dd HH:mm:ss') + ' UTC'"])
        .output()
        .ok();
    
    let timestamp = output
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    
    println!("cargo:rustc-env=BUILT_TIMESTAMP={}", timestamp);
    
    // Rerun if build.rs changes
    println!("cargo:rerun-if-changed=build.rs");
}
