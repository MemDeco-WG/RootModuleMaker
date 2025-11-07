//! Sync module root — re-exports sync submodules without using mod.rs
use anyhow::Result;

pub mod discovery;
pub mod version;
pub mod author;
pub mod registry;
pub mod sync;
pub mod versioning;

/// Top-level orchestrator for syncing projects (stub)
pub fn sync_projects() -> Result<()> {
    Ok(())
}
