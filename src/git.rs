use git2::{BranchType, Delta, ErrorCode, Repository};
use std::env;

/// A git branch.
#[derive(Clone, Debug)]
pub struct GitBranch {
    inner: String,
}

impl GitBranch {
    /// Builds an [`GitBranch`] from the environment.
    pub fn from_env() -> Option<Self> {
        let repo = Repository::discover(env::current_dir().ok()?).ok()?;
        let head_branch_name = match repo.head() {
            Ok(head) => head.shorthand()?.to_owned(),
            Err(e) => {
                if e.code() == ErrorCode::UnbornBranch {
                    "master".to_owned()
                } else {
                    return None;
                }
            }
        };
        Some(Self {
            inner: head_branch_name,
        })
    }

    /// Returns the name of the branch.
    pub fn name(&self) -> &str {
        self.inner.as_ref()
    }
}

/// Status of a git repository.
#[derive(Clone, Debug, Default)]
pub struct GitStatus {
    pub staged: usize,
    pub unstaged: usize,
    pub untracked: usize,
    pub ahead: usize,
    pub behind: usize,
}

impl GitStatus {
    /// Builds a [`GitStatus`] from the environment.
    pub fn from_env() -> Option<Self> {
        let repo = Repository::discover(env::current_dir().ok()?).ok()?;
        let mut repo_status = Self::default();

        if let Ok(statuses) = repo.statuses(None) {
            for status in statuses.iter() {
                if let Some(diff_delta) = status.head_to_index() {
                    match diff_delta.status() {
                        Delta::Added
                        | Delta::Deleted
                        | Delta::Modified
                        | Delta::Renamed
                        | Delta::Typechange => repo_status.staged += 1,
                        _ => (),
                    }
                }
                if let Some(diff_delta) = status.index_to_workdir() {
                    match diff_delta.status() {
                        Delta::Added
                        | Delta::Deleted
                        | Delta::Modified
                        | Delta::Renamed
                        | Delta::Typechange => repo_status.unstaged += 1,
                        Delta::Untracked => repo_status.untracked += 1,
                        _ => (),
                    }
                }
            }
        }

        if let Some((ahead, behind)) = get_ahead_behind(&repo) {
            repo_status.ahead = ahead;
            repo_status.behind = behind;
        }

        Some(repo_status)
    }
}

fn get_ahead_behind(repo: &Repository) -> Option<(usize, usize)> {
    let head = repo.head().ok()?;
    let head_branch = repo
        .find_branch(head.shorthand()?, BranchType::Local)
        .ok()?;
    let upstream = head_branch.upstream().ok()?;
    repo.graph_ahead_behind(head.target()?, upstream.get().target()?)
        .ok()
}
