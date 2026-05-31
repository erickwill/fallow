//! Typed action payloads for health findings and related outputs.

use serde::Serialize;

/// Action attached to a [`ComplexityViolation`].
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct HealthFindingAction {
    #[serde(rename = "type")]
    pub kind: HealthFindingActionType,
    pub auto_fixable: bool,
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placement: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_path: Option<String>,
}

/// Discriminant for [`HealthFindingAction::kind`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "kebab-case")]
pub enum HealthFindingActionType {
    /// Refactor the function.
    RefactorFunction,
    /// Add tests for a CRAP-triggered finding with no coverage.
    AddTests,
    /// Increase test coverage for a CRAP-triggered finding with partial coverage.
    IncreaseCoverage,
    /// Suppress with an HTML comment at the top of the template.
    SuppressFile,
    /// Suppress with an inline comment above the function.
    SuppressLine,
}

/// Action attached to a [`HotspotEntry`].
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct HotspotAction {
    #[serde(rename = "type")]
    pub kind: HotspotActionType,
    pub auto_fixable: bool,
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggested_pattern: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub heuristic: Option<HotspotActionHeuristic>,
}

/// Discriminant for [`HotspotAction::kind`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "kebab-case")]
pub enum HotspotActionType {
    /// Refactor the hotspot file.
    RefactorFile,
    /// Add test coverage for the hotspot file.
    AddTests,
    /// Bus factor of 1.
    LowBusFactor,
    /// Hotspot matches no CODEOWNERS rule.
    UnownedHotspot,
    /// Ownership has drifted to a new top contributor.
    OwnershipDrift,
}

/// Strategy discriminant for the suggested CODEOWNERS pattern.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "kebab-case")]
pub enum HotspotActionHeuristic {
    /// Suggest the deepest directory containing the file.
    DirectoryDeepest,
}

/// Action attached to a [`RefactoringTarget`].
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct RefactoringTargetAction {
    #[serde(rename = "type")]
    pub kind: RefactoringTargetActionType,
    pub auto_fixable: bool,
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

/// Discriminant for [`RefactoringTargetAction::kind`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "kebab-case")]
pub enum RefactoringTargetActionType {
    /// Apply the recommended refactoring.
    ApplyRefactoring,
    /// Suppress the underlying complexity finding.
    SuppressLine,
}

/// Action attached to an [`UntestedFile`] coverage-gap finding.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct UntestedFileAction {
    #[serde(rename = "type")]
    pub kind: UntestedFileActionType,
    pub auto_fixable: bool,
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

/// Discriminant for [`UntestedFileAction::kind`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "kebab-case")]
pub enum UntestedFileActionType {
    /// Scaffold tests that exercise the runtime file.
    AddTests,
    /// Suppress coverage-gap reporting for this file with a file-level comment.
    SuppressFile,
}

/// Action attached to an [`UntestedExport`] coverage-gap finding.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct UntestedExportAction {
    #[serde(rename = "type")]
    pub kind: UntestedExportActionType,
    pub auto_fixable: bool,
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

/// Discriminant for [`UntestedExportAction::kind`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "kebab-case")]
pub enum UntestedExportActionType {
    /// Import and exercise the export from a test-reachable module.
    AddTestImport,
    /// Suppress coverage-gap reporting for the export's file with a
    /// file-level comment.
    SuppressFile,
}
