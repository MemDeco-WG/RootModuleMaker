//! Version command implementation for RMM
//! Displays the current version of the RMM tool

/// Get the version string
pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Display version information
pub fn show_version() {
    println!("RMM version {}", get_version());
}
