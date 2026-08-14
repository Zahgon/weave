
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, thiserror::Error)]
pub enum GitError {
    
    #[error("could not run `git {argv}`: {source}")]
    NotRunnable {
        argv: String,
        #[source]
        source: std::io::Error,
    },

    #[error("not inside a git repository: {dir}")]
    NotARepository { dir: String },

    #[error("no merge base between '{head}' and '{branch}'")]
    NoMergeBase { head: String, branch: String },

    #[error("`git {argv}` exited with {status}: {stderr}")]
    Refused {
        argv: String,
        status: i32,
        stderr: String,
    },
}

fn run(dir: &Path, args: &[&str]) -> Result<std::process::Output, GitError> { panic!("STUB: not implemented") }

fn refusal(args: &[&str], output: &std::process::Output) -> GitError { panic!("STUB: not implemented") }

pub fn find_repo_root(dir: &Path) -> Result<PathBuf, GitError> { panic!("STUB: not implemented") }

pub fn find_repo_root_from_path(path: &Path) -> Result<PathBuf, GitError> { panic!("STUB: not implemented") }

pub fn find_merge_base(repo_root: &Path, head: &str, branch: &str) -> Result<String, GitError> { panic!("STUB: not implemented") }

pub fn git_show(repo_root: &Path, rev: &str, file: &str) -> Result<String, GitError> { panic!("STUB: not implemented") }

pub fn git_show_optional(
    repo_root: &Path,
    rev: &str,
    file: &str,
) -> Result<Option<String>, GitError> { panic!("STUB: not implemented") }

pub fn get_changed_files(
    repo_root: &Path,
    merge_base: &str,
    head: &str,
    branch: &str,
) -> Result<Vec<String>, GitError> { panic!("STUB: not implemented") }

pub fn diff_files(
    repo_root: &Path,
    base_ref: &str,
    target_ref: &str,
) -> Result<Vec<String>, GitError> { panic!("STUB: not implemented") }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_missing_repository_is_its_own_variant_not_a_message() {
        let outside = std::env::temp_dir();
        match find_repo_root_from_path(&outside) {
            Err(GitError::NotARepository { dir }) => {
                assert!(dir.contains(&outside.display().to_string()));
            }
            
            Ok(_) => {}
            Err(other) => panic!("expected NotARepository, got {other:?}"),
        }
    }

    #[test]
    fn a_refusal_carries_gits_own_status_and_argv() {
        match git_show(Path::new("."), "weave-no-such-rev", "no/such/file") {
            Err(GitError::Refused {
                argv,
                status,
                stderr,
            }) => {
                assert!(argv.starts_with("show weave-no-such-rev:"));
                assert_ne!(status, 0);
                assert!(!stderr.is_empty());
            }
            
            Err(GitError::NotRunnable { .. }) => {}
            other => panic!("expected Refused, got {other:?}"),
        }
    }

    #[test]
    fn two_unrelated_refs_report_both_operands() {
        match find_merge_base(Path::new("."), "weave-no-such-ref-a", "weave-no-such-ref-b") {
            Err(GitError::NoMergeBase { head, branch }) => {
                assert_eq!(head, "weave-no-such-ref-a");
                assert_eq!(branch, "weave-no-such-ref-b");
            }
            Err(GitError::NotRunnable { .. }) => {}
            other => panic!("expected NoMergeBase, got {other:?}"),
        }
    }
}
