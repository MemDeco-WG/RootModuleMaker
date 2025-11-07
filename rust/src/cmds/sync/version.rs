use std::path::Path;
use crate::core::types::RmmCore;
use anyhow::Result;

pub struct VersionInfo {
    pub version: String,
    pub version_code: String,
}

impl VersionInfo {
    pub fn new(version: &str, version_code: &str) -> Self {
        VersionInfo { version: version.to_string(), version_code: version_code.to_string() }
    }

    pub fn smart_bump_version(&mut self, _project_path: &Path) {
        // mutate self.version as needed
    }

    pub fn from_module_prop(_project_path: &Path) -> Result<Self> {
        Ok(VersionInfo::new("0.0.1", "1"))
    }

    pub fn update_module_prop(&self, _project_path: &Path) -> Result<()> {
        Ok(())
    }
}

pub fn sync_version_info(_core: &RmmCore, _project_path: &Path) -> Result<()> {
    Ok(())
}

pub fn generate_version_code(_project_path: &Path) -> String {
    String::new()
}

pub fn smart_version_bump(_current_version: &str, _project_path: &Path) -> String {
    String::new()
}

pub fn sync_update_json(_project_path: &Path, _version_info: &VersionInfo) -> Result<()> {
    Ok(())
}

pub fn sync_global_version(_core: &RmmCore, _project_version: &str) -> Result<()> {
    Ok(())
}

pub fn extract_major_version(version: &str) -> Result<u32, std::num::ParseIntError> {
    let parts: Vec<&str> = version.split('.').collect();
    parts.get(0).unwrap_or(&"0").parse()
}
