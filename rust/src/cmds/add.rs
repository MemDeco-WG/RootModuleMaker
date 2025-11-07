use std::path::Path;

use anyhow::Result;
use clap::Args;

use crate::core::deps::{DependencyManager, DependencySpec};
use crate::core::types::{ResolvedDependency, RmmCore};
use crate::core::runtime::RuntimeTarget;

/// Options for the `add` command (internal representation).
///
/// This struct captures the behavior that the CLI `AddArgs` maps to. It is
/// convenient for internal APIs that perform the actual orchestration.
#[derive(Debug, Clone)]
pub struct AddOptions {
    /// Persist the dependency declaration into `rmmproject.toml` under
    /// `tool.rmm.dependencies`.
    pub save: bool,

    /// Resolve and/or install pre-release versions.
    pub allow_prerelease: bool,

    /// Optional runtime target to bias resolution/packaging (e.g. `kernelsu`).
    pub target: Option<RuntimeTarget>,

    /// Optional explicit release asset name to select (when using releases).
    pub asset: Option<String>,

    /// Optional alias to store the dependency under in the manifest.
    pub alias: Option<String>,

    /// Don't perform network fetch or write files; only show what would happen.
    pub dry_run: bool,

    /// Resolve and persist but do not install the artifact.
    pub no_install: bool,

    /// Verbose output for debugging.
    pub verbose: bool,
}

impl Default for AddOptions {
    fn default() -> Self {
        AddOptions {
            save: true,
            allow_prerelease: false,
            target: None,
            asset: None,
            alias: None,
            dry_run: false,
            no_install: false,
            verbose: false,
        }
    }
}

/// Clap-based arguments for `rmm add`
///
/// This struct is intended to be used directly by the CLI layer (clap derive).
#[derive(Debug, Clone, Args)]
pub struct AddArgs {
    /// Dependency specification. Examples:
    /// - `user/repo@^1.2.0`
    /// - `github:user/repo@1.2.3`
    /// - a structured TOML inline spec may also be supported by the parser
    pub spec: String,

    /// Do not persist the dependency to the project's manifest (rmmproject.toml)
    #[clap(long = "no-save", action)]
    pub no_save: bool,

    /// Allow pre-release versions during resolution (semver pre-release)
    #[clap(long = "allow-prerelease", action)]
    pub allow_prerelease: bool,

    /// Target runtime (kernelsu|magisk). Influences packaging decisions.
    #[clap(long = "target")]
    pub target: Option<String>,

    /// Asset name (select an asset from a release)
    #[clap(long = "asset")]
    pub asset: Option<String>,

    /// Install under a different alias/id
    #[clap(long = "alias")]
    pub alias: Option<String>,

    /// Dry run: show what would happen but do not download or change files
    #[clap(long = "dry-run", action)]
    pub dry_run: bool,

    /// Resolve and persist but skip actual install
    #[clap(long = "no-install", action)]
    pub no_install: bool,

    /// Verbose output
    #[clap(long = "verbose", short = 'v', action)]
    pub verbose: bool,
}

/// High-level CLI entrypoint that the application should call when the user
/// invokes `rmm add ...`.
///
/// This function translates `AddArgs` into internal `AddOptions`, parses the
/// spec into a `DependencySpec` and delegates to `add_parsed_spec`.
pub fn add_cli_entry(core: &RmmCore, project_root: &Path, args: AddArgs) -> Result<ResolvedDependency> {
    let _ = (core, project_root, args);
    // Signature-only: expected flow:
    // 1. Map AddArgs -> AddOptions
    // 2. Parse args.spec -> DependencySpec (via parse_raw_spec)
    // 3. Call add_parsed_spec(...) to orchestrate resolution, fetch and manifest updates
    todo!()
}

/// Parse a raw/short dependency spec string into a structured `DependencySpec`.
/// This function should accept the short forms used on CLI and return a fully
/// populated spec that the resolver understands.
pub fn parse_raw_spec(raw_spec: &str) -> Result<DependencySpec> {
    let _ = raw_spec;
    // Signature-only: delegate to DependencySpec::parse or custom parser.
    todo!()
}

/// Orchestrate the add workflow for an already-parsed `DependencySpec`.
///
/// Responsibilities (signature-only):
/// - Resolve the spec to a concrete `ResolvedDependency` (DependencyManager::resolve)
/// - Optionally fetch the artifact into cache (DependencyManager::fetch_resolved)
/// - Update lockfile and manifest (helpers like write_lock_entry / persist_dependency_to_manifest)
/// - Optionally install or vendor the artifact into the project/module layout
pub fn add_parsed_spec(core: &RmmCore, project_root: &Path, spec: &DependencySpec, options: AddOptions) -> Result<ResolvedDependency> {
    let _ = (core, project_root, spec, options);
    todo!()
}

/// Persist a dependency declaration into the project's manifest (`rmmproject.toml`).
/// This must perform an atomic update (write to temp file then rename) and handle
/// deduplication/merging of existing declarations.
pub fn persist_dependency_to_manifest(project_root: &Path, spec: &DependencySpec) -> Result<()> {
    let _ = (project_root, spec);
    todo!()
}

/// Write or update a lockfile entry for the resolved dependency. Signature-only.
pub fn write_lock_entry(project_root: &Path, resolved: &ResolvedDependency) -> Result<()> {
    let _ = (project_root, resolved);
    todo!()
}

/// Install a resolved artifact (fetch + place into cache or vendor directory).
/// Returns the path to the installed artifact.
/// This is intentionally a signature (installer-specific behavior lives in cmds).
pub fn install_resolved_artifact(project_root: &Path, resolved: &ResolvedDependency, options: &AddOptions) -> Result<std::path::PathBuf> {
    let _ = (project_root, resolved, options);
    todo!()
}

/// Helper that converts `AddArgs` (clap) into internal `AddOptions`.
pub fn options_from_args(args: &AddArgs) -> AddOptions {
    AddOptions {
        save: !args.no_save,
        allow_prerelease: args.allow_prerelease,
        target: args.target.as_ref().and_then(|t| Some(match t.as_str() {
            "kernelsu" | "ksu" => RuntimeTarget::KernelSU,
            "magisk" => RuntimeTarget::Magisk,
            _ => RuntimeTarget::Unknown,
        })),
        asset: args.asset.clone(),
        alias: args.alias.clone(),
        dry_run: args.dry_run,
        no_install: args.no_install,
        verbose: args.verbose,
    }
}

/// Short usage string for embedded help or programmatic usage display.
pub fn add_usage() -> &'static str {
    "rmm add <package> [--no-save] [--allow-prerelease] [--target kernelsu|magisk] [--alias name] [--asset asset-name] [--dry-run] [--no-install]"
}
