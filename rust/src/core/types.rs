//! Domain types used across core and commands — canonicalized signatures for
//! rmmproject.toml (pyproject-like), module.prop and update.json.
//!
//! This file only defines structures and function signatures (no concrete
//! implementations). Implementations should live in the `cmds` layer or a
//! dedicated `io`/`sync` module. All I/O / parsing / sync functions below are
//! intentionally left as `todo!()` to make the intent explicit while keeping
//! this file focused on types and signatures.

use std::path::{Path, PathBuf};


use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;

/// Dependency source type hint (keeps domain model in `types` so core modules
/// and deps implementations can both reference the canonical shape).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceType {
    Github,
    Git,
    Http,
    Local,
    Other(String),
}

impl From<&str> for SourceType {
    fn from(s: &str) -> Self {
        match s {
            "github" => SourceType::Github,
            "git" => SourceType::Git,
            "http" => SourceType::Http,
            "local" => SourceType::Local,
            other => SourceType::Other(other.to_string()),
        }
    }
}

/// Canonical dependency declaration used in `rmmproject.toml` under
/// `tool.rmm.dependencies`. Keeping this in `types` makes it the single source
/// of truth for all subsystems.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencySpec {
    /// Source identifier (e.g. "github:user/repo" or "user/repo").
    pub source: String,

    /// Optional parsed source type (derived from `source`, convenience).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<SourceType>,

    /// Version requirement (semver range, exact, tag, etc).
    pub version: Option<String>,

    /// Asset selector when the source is a release containing multiple assets.
    pub asset: Option<String>,

    /// Optional alias for local installation name.
    pub alias: Option<String>,

    /// Optional metadata bag for source-specific options.
    pub meta: Option<JsonValue>,
}

impl DependencySpec {
    /// Parse a CLI short-form spec into a structured `DependencySpec`.
    /// Signature only; implementation belongs to CLI or resolver helpers.
    pub fn parse(_raw: &str) -> Result<Self> {
        todo!()
    }
}

/// A resolved dependency (concrete version + download URL) produced by a
/// resolver. Placed here so lock/manifest code can reference it directly.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedDependency {
    pub spec: DependencySpec,
    pub resolved_version: String,
    pub download_url: String,
    pub sha256: Option<String>,
    pub size: Option<u64>,
    pub source: String,
    pub meta: Option<BTreeMap<String, JsonValue>>,
}

/// Lockfile entry describing an installed dependency.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyLockEntry {
    pub id: String,
    pub resolved_version: String,
    pub download_url: String,
    pub sha256: Option<String>,
    pub source: String,
    pub installed_path: Option<String>,
    pub meta: Option<BTreeMap<String, JsonValue>>,
}

/// High-level lockfile model (rmm.lock)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RmmLock {
    pub lock_version: String,
    pub generated_at: String,
    pub entries: BTreeMap<String, DependencyLockEntry>,
}

impl RmmLock {
    /// Read lockfile from project root and parse. Signature only.
    pub fn read_from_project(_project_root: &Path) -> Result<Self> {
        todo!()
    }

    /// Write lockfile to project root. Signature only.
    pub fn write_to_project(&self, _project_root: &Path) -> Result<()> {
        let _ = self;
        todo!()
    }
}

/// Alias for results returned by functions in `rmmcore::core::types`.
/// Uses the fully-qualified error type `crate::core::error::CoreError`.
pub type Result<T> = std::result::Result<T, crate::core::error::CoreError>;

/// Core state holder for the RMM system (minimal/stub).
#[derive(Debug, Clone, Default)]
pub struct RmmCore {
    /// Workspace-level meta configuration
    pub meta_config: MetaConfig,

    /// Optional parsed rmmproject for the workspace root (if loaded)
    pub rmm_project: Option<RmmProject>,

    /// Features manager (kept as-is from existing design)
    pub features_manager: crate::core::features::FeaturesManager,
}

/// Workspace-level metadata (e.g. rmm workspace containing multiple projects)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MetaConfig {
    /// List of project ids/paths in the workspace
    pub projects: Vec<String>,

    /// Optional global author information to be used as a fallback
    pub default_author: Option<Author>,
}

/// Simple author representation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub email: Option<String>,
}

impl Author {
    /// whether this author is the default/empty placeholder
    pub fn is_default(&self) -> bool {
        self.name.is_empty() && self.email.is_none()
    }

    /// Attempt to derive author from a git repo path (signature only)
    pub fn from_git(_path: &std::path::Path) -> Option<Self> {
        // Implementation belongs to core::git or cmds layer
        todo!()
    }

    /// Merge author information from multiple sources.
    /// Priority: primary > fallback > fallback2.
    pub fn merge(primary: Option<&Author>, fallback: Option<&Author>, fallback2: Option<&Author>) -> Author {
        use crate::core::author::merge_authors;
        merge_authors(primary, fallback, fallback2)
    }
}

/// Representation of a pyproject-like table that contains package metadata.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectTable {
    pub name: String,
    pub version: Option<String>,
    pub description: Option<String>,
    pub readme: Option<String>,
    pub authors: Option<Vec<Author>>,
    pub license: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub classifiers: Option<Vec<String>>,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    // Additional fields from pyproject/project may be added here as needed.
}

/// Tool-specific configuration root for rmm values inside `tool.*` tables in
/// TOML. Typical layout: `tool.rmm` in `pyproject.toml` or `rmmproject.toml`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToolTable {
    /// rmm-specific configuration nested under `tool.rmm`
    pub rmm: Option<RmmToolTable>,
}

/// rmm-specific tool table: contains hints/overrides used to render
/// `module.prop`, `update.json`, dependency declarations and optional WebUI
/// configuration for module-provided web pages.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RmmToolTable {
    /// Optional explicit module properties override
    pub module: Option<ModuleProp>,

    /// Optional explicit update.json override / template information
    pub update: Option<UpdateJson>,

    /// Dependency declarations for the project. Uses structured `DependencySpec`.
    pub dependencies: Option<Vec<DependencySpec>>,

    /// Optional WebUI configuration for modules that expose an HTML UI under
    /// the `webroot` directory. When present, KernelSU manager may display the
    /// UI in a WebView and provide the JavaScript runtime API described by the
    /// WebUI spec.
    pub webui: Option<WebUiConfig>,
}

/// Top-level structure representing the `rmmproject.toml` (pyproject-like)
/// configuration file. It intentionally mirrors `pyproject.toml` layout:
/// - `project` (PEP 621-like)
/// - `tool.rmm` for rmm-specific extensions.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RmmProject {
    pub project: ProjectTable,
    pub tool: Option<ToolTable>,
}

/// Module properties (module.prop) representation for KernelSU / Magisk modules.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModuleProp {
    /// Unique id of the module (required for update.json linking).
    /// MUST match the KernelSU module id regex:
    /// ^[a-zA-Z][a-zA-Z0-9._-]+$
    pub id: String,

    /// Human readable module name
    pub name: Option<String>,

    /// Version string (e.g. "1.2.3")
    pub version: Option<String>,

    /// Numeric version code (for update.json / module comparisons)
    /// Serialized key in module.prop files is `versionCode`.
    #[serde(rename = "versionCode")]
    pub version_code: Option<u64>,

    /// Author information (name + optional email). For simple module.prop this
    /// may be a single-line string; tooling can convert between structured
    /// `Author` and single-line representation when reading/writing.
    pub author: Option<Author>,

    /// Short description
    pub description: Option<String>,

    /// Optional additional free-form key/value pairs. These are preserved and
    /// round-tripped when reading/writing module.prop files.
    #[serde(flatten)]
    pub extra: BTreeMap<String, String>,
}

/// Validation and helpers for `ModuleProp`.
impl ModuleProp {
    /// Module ID regex required by KernelSU module specification.
    /// Matches: starts with an ASCII letter, followed by letters, digits, dot, underscore or hyphen.
    pub const ID_REGEX: &'static str = "^[a-zA-Z][a-zA-Z0-9._-]+$";

    /// Validate the module.prop semantics (signature only — no implementation).
    ///
    /// Checks that required fields are present and that `id` matches the
    /// KernelSU module id pattern and that `versionCode` (if present) is a
    /// non-negative integer. Implementations should return a descriptive
    /// `CoreError` on failure.
    pub fn validate(&self) -> Result<()> {
        // Implementation belongs to validation layer / cmds and must not perform I/O here.
        let _ = self;
        todo!()
    }
}

/// update.json file representation (Magisk update metadata).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateJson {
    /// Version string
    pub version: Option<String>,

    /// Version code as integer
    #[serde(rename = "versionCode")]
    pub version_code: Option<u32>,

    /// Download URL for the module zip
    pub url: Option<String>,

    /// Human readable changelog or release notes
    pub changelog: Option<String>,

    /// Timestamp of the release (ISO 8601 or unix epoch in string form)
    pub timestamp: Option<String>,

    /// Optional artifact checksum (sha256 or similar)
    pub file_hash: Option<String>,

    /// Optional file size in bytes
    pub file_size: Option<u64>,

    /// Optional target architecture / variant
    pub arch: Option<String>,

    /// Optional signature or verification data
    pub signature: Option<String>,

    /// Optional module id (to correlate with module.prop)
    #[serde(rename = "moduleId")]
    pub module_id: Option<String>,

    /// Extra arbitrary fields allowed
    #[serde(flatten)]
    pub extra: BTreeMap<String, JsonValue>,
}

/// WebUI configuration for a module.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebUiConfig {
    /// Relative path to the webroot directory inside the module (e.g. "webroot")
    pub webroot: Option<String>,

    /// Entry HTML file name (e.g. "index.html")
    pub entry: Option<String>,

    /// Whether the WebUI is enabled and should be shown by the manager
    pub enabled: Option<bool>,

    /// Optional Content-Security-Policy hint for the page
    pub csp: Option<String>,

    /// Optional allowed origins for cross-window messaging
    pub allowed_origins: Option<Vec<String>>,

    /// Extra arbitrary fields preserved for forward-compatibility
    #[serde(flatten)]
    pub extra: BTreeMap<String, JsonValue>,
}

/// Project scan result (used by scanner)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectScanResult {
    pub name: String,
    pub path: PathBuf,
}

/// Git information (minimal)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GitInfo {
    pub branch: Option<String>,
    pub remote_url: Option<String>,
    pub commit: Option<String>,
}

//
// Conversion / sync function signatures.
//
// These are intentionally only signatures (no logic). They exist here to make
// the intended public API explicit to other modules. Implementations should
// not be placed in this types module.
//

/// Parse a pyproject.toml (content as &str) and produce an `RmmProject`
/// representation that merges/normalizes `project` and `tool.rmm` sections.
///
/// Note: no parsing is performed here; implementation must handle TOML parsing,
/// validation and any schema migration logic.
pub fn rmm_from_pyproject(_pyproject_toml: &str) -> Result<RmmProject> {
    todo!()
}

/// Convert an `RmmProject` into the `ModuleProp` representation that will be
/// written to `module.prop`. This conversion should prefer explicit
/// `tool.rmm.module` overrides, then fall back to `project` metadata.
pub fn rmm_to_module_prop(_rmm: &RmmProject) -> Result<ModuleProp> {
    todo!()
}

/// Convert an `RmmProject` and/or `ModuleProp` into an `UpdateJson` struct.
/// `version_code_override` may be provided to enforce numeric version codes.
/// Implementations should ensure semantic consistency between `module.prop`
/// and `update.json`.
pub fn rmm_to_update_json(_rmm: &RmmProject, _version_code_override: Option<u32>) -> Result<UpdateJson> {
    todo!()
}

/// Ensure `rmmproject.toml` (or `pyproject.toml`) and the on-disk
/// `module.prop` and `update.json` (if present) are synchronized.
///
/// This is a high-level signature intended to be called from CLI commands or
/// sync subsystems. It must not perform I/O when used in pure contexts; the
/// concrete implementation will.
pub fn ensure_files_in_sync(_project_root: &Path) -> Result<()> {
    todo!()
}

/// Read module.prop from disk and deserialize into `ModuleProp`.
pub fn read_module_prop_from_disk(_path: &Path) -> Result<ModuleProp> {
    todo!()
}

/// Write `ModuleProp` as `module.prop` to disk (string format, not TOML).
pub fn write_module_prop_to_disk(_path: &Path, _module: &ModuleProp) -> Result<()> {
    todo!()
}

/// Read update.json from disk and deserialize into `UpdateJson`.
pub fn read_update_json_from_disk(_path: &Path) -> Result<UpdateJson> {
    todo!()
}

/// Write `UpdateJson` to disk as JSON.
pub fn write_update_json_to_disk(_path: &Path, _update: &UpdateJson) -> Result<()> {
    todo!()
}
