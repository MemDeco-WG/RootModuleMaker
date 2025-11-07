use std::path::Path;
use crate::core::error::CoreError;

/// Parse a GitHub remote url and return (owner, repo) if recognized.
pub fn parse_github_url(_url: &str) -> Option<(String, String)> {
    None
}

/// Generate an update.json URL for a remote + project id (stub)
pub fn generate_update_json_url(_remote_url: &str, _project_id: &str) -> String {
    String::new()
}

pub fn get_git_repo_root(_path: &Path) -> Result<std::path::PathBuf, CoreError> {
    Err(CoreError::Other("not implemented".to_string()))
}

pub fn normalize_path(path: &Path) -> String {
    path.to_string_lossy().to_string()
}
