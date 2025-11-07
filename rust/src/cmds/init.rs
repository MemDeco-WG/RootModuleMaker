use std::path::Path;
use anyhow::Result;
use clap::Args;

use crate::core::types::RmmCore;

/// Clap-based arguments for `rmm init` (signature-only).
#[derive(Debug, Clone, Args)]
pub struct InitArgs {
    /// Optional project path to create. If omitted, use current directory.
    #[clap(value_parser)]
    pub project_path: Option<String>,

    /// Project identifier (module id). Example: my.module.id
    #[clap(long = "id")]
    pub project_id: Option<String>,

    /// Author name
    #[clap(long = "author")]
    pub author: Option<String>,

    /// Author email
    #[clap(long = "email")]
    pub email: Option<String>,

    /// Template name to use when creating the project (optional)
    #[clap(long = "template")]
    pub template: Option<String>,

    /// Verbose output
    #[clap(long = "verbose", short = 'v', action)]
    pub verbose: bool,
}

/// CLI entrypoint signature for the `init` command (clap)
/// Signature-only: real implementation should map `InitArgs` to `init_project`.
pub async fn init_cli_entry(core: &RmmCore, project_root: &Path, args: InitArgs) -> Result<()> {
    let _ = (core, project_root, args);
    todo!()
}

/// Initialize a new project at `project_path` with given id, author and email.
/// If template is provided, copy files from the template directory.
pub async fn init_project(project_path: &Path, project_id: &str, author: &str, email: &str, template: Option<&str>) -> Result<()> {
    // Create project directory
    std::fs::create_dir_all(project_path)?;

    // If template is specified, copy template files
    if let Some(template_name) = template {
        copy_template_files(project_path, template_name)?;
    }

    // Generate basic files (module.prop, etc.)
    generate_basic_files(project_path, project_id, author, email).await?;

    Ok(())
}

/// Copy files from a template directory to the project path
fn copy_template_files(project_path: &Path, template_name: &str) -> Result<()> {
    // Assume templates are in a fixed directory, e.g., ~/.rmm/templates/
    let template_dir = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?
        .join(".rmm")
        .join("templates")
        .join(template_name);

    if !template_dir.exists() {
        return Err(anyhow::anyhow!("Template '{}' not found", template_name));
    }

    // Copy all files from template_dir to project_path
    for entry in std::fs::read_dir(&template_dir)? {
        let entry = entry?;
        let src_path = entry.path();
        let dest_path = project_path.join(entry.file_name());

        if src_path.is_file() {
            std::fs::copy(&src_path, &dest_path)?;
        } else if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dest_path)?;
        }
    }

    Ok(())
}

/// Recursively copy a directory
fn copy_dir_recursive(src: &Path, dest: &Path) -> Result<()> {
    std::fs::create_dir_all(dest)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if src_path.is_file() {
            std::fs::copy(&src_path, &dest_path)?;
        } else if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dest_path)?;
        }
    }
    Ok(())
}

/// Generate basic project files
async fn generate_basic_files(project_path: &Path, project_id: &str, author: &str, _email: &str) -> Result<()> {
    // Generate module.prop
    let module_prop_content = format!(
        "id={}\nauthor={}\nversion=1.0.0\n",
        project_id, author
    );
    std::fs::write(project_path.join("module.prop"), module_prop_content)?;

    // Other basic files can be added here

    Ok(())
}
