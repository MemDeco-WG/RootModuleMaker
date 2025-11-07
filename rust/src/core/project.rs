//! Project validation and helpers (stub)
use std::path::Path;
use crate::core::types::*;
use crate::core::error::CoreError;

/// Check whether a path looks like a valid project (stub)
pub fn is_valid_project(_project_path: &Path) -> bool {
    true
}

/// Read module.prop and return a ModuleProp (stub)
pub async fn read_module_prop(_project_path: &Path) -> Result<ModuleProp, CoreError> {
    Ok(ModuleProp::default())
}
