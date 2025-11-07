//! Feature matrix and management stubs
//!
//! This module provides lightweight type stubs that model the feature set
//! described in the user's table. All functions are stubs returning `RmmResult`.

use crate::core::error::CoreError;

/// Feature identifiers for the system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Feature {
    ActionWorkflow,
    FullBuild,
    RunAnywhere,
    PrebuildScript,
    PostbuildScript,
    TemplateInit,
    MultiProjectBuild,
    DependencyManagement,
    MultiModuleMerge,
    ModuleRegistry,
    AIAssist,
    Notifications,
    ProxyAcceleration,
    VMEmulation,
    BuildLogs,
    QuickInstall,
    GPGSigning,
}

/// Support level for a feature in the new mode
#[derive(Debug, Clone)]
pub enum SupportLevel {
    Unsupported,
    Supported,
    InDevelopment,
    Experimental,
    Planned,
}

/// Light-weight descriptor for a feature
#[derive(Debug, Clone)]
pub struct FeatureSpec {
    pub feature: Feature,
    pub level: SupportLevel,
    pub notes: Option<String>,
}

/// Manager responsible for querying and toggling features (stub)
#[derive(Debug, Clone, Default)]
pub struct FeaturesManager;

impl FeaturesManager {
    /// Create a fresh manager (stub)
    pub fn new() -> Self {
        FeaturesManager
    }

    /// Return a list of known feature specs (stub values matching the table)
    pub fn list_features(&self) -> Vec<FeatureSpec> {
        use Feature::*;
        use SupportLevel::*;

        vec![
            FeatureSpec { feature: ActionWorkflow, level: Supported, notes: None },
            FeatureSpec { feature: FullBuild, level: Supported, notes: None },
            FeatureSpec { feature: RunAnywhere, level: Supported, notes: Some("Run without new GitHub repo".into()) },
            FeatureSpec { feature: PrebuildScript, level: Supported, notes: None },
            FeatureSpec { feature: PostbuildScript, level: Supported, notes: None },
            FeatureSpec { feature: TemplateInit, level: InDevelopment, notes: Some("not perfect".into()) },
            FeatureSpec { feature: MultiProjectBuild, level: InDevelopment, notes: Some("work in progress".into()) },
            FeatureSpec { feature: DependencyManagement, level: InDevelopment, notes: None },
            FeatureSpec { feature: MultiModuleMerge, level: Supported, notes: None },
            FeatureSpec { feature: ModuleRegistry, level: InDevelopment, notes: None },
            FeatureSpec { feature: AIAssist, level: Supported, notes: Some("AI test/audit/opt/fix".into()) },
            FeatureSpec { feature: Notifications, level: InDevelopment, notes: None },
            FeatureSpec { feature: ProxyAcceleration, level: Supported, notes: None },
            FeatureSpec { feature: VMEmulation, level: Experimental, notes: Some("experimental".into()) },
            FeatureSpec { feature: BuildLogs, level: Supported, notes: None },
            FeatureSpec { feature: QuickInstall, level: InDevelopment, notes: None },
            FeatureSpec { feature: GPGSigning, level: Planned, notes: None },
        ]
    }

    /// Query if a feature is considered available (stub)
    pub fn is_supported(&self, feature: Feature) -> bool {
        self.list_features().into_iter().any(|s| s.feature == feature && matches!(s.level, SupportLevel::Supported | SupportLevel::InDevelopment | SupportLevel::Experimental))
    }

    /// Enable a feature (no-op stub)
    pub fn enable_feature(&self, _feature: Feature) -> Result<(), CoreError> {
        Ok(())
    }

    /// Disable a feature (no-op stub)
    pub fn disable_feature(&self, _feature: Feature) -> Result<(), CoreError> {
        Ok(())
    }

    /// Configure a feature with a free-form string (stub)
    pub fn configure_feature(&self, _feature: Feature, _config: Option<&str>) -> Result<(), CoreError> {
        Ok(())
    }
}
