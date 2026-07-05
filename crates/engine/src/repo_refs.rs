//! Engine-owned repository reference probes.

use std::path::Path;
use std::process::Command;

use fallow_config::WorkspaceInfo;

/// Resolve a concrete `--changed-workspaces` ref for project-level next steps.
///
/// Returns `None` when the project has no workspaces, is not a git repository,
/// or has no resolvable remote default branch.
#[must_use]
pub fn default_workspace_ref(root: &Path) -> Option<String> {
    let workspaces = crate::discover::discover_workspace_packages(root);
    default_workspace_ref_for_workspaces(root, &workspaces)
}

/// Resolve a concrete `--changed-workspaces` ref using existing workspace data.
#[must_use]
pub fn default_workspace_ref_for_workspaces(
    root: &Path,
    workspaces: &[WorkspaceInfo],
) -> Option<String> {
    if workspaces.is_empty() || !crate::churn::is_git_repo(root) {
        return None;
    }
    if let Some(reference) = run_git(
        root,
        &[
            "symbolic-ref",
            "--quiet",
            "--short",
            "refs/remotes/origin/HEAD",
        ],
    ) {
        let reference = reference.trim();
        if !reference.is_empty() {
            return Some(reference.to_owned());
        }
    }
    ["origin/main", "origin/master"]
        .into_iter()
        .find(|candidate| git_ref_exists(root, candidate))
        .map(str::to_owned)
}

/// Git identities for the current user in forms useful for self-routing.
///
/// Includes `user.email`, its local-part handle, a GitHub no-reply unwrapped
/// handle when applicable, and `user.name`. Missing config values are ignored.
#[must_use]
pub fn current_user_identities(root: &Path) -> Vec<String> {
    let mut ids = Vec::new();
    if let Some(email) = read_git_config(root, "user.email") {
        if let Some((local, _)) = email.split_once('@') {
            ids.push(local.rsplit('+').next().unwrap_or(local).to_owned());
        }
        ids.push(email);
    }
    if let Some(name) = read_git_config(root, "user.name") {
        ids.push(name);
    }
    ids
}

fn read_git_config(root: &Path, key: &str) -> Option<String> {
    let value = run_git(root, &["config", "--get", key])?;
    let trimmed = value.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_owned())
}

fn git_ref_exists(root: &Path, reference: &str) -> bool {
    run_git(root, &["rev-parse", "--verify", "--quiet", reference]).is_some()
}

#[expect(
    clippy::disallowed_methods,
    reason = "canonical engine-owned git spawn wrapper for default remote refs"
)]
fn run_git(root: &Path, args: &[&str]) -> Option<String> {
    let mut command = Command::new("git");
    crate::changed_files::clear_ambient_git_env(&mut command);
    let output = command.arg("-C").arg(root).args(args).output().ok()?;
    if !output.status.success() {
        return None;
    }
    String::from_utf8(output.stdout).ok()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn default_workspace_ref_skips_projects_without_workspaces() {
        assert!(default_workspace_ref_for_workspaces(Path::new("/repo"), &[]).is_none());
    }

    #[test]
    fn default_workspace_ref_skips_non_git_workspace_projects() {
        let workspace = WorkspaceInfo {
            root: PathBuf::from("/repo/packages/app"),
            name: "app".to_owned(),
            is_internal_dependency: false,
        };

        assert!(default_workspace_ref_for_workspaces(Path::new("/repo"), &[workspace]).is_none());
    }

    #[test]
    fn current_user_identities_empty_when_git_config_is_unavailable() {
        assert!(current_user_identities(Path::new("/repo")).is_empty());
    }
}
