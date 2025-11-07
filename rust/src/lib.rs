// Root crate entry — exports core and cmds modules
//! Lightweight crate root for newly created stubs.

use anyhow::Result;

pub mod core;
pub mod cmds;

/// Optional helper to initialize the library-level components.
pub fn init() -> Result<()> {
    // initialization placeholder
    Ok(())
}
