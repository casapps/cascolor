// Version information using built crate

include!(concat!(env!("OUT_DIR"), "/built.rs"));

pub fn print_version() {
    let version = PKG_VERSION;
    let commit = GIT_COMMIT_HASH.unwrap_or("unknown");
    let build_time = BUILT_TIME_UTC;
    
    // Get timezone
    let tz = std::env::var("TZ")
        .ok()
        .or_else(|| {
            // Try to get system timezone
            if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
                std::fs::read_to_string("/etc/timezone")
                    .ok()
                    .map(|s| s.trim().to_string())
            } else {
                None
            }
        })
        .unwrap_or_else(|| "UTC".to_string());
    
    println!("cascolor {}", version);
    println!("Commit: {}", commit);
    println!("Built: {} {}", build_time, tz);
}

pub fn get_version() -> &'static str {
    PKG_VERSION
}

pub fn get_commit() -> &'static str {
    GIT_COMMIT_HASH.unwrap_or("unknown")
}
