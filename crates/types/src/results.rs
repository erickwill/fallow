//! Analysis result types for all issue categories.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::extract::MemberKind;
use crate::output_dead_code::{
    BoundaryViolationFinding, CircularDependencyFinding, DuplicateExportFinding,
    EmptyCatalogGroupFinding, MisconfiguredDependencyOverrideFinding, PrivateTypeLeakFinding,
    ReExportCycleFinding, TestOnlyDependencyFinding, TypeOnlyDependencyFinding,
    UnlistedDependencyFinding, UnresolvedCatalogReferenceFinding, UnresolvedImportFinding,
    UnusedCatalogEntryFinding, UnusedClassMemberFinding, UnusedDependencyFinding,
    UnusedDependencyOverrideFinding, UnusedDevDependencyFinding, UnusedEnumMemberFinding,
    UnusedExportFinding, UnusedFileFinding, UnusedOptionalDependencyFinding, UnusedTypeFinding,
};
use crate::serde_path;
use crate::suppress::{IssueKind, closest_known_kind_name};

/// Summary of detected entry points grouped by discovery source.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct EntryPointSummary {
    pub total: usize,
    pub by_source: Vec<(String, usize)>,
}

/// Complete analysis results.
#[derive(Debug, Default, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct AnalysisResults {
    pub unused_files: Vec<UnusedFileFinding>,
    pub unused_exports: Vec<UnusedExportFinding>,
    pub unused_types: Vec<UnusedTypeFinding>,
    pub private_type_leaks: Vec<PrivateTypeLeakFinding>,
    pub unused_dependencies: Vec<UnusedDependencyFinding>,
    pub unused_dev_dependencies: Vec<UnusedDevDependencyFinding>,
    pub unused_optional_dependencies: Vec<UnusedOptionalDependencyFinding>,
    pub unused_enum_members: Vec<UnusedEnumMemberFinding>,
    pub unused_class_members: Vec<UnusedClassMemberFinding>,
    pub unresolved_imports: Vec<UnresolvedImportFinding>,
    pub unlisted_dependencies: Vec<UnlistedDependencyFinding>,
    pub duplicate_exports: Vec<DuplicateExportFinding>,
    pub type_only_dependencies: Vec<TypeOnlyDependencyFinding>,
    #[serde(default)]
    pub test_only_dependencies: Vec<TestOnlyDependencyFinding>,
    pub circular_dependencies: Vec<CircularDependencyFinding>,
    #[serde(default)]
    pub re_export_cycles: Vec<ReExportCycleFinding>,
    #[serde(default)]
    pub boundary_violations: Vec<BoundaryViolationFinding>,
    #[serde(default)]
    pub stale_suppressions: Vec<StaleSuppression>,
    #[serde(default)]
    pub unused_catalog_entries: Vec<UnusedCatalogEntryFinding>,
    #[serde(default)]
    pub empty_catalog_groups: Vec<EmptyCatalogGroupFinding>,
    #[serde(default)]
    pub unresolved_catalog_references: Vec<UnresolvedCatalogReferenceFinding>,
    #[serde(default)]
    pub unused_dependency_overrides: Vec<UnusedDependencyOverrideFinding>,
    #[serde(default)]
    pub misconfigured_dependency_overrides: Vec<MisconfiguredDependencyOverrideFinding>,
    #[serde(skip)]
    pub suppression_count: usize,
    #[serde(skip)]
    pub active_suppressions: Vec<ActiveSuppression>,
    #[serde(skip)]
    pub feature_flags: Vec<FeatureFlag>,
    #[serde(skip)]
    pub export_usages: Vec<ExportUsage>,
    #[serde(skip)]
    pub entry_point_summary: Option<EntryPointSummary>,
}

impl AnalysisResults {
    /// Total number of issues found.
    #[must_use]
    pub const fn total_issues(&self) -> usize {
        self.unused_files.len()
            + self.unused_exports.len()
            + self.unused_types.len()
            + self.private_type_leaks.len()
            + self.unused_dependencies.len()
            + self.unused_dev_dependencies.len()
            + self.unused_optional_dependencies.len()
            + self.unused_enum_members.len()
            + self.unused_class_members.len()
            + self.unresolved_imports.len()
            + self.unlisted_dependencies.len()
            + self.duplicate_exports.len()
            + self.type_only_dependencies.len()
            + self.test_only_dependencies.len()
            + self.circular_dependencies.len()
            + self.re_export_cycles.len()
            + self.boundary_violations.len()
            + self.stale_suppressions.len()
            + self.unused_catalog_entries.len()
            + self.empty_catalog_groups.len()
            + self.unresolved_catalog_references.len()
            + self.unused_dependency_overrides.len()
            + self.misconfigured_dependency_overrides.len()
    }

    #[must_use]
    pub const fn has_issues(&self) -> bool {
        self.total_issues() > 0
    }

    /// Merge `other` into `self`, taking the union of every field.
    pub fn merge_into(&mut self, other: Self) {
        let Self {
            unused_files,
            unused_exports,
            unused_types,
            private_type_leaks,
            unused_dependencies,
            unused_dev_dependencies,
            unused_optional_dependencies,
            unused_enum_members,
            unused_class_members,
            unresolved_imports,
            unlisted_dependencies,
            duplicate_exports,
            type_only_dependencies,
            test_only_dependencies,
            circular_dependencies,
            re_export_cycles,
            boundary_violations,
            stale_suppressions,
            unused_catalog_entries,
            empty_catalog_groups,
            unresolved_catalog_references,
            unused_dependency_overrides,
            misconfigured_dependency_overrides,
            suppression_count,
            active_suppressions,
            feature_flags,
            export_usages,
            entry_point_summary,
        } = other;

        self.unused_files.extend(unused_files);
        self.unused_exports.extend(unused_exports);
        self.unused_types.extend(unused_types);
        self.private_type_leaks.extend(private_type_leaks);
        self.unused_dependencies.extend(unused_dependencies);
        self.unused_dev_dependencies.extend(unused_dev_dependencies);
        self.unused_optional_dependencies
            .extend(unused_optional_dependencies);
        self.unused_enum_members.extend(unused_enum_members);
        self.unused_class_members.extend(unused_class_members);
        self.unresolved_imports.extend(unresolved_imports);
        self.unlisted_dependencies.extend(unlisted_dependencies);
        self.duplicate_exports.extend(duplicate_exports);
        self.type_only_dependencies.extend(type_only_dependencies);
        self.test_only_dependencies.extend(test_only_dependencies);
        self.circular_dependencies.extend(circular_dependencies);
        self.re_export_cycles.extend(re_export_cycles);
        self.boundary_violations.extend(boundary_violations);
        self.stale_suppressions.extend(stale_suppressions);
        self.unused_catalog_entries.extend(unused_catalog_entries);
        self.empty_catalog_groups.extend(empty_catalog_groups);
        self.unresolved_catalog_references
            .extend(unresolved_catalog_references);
        self.unused_dependency_overrides
            .extend(unused_dependency_overrides);
        self.misconfigured_dependency_overrides
            .extend(misconfigured_dependency_overrides);
        self.feature_flags.extend(feature_flags);
        self.export_usages.extend(export_usages);
        self.active_suppressions.extend(active_suppressions);
        self.suppression_count += suppression_count;
        if self.entry_point_summary.is_none() {
            self.entry_point_summary = entry_point_summary;
        }
    }

    /// Sort all result arrays for deterministic output ordering.
    #[expect(
        clippy::too_many_lines,
        reason = "one short sort_by per result array; splitting would add indirection without clarity"
    )]
    pub fn sort(&mut self) {
        self.unused_files
            .sort_by(|a, b| a.file.path.cmp(&b.file.path));

        self.unused_exports.sort_by(|a, b| {
            a.export
                .path
                .cmp(&b.export.path)
                .then(a.export.line.cmp(&b.export.line))
                .then(a.export.export_name.cmp(&b.export.export_name))
        });

        self.unused_types.sort_by(|a, b| {
            a.export
                .path
                .cmp(&b.export.path)
                .then(a.export.line.cmp(&b.export.line))
                .then(a.export.export_name.cmp(&b.export.export_name))
        });

        self.private_type_leaks.sort_by(|a, b| {
            a.leak
                .path
                .cmp(&b.leak.path)
                .then(a.leak.line.cmp(&b.leak.line))
                .then(a.leak.export_name.cmp(&b.leak.export_name))
                .then(a.leak.type_name.cmp(&b.leak.type_name))
        });

        self.unused_dependencies.sort_by(|a, b| {
            a.dep
                .path
                .cmp(&b.dep.path)
                .then(a.dep.line.cmp(&b.dep.line))
                .then(a.dep.package_name.cmp(&b.dep.package_name))
        });

        self.unused_dev_dependencies.sort_by(|a, b| {
            a.dep
                .path
                .cmp(&b.dep.path)
                .then(a.dep.line.cmp(&b.dep.line))
                .then(a.dep.package_name.cmp(&b.dep.package_name))
        });

        self.unused_optional_dependencies.sort_by(|a, b| {
            a.dep
                .path
                .cmp(&b.dep.path)
                .then(a.dep.line.cmp(&b.dep.line))
                .then(a.dep.package_name.cmp(&b.dep.package_name))
        });

        self.unused_enum_members.sort_by(|a, b| {
            a.member
                .path
                .cmp(&b.member.path)
                .then(a.member.line.cmp(&b.member.line))
                .then(a.member.parent_name.cmp(&b.member.parent_name))
                .then(a.member.member_name.cmp(&b.member.member_name))
        });

        self.unused_class_members.sort_by(|a, b| {
            a.member
                .path
                .cmp(&b.member.path)
                .then(a.member.line.cmp(&b.member.line))
                .then(a.member.parent_name.cmp(&b.member.parent_name))
                .then(a.member.member_name.cmp(&b.member.member_name))
        });

        self.unresolved_imports.sort_by(|a, b| {
            a.import
                .path
                .cmp(&b.import.path)
                .then(a.import.line.cmp(&b.import.line))
                .then(a.import.col.cmp(&b.import.col))
                .then(a.import.specifier.cmp(&b.import.specifier))
        });

        self.unlisted_dependencies
            .sort_by(|a, b| a.dep.package_name.cmp(&b.dep.package_name));
        for dep in &mut self.unlisted_dependencies {
            dep.dep
                .imported_from
                .sort_by(|a, b| a.path.cmp(&b.path).then(a.line.cmp(&b.line)));
        }

        self.duplicate_exports
            .sort_by(|a, b| a.export.export_name.cmp(&b.export.export_name));
        for dup in &mut self.duplicate_exports {
            dup.export
                .locations
                .sort_by(|a, b| a.path.cmp(&b.path).then(a.line.cmp(&b.line)));
        }

        self.type_only_dependencies.sort_by(|a, b| {
            a.dep
                .path
                .cmp(&b.dep.path)
                .then(a.dep.line.cmp(&b.dep.line))
                .then(a.dep.package_name.cmp(&b.dep.package_name))
        });

        self.test_only_dependencies.sort_by(|a, b| {
            a.dep
                .path
                .cmp(&b.dep.path)
                .then(a.dep.line.cmp(&b.dep.line))
                .then(a.dep.package_name.cmp(&b.dep.package_name))
        });

        self.circular_dependencies.sort_by(|a, b| {
            a.cycle
                .files
                .cmp(&b.cycle.files)
                .then(a.cycle.length.cmp(&b.cycle.length))
        });

        self.re_export_cycles
            .sort_by(|a, b| a.cycle.files.cmp(&b.cycle.files));

        self.boundary_violations.sort_by(|a, b| {
            a.violation
                .from_path
                .cmp(&b.violation.from_path)
                .then(a.violation.line.cmp(&b.violation.line))
                .then(a.violation.col.cmp(&b.violation.col))
                .then(a.violation.to_path.cmp(&b.violation.to_path))
        });

        self.stale_suppressions.sort_by(|a, b| {
            a.path
                .cmp(&b.path)
                .then(a.line.cmp(&b.line))
                .then(a.col.cmp(&b.col))
        });

        self.unused_catalog_entries.sort_by(|a, b| {
            a.entry
                .path
                .cmp(&b.entry.path)
                .then_with(|| {
                    catalog_sort_key(&a.entry.catalog_name)
                        .cmp(&catalog_sort_key(&b.entry.catalog_name))
                })
                .then(a.entry.catalog_name.cmp(&b.entry.catalog_name))
                .then(a.entry.entry_name.cmp(&b.entry.entry_name))
        });
        for finding in &mut self.unused_catalog_entries {
            finding.entry.hardcoded_consumers.sort();
            finding.entry.hardcoded_consumers.dedup();
        }

        self.empty_catalog_groups.sort_by(|a, b| {
            a.group
                .path
                .cmp(&b.group.path)
                .then_with(|| {
                    catalog_sort_key(&a.group.catalog_name)
                        .cmp(&catalog_sort_key(&b.group.catalog_name))
                })
                .then(a.group.catalog_name.cmp(&b.group.catalog_name))
                .then(a.group.line.cmp(&b.group.line))
        });

        self.unresolved_catalog_references.sort_by(|a, b| {
            a.reference
                .path
                .cmp(&b.reference.path)
                .then(a.reference.line.cmp(&b.reference.line))
                .then_with(|| {
                    catalog_sort_key(&a.reference.catalog_name)
                        .cmp(&catalog_sort_key(&b.reference.catalog_name))
                })
                .then(a.reference.catalog_name.cmp(&b.reference.catalog_name))
                .then(a.reference.entry_name.cmp(&b.reference.entry_name))
        });
        for finding in &mut self.unresolved_catalog_references {
            finding.reference.available_in_catalogs.sort();
            finding.reference.available_in_catalogs.dedup();
        }

        self.unused_dependency_overrides.sort_by(|a, b| {
            a.entry
                .path
                .cmp(&b.entry.path)
                .then(a.entry.line.cmp(&b.entry.line))
                .then(a.entry.raw_key.cmp(&b.entry.raw_key))
        });

        self.misconfigured_dependency_overrides.sort_by(|a, b| {
            a.entry
                .path
                .cmp(&b.entry.path)
                .then(a.entry.line.cmp(&b.entry.line))
                .then(a.entry.raw_key.cmp(&b.entry.raw_key))
        });

        self.feature_flags.sort_by(|a, b| {
            a.path
                .cmp(&b.path)
                .then(a.line.cmp(&b.line))
                .then(a.flag_name.cmp(&b.flag_name))
        });

        for usage in &mut self.export_usages {
            usage.reference_locations.sort_by(|a, b| {
                a.path
                    .cmp(&b.path)
                    .then(a.line.cmp(&b.line))
                    .then(a.col.cmp(&b.col))
            });
        }
        self.export_usages.sort_by(|a, b| {
            a.path
                .cmp(&b.path)
                .then(a.line.cmp(&b.line))
                .then(a.export_name.cmp(&b.export_name))
        });
    }
}

/// Sort key for catalog names.
fn catalog_sort_key(name: &str) -> (u8, &str) {
    if name == "default" {
        (0, name)
    } else {
        (1, name)
    }
}

/// A file that is not reachable from any entry point.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct UnusedFile {
    #[serde(serialize_with = "serde_path::serialize")]
    pub path: PathBuf,
}

/// An export that is never imported by other modules.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct UnusedExport {
    #[serde(serialize_with = "serde_path::serialize")]
    pub path: PathBuf,
    pub export_name: String,
    pub is_type_only: bool,
    pub line: u32,
    pub col: u32,
    pub span_start: u32,
    pub is_re_export: bool,
}

/// A public export signature that references a same-file private type.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PrivateTypeLeak {
    #[serde(serialize_with = "serde_path::serialize")]
    pub path: PathBuf,
    pub export_name: String,
    pub type_name: String,
    pub line: u32,
    pub col: u32,
    pub span_start: u32,
}

/// A dependency that is listed in package.json but never imported.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct UnusedDependency {
    pub package_name: String,
    pub location: DependencyLocation,
    #[serde(serialize_with = "serde_path::serialize")]
    pub path: PathBuf,
    pub line: u32,
    #[serde(
        serialize_with = "serde_path::serialize_vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    #[cfg_attr(feature = "schema", schemars(default))]
    pub used_in_workspaces: Vec<PathBuf>,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum DependencyLocation {
    Dependencies,
    DevDependencies,
    OptionalDependencies,
}

/// An unused enum or class member.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct UnusedMember {
    #[serde(serialize_with = "serde_path::serialize")]
    pub path: PathBuf,
    pub parent_name: String,
    pub member_name: String,
    pub kind: MemberKind,
    pub line: u32,
    pub col: u32,
}

/// An import that could not be resolved.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct UnresolvedImport {
    #[serde(serialize_with = "serde_path::serialize")]
    pub path: PathBuf,
    pub specifier: String,
    pub line: u32,
    pub col: u32,
    pub specifier_col: u32,
}

/// A dependency used in code but not listed in package.json.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct UnlistedDependency {
    pub package_name: String,
    pub imported_from: Vec<ImportSite>,
}

/// A location where an import occurs.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ImportSite {
    #[serde(serialize_with = "serde_path::serialize")]
    pub path: PathBuf,
    pub line: u32,
    pub col: u32,
}

/// An export that appears multiple times across the project.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DuplicateExport {
    pub export_name: String,
    pub locations: Vec<DuplicateLocation>,
}

/// A location where a duplicate export appears.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DuplicateLocation {
    #[serde(serialize_with = "serde_path::serialize")]
    pub path: PathBuf,
    pub line: u32,
    pub col: u32,
}

/// A production dependency that is only used via type-only imports.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TypeOnlyDependency {
    pub package_name: String,
    #[serde(serialize_with = "serde_path::serialize")]
    pub path: PathBuf,
    pub line: u32,
}

/// A pnpm catalog entry declared in pnpm-workspace.yaml that no workspace package
/// references via the `catalog:` protocol.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct UnusedCatalogEntry {
    pub entry_name: String,
    pub catalog_name: String,
    #[serde(serialize_with = "serde_path::serialize")]
    pub path: PathBuf,
    pub line: u32,
    #[serde(
        default,
        serialize_with = "serde_path::serialize_vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub hardcoded_consumers: Vec<PathBuf>,
}

/// A named `catalogs.<name>:` group with no entries.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct EmptyCatalogGroup {
    pub catalog_name: String,
    #[serde(serialize_with = "serde_path::serialize")]
    pub path: PathBuf,
    pub line: u32,
}

/// A workspace package.json reference pointing at a catalog missing the package.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct UnresolvedCatalogReference {
    pub entry_name: String,
    pub catalog_name: String,
    #[serde(serialize_with = "serde_path::serialize")]
    pub path: PathBuf,
    pub line: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub available_in_catalogs: Vec<String>,
}

/// Where an override entry was declared.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum DependencyOverrideSource {
    #[serde(rename = "pnpm-workspace.yaml")]
    PnpmWorkspaceYaml,
    #[serde(rename = "package.json")]
    PnpmPackageJson,
}

impl DependencyOverrideSource {
    #[must_use]
    pub const fn as_label(&self) -> &'static str {
        match self {
            Self::PnpmWorkspaceYaml => "pnpm-workspace.yaml",
            Self::PnpmPackageJson => "package.json",
        }
    }
}

impl std::fmt::Display for DependencyOverrideSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_label())
    }
}

/// An entry in pnpm's `overrides:` map whose target package is not declared in any workspace
/// `package.json` and is not present in `pnpm-lock.yaml`.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct UnusedDependencyOverride {
    pub raw_key: String,
    pub target_package: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_package: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_constraint: Option<String>,
    pub version_range: String,
    pub source: DependencyOverrideSource,
    #[serde(serialize_with = "serde_path::serialize")]
    pub path: PathBuf,
    pub line: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hint: Option<String>,
}

/// Why a dependency-override entry is misconfigured.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "kebab-case")]
pub enum DependencyOverrideMisconfigReason {
    UnparsableKey,
    EmptyValue,
}

impl DependencyOverrideMisconfigReason {
    #[must_use]
    pub const fn describe(self) -> &'static str {
        match self {
            Self::UnparsableKey => "override key cannot be parsed",
            Self::EmptyValue => "override value is missing or empty",
        }
    }
}

/// An override entry whose key or value is malformed.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct MisconfiguredDependencyOverride {
    pub raw_key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_package: Option<String>,
    pub raw_value: String,
    pub reason: DependencyOverrideMisconfigReason,
    pub source: DependencyOverrideSource,
    #[serde(serialize_with = "serde_path::serialize")]
    pub path: PathBuf,
    pub line: u32,
}

/// A production dependency that is only imported by test files.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TestOnlyDependency {
    pub package_name: String,
    #[serde(serialize_with = "serde_path::serialize")]
    pub path: PathBuf,
    pub line: u32,
}

/// A circular dependency chain detected in the module graph.
///
/// The `line` and `col` fields carry `#[serde(default)]` so callers reading
/// historical baseline JSON without these fields can still deserialize the
/// struct, but the JSON output layer always emits them (u32 always
/// serializes, never via `skip_serializing_if`). The schemars derive sees
/// the serde defaults and marks both fields optional in the generated
/// schema; the explicit `extend("required" = ...)` override here keeps the
/// schema's `required` array honest about what the JSON output actually
/// contains.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(extend("required" = ["files", "length", "line", "col"])))]
pub struct CircularDependency {
    #[serde(serialize_with = "serde_path::serialize_vec")]
    pub files: Vec<PathBuf>,
    pub length: usize,
    #[serde(default)]
    pub line: u32,
    #[serde(default)]
    pub col: u32,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_cross_package: bool,
}

/// A cycle or self-loop in the re-export edge subgraph.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ReExportCycle {
    #[serde(serialize_with = "serde_path::serialize_vec")]
    pub files: Vec<PathBuf>,
    pub kind: ReExportCycleKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "kebab-case")]
pub enum ReExportCycleKind {
    MultiNode,
    SelfLoop,
}

/// An import that crosses an architecture boundary rule.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct BoundaryViolation {
    #[serde(serialize_with = "serde_path::serialize")]
    pub from_path: PathBuf,
    #[serde(serialize_with = "serde_path::serialize")]
    pub to_path: PathBuf,
    pub from_zone: String,
    pub to_zone: String,
    pub import_specifier: String,
    pub line: u32,
    pub col: u32,
}

/// The origin of a stale suppression: inline comment or JSDoc tag.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum SuppressionOrigin {
    Comment {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        issue_kind: Option<String>,
        is_file_level: bool,
        #[serde(default = "default_true", skip_serializing_if = "is_true")]
        kind_known: bool,
    },
    JsdocTag {
        export_name: String,
    },
}

#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "serde skip_serializing_if takes a reference by contract"
)]
const fn is_true(b: &bool) -> bool {
    *b
}

/// Default for `SuppressionOrigin::Comment.kind_known` when the field is
/// absent from a deserialized payload, paired with `skip_serializing_if = is_true`
/// so schemars marks the field non-required in the generated JSON Schema AND
/// the absent case round-trips to the recognized-kind interpretation.
/// Referenced by the always-emitted `#[serde(default = "default_true")]`
/// attribute. Today `SuppressionOrigin` derives only `Serialize`, so serde
/// itself never calls this; schemars (under the `schema` feature) reads the
/// attribute textually to mark `kind_known` non-required. The `cfg_attr`
/// applies `#[expect(dead_code)]` only on builds WITHOUT the `schema` feature
/// (where the function is genuinely dead): under the feature schemars
/// references it, the lint does not fire, and an unconditional `#[expect]`
/// would be unfulfilled. The function stays un-gated so a future
/// `Deserialize` derive on `SuppressionOrigin` does not produce a missing-
/// function compile error on non-`schema` builds.
#[cfg_attr(
    not(feature = "schema"),
    expect(
        dead_code,
        reason = "referenced via #[serde(default = ...)]; only consumed by schemars under the `schema` feature, dead on default builds today"
    )
)]
const fn default_true() -> bool {
    true
}

/// A suppression comment or JSDoc tag that no longer matches any issue.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct StaleSuppression {
    #[serde(serialize_with = "serde_path::serialize")]
    pub path: PathBuf,
    pub line: u32,
    pub col: u32,
    pub origin: SuppressionOrigin,
}

impl StaleSuppression {
    /// Produce a human-readable description of this stale suppression.
    #[must_use]
    pub fn description(&self) -> String {
        match &self.origin {
            SuppressionOrigin::Comment {
                issue_kind,
                is_file_level,
                ..
            } => {
                let directive = if *is_file_level {
                    "fallow-ignore-file"
                } else {
                    "fallow-ignore-next-line"
                };
                match issue_kind {
                    Some(kind) => format!("// {directive} {kind}"),
                    None => format!("// {directive}"),
                }
            }
            SuppressionOrigin::JsdocTag { export_name } => {
                format!("@expected-unused on {export_name}")
            }
        }
    }

    /// Produce an explanation of why this suppression is stale.
    ///
    /// For comment suppressions where `kind_known == false`, surfaces the
    /// unknown token plus a Levenshtein "did you mean?" hint when one is
    /// within edit distance 2. Other tokens on the same comment line still
    /// apply normally (see issue #449).
    #[must_use]
    pub fn explanation(&self) -> String {
        match &self.origin {
            SuppressionOrigin::Comment {
                issue_kind,
                is_file_level,
                kind_known,
            } => {
                let scope = if *is_file_level {
                    "in this file"
                } else {
                    "on the next line"
                };
                match issue_kind {
                    Some(kind) if !*kind_known => match closest_known_kind_name(kind) {
                        Some(suggestion) => format!(
                            "'{kind}' is not a recognized fallow issue kind. Did you mean '{suggestion}'? Other tokens on this line still apply."
                        ),
                        None => format!(
                            "'{kind}' is not a recognized fallow issue kind. Other tokens on this line still apply."
                        ),
                    },
                    Some(kind) => format!("no {kind} issue found {scope}"),
                    None => format!("no issues found {scope}"),
                }
            }
            SuppressionOrigin::JsdocTag { export_name } => {
                format!("{export_name} is now used")
            }
        }
    }

    /// The suppressed `IssueKind`, if this was a comment suppression with a specific known kind.
    ///
    /// Returns `None` for unknown-kind comments (`kind_known == false`) and
    /// for JSDoc tags.
    #[must_use]
    pub fn suppressed_kind(&self) -> Option<IssueKind> {
        match &self.origin {
            SuppressionOrigin::Comment {
                issue_kind,
                kind_known: true,
                ..
            } => issue_kind.as_deref().and_then(IssueKind::parse),
            SuppressionOrigin::Comment { .. } | SuppressionOrigin::JsdocTag { .. } => None,
        }
    }

    /// Per-format display message combining `description()` and `explanation()`
    /// for the unknown-kind case so SARIF, CodeClimate, and compact consumers
    /// surface the typo-fix copy and Levenshtein hint without needing to
    /// branch on `origin.kind_known` themselves. Stale-but-known and JSDoc
    /// origins keep the bare `description()` so existing wire bytes stay
    /// unchanged. See issue #449.
    #[must_use]
    pub fn display_message(&self) -> String {
        match &self.origin {
            SuppressionOrigin::Comment {
                kind_known: false, ..
            } => format!("{} ({})", self.description(), self.explanation()),
            SuppressionOrigin::Comment { .. } | SuppressionOrigin::JsdocTag { .. } => {
                self.description()
            }
        }
    }
}

/// A suppression comment present in an analyzed file this run.
///
/// This is the "active-suppression state" the Fallow Impact value report needs
/// to tell a genuinely resolved finding (the code was fixed) from one merely
/// silenced by a newly-added `fallow-ignore`. It captures every PRESENT marker,
/// not only the ones a detector consumed: complexity and code-duplication
/// suppressions are consumed in the CLI layer rather than the core suppression
/// context, so presence is the single uniform signal that covers all impact
/// categories. A present-but-stale marker is harmless because impact keys on a
/// suppression that newly appeared between two recorded runs. It is internal:
/// never serialized into the public JSON output schema (the field on
/// [`AnalysisResults`] is `#[serde(skip)]`), only read in-process by
/// `fallow impact`.
#[derive(Debug, Clone)]
pub struct ActiveSuppression {
    pub path: PathBuf,
    pub kind: Option<String>,
    pub is_file_level: bool,
}

/// The detection method used to identify a feature flag.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum FlagKind {
    EnvironmentVariable,
    SdkCall,
    ConfigObject,
}

/// Detection confidence for a feature flag finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum FlagConfidence {
    Low,
    Medium,
    High,
}

/// A detected feature flag use site.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct FeatureFlag {
    #[serde(serialize_with = "serde_path::serialize")]
    pub path: PathBuf,
    pub flag_name: String,
    pub kind: FlagKind,
    pub confidence: FlagConfidence,
    pub line: u32,
    pub col: u32,
    #[serde(skip)]
    pub guard_span_start: Option<u32>,
    #[serde(skip)]
    pub guard_span_end: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sdk_name: Option<String>,
    #[serde(skip)]
    pub guard_line_start: Option<u32>,
    #[serde(skip)]
    pub guard_line_end: Option<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub guarded_dead_exports: Vec<String>,
}

const _: () = assert!(std::mem::size_of::<FeatureFlag>() <= 160);

/// Usage count for an export symbol. Used by the LSP Code Lens to show
/// reference counts above each export declaration.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ExportUsage {
    #[serde(serialize_with = "serde_path::serialize")]
    pub path: PathBuf,
    pub export_name: String,
    pub line: u32,
    pub col: u32,
    pub reference_count: usize,
    pub reference_locations: Vec<ReferenceLocation>,
}

/// A location where an export is referenced (import site in another file).
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ReferenceLocation {
    #[serde(serialize_with = "serde_path::serialize")]
    pub path: PathBuf,
    pub line: u32,
    pub col: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::output_dead_code::{
        BoundaryViolationFinding, CircularDependencyFinding, UnresolvedImportFinding,
        UnusedClassMemberFinding, UnusedEnumMemberFinding, UnusedExportFinding, UnusedFileFinding,
        UnusedTypeFinding,
    };

    #[test]
    fn empty_results_no_issues() {
        let results = AnalysisResults::default();
        assert_eq!(results.total_issues(), 0);
        assert!(!results.has_issues());
    }

    #[test]
    fn results_with_unused_file() {
        let mut results = AnalysisResults::default();
        results
            .unused_files
            .push(UnusedFileFinding::with_actions(UnusedFile {
                path: PathBuf::from("test.ts"),
            }));
        assert_eq!(results.total_issues(), 1);
        assert!(results.has_issues());
    }

    #[test]
    fn results_with_unused_export() {
        let mut results = AnalysisResults::default();
        results
            .unused_exports
            .push(UnusedExportFinding::with_actions(UnusedExport {
                path: PathBuf::from("test.ts"),
                export_name: "foo".to_string(),
                is_type_only: false,
                line: 1,
                col: 0,
                span_start: 0,
                is_re_export: false,
            }));
        assert_eq!(results.total_issues(), 1);
        assert!(results.has_issues());
    }

    fn test_unused_export(path: &str, export_name: &str, is_type_only: bool) -> UnusedExport {
        UnusedExport {
            path: PathBuf::from(path),
            export_name: export_name.to_string(),
            is_type_only,
            line: 1,
            col: 0,
            span_start: 0,
            is_re_export: false,
        }
    }

    fn test_unused_dependency(
        package_name: &str,
        location: DependencyLocation,
    ) -> UnusedDependency {
        UnusedDependency {
            package_name: package_name.to_string(),
            location,
            path: PathBuf::from("package.json"),
            line: 5,
            used_in_workspaces: Vec::new(),
        }
    }

    fn test_unused_member(member_name: &str, kind: MemberKind) -> UnusedMember {
        UnusedMember {
            path: PathBuf::from("members.ts"),
            parent_name: "Parent".to_string(),
            member_name: member_name.to_string(),
            kind,
            line: 1,
            col: 0,
        }
    }

    #[test]
    fn results_total_counts_all_types() {
        let results = AnalysisResults {
            unused_files: vec![UnusedFileFinding::with_actions(UnusedFile {
                path: PathBuf::from("a.ts"),
            })],
            unused_exports: vec![UnusedExportFinding::with_actions(test_unused_export(
                "b.ts", "x", false,
            ))],
            unused_types: vec![UnusedTypeFinding::with_actions(test_unused_export(
                "c.ts", "T", true,
            ))],
            unused_dependencies: vec![UnusedDependencyFinding::with_actions(
                test_unused_dependency("dep", DependencyLocation::Dependencies),
            )],
            unused_dev_dependencies: vec![UnusedDevDependencyFinding::with_actions(
                test_unused_dependency("dev", DependencyLocation::DevDependencies),
            )],
            unused_enum_members: vec![UnusedEnumMemberFinding::with_actions(test_unused_member(
                "A",
                MemberKind::EnumMember,
            ))],
            unused_class_members: vec![UnusedClassMemberFinding::with_actions(test_unused_member(
                "m",
                MemberKind::ClassMethod,
            ))],
            unresolved_imports: vec![UnresolvedImportFinding::with_actions(UnresolvedImport {
                path: PathBuf::from("f.ts"),
                specifier: "./missing".to_string(),
                line: 1,
                col: 0,
                specifier_col: 0,
            })],
            unlisted_dependencies: vec![UnlistedDependencyFinding::with_actions(
                UnlistedDependency {
                    package_name: "unlisted".to_string(),
                    imported_from: vec![ImportSite {
                        path: PathBuf::from("g.ts"),
                        line: 1,
                        col: 0,
                    }],
                },
            )],
            duplicate_exports: vec![DuplicateExportFinding::with_actions(DuplicateExport {
                export_name: "dup".to_string(),
                locations: vec![
                    DuplicateLocation {
                        path: PathBuf::from("h.ts"),
                        line: 15,
                        col: 0,
                    },
                    DuplicateLocation {
                        path: PathBuf::from("i.ts"),
                        line: 30,
                        col: 0,
                    },
                ],
            })],
            unused_optional_dependencies: vec![UnusedOptionalDependencyFinding::with_actions(
                test_unused_dependency("optional", DependencyLocation::OptionalDependencies),
            )],
            type_only_dependencies: vec![TypeOnlyDependencyFinding::with_actions(
                TypeOnlyDependency {
                    package_name: "type-only".to_string(),
                    path: PathBuf::from("package.json"),
                    line: 8,
                },
            )],
            test_only_dependencies: vec![TestOnlyDependencyFinding::with_actions(
                TestOnlyDependency {
                    package_name: "test-only".to_string(),
                    path: PathBuf::from("package.json"),
                    line: 9,
                },
            )],
            circular_dependencies: vec![CircularDependencyFinding::with_actions(
                CircularDependency {
                    files: vec![PathBuf::from("a.ts"), PathBuf::from("b.ts")],
                    length: 2,
                    line: 3,
                    col: 0,
                    is_cross_package: false,
                },
            )],
            boundary_violations: vec![BoundaryViolationFinding::with_actions(BoundaryViolation {
                from_path: PathBuf::from("src/ui/Button.tsx"),
                to_path: PathBuf::from("src/db/queries.ts"),
                from_zone: "ui".to_string(),
                to_zone: "database".to_string(),
                import_specifier: "../db/queries".to_string(),
                line: 3,
                col: 0,
            })],
            ..Default::default()
        };

        assert_eq!(results.total_issues(), 15);
        assert!(results.has_issues());
    }

    #[test]
    fn total_issues_and_has_issues_are_consistent() {
        let results = AnalysisResults::default();
        assert_eq!(results.total_issues(), 0);
        assert!(!results.has_issues());
        assert_eq!(results.total_issues() > 0, results.has_issues());
    }

    #[test]
    fn total_issues_sums_all_categories_independently() {
        let mut results = AnalysisResults::default();
        results
            .unused_files
            .push(UnusedFileFinding::with_actions(UnusedFile {
                path: PathBuf::from("a.ts"),
            }));
        assert_eq!(results.total_issues(), 1);

        results
            .unused_files
            .push(UnusedFileFinding::with_actions(UnusedFile {
                path: PathBuf::from("b.ts"),
            }));
        assert_eq!(results.total_issues(), 2);

        results
            .unresolved_imports
            .push(UnresolvedImportFinding::with_actions(UnresolvedImport {
                path: PathBuf::from("c.ts"),
                specifier: "./missing".to_string(),
                line: 1,
                col: 0,
                specifier_col: 0,
            }));
        assert_eq!(results.total_issues(), 3);
    }

    #[test]
    fn default_results_all_fields_empty() {
        let r = AnalysisResults::default();
        assert!(r.unused_files.is_empty());
        assert!(r.unused_exports.is_empty());
        assert!(r.unused_types.is_empty());
        assert!(r.unused_dependencies.is_empty());
        assert!(r.unused_dev_dependencies.is_empty());
        assert!(r.unused_optional_dependencies.is_empty());
        assert!(r.unused_enum_members.is_empty());
        assert!(r.unused_class_members.is_empty());
        assert!(r.unresolved_imports.is_empty());
        assert!(r.unlisted_dependencies.is_empty());
        assert!(r.duplicate_exports.is_empty());
        assert!(r.type_only_dependencies.is_empty());
        assert!(r.test_only_dependencies.is_empty());
        assert!(r.circular_dependencies.is_empty());
        assert!(r.boundary_violations.is_empty());
        assert!(r.unused_catalog_entries.is_empty());
        assert!(r.unresolved_catalog_references.is_empty());
        assert!(r.export_usages.is_empty());
    }

    #[test]
    fn entry_point_summary_default() {
        let summary = EntryPointSummary::default();
        assert_eq!(summary.total, 0);
        assert!(summary.by_source.is_empty());
    }

    #[test]
    fn entry_point_summary_not_in_default_results() {
        let r = AnalysisResults::default();
        assert!(r.entry_point_summary.is_none());
    }

    #[test]
    fn entry_point_summary_some_preserves_data() {
        let r = AnalysisResults {
            entry_point_summary: Some(EntryPointSummary {
                total: 5,
                by_source: vec![("package.json".to_string(), 2), ("plugin".to_string(), 3)],
            }),
            ..AnalysisResults::default()
        };
        let summary = r.entry_point_summary.as_ref().unwrap();
        assert_eq!(summary.total, 5);
        assert_eq!(summary.by_source.len(), 2);
        assert_eq!(summary.by_source[0], ("package.json".to_string(), 2));
    }

    #[test]
    fn sort_unused_files_by_path() {
        let mut r = AnalysisResults::default();
        r.unused_files
            .push(UnusedFileFinding::with_actions(UnusedFile {
                path: PathBuf::from("z.ts"),
            }));
        r.unused_files
            .push(UnusedFileFinding::with_actions(UnusedFile {
                path: PathBuf::from("a.ts"),
            }));
        r.unused_files
            .push(UnusedFileFinding::with_actions(UnusedFile {
                path: PathBuf::from("m.ts"),
            }));
        r.sort();
        let paths: Vec<_> = r
            .unused_files
            .iter()
            .map(|f| f.file.path.to_string_lossy().to_string())
            .collect();
        assert_eq!(paths, vec!["a.ts", "m.ts", "z.ts"]);
    }

    #[test]
    fn sort_unused_exports_by_path_line_name() {
        let mut r = AnalysisResults::default();
        let mk = |path: &str, line: u32, name: &str| {
            UnusedExportFinding::with_actions(UnusedExport {
                path: PathBuf::from(path),
                export_name: name.to_string(),
                is_type_only: false,
                line,
                col: 0,
                span_start: 0,
                is_re_export: false,
            })
        };
        r.unused_exports.push(mk("b.ts", 5, "beta"));
        r.unused_exports.push(mk("a.ts", 10, "zeta"));
        r.unused_exports.push(mk("a.ts", 10, "alpha"));
        r.unused_exports.push(mk("a.ts", 1, "gamma"));
        r.sort();
        let keys: Vec<_> = r
            .unused_exports
            .iter()
            .map(|e| {
                format!(
                    "{}:{}:{}",
                    e.export.path.to_string_lossy(),
                    e.export.line,
                    e.export.export_name
                )
            })
            .collect();
        assert_eq!(
            keys,
            vec![
                "a.ts:1:gamma",
                "a.ts:10:alpha",
                "a.ts:10:zeta",
                "b.ts:5:beta"
            ]
        );
    }

    #[test]
    fn sort_unused_types_by_path_line_name() {
        let mut r = AnalysisResults::default();
        let mk = |path: &str, line: u32, name: &str| {
            UnusedTypeFinding::with_actions(UnusedExport {
                path: PathBuf::from(path),
                export_name: name.to_string(),
                is_type_only: true,
                line,
                col: 0,
                span_start: 0,
                is_re_export: false,
            })
        };
        r.unused_types.push(mk("z.ts", 1, "Z"));
        r.unused_types.push(mk("a.ts", 1, "A"));
        r.sort();
        assert_eq!(r.unused_types[0].export.path, PathBuf::from("a.ts"));
        assert_eq!(r.unused_types[1].export.path, PathBuf::from("z.ts"));
    }

    #[test]
    fn sort_unused_dependencies_by_path_line_name() {
        let mut r = AnalysisResults::default();
        let mk = |path: &str, line: u32, name: &str| {
            UnusedDependencyFinding::with_actions(UnusedDependency {
                package_name: name.to_string(),
                location: DependencyLocation::Dependencies,
                path: PathBuf::from(path),
                line,
                used_in_workspaces: Vec::new(),
            })
        };
        r.unused_dependencies.push(mk("b/package.json", 3, "zlib"));
        r.unused_dependencies.push(mk("a/package.json", 5, "react"));
        r.unused_dependencies.push(mk("a/package.json", 5, "axios"));
        r.sort();
        let names: Vec<_> = r
            .unused_dependencies
            .iter()
            .map(|d| d.dep.package_name.as_str())
            .collect();
        assert_eq!(names, vec!["axios", "react", "zlib"]);
    }

    #[test]
    fn sort_unused_dev_dependencies() {
        let mut r = AnalysisResults::default();
        r.unused_dev_dependencies
            .push(UnusedDevDependencyFinding::with_actions(UnusedDependency {
                package_name: "vitest".to_string(),
                location: DependencyLocation::DevDependencies,
                path: PathBuf::from("package.json"),
                line: 10,
                used_in_workspaces: Vec::new(),
            }));
        r.unused_dev_dependencies
            .push(UnusedDevDependencyFinding::with_actions(UnusedDependency {
                package_name: "jest".to_string(),
                location: DependencyLocation::DevDependencies,
                path: PathBuf::from("package.json"),
                line: 5,
                used_in_workspaces: Vec::new(),
            }));
        r.sort();
        assert_eq!(r.unused_dev_dependencies[0].dep.package_name, "jest");
        assert_eq!(r.unused_dev_dependencies[1].dep.package_name, "vitest");
    }

    #[test]
    fn sort_unused_optional_dependencies() {
        let mut r = AnalysisResults::default();
        r.unused_optional_dependencies
            .push(UnusedOptionalDependencyFinding::with_actions(
                UnusedDependency {
                    package_name: "zod".to_string(),
                    location: DependencyLocation::OptionalDependencies,
                    path: PathBuf::from("package.json"),
                    line: 3,
                    used_in_workspaces: Vec::new(),
                },
            ));
        r.unused_optional_dependencies
            .push(UnusedOptionalDependencyFinding::with_actions(
                UnusedDependency {
                    package_name: "ajv".to_string(),
                    location: DependencyLocation::OptionalDependencies,
                    path: PathBuf::from("package.json"),
                    line: 2,
                    used_in_workspaces: Vec::new(),
                },
            ));
        r.sort();
        assert_eq!(r.unused_optional_dependencies[0].dep.package_name, "ajv");
        assert_eq!(r.unused_optional_dependencies[1].dep.package_name, "zod");
    }

    #[test]
    fn sort_unused_enum_members_by_path_line_parent_member() {
        let mut r = AnalysisResults::default();
        let mk = |path: &str, line: u32, parent: &str, member: &str| {
            UnusedEnumMemberFinding::with_actions(UnusedMember {
                path: PathBuf::from(path),
                parent_name: parent.to_string(),
                member_name: member.to_string(),
                kind: MemberKind::EnumMember,
                line,
                col: 0,
            })
        };
        r.unused_enum_members.push(mk("a.ts", 5, "Status", "Z"));
        r.unused_enum_members.push(mk("a.ts", 5, "Status", "A"));
        r.unused_enum_members.push(mk("a.ts", 1, "Direction", "Up"));
        r.sort();
        let keys: Vec<_> = r
            .unused_enum_members
            .iter()
            .map(|m| format!("{}:{}", m.member.parent_name, m.member.member_name))
            .collect();
        assert_eq!(keys, vec!["Direction:Up", "Status:A", "Status:Z"]);
    }

    #[test]
    fn sort_unused_class_members() {
        let mut r = AnalysisResults::default();
        let mk = |path: &str, line: u32, parent: &str, member: &str| {
            UnusedClassMemberFinding::with_actions(UnusedMember {
                path: PathBuf::from(path),
                parent_name: parent.to_string(),
                member_name: member.to_string(),
                kind: MemberKind::ClassMethod,
                line,
                col: 0,
            })
        };
        r.unused_class_members.push(mk("b.ts", 1, "Foo", "z"));
        r.unused_class_members.push(mk("a.ts", 1, "Bar", "a"));
        r.sort();
        assert_eq!(r.unused_class_members[0].member.path, PathBuf::from("a.ts"));
        assert_eq!(r.unused_class_members[1].member.path, PathBuf::from("b.ts"));
    }

    #[test]
    fn sort_unresolved_imports_by_path_line_col_specifier() {
        let mut r = AnalysisResults::default();
        let mk = |path: &str, line: u32, col: u32, spec: &str| {
            UnresolvedImportFinding::with_actions(UnresolvedImport {
                path: PathBuf::from(path),
                specifier: spec.to_string(),
                line,
                col,
                specifier_col: 0,
            })
        };
        r.unresolved_imports.push(mk("a.ts", 5, 0, "./z"));
        r.unresolved_imports.push(mk("a.ts", 5, 0, "./a"));
        r.unresolved_imports.push(mk("a.ts", 1, 0, "./m"));
        r.sort();
        let specs: Vec<_> = r
            .unresolved_imports
            .iter()
            .map(|i| i.import.specifier.as_str())
            .collect();
        assert_eq!(specs, vec!["./m", "./a", "./z"]);
    }

    #[test]
    fn sort_unlisted_dependencies_by_name_and_inner_sites() {
        let mut r = AnalysisResults::default();
        r.unlisted_dependencies
            .push(UnlistedDependencyFinding::with_actions(
                UnlistedDependency {
                    package_name: "zod".to_string(),
                    imported_from: vec![
                        ImportSite {
                            path: PathBuf::from("b.ts"),
                            line: 10,
                            col: 0,
                        },
                        ImportSite {
                            path: PathBuf::from("a.ts"),
                            line: 1,
                            col: 0,
                        },
                    ],
                },
            ));
        r.unlisted_dependencies
            .push(UnlistedDependencyFinding::with_actions(
                UnlistedDependency {
                    package_name: "axios".to_string(),
                    imported_from: vec![ImportSite {
                        path: PathBuf::from("c.ts"),
                        line: 1,
                        col: 0,
                    }],
                },
            ));
        r.sort();

        assert_eq!(r.unlisted_dependencies[0].dep.package_name, "axios");
        assert_eq!(r.unlisted_dependencies[1].dep.package_name, "zod");

        let zod_sites: Vec<_> = r.unlisted_dependencies[1]
            .dep
            .imported_from
            .iter()
            .map(|s| s.path.to_string_lossy().to_string())
            .collect();
        assert_eq!(zod_sites, vec!["a.ts", "b.ts"]);
    }

    #[test]
    fn sort_duplicate_exports_by_name_and_inner_locations() {
        let mut r = AnalysisResults::default();
        r.duplicate_exports
            .push(DuplicateExportFinding::with_actions(DuplicateExport {
                export_name: "z".to_string(),
                locations: vec![
                    DuplicateLocation {
                        path: PathBuf::from("c.ts"),
                        line: 1,
                        col: 0,
                    },
                    DuplicateLocation {
                        path: PathBuf::from("a.ts"),
                        line: 5,
                        col: 0,
                    },
                ],
            }));
        r.duplicate_exports
            .push(DuplicateExportFinding::with_actions(DuplicateExport {
                export_name: "a".to_string(),
                locations: vec![DuplicateLocation {
                    path: PathBuf::from("b.ts"),
                    line: 1,
                    col: 0,
                }],
            }));
        r.sort();

        assert_eq!(r.duplicate_exports[0].export.export_name, "a");
        assert_eq!(r.duplicate_exports[1].export.export_name, "z");

        let z_locs: Vec<_> = r.duplicate_exports[1]
            .export
            .locations
            .iter()
            .map(|l| l.path.to_string_lossy().to_string())
            .collect();
        assert_eq!(z_locs, vec!["a.ts", "c.ts"]);
    }

    #[test]
    fn sort_type_only_dependencies() {
        let mut r = AnalysisResults::default();
        r.type_only_dependencies
            .push(TypeOnlyDependencyFinding::with_actions(
                TypeOnlyDependency {
                    package_name: "zod".to_string(),
                    path: PathBuf::from("package.json"),
                    line: 10,
                },
            ));
        r.type_only_dependencies
            .push(TypeOnlyDependencyFinding::with_actions(
                TypeOnlyDependency {
                    package_name: "ajv".to_string(),
                    path: PathBuf::from("package.json"),
                    line: 5,
                },
            ));
        r.sort();
        assert_eq!(r.type_only_dependencies[0].dep.package_name, "ajv");
        assert_eq!(r.type_only_dependencies[1].dep.package_name, "zod");
    }

    #[test]
    fn sort_test_only_dependencies() {
        let mut r = AnalysisResults::default();
        r.test_only_dependencies
            .push(TestOnlyDependencyFinding::with_actions(
                TestOnlyDependency {
                    package_name: "vitest".to_string(),
                    path: PathBuf::from("package.json"),
                    line: 15,
                },
            ));
        r.test_only_dependencies
            .push(TestOnlyDependencyFinding::with_actions(
                TestOnlyDependency {
                    package_name: "jest".to_string(),
                    path: PathBuf::from("package.json"),
                    line: 10,
                },
            ));
        r.sort();
        assert_eq!(r.test_only_dependencies[0].dep.package_name, "jest");
        assert_eq!(r.test_only_dependencies[1].dep.package_name, "vitest");
    }

    #[test]
    fn sort_circular_dependencies_by_files_then_length() {
        let mut r = AnalysisResults::default();
        r.circular_dependencies
            .push(CircularDependencyFinding::with_actions(
                CircularDependency {
                    files: vec![PathBuf::from("b.ts"), PathBuf::from("c.ts")],
                    length: 2,
                    line: 1,
                    col: 0,
                    is_cross_package: false,
                },
            ));
        r.circular_dependencies
            .push(CircularDependencyFinding::with_actions(
                CircularDependency {
                    files: vec![PathBuf::from("a.ts"), PathBuf::from("b.ts")],
                    length: 2,
                    line: 1,
                    col: 0,
                    is_cross_package: true,
                },
            ));
        r.sort();
        assert_eq!(
            r.circular_dependencies[0].cycle.files[0],
            PathBuf::from("a.ts")
        );
        assert_eq!(
            r.circular_dependencies[1].cycle.files[0],
            PathBuf::from("b.ts")
        );
    }

    #[test]
    fn sort_boundary_violations() {
        let mut r = AnalysisResults::default();
        let mk = |from: &str, line: u32, col: u32, to: &str| {
            BoundaryViolationFinding::with_actions(BoundaryViolation {
                from_path: PathBuf::from(from),
                to_path: PathBuf::from(to),
                from_zone: "a".to_string(),
                to_zone: "b".to_string(),
                import_specifier: to.to_string(),
                line,
                col,
            })
        };
        r.boundary_violations.push(mk("z.ts", 1, 0, "a.ts"));
        r.boundary_violations.push(mk("a.ts", 5, 0, "b.ts"));
        r.boundary_violations.push(mk("a.ts", 1, 0, "c.ts"));
        r.sort();
        let from_paths: Vec<_> = r
            .boundary_violations
            .iter()
            .map(|v| {
                format!(
                    "{}:{}",
                    v.violation.from_path.to_string_lossy(),
                    v.violation.line
                )
            })
            .collect();
        assert_eq!(from_paths, vec!["a.ts:1", "a.ts:5", "z.ts:1"]);
    }

    #[test]
    fn sort_export_usages_and_inner_reference_locations() {
        let mut r = AnalysisResults::default();
        r.export_usages.push(ExportUsage {
            path: PathBuf::from("z.ts"),
            export_name: "foo".to_string(),
            line: 1,
            col: 0,
            reference_count: 2,
            reference_locations: vec![
                ReferenceLocation {
                    path: PathBuf::from("c.ts"),
                    line: 10,
                    col: 0,
                },
                ReferenceLocation {
                    path: PathBuf::from("a.ts"),
                    line: 5,
                    col: 0,
                },
            ],
        });
        r.export_usages.push(ExportUsage {
            path: PathBuf::from("a.ts"),
            export_name: "bar".to_string(),
            line: 1,
            col: 0,
            reference_count: 1,
            reference_locations: vec![ReferenceLocation {
                path: PathBuf::from("b.ts"),
                line: 1,
                col: 0,
            }],
        });
        r.sort();

        assert_eq!(r.export_usages[0].path, PathBuf::from("a.ts"));
        assert_eq!(r.export_usages[1].path, PathBuf::from("z.ts"));

        let refs: Vec<_> = r.export_usages[1]
            .reference_locations
            .iter()
            .map(|l| l.path.to_string_lossy().to_string())
            .collect();
        assert_eq!(refs, vec!["a.ts", "c.ts"]);
    }

    #[test]
    fn sort_empty_results_is_noop() {
        let mut r = AnalysisResults::default();
        r.sort(); // should not panic
        assert_eq!(r.total_issues(), 0);
    }

    #[test]
    fn sort_single_element_lists_stable() {
        let mut r = AnalysisResults::default();
        r.unused_files
            .push(UnusedFileFinding::with_actions(UnusedFile {
                path: PathBuf::from("only.ts"),
            }));
        r.sort();
        assert_eq!(r.unused_files[0].file.path, PathBuf::from("only.ts"));
    }

    #[test]
    fn serialize_empty_results() {
        let r = AnalysisResults::default();
        let json = serde_json::to_value(&r).unwrap();

        assert!(json["unused_files"].as_array().unwrap().is_empty());
        assert!(json["unused_exports"].as_array().unwrap().is_empty());
        assert!(json["circular_dependencies"].as_array().unwrap().is_empty());

        assert!(json.get("export_usages").is_none());
        assert!(json.get("entry_point_summary").is_none());
    }

    #[test]
    fn serialize_unused_file_path() {
        let r = UnusedFile {
            path: PathBuf::from("src/utils/index.ts"),
        };
        let json = serde_json::to_value(&r).unwrap();
        assert_eq!(json["path"], "src/utils/index.ts");
    }

    #[test]
    fn serialize_dependency_location_camel_case() {
        let dep = UnusedDependency {
            package_name: "react".to_string(),
            location: DependencyLocation::DevDependencies,
            path: PathBuf::from("package.json"),
            line: 5,
            used_in_workspaces: Vec::new(),
        };
        let json = serde_json::to_value(&dep).unwrap();
        assert_eq!(json["location"], "devDependencies");

        let dep2 = UnusedDependency {
            package_name: "react".to_string(),
            location: DependencyLocation::Dependencies,
            path: PathBuf::from("package.json"),
            line: 3,
            used_in_workspaces: Vec::new(),
        };
        let json2 = serde_json::to_value(&dep2).unwrap();
        assert_eq!(json2["location"], "dependencies");

        let dep3 = UnusedDependency {
            package_name: "fsevents".to_string(),
            location: DependencyLocation::OptionalDependencies,
            path: PathBuf::from("package.json"),
            line: 7,
            used_in_workspaces: Vec::new(),
        };
        let json3 = serde_json::to_value(&dep3).unwrap();
        assert_eq!(json3["location"], "optionalDependencies");
    }

    #[test]
    fn serialize_circular_dependency_skips_false_cross_package() {
        let cd = CircularDependency {
            files: vec![PathBuf::from("a.ts"), PathBuf::from("b.ts")],
            length: 2,
            line: 1,
            col: 0,
            is_cross_package: false,
        };
        let json = serde_json::to_value(&cd).unwrap();
        assert!(json.get("is_cross_package").is_none());
    }

    #[test]
    fn serialize_circular_dependency_includes_true_cross_package() {
        let cd = CircularDependency {
            files: vec![PathBuf::from("a.ts"), PathBuf::from("b.ts")],
            length: 2,
            line: 1,
            col: 0,
            is_cross_package: true,
        };
        let json = serde_json::to_value(&cd).unwrap();
        assert_eq!(json["is_cross_package"], true);
    }

    #[test]
    fn serialize_unused_export_fields() {
        let e = UnusedExport {
            path: PathBuf::from("src/mod.ts"),
            export_name: "helper".to_string(),
            is_type_only: true,
            line: 42,
            col: 7,
            span_start: 100,
            is_re_export: true,
        };
        let json = serde_json::to_value(&e).unwrap();
        assert_eq!(json["path"], "src/mod.ts");
        assert_eq!(json["export_name"], "helper");
        assert_eq!(json["is_type_only"], true);
        assert_eq!(json["line"], 42);
        assert_eq!(json["col"], 7);
        assert_eq!(json["span_start"], 100);
        assert_eq!(json["is_re_export"], true);
    }

    #[test]
    fn serialize_boundary_violation_fields() {
        let v = BoundaryViolation {
            from_path: PathBuf::from("src/ui/button.tsx"),
            to_path: PathBuf::from("src/db/queries.ts"),
            from_zone: "ui".to_string(),
            to_zone: "db".to_string(),
            import_specifier: "../db/queries".to_string(),
            line: 3,
            col: 0,
        };
        let json = serde_json::to_value(&v).unwrap();
        assert_eq!(json["from_path"], "src/ui/button.tsx");
        assert_eq!(json["to_path"], "src/db/queries.ts");
        assert_eq!(json["from_zone"], "ui");
        assert_eq!(json["to_zone"], "db");
        assert_eq!(json["import_specifier"], "../db/queries");
    }

    #[test]
    fn serialize_unlisted_dependency_with_import_sites() {
        let d = UnlistedDependency {
            package_name: "chalk".to_string(),
            imported_from: vec![
                ImportSite {
                    path: PathBuf::from("a.ts"),
                    line: 1,
                    col: 0,
                },
                ImportSite {
                    path: PathBuf::from("b.ts"),
                    line: 5,
                    col: 3,
                },
            ],
        };
        let json = serde_json::to_value(&d).unwrap();
        assert_eq!(json["package_name"], "chalk");
        let sites = json["imported_from"].as_array().unwrap();
        assert_eq!(sites.len(), 2);
        assert_eq!(sites[0]["path"], "a.ts");
        assert_eq!(sites[1]["line"], 5);
    }

    #[test]
    fn serialize_duplicate_export_with_locations() {
        let d = DuplicateExport {
            export_name: "Button".to_string(),
            locations: vec![
                DuplicateLocation {
                    path: PathBuf::from("src/a.ts"),
                    line: 10,
                    col: 0,
                },
                DuplicateLocation {
                    path: PathBuf::from("src/b.ts"),
                    line: 20,
                    col: 5,
                },
            ],
        };
        let json = serde_json::to_value(&d).unwrap();
        assert_eq!(json["export_name"], "Button");
        let locs = json["locations"].as_array().unwrap();
        assert_eq!(locs.len(), 2);
        assert_eq!(locs[0]["line"], 10);
        assert_eq!(locs[1]["col"], 5);
    }

    #[test]
    fn serialize_type_only_dependency() {
        let d = TypeOnlyDependency {
            package_name: "@types/react".to_string(),
            path: PathBuf::from("package.json"),
            line: 12,
        };
        let json = serde_json::to_value(&d).unwrap();
        assert_eq!(json["package_name"], "@types/react");
        assert_eq!(json["line"], 12);
    }

    #[test]
    fn serialize_test_only_dependency() {
        let d = TestOnlyDependency {
            package_name: "vitest".to_string(),
            path: PathBuf::from("package.json"),
            line: 8,
        };
        let json = serde_json::to_value(&d).unwrap();
        assert_eq!(json["package_name"], "vitest");
        assert_eq!(json["line"], 8);
    }

    #[test]
    fn serialize_unused_member() {
        let m = UnusedMember {
            path: PathBuf::from("enums.ts"),
            parent_name: "Status".to_string(),
            member_name: "Pending".to_string(),
            kind: MemberKind::EnumMember,
            line: 3,
            col: 4,
        };
        let json = serde_json::to_value(&m).unwrap();
        assert_eq!(json["parent_name"], "Status");
        assert_eq!(json["member_name"], "Pending");
        assert_eq!(json["line"], 3);
    }

    #[test]
    fn serialize_unresolved_import() {
        let i = UnresolvedImport {
            path: PathBuf::from("app.ts"),
            specifier: "./missing-module".to_string(),
            line: 7,
            col: 0,
            specifier_col: 21,
        };
        let json = serde_json::to_value(&i).unwrap();
        assert_eq!(json["specifier"], "./missing-module");
        assert_eq!(json["specifier_col"], 21);
    }

    #[test]
    fn deserialize_circular_dependency_with_defaults() {
        let json = r#"{"files":["a.ts","b.ts"],"length":2}"#;
        let cd: CircularDependency = serde_json::from_str(json).unwrap();
        assert_eq!(cd.files.len(), 2);
        assert_eq!(cd.length, 2);
        assert_eq!(cd.line, 0);
        assert_eq!(cd.col, 0);
        assert!(!cd.is_cross_package);
    }

    #[test]
    fn deserialize_circular_dependency_with_all_fields() {
        let json =
            r#"{"files":["a.ts","b.ts"],"length":2,"line":5,"col":10,"is_cross_package":true}"#;
        let cd: CircularDependency = serde_json::from_str(json).unwrap();
        assert_eq!(cd.line, 5);
        assert_eq!(cd.col, 10);
        assert!(cd.is_cross_package);
    }

    #[test]
    fn clone_results_are_independent() {
        let mut r = AnalysisResults::default();
        r.unused_files
            .push(UnusedFileFinding::with_actions(UnusedFile {
                path: PathBuf::from("a.ts"),
            }));
        let mut cloned = r.clone();
        cloned
            .unused_files
            .push(UnusedFileFinding::with_actions(UnusedFile {
                path: PathBuf::from("b.ts"),
            }));
        assert_eq!(r.total_issues(), 1);
        assert_eq!(cloned.total_issues(), 2);
    }

    #[test]
    fn export_usages_not_counted_in_total_issues() {
        let mut r = AnalysisResults::default();
        r.export_usages.push(ExportUsage {
            path: PathBuf::from("mod.ts"),
            export_name: "foo".to_string(),
            line: 1,
            col: 0,
            reference_count: 3,
            reference_locations: vec![],
        });
        assert_eq!(r.total_issues(), 0);
        assert!(!r.has_issues());
    }

    #[test]
    fn entry_point_summary_not_counted_in_total_issues() {
        let r = AnalysisResults {
            entry_point_summary: Some(EntryPointSummary {
                total: 10,
                by_source: vec![("config".to_string(), 10)],
            }),
            ..AnalysisResults::default()
        };
        assert_eq!(r.total_issues(), 0);
        assert!(!r.has_issues());
    }
}
