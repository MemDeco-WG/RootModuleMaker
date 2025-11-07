use std::path::Path;
use anyhow::Result;
use crate::core::types::RmmCore;
use crate::core::types::MetaConfig;



pub fn sync_specific_project(_core: &RmmCore, _project_name: &str) -> Result<()> {
    Ok(())
}

pub fn search_and_add_project(_core: &RmmCore, _project_name: &str, _meta: &mut MetaConfig) -> Result<()> {
    Ok(())
}

pub async fn sync_project_metadata(_core: &RmmCore, _project_path: &Path, _meta: &mut MetaConfig) -> Result<()> {
    crate::cmds::sync::author::sync_author_info(_core, _project_path, _meta).await
}

pub fn sync_version_info(_core: &RmmCore, _project_path: &Path) -> Result<()> {
    crate::cmds::sync::version::sync_version_info(_core, _project_path)
}



pub fn sync_all_projects(_core: &RmmCore) -> Result<()> {
    crate::cmds::sync::sync::perform_project_sync()
}

pub fn remove_duplicate_projects(_core: &RmmCore) -> Result<Vec<String>> {
    Ok(vec![])
}
