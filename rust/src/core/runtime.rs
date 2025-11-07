/*!
runtime.rs

Runtime-specific enums and validation/rendering signatures to capture the
differences between Magisk and KernelSU module behavior.

This file only provides data shapes and function signatures. Implementations
belong to cmds/installer/packager layers. All functions here are intentionally
left as signatures / `todo!()` to keep this module implementation-free.
*/

use std::path::{Path, PathBuf};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::core::error::CoreError;

/// Result alias specific to runtime operations.
pub type RuntimeResult<T> = std::result::Result<T, CoreError>;

/// Target runtime environment for a module or packaged artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuntimeTarget {
    /// Magisk runtime
    Magisk,
    /// KernelSU runtime
    KernelSU,
    /// Compatible with both Magisk and KernelSU (no special transforms needed)
    Both,
    /// Unknown or not specified
    Unknown,
}

impl RuntimeTarget {
    /// Parse a textual hint into `RuntimeTarget`.
    /// Accepts "magisk", "kernelsu", "both", case-insensitive.
    pub fn from_str_hint(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "magisk" => RuntimeTarget::Magisk,
            "kernelsu" | "kernel-su" | "ksu" => RuntimeTarget::KernelSU,
            "both" => RuntimeTarget::Both,
            _ => RuntimeTarget::Unknown,
        }
    }

    /// Human-friendly name for the runtime
    pub fn as_str(&self) -> &'static str {
        match self {
            RuntimeTarget::Magisk => "magisk",
            RuntimeTarget::KernelSU => "kernelsu",
            RuntimeTarget::Both => "both",
            RuntimeTarget::Unknown => "unknown",
        }
    }
}

/// How a module intends files to be deleted/replaced on the device.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeleteStrategy {
    /// Use `mknod filename c 0 0` whiteout (KernelSU overlayfs behavior)
    Mknod,

    /// Use Magisk's `.replace` folder replacement strategy
    Replace,

    /// Custom strategy string for installer-specific handling
    Custom(String),
}

/// Kinds of module scripts and when they run. These map to well-known files
/// inside a module directory (customize.sh, post-fs-data.sh, post-mount.sh,
/// service.sh, boot-completed.sh, uninstall.sh).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash, PartialOrd, Ord)]
pub enum ScriptKind {
    Customize,
    PostFsData,
    PostMount,
    Service,
    BootCompleted,
    Uninstall,
}

/// Zygisk related support hint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ZygiskSupport {
    None,
    /// Provided via ZygiskNext adapter
    ZygiskNext,
    /// Native Zygisk support (Magisk)
    Native,
}

/// RenderPlan is a generic JSON value describing how a module was transformed
/// for a target runtime. Implementations should provide a stable schema for
/// this manifest (e.g. removed files, added hints, rewritten scripts).
pub type RenderPlan = JsonValue;

/// Detect the runtime target from environment variables or platform hints.
///
/// Signature only — a real implementation may inspect a set of environment
/// variables, command-line flags, or installer-provided metadata. For example,
/// KernelSU sets `KSU=true` in module script environments; Magisk sets
/// `MAGISK_VER` / `MAGISK_VER_CODE`. Detection should be non-invasive.
pub fn detect_runtime_environment(env: &HashMap<String, String>) -> RuntimeTarget {
    // Signature-only (light heuristic example is okay but concrete logic belongs to installer)
    if let Some(v) = env.get("KSU") {
        if v == "true" || v == "1" {
            return RuntimeTarget::KernelSU;
        }
    }
    if env.contains_key("MAGISK_VER") || env.contains_key("MAGISK_VER_CODE") {
        return RuntimeTarget::Magisk;
    }
    RuntimeTarget::Unknown
}

/// Validate that a module at `module_root` is compatible with `target`.
///
/// This includes checking:
/// - `module.prop` semantics (id regex, versionCode integer)
/// - Presence/absence of Magisk-specific artifacts (e.g. `.replace` files)
/// - That declared `delete_strategy` is supported by the target
/// - That required script kinds are supported by target
///
/// `strict` controls whether compatibility warnings should be promoted to
/// errors (`true`) or returned as non-fatal diagnostics (`false`).
pub fn validate_module_for_runtime(
    module_root: &Path,
    target: RuntimeTarget,
    strict: bool,
) -> RuntimeResult<()> {
    // Signature only — actual checks (file system inspection, parsing) belong
    // to the installer / validation implementation in cmds layer.
    let _ = module_root;
    let _ = target;
    let _ = strict;
    todo!()
}

/// Render or transform module files for a particular target runtime.
///
/// Examples of typical transforms:
/// - Remove `.replace` files when targeting KernelSU and instead record REMOVEs via `ksu-hints`
/// - Convert REPLACE declarations to installer-understandable metadata
/// - Optionally rewrite shebangs or busybox paths if the author requested it
///
/// Writes the transformed module into `out_dir` (which should be created by
/// the caller) and returns a `RenderPlan` describing transformations applied.
pub fn render_module_for_target(
    module_root: &Path,
    target: RuntimeTarget,
    out_dir: &Path,
) -> RuntimeResult<RenderPlan> {
    let _ = module_root;
    let _ = target;
    let _ = out_dir;
    todo!()
}

/// Package a project module ready for distribution for an optional target.
/// If `target` is `None`, a universal package is produced (contains both
/// Magisk and KernelSU hints where reasonable). Returns the path to the
/// generated package (zip).
///
/// `project_root` is the workspace project directory containing `rmmproject.toml`
/// and module sources.
pub fn package_module_for_target(
    project_root: &Path,
    target: Option<RuntimeTarget>,
) -> RuntimeResult<PathBuf> {
    let _ = project_root;
    let _ = target;
    todo!()
}

/// Convenience helper: return the runtime's default BusyBox path hint. This is
/// only a hint; installers should not mutate module scripts unless explicitly
/// requested by the module author.
pub fn default_busybox_path_for_target(target: &RuntimeTarget) -> Option<&'static str> {
    match target {
        RuntimeTarget::Magisk => Some("/data/adb/magisk/busybox"),
        RuntimeTarget::KernelSU => Some("/data/adb/ksu/bin/busybox"),
        _ => None,
    }
}

/// Produce a small metadata manifest that installers can use to make decisions
/// about how to install a module (e.g. whether recovery install is supported,
/// which delete strategy to use, required JS APIs for WebUI, etc.).
///
/// This function should only inspect declarative data (module.prop, rmmproject
/// tool fields) and not perform network IO. Returns a JSON object manifest.
pub fn produce_install_manifest(module_root: &Path) -> RuntimeResult<JsonValue> {
    let _ = module_root;
    todo!()
}
