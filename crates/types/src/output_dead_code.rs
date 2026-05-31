//! Typed wrappers for dead-code findings with fixed action sets.

use serde::Serialize;

use crate::envelope::AuditIntroduced;
use crate::output::{
    AddToConfigAction, AddToConfigKind, AddToConfigValue, FixAction, FixActionType,
    IgnoreExportsRule, IssueAction, SuppressFileAction, SuppressFileKind, SuppressLineAction,
    SuppressLineKind, SuppressLineScope,
};
use crate::results::{
    BoundaryViolation, CircularDependency, DependencyOverrideSource, DuplicateExport,
    EmptyCatalogGroup, MisconfiguredDependencyOverride, PrivateTypeLeak, ReExportCycle,
    ReExportCycleKind, TestOnlyDependency, TypeOnlyDependency, UnlistedDependency,
    UnresolvedCatalogReference, UnresolvedImport, UnusedCatalogEntry, UnusedDependency,
    UnusedDependencyOverride, UnusedExport, UnusedFile, UnusedMember,
};

/// Shared note for the `duplicate-exports` fix action.
pub const NAMESPACE_BARREL_HINT: &str = "If every location is the sole `index.*` of its directory, this is likely an intentional namespace-barrel API. Prefer adding these files to `ignoreExports` over removing exports.";

const IGNORE_EXPORTS_VALUE_SCHEMA: &str =
    "https://raw.githubusercontent.com/fallow-rs/fallow/main/schema.json#/properties/ignoreExports";

const IGNORE_CATALOG_REFERENCES_VALUE_SCHEMA: &str = "https://raw.githubusercontent.com/fallow-rs/fallow/main/schema.json#/properties/ignoreCatalogReferences/items";

const IGNORE_DEPENDENCY_OVERRIDES_VALUE_SCHEMA: &str = "https://raw.githubusercontent.com/fallow-rs/fallow/main/schema.json#/properties/ignoreDependencyOverrides/items";

/// Actions wrapper for an [`UnusedFile`] finding.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct UnusedFileFinding {
    #[serde(flatten)]
    pub file: UnusedFile,
    pub actions: Vec<IssueAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub introduced: Option<AuditIntroduced>,
}

impl UnusedFileFinding {
    #[must_use]
    pub fn with_actions(file: UnusedFile) -> Self {
        let actions = vec![
            IssueAction::Fix(FixAction {
                kind: FixActionType::DeleteFile,
                auto_fixable: false,
                description: "Delete this file".to_string(),
                note: Some(
                    "File deletion may remove runtime functionality not visible to static analysis"
                        .to_string(),
                ),
                available_in_catalogs: None,
                suggested_target: None,
            }),
            IssueAction::SuppressFile(SuppressFileAction {
                kind: SuppressFileKind::SuppressFile,
                auto_fixable: false,
                description: "Suppress with a file-level comment at the top of the file"
                    .to_string(),
                comment: "// fallow-ignore-file unused-file".to_string(),
            }),
        ];
        Self {
            file,
            actions,
            introduced: None,
        }
    }
}

/// Actions wrapper for a [`PrivateTypeLeak`] finding.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PrivateTypeLeakFinding {
    #[serde(flatten)]
    pub leak: PrivateTypeLeak,
    pub actions: Vec<IssueAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub introduced: Option<AuditIntroduced>,
}

impl PrivateTypeLeakFinding {
    #[must_use]
    pub fn with_actions(leak: PrivateTypeLeak) -> Self {
        let actions = vec![
            IssueAction::Fix(FixAction {
                kind: FixActionType::ExportType,
                auto_fixable: false,
                description: "Export the referenced private type by name".to_string(),
                note: Some(
                    "Keep the type exported while it is part of a public signature".to_string(),
                ),
                available_in_catalogs: None,
                suggested_target: None,
            }),
            IssueAction::SuppressLine(SuppressLineAction {
                kind: SuppressLineKind::SuppressLine,
                auto_fixable: false,
                description: "Suppress with an inline comment above the line".to_string(),
                comment: "// fallow-ignore-next-line private-type-leak".to_string(),
                scope: None,
            }),
        ];
        Self {
            leak,
            actions,
            introduced: None,
        }
    }
}

/// Actions wrapper for an [`UnresolvedImport`] finding.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct UnresolvedImportFinding {
    #[serde(flatten)]
    pub import: UnresolvedImport,
    pub actions: Vec<IssueAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub introduced: Option<AuditIntroduced>,
}

impl UnresolvedImportFinding {
    #[must_use]
    pub fn with_actions(import: UnresolvedImport) -> Self {
        let actions = vec![
            IssueAction::Fix(FixAction {
                kind: FixActionType::ResolveImport,
                auto_fixable: false,
                description: "Fix the import specifier or install the missing module".to_string(),
                note: Some(
                    "Verify the module path and check tsconfig paths configuration".to_string(),
                ),
                available_in_catalogs: None,
                suggested_target: None,
            }),
            IssueAction::AddToConfig(AddToConfigAction {
                kind: AddToConfigKind::AddToConfig,
                auto_fixable: false,
                description: format!(
                    "Add \"{}\" to ignoreUnresolvedImports in fallow config",
                    import.specifier
                ),
                config_key: "ignoreUnresolvedImports".to_string(),
                value: AddToConfigValue::Scalar(import.specifier.clone()),
                value_schema: Some(
                    "https://raw.githubusercontent.com/fallow-rs/fallow/main/schema.json#/properties/ignoreUnresolvedImports/items"
                        .to_string(),
                ),
            }),
            IssueAction::SuppressLine(SuppressLineAction {
                kind: SuppressLineKind::SuppressLine,
                auto_fixable: false,
                description: "Suppress with an inline comment above the line".to_string(),
                comment: "// fallow-ignore-next-line unresolved-import".to_string(),
                scope: None,
            }),
        ];
        Self {
            import,
            actions,
            introduced: None,
        }
    }
}

/// Actions wrapper for a [`CircularDependency`] finding.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CircularDependencyFinding {
    #[serde(flatten)]
    pub cycle: CircularDependency,
    pub actions: Vec<IssueAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub introduced: Option<AuditIntroduced>,
}

impl CircularDependencyFinding {
    #[must_use]
    pub fn with_actions(cycle: CircularDependency) -> Self {
        let actions = vec![
            IssueAction::Fix(FixAction {
                kind: FixActionType::RefactorCycle,
                auto_fixable: false,
                description: "Extract shared logic into a separate module to break the cycle"
                    .to_string(),
                note: Some(
                    "Circular imports can cause initialization issues and make code harder to reason about"
                        .to_string(),
                ),
                available_in_catalogs: None,
                suggested_target: None,
            }),
            IssueAction::SuppressLine(SuppressLineAction {
                kind: SuppressLineKind::SuppressLine,
                auto_fixable: false,
                description: "Suppress with an inline comment above the line".to_string(),
                comment: "// fallow-ignore-next-line circular-dependency".to_string(),
                scope: None,
            }),
        ];
        Self {
            cycle,
            actions,
            introduced: None,
        }
    }
}

/// Actions wrapper for a [`ReExportCycle`] finding.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ReExportCycleFinding {
    #[serde(flatten)]
    pub cycle: ReExportCycle,
    pub actions: Vec<IssueAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub introduced: Option<AuditIntroduced>,
}

impl ReExportCycleFinding {
    #[must_use]
    pub fn with_actions(cycle: ReExportCycle) -> Self {
        let suppress_description = match cycle.kind {
            ReExportCycleKind::SelfLoop => {
                "Suppress with a file-level comment at the top of this file. \
                 The cycle is a self-loop, so the suppression covers the entire finding."
                    .to_string()
            }
            ReExportCycleKind::MultiNode => {
                "Suppress with a file-level comment at the top of this file. \
                 One suppression on any member breaks the cycle for every member \
                 (see the sibling `files` array)."
                    .to_string()
            }
        };
        let actions = vec![
            IssueAction::Fix(FixAction {
                kind: FixActionType::RefactorReExportCycle,
                auto_fixable: false,
                description: "Remove one `export * from` (or `export { ... } from`) \
                              statement on any one member to break the cycle"
                    .to_string(),
                note: Some(
                    "Re-export cycles are structurally a no-op: chain propagation through \
                     the loop never reaches a terminating module, so imports from any member \
                     may silently come up empty."
                        .to_string(),
                ),
                available_in_catalogs: None,
                suggested_target: None,
            }),
            IssueAction::SuppressFile(SuppressFileAction {
                kind: SuppressFileKind::SuppressFile,
                auto_fixable: false,
                description: suppress_description,
                comment: "// fallow-ignore-file re-export-cycle".to_string(),
            }),
        ];
        Self {
            cycle,
            actions,
            introduced: None,
        }
    }
}

/// Actions wrapper for a [`BoundaryViolation`] finding.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct BoundaryViolationFinding {
    #[serde(flatten)]
    pub violation: BoundaryViolation,
    pub actions: Vec<IssueAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub introduced: Option<AuditIntroduced>,
}

impl BoundaryViolationFinding {
    #[must_use]
    pub fn with_actions(violation: BoundaryViolation) -> Self {
        let actions = vec![
            IssueAction::Fix(FixAction {
                kind: FixActionType::RefactorBoundary,
                auto_fixable: false,
                description: "Move the import through an allowed zone or restructure the dependency"
                    .to_string(),
                note: Some(
                    "This import crosses an architecture boundary that is not permitted by the configured rules"
                        .to_string(),
                ),
                available_in_catalogs: None,
                suggested_target: None,
            }),
            IssueAction::SuppressLine(SuppressLineAction {
                kind: SuppressLineKind::SuppressLine,
                auto_fixable: false,
                description: "Suppress with an inline comment above the line".to_string(),
                comment: "// fallow-ignore-next-line boundary-violation".to_string(),
                scope: None,
            }),
        ];
        Self {
            violation,
            actions,
            introduced: None,
        }
    }
}

/// Actions wrapper for an [`UnusedExport`] finding.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct UnusedExportFinding {
    #[serde(flatten)]
    pub export: UnusedExport,
    pub actions: Vec<IssueAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub introduced: Option<AuditIntroduced>,
}

impl UnusedExportFinding {
    #[must_use]
    pub fn with_actions(export: UnusedExport) -> Self {
        let note = if export.is_re_export {
            Some(
                "This finding originates from a re-export; verify it is not part of your public API before removing"
                    .to_string(),
            )
        } else {
            None
        };
        let actions = vec![
            IssueAction::Fix(FixAction {
                kind: FixActionType::RemoveExport,
                auto_fixable: true,
                description: "Remove the unused export from the public API".to_string(),
                note,
                available_in_catalogs: None,
                suggested_target: None,
            }),
            IssueAction::SuppressLine(SuppressLineAction {
                kind: SuppressLineKind::SuppressLine,
                auto_fixable: false,
                description: "Suppress with an inline comment above the line".to_string(),
                comment: "// fallow-ignore-next-line unused-export".to_string(),
                scope: None,
            }),
        ];
        Self {
            export,
            actions,
            introduced: None,
        }
    }
}

/// Actions wrapper for an [`UnusedExport`] finding consumed under `unused_types`.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct UnusedTypeFinding {
    #[serde(flatten)]
    pub export: UnusedExport,
    pub actions: Vec<IssueAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub introduced: Option<AuditIntroduced>,
}

impl UnusedTypeFinding {
    #[must_use]
    pub fn with_actions(export: UnusedExport) -> Self {
        let note = if export.is_re_export {
            Some(
                "This finding originates from a re-export; verify it is not part of your public API before removing"
                    .to_string(),
            )
        } else {
            None
        };
        let actions = vec![
            IssueAction::Fix(FixAction {
                kind: FixActionType::RemoveExport,
                auto_fixable: true,
                description:
                    "Remove the `export` (or `export type`) keyword from the type declaration"
                        .to_string(),
                note,
                available_in_catalogs: None,
                suggested_target: None,
            }),
            IssueAction::SuppressLine(SuppressLineAction {
                kind: SuppressLineKind::SuppressLine,
                auto_fixable: false,
                description: "Suppress with an inline comment above the line".to_string(),
                comment: "// fallow-ignore-next-line unused-type".to_string(),
                scope: None,
            }),
        ];
        Self {
            export,
            actions,
            introduced: None,
        }
    }
}

/// Actions wrapper for an [`UnusedMember`] finding consumed under `unused_enum_members`.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct UnusedEnumMemberFinding {
    #[serde(flatten)]
    pub member: UnusedMember,
    pub actions: Vec<IssueAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub introduced: Option<AuditIntroduced>,
}

impl UnusedEnumMemberFinding {
    #[must_use]
    pub fn with_actions(member: UnusedMember) -> Self {
        let actions = vec![
            IssueAction::Fix(FixAction {
                kind: FixActionType::RemoveEnumMember,
                auto_fixable: true,
                description: "Remove this enum member".to_string(),
                note: None,
                available_in_catalogs: None,
                suggested_target: None,
            }),
            IssueAction::SuppressLine(SuppressLineAction {
                kind: SuppressLineKind::SuppressLine,
                auto_fixable: false,
                description: "Suppress with an inline comment above the line".to_string(),
                comment: "// fallow-ignore-next-line unused-enum-member".to_string(),
                scope: None,
            }),
        ];
        Self {
            member,
            actions,
            introduced: None,
        }
    }
}

/// Actions wrapper for an [`UnusedMember`] finding consumed under `unused_class_members`.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct UnusedClassMemberFinding {
    #[serde(flatten)]
    pub member: UnusedMember,
    pub actions: Vec<IssueAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub introduced: Option<AuditIntroduced>,
}

impl UnusedClassMemberFinding {
    #[must_use]
    pub fn with_actions(member: UnusedMember) -> Self {
        let actions = vec![
            IssueAction::Fix(FixAction {
                kind: FixActionType::RemoveClassMember,
                auto_fixable: false,
                description: "Remove this class member".to_string(),
                note: Some(
                    "Class member may be used via dependency injection or decorators".to_string(),
                ),
                available_in_catalogs: None,
                suggested_target: None,
            }),
            IssueAction::SuppressLine(SuppressLineAction {
                kind: SuppressLineKind::SuppressLine,
                auto_fixable: false,
                description: "Suppress with an inline comment above the line".to_string(),
                comment: "// fallow-ignore-next-line unused-class-member".to_string(),
                scope: None,
            }),
        ];
        Self {
            member,
            actions,
            introduced: None,
        }
    }
}

fn build_unused_dependency_actions(
    dep: &UnusedDependency,
    package_json_location: &str,
    suppress_issue_kind: &str,
) -> Vec<IssueAction> {
    let mut actions = Vec::with_capacity(2);
    let cross_workspace = !dep.used_in_workspaces.is_empty();
    actions.push(if cross_workspace {
        IssueAction::Fix(FixAction {
            kind: FixActionType::MoveDependency,
            auto_fixable: false,
            description: "Move this dependency to the workspace package.json that imports it"
                .to_string(),
            note: Some(
                "fallow fix will not remove dependencies that are imported by another workspace"
                    .to_string(),
            ),
            available_in_catalogs: None,
            suggested_target: None,
        })
    } else {
        IssueAction::Fix(FixAction {
            kind: FixActionType::RemoveDependency,
            auto_fixable: true,
            description: format!("Remove from {package_json_location} in package.json"),
            note: None,
            available_in_catalogs: None,
            suggested_target: None,
        })
    });
    actions.push(build_ignore_dependencies_suppress_action(
        &dep.package_name,
        suppress_issue_kind,
    ));
    actions
}

fn build_ignore_dependencies_suppress_action(
    package_name: &str,
    _suppress_issue_kind: &str,
) -> IssueAction {
    IssueAction::AddToConfig(AddToConfigAction {
        kind: AddToConfigKind::AddToConfig,
        auto_fixable: false,
        description: format!("Add \"{package_name}\" to ignoreDependencies in fallow config"),
        config_key: "ignoreDependencies".to_string(),
        value: AddToConfigValue::Scalar(package_name.to_string()),
        value_schema: Some(
            "https://raw.githubusercontent.com/fallow-rs/fallow/main/schema.json#/properties/ignoreDependencies/items"
                .to_string(),
        ),
    })
}

/// Actions wrapper for an [`UnusedDependency`] finding consumed under `unused_dependencies`.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct UnusedDependencyFinding {
    #[serde(flatten)]
    pub dep: UnusedDependency,
    pub actions: Vec<IssueAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub introduced: Option<AuditIntroduced>,
}

impl UnusedDependencyFinding {
    #[must_use]
    pub fn with_actions(dep: UnusedDependency) -> Self {
        let actions = build_unused_dependency_actions(&dep, "dependencies", "unused-dependency");
        Self {
            dep,
            actions,
            introduced: None,
        }
    }
}

/// Actions wrapper for an [`UnusedDependency`] finding consumed under `unused_dev_dependencies`.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct UnusedDevDependencyFinding {
    #[serde(flatten)]
    pub dep: UnusedDependency,
    pub actions: Vec<IssueAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub introduced: Option<AuditIntroduced>,
}

impl UnusedDevDependencyFinding {
    #[must_use]
    pub fn with_actions(dep: UnusedDependency) -> Self {
        let actions =
            build_unused_dependency_actions(&dep, "devDependencies", "unused-dev-dependency");
        Self {
            dep,
            actions,
            introduced: None,
        }
    }
}

/// Actions wrapper for an [`UnusedDependency`] finding consumed under `unused_optional_dependencies`.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct UnusedOptionalDependencyFinding {
    #[serde(flatten)]
    pub dep: UnusedDependency,
    pub actions: Vec<IssueAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub introduced: Option<AuditIntroduced>,
}

impl UnusedOptionalDependencyFinding {
    #[must_use]
    pub fn with_actions(dep: UnusedDependency) -> Self {
        let actions =
            build_unused_dependency_actions(&dep, "optionalDependencies", "unused-dependency");
        Self {
            dep,
            actions,
            introduced: None,
        }
    }
}

/// Actions wrapper for an [`UnlistedDependency`] finding.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct UnlistedDependencyFinding {
    #[serde(flatten)]
    pub dep: UnlistedDependency,
    pub actions: Vec<IssueAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub introduced: Option<AuditIntroduced>,
}

impl UnlistedDependencyFinding {
    #[must_use]
    pub fn with_actions(dep: UnlistedDependency) -> Self {
        let actions = vec![
            IssueAction::Fix(FixAction {
                kind: FixActionType::InstallDependency,
                auto_fixable: false,
                description: "Add this package to dependencies in package.json".to_string(),
                note: Some(
                    "Verify this package should be a direct dependency before adding".to_string(),
                ),
                available_in_catalogs: None,
                suggested_target: None,
            }),
            build_ignore_dependencies_suppress_action(&dep.package_name, "unlisted-dependency"),
        ];
        Self {
            dep,
            actions,
            introduced: None,
        }
    }
}

/// Actions wrapper for a [`TypeOnlyDependency`] finding.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TypeOnlyDependencyFinding {
    #[serde(flatten)]
    pub dep: TypeOnlyDependency,
    pub actions: Vec<IssueAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub introduced: Option<AuditIntroduced>,
}

impl TypeOnlyDependencyFinding {
    #[must_use]
    pub fn with_actions(dep: TypeOnlyDependency) -> Self {
        let actions = vec![
            IssueAction::Fix(FixAction {
                kind: FixActionType::MoveToDev,
                auto_fixable: false,
                description: "Move to devDependencies (only type imports are used)".to_string(),
                note: Some(
                    "Type imports are erased at runtime so this dependency is not needed in production"
                        .to_string(),
                ),
                available_in_catalogs: None,
                suggested_target: None,
            }),
            build_ignore_dependencies_suppress_action(&dep.package_name, "type-only-dependency"),
        ];
        Self {
            dep,
            actions,
            introduced: None,
        }
    }
}

/// Actions wrapper for a [`TestOnlyDependency`] finding.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TestOnlyDependencyFinding {
    #[serde(flatten)]
    pub dep: TestOnlyDependency,
    pub actions: Vec<IssueAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub introduced: Option<AuditIntroduced>,
}

impl TestOnlyDependencyFinding {
    #[must_use]
    pub fn with_actions(dep: TestOnlyDependency) -> Self {
        let actions = vec![
            IssueAction::Fix(FixAction {
                kind: FixActionType::MoveToDev,
                auto_fixable: false,
                description: "Move to devDependencies (only test files import this)".to_string(),
                note: Some(
                    "Only test files import this package so it does not need to be a production dependency"
                        .to_string(),
                ),
                available_in_catalogs: None,
                suggested_target: None,
            }),
            build_ignore_dependencies_suppress_action(&dep.package_name, "test-only-dependency"),
        ];
        Self {
            dep,
            actions,
            introduced: None,
        }
    }
}

/// Actions wrapper for a [`DuplicateExport`] finding.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DuplicateExportFinding {
    #[serde(flatten)]
    pub export: DuplicateExport,
    pub actions: Vec<IssueAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub introduced: Option<AuditIntroduced>,
}

impl DuplicateExportFinding {
    #[must_use]
    pub fn with_actions(export: DuplicateExport) -> Self {
        let mut actions: Vec<IssueAction> = Vec::with_capacity(3);

        if let Some(rules) = build_duplicate_exports_ignore_rules(&export) {
            actions.push(IssueAction::AddToConfig(AddToConfigAction {
                kind: AddToConfigKind::AddToConfig,
                auto_fixable: false,
                description: "Add an ignoreExports rule so these files are excluded from duplicate-export grouping (use when this duplication is an intentional namespace-barrel API).".to_string(),
                config_key: "ignoreExports".to_string(),
                value: AddToConfigValue::ExportsRules(rules),
                value_schema: Some(IGNORE_EXPORTS_VALUE_SCHEMA.to_string()),
            }));
        }

        actions.push(IssueAction::Fix(FixAction {
            kind: FixActionType::RemoveDuplicate,
            auto_fixable: false,
            description: "Keep one canonical export location and remove the others".to_string(),
            note: Some(NAMESPACE_BARREL_HINT.to_string()),
            available_in_catalogs: None,
            suggested_target: None,
        }));

        actions.push(IssueAction::SuppressLine(SuppressLineAction {
            kind: SuppressLineKind::SuppressLine,
            auto_fixable: false,
            description: "Suppress with an inline comment above the line".to_string(),
            comment: "// fallow-ignore-next-line duplicate-export".to_string(),
            scope: Some(SuppressLineScope::PerLocation),
        }));

        Self {
            export,
            actions,
            introduced: None,
        }
    }

    pub fn set_config_fixable(&mut self, fixable: bool) {
        if let Some(IssueAction::AddToConfig(action)) = self.actions.first_mut() {
            action.auto_fixable = fixable;
        }
    }
}

fn build_duplicate_exports_ignore_rules(
    export: &DuplicateExport,
) -> Option<Vec<IgnoreExportsRule>> {
    let mut entries: Vec<IgnoreExportsRule> = Vec::with_capacity(export.locations.len());
    for loc in &export.locations {
        let path = loc.path.to_string_lossy().replace('\\', "/");
        if path.is_empty() {
            continue;
        }
        if entries.iter().any(|existing| existing.file == path) {
            continue;
        }
        entries.push(IgnoreExportsRule {
            file: path,
            exports: vec!["*".to_string()],
        });
    }
    if entries.is_empty() {
        None
    } else {
        Some(entries)
    }
}

/// Actions wrapper for an [`UnusedCatalogEntry`] finding.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct UnusedCatalogEntryFinding {
    #[serde(flatten)]
    pub entry: UnusedCatalogEntry,
    pub actions: Vec<IssueAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub introduced: Option<AuditIntroduced>,
}

impl UnusedCatalogEntryFinding {
    #[must_use]
    pub fn with_actions(entry: UnusedCatalogEntry) -> Self {
        let auto_fixable = entry.hardcoded_consumers.is_empty();
        let actions = vec![
            IssueAction::Fix(FixAction {
                kind: FixActionType::RemoveCatalogEntry,
                auto_fixable,
                description: "Remove the entry from pnpm-workspace.yaml".to_string(),
                note: Some(
                    "If any consumer declares the same package with a hardcoded version, switch the consumer to `catalog:` before removing"
                        .to_string(),
                ),
                available_in_catalogs: None,
                suggested_target: None,
            }),
            IssueAction::SuppressLine(SuppressLineAction {
                kind: SuppressLineKind::SuppressLine,
                auto_fixable: false,
                description: "Suppress with a YAML comment above the line".to_string(),
                comment: "# fallow-ignore-next-line unused-catalog-entry".to_string(),
                scope: None,
            }),
        ];
        Self {
            entry,
            actions,
            introduced: None,
        }
    }
}

/// Actions wrapper for an [`EmptyCatalogGroup`] finding.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct EmptyCatalogGroupFinding {
    #[serde(flatten)]
    pub group: EmptyCatalogGroup,
    pub actions: Vec<IssueAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub introduced: Option<AuditIntroduced>,
}

impl EmptyCatalogGroupFinding {
    #[must_use]
    pub fn with_actions(group: EmptyCatalogGroup) -> Self {
        let actions = vec![
            IssueAction::Fix(FixAction {
                kind: FixActionType::RemoveEmptyCatalogGroup,
                auto_fixable: true,
                description: "Remove the empty named catalog group from pnpm-workspace.yaml"
                    .to_string(),
                note: Some(
                    "Only named groups under `catalogs:` are flagged; the top-level `catalog:` hook is intentionally ignored"
                        .to_string(),
                ),
                available_in_catalogs: None,
                suggested_target: None,
            }),
            IssueAction::SuppressLine(SuppressLineAction {
                kind: SuppressLineKind::SuppressLine,
                auto_fixable: false,
                description: "Suppress with a YAML comment above the line".to_string(),
                comment: "# fallow-ignore-next-line empty-catalog-group".to_string(),
                scope: None,
            }),
        ];
        Self {
            group,
            actions,
            introduced: None,
        }
    }
}

/// Actions wrapper for an [`UnresolvedCatalogReference`] finding.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct UnresolvedCatalogReferenceFinding {
    #[serde(flatten)]
    pub reference: UnresolvedCatalogReference,
    pub actions: Vec<IssueAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub introduced: Option<AuditIntroduced>,
}

impl UnresolvedCatalogReferenceFinding {
    #[must_use]
    pub fn with_actions(reference: UnresolvedCatalogReference) -> Self {
        let consumer_path = reference.path.to_string_lossy().replace('\\', "/");
        let primary = if reference.available_in_catalogs.is_empty() {
            IssueAction::Fix(FixAction {
                kind: FixActionType::AddCatalogEntry,
                auto_fixable: false,
                description: format!(
                    "Add `{}` to the `{}` catalog in pnpm-workspace.yaml",
                    reference.entry_name, reference.catalog_name
                ),
                note: Some(
                    "Pin a version that satisfies the consumer's import; no other catalog declares this package today"
                        .to_string(),
                ),
                available_in_catalogs: None,
                suggested_target: None,
            })
        } else {
            let available = reference.available_in_catalogs.clone();
            let suggested_target = (available.len() == 1).then(|| available[0].clone());
            IssueAction::Fix(FixAction {
                kind: FixActionType::UpdateCatalogReference,
                auto_fixable: false,
                description: format!(
                    "Switch the reference from `catalog:{}` to a catalog that declares `{}`",
                    reference.catalog_name, reference.entry_name
                ),
                note: None,
                available_in_catalogs: Some(available),
                suggested_target,
            })
        };

        let fallback = IssueAction::Fix(FixAction {
            kind: FixActionType::RemoveCatalogReference,
            auto_fixable: false,
            description:
                "Remove the catalog reference and pin a hardcoded version in package.json"
                    .to_string(),
            note: Some(
                "Use only when neither another catalog declares the package nor the named catalog should grow to include it"
                    .to_string(),
            ),
            available_in_catalogs: None,
            suggested_target: None,
        });

        let mut suppress_value = serde_json::Map::new();
        suppress_value.insert(
            "package".to_string(),
            serde_json::Value::String(reference.entry_name.clone()),
        );
        suppress_value.insert(
            "catalog".to_string(),
            serde_json::Value::String(reference.catalog_name.clone()),
        );
        suppress_value.insert(
            "consumer".to_string(),
            serde_json::Value::String(consumer_path),
        );
        let suppress = IssueAction::AddToConfig(AddToConfigAction {
            kind: AddToConfigKind::AddToConfig,
            auto_fixable: false,
            description: "Suppress this reference via ignoreCatalogReferences in fallow config (use when the catalog edit is intentionally landing in a separate PR or the package is a placeholder).".to_string(),
            config_key: "ignoreCatalogReferences".to_string(),
            value: AddToConfigValue::RuleObject(suppress_value),
            value_schema: Some(IGNORE_CATALOG_REFERENCES_VALUE_SCHEMA.to_string()),
        });

        Self {
            reference,
            actions: vec![primary, fallback, suppress],
            introduced: None,
        }
    }
}

/// Wire-shape envelope for an [`UnusedDependencyOverride`] finding. Carries
/// a `remove-dependency-override` primary plus an `add-to-config`
/// `ignoreDependencyOverrides` suppress scoped to the target package and
/// declaration source.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct UnusedDependencyOverrideFinding {
    /// The underlying finding.
    #[serde(flatten)]
    pub entry: UnusedDependencyOverride,
    /// Suggested next steps. Always emitted.
    pub actions: Vec<IssueAction>,
    /// Set by the audit pass when this finding is introduced relative to
    /// the merge-base.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub introduced: Option<AuditIntroduced>,
}

impl UnusedDependencyOverrideFinding {
    /// Build the wrapper.
    #[must_use]
    pub fn with_actions(entry: UnusedDependencyOverride) -> Self {
        let mut actions: Vec<IssueAction> = Vec::with_capacity(2);
        actions.push(IssueAction::Fix(FixAction {
            kind: FixActionType::RemoveDependencyOverride,
            auto_fixable: false,
            description: "Remove the override entry from pnpm-workspace.yaml or pnpm.overrides"
                .to_string(),
            note: Some(
                "Conservative static check; verify against `pnpm install --frozen-lockfile` before removing in case the override targets a transitive dependency (CVE-fix pattern)"
                    .to_string(),
            ),
            available_in_catalogs: None,
            suggested_target: None,
        }));

        if let Some(suppress) = build_ignore_dependency_overrides_suppress(
            Some(&entry.target_package),
            &entry.raw_key,
            entry.source,
        ) {
            actions.push(suppress);
        }

        Self {
            entry,
            actions,
            introduced: None,
        }
    }
}

/// Wire-shape envelope for a [`MisconfiguredDependencyOverride`] finding.
/// Carries a `fix-dependency-override` primary plus the conditional
/// `add-to-config` `ignoreDependencyOverrides` suppress (skipped when both
/// `target_package` and `raw_key` are empty, since the rule matcher keys on
/// a non-empty package name).
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct MisconfiguredDependencyOverrideFinding {
    /// The underlying finding.
    #[serde(flatten)]
    pub entry: MisconfiguredDependencyOverride,
    /// Suggested next steps. Always emitted.
    pub actions: Vec<IssueAction>,
    /// Set by the audit pass when this finding is introduced relative to
    /// the merge-base.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub introduced: Option<AuditIntroduced>,
}

impl MisconfiguredDependencyOverrideFinding {
    /// Build the wrapper. The suppress action is omitted when neither
    /// `target_package` (set on `EmptyValue` cases) nor `raw_key` provides a
    /// non-empty package name; an `ignoreDependencyOverrides` entry with
    /// `package: ""` would be silently ignored by the config parser.
    #[must_use]
    pub fn with_actions(entry: MisconfiguredDependencyOverride) -> Self {
        let mut actions: Vec<IssueAction> = Vec::with_capacity(2);
        actions.push(IssueAction::Fix(FixAction {
            kind: FixActionType::FixDependencyOverride,
            auto_fixable: false,
            description:
                "Fix the override key or value: pnpm refuses to honor entries with an unparsable key or empty value"
                    .to_string(),
            note: Some(
                "Common shapes: bare `pkg`, scoped `@scope/pkg`, version-selector `pkg@<2`, parent-chain `parent>child`. Valid values include semver ranges, `-` (removal), `$ref` (self-ref), and `npm:alias@^1`."
                    .to_string(),
            ),
            available_in_catalogs: None,
            suggested_target: None,
        }));

        if let Some(suppress) = build_ignore_dependency_overrides_suppress(
            entry.target_package.as_deref(),
            &entry.raw_key,
            entry.source,
        ) {
            actions.push(suppress);
        }

        Self {
            entry,
            actions,
            introduced: None,
        }
    }
}

/// Shared `add-to-config` `ignoreDependencyOverrides` builder for the two
/// override findings. Returns `None` when no non-empty package name is
/// available; the config parser silently drops entries with an empty
/// `package` field, so emitting one would be a no-op that misleads agents.
fn build_ignore_dependency_overrides_suppress(
    target_package: Option<&str>,
    raw_key: &str,
    source: DependencyOverrideSource,
) -> Option<IssueAction> {
    let package = target_package
        .filter(|s| !s.is_empty())
        .or_else(|| Some(raw_key).filter(|s| !s.is_empty()))?
        .to_string();
    let mut value = serde_json::Map::new();
    value.insert("package".to_string(), serde_json::Value::String(package));
    value.insert(
        "source".to_string(),
        serde_json::Value::String(source.as_label().to_string()),
    );
    Some(IssueAction::AddToConfig(AddToConfigAction {
        kind: AddToConfigKind::AddToConfig,
        auto_fixable: false,
        description: "Suppress this override finding via ignoreDependencyOverrides in fallow config (use for CVE-fix overrides that target a purely-transitive package).".to_string(),
        config_key: "ignoreDependencyOverrides".to_string(),
        value: AddToConfigValue::RuleObject(value),
        value_schema: Some(IGNORE_DEPENDENCY_OVERRIDES_VALUE_SCHEMA.to_string()),
    }))
}

#[cfg(test)]
mod position_0_invariants {
    use super::*;
    use crate::output::FixActionType;
    use crate::results::{DependencyOverrideSource, DuplicateLocation};
    use std::path::PathBuf;

    /// Helper: extract the kebab-case `type` discriminant from an
    /// [`IssueAction`] at a specific position. Returns `None` when the
    /// position is out of bounds or the action shape lacks a discriminant
    /// (today every variant has one).
    fn action_type(action: &IssueAction) -> &'static str {
        match action {
            IssueAction::Fix(fix) => match fix.kind {
                FixActionType::RemoveExport => "remove-export",
                FixActionType::DeleteFile => "delete-file",
                FixActionType::RemoveDependency => "remove-dependency",
                FixActionType::MoveDependency => "move-dependency",
                FixActionType::RemoveEnumMember => "remove-enum-member",
                FixActionType::RemoveClassMember => "remove-class-member",
                FixActionType::ResolveImport => "resolve-import",
                FixActionType::InstallDependency => "install-dependency",
                FixActionType::RemoveDuplicate => "remove-duplicate",
                FixActionType::MoveToDev => "move-to-dev",
                FixActionType::RefactorCycle => "refactor-cycle",
                FixActionType::RefactorReExportCycle => "refactor-re-export-cycle",
                FixActionType::RefactorBoundary => "refactor-boundary",
                FixActionType::ExportType => "export-type",
                FixActionType::RemoveCatalogEntry => "remove-catalog-entry",
                FixActionType::RemoveEmptyCatalogGroup => "remove-empty-catalog-group",
                FixActionType::UpdateCatalogReference => "update-catalog-reference",
                FixActionType::AddCatalogEntry => "add-catalog-entry",
                FixActionType::RemoveCatalogReference => "remove-catalog-reference",
                FixActionType::RemoveDependencyOverride => "remove-dependency-override",
                FixActionType::FixDependencyOverride => "fix-dependency-override",
            },
            IssueAction::SuppressLine(_) => "suppress-line",
            IssueAction::SuppressFile(_) => "suppress-file",
            IssueAction::AddToConfig(_) => "add-to-config",
        }
    }

    #[test]
    fn unresolved_import_actions_include_ignore_unresolved_imports_config_suppress() {
        let inner = UnresolvedImport {
            specifier: "@example/icons".to_string(),
            path: PathBuf::from("src/index.ts"),
            line: 4,
            col: 12,
            specifier_col: 18,
        };
        let finding = UnresolvedImportFinding::with_actions(inner);

        assert_eq!(action_type(&finding.actions[0]), "resolve-import");
        assert_eq!(action_type(&finding.actions[1]), "add-to-config");
        let IssueAction::AddToConfig(action) = &finding.actions[1] else {
            panic!("position-1 should be AddToConfig");
        };
        assert!(!action.auto_fixable);
        assert_eq!(action.config_key, "ignoreUnresolvedImports");
        let AddToConfigValue::Scalar(value) = &action.value else {
            panic!("ignoreUnresolvedImports action should carry a scalar value");
        };
        assert_eq!(value, "@example/icons");
        assert_eq!(
            action.value_schema.as_deref(),
            Some(
                "https://raw.githubusercontent.com/fallow-rs/fallow/main/schema.json#/properties/ignoreUnresolvedImports/items"
            )
        );
    }

    #[test]
    fn unresolved_catalog_position_0_is_add_when_no_alternatives() {
        let inner = UnresolvedCatalogReference {
            entry_name: "react".to_string(),
            catalog_name: "default".to_string(),
            path: PathBuf::from("apps/web/package.json"),
            line: 7,
            available_in_catalogs: Vec::new(),
        };
        let finding = UnresolvedCatalogReferenceFinding::with_actions(inner);
        assert_eq!(
            action_type(&finding.actions[0]),
            "add-catalog-entry",
            "position-0 must be `add-catalog-entry` when no alternative catalog declares the package"
        );
        let IssueAction::Fix(fix) = &finding.actions[0] else {
            panic!("position-0 should be an IssueAction::Fix");
        };
        assert!(
            fix.available_in_catalogs.is_none(),
            "add-catalog-entry must NOT carry available_in_catalogs"
        );
        assert!(
            fix.suggested_target.is_none(),
            "add-catalog-entry must NOT carry suggested_target"
        );
    }

    #[test]
    fn unresolved_catalog_position_0_is_update_when_alternatives_exist() {
        let inner = UnresolvedCatalogReference {
            entry_name: "react".to_string(),
            catalog_name: "default".to_string(),
            path: PathBuf::from("apps/web/package.json"),
            line: 7,
            available_in_catalogs: vec!["react18".to_string()],
        };
        let finding = UnresolvedCatalogReferenceFinding::with_actions(inner);
        assert_eq!(
            action_type(&finding.actions[0]),
            "update-catalog-reference",
            "position-0 must be `update-catalog-reference` when at least one alternative catalog declares the package"
        );
        let IssueAction::Fix(fix) = &finding.actions[0] else {
            panic!("position-0 should be an IssueAction::Fix");
        };
        assert_eq!(
            fix.available_in_catalogs.as_deref(),
            Some(&["react18".to_string()][..]),
            "update-catalog-reference must carry the alternative list"
        );
        assert_eq!(
            fix.suggested_target.as_deref(),
            Some("react18"),
            "single-alternative case must surface `suggested_target` for deterministic agents"
        );

        let inner_two = UnresolvedCatalogReference {
            entry_name: "react".to_string(),
            catalog_name: "default".to_string(),
            path: PathBuf::from("apps/web/package.json"),
            line: 7,
            available_in_catalogs: vec!["react17".to_string(), "react18".to_string()],
        };
        let finding_two = UnresolvedCatalogReferenceFinding::with_actions(inner_two);
        assert_eq!(
            action_type(&finding_two.actions[0]),
            "update-catalog-reference"
        );
        let IssueAction::Fix(fix_two) = &finding_two.actions[0] else {
            panic!("position-0 should be an IssueAction::Fix");
        };
        assert!(
            fix_two.suggested_target.is_none(),
            "multi-alternative case must NOT carry `suggested_target` (agent must pick)"
        );
    }

    #[test]
    fn duplicate_exports_position_0_is_add_to_config_not_remove_duplicate() {
        let inner = DuplicateExport {
            export_name: "Root".to_string(),
            locations: vec![
                DuplicateLocation {
                    path: PathBuf::from("components/ui/accordion/index.ts"),
                    line: 1,
                    col: 0,
                },
                DuplicateLocation {
                    path: PathBuf::from("components/ui/dialog/index.ts"),
                    line: 1,
                    col: 0,
                },
            ],
        };
        let finding = DuplicateExportFinding::with_actions(inner);
        assert_eq!(
            action_type(&finding.actions[0]),
            "add-to-config",
            "position-0 must be `add-to-config` (safe `ignoreExports` path), NOT `remove-duplicate`"
        );
        assert_eq!(
            action_type(&finding.actions[1]),
            "remove-duplicate",
            "position-1 must be the destructive `remove-duplicate` fallback"
        );

        let mut promoted = finding;
        promoted.set_config_fixable(true);
        assert_eq!(action_type(&promoted.actions[0]), "add-to-config");
        let IssueAction::AddToConfig(action) = &promoted.actions[0] else {
            panic!("position-0 should still be AddToConfig after set_config_fixable");
        };
        assert!(
            action.auto_fixable,
            "set_config_fixable(true) must flip auto_fixable"
        );
    }

    #[test]
    fn duplicate_exports_no_locations_falls_through_to_remove_duplicate() {
        let inner = DuplicateExport {
            export_name: "Root".to_string(),
            locations: Vec::new(),
        };
        let finding = DuplicateExportFinding::with_actions(inner);
        assert_eq!(
            action_type(&finding.actions[0]),
            "remove-duplicate",
            "with no locations there is no ignoreExports rule to suggest; the destructive remove becomes position-0"
        );

        let mut promoted = finding;
        promoted.set_config_fixable(true);
        assert_eq!(
            action_type(&promoted.actions[0]),
            "remove-duplicate",
            "set_config_fixable is a no-op when position-0 is not add-to-config"
        );
    }

    #[test]
    fn misconfigured_override_drops_suppress_when_no_package_name() {
        let inner = MisconfiguredDependencyOverride {
            raw_key: String::new(),
            target_package: None,
            raw_value: String::new(),
            reason: crate::results::DependencyOverrideMisconfigReason::EmptyValue,
            source: DependencyOverrideSource::PnpmWorkspaceYaml,
            path: PathBuf::from("pnpm-workspace.yaml"),
            line: 12,
        };
        let finding = MisconfiguredDependencyOverrideFinding::with_actions(inner);
        assert_eq!(finding.actions.len(), 1);
        assert_eq!(action_type(&finding.actions[0]), "fix-dependency-override");
    }
}
