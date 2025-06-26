use anyhow::{Context, Result};
use chrono::{Local, TimeZone};
use git2::{Repository, StatusOptions, StatusShow};
use std::path::{Path, PathBuf};

use crate::git::status::{CommitInfo, FileStatus, FileStatusType, RemoteStatus};

pub struct GitRepository {
    repo: Repository,
    path: PathBuf,
}

impl GitRepository {
    pub fn open(path: &Path) -> Result<Self> {
        let repo = Repository::open(path)
            .context("Failed to open Git repository. Make sure you're in a Git repository.")?;
        
        Ok(Self {
            path: path.to_path_buf(),
            repo,
        })
    }

    pub fn current_branch(&self) -> Result<String> {
        let head = self.repo.head().context("Failed to get HEAD reference")?;
        
        if let Some(name) = head.shorthand() {
            Ok(name.to_string())
        } else {
            Ok("HEAD".to_string())
        }
    }

    pub fn staged_files(&self) -> Result<Vec<FileStatus>> {
        let mut opts = StatusOptions::new();
        opts.show(StatusShow::Index);
        opts.include_untracked(false);
        
        self.get_status_files(opts, true)
    }

    pub fn unstaged_files(&self) -> Result<Vec<FileStatus>> {
        let mut opts = StatusOptions::new();
        opts.show(StatusShow::Workdir);
        opts.include_untracked(true);
        
        self.get_status_files(opts, false)
    }

    fn get_status_files(&self, mut opts: StatusOptions, staged: bool) -> Result<Vec<FileStatus>> {
        let statuses = self.repo.statuses(Some(&mut opts))?;
        let mut files = Vec::new();

        for entry in statuses.iter() {
            let status = entry.status();
            let path = entry.path().unwrap_or_default();
            
            let file_status = if staged {
                self.get_staged_status(status)
            } else {
                self.get_unstaged_status(status)
            };

            if let Some(status_type) = file_status {
                files.push(FileStatus {
                    path: PathBuf::from(path),
                    status: status_type,
                    staged,
                });
            }
        }

        Ok(files)
    }

    fn get_staged_status(&self, status: git2::Status) -> Option<FileStatusType> {
        if status.contains(git2::Status::INDEX_NEW) {
            Some(FileStatusType::Added)
        } else if status.contains(git2::Status::INDEX_MODIFIED) {
            Some(FileStatusType::Modified)
        } else if status.contains(git2::Status::INDEX_DELETED) {
            Some(FileStatusType::Deleted)
        } else if status.contains(git2::Status::INDEX_RENAMED) {
            Some(FileStatusType::Renamed)
        } else {
            None
        }
    }

    fn get_unstaged_status(&self, status: git2::Status) -> Option<FileStatusType> {
        if status.contains(git2::Status::WT_NEW) {
            Some(FileStatusType::Untracked)
        } else if status.contains(git2::Status::WT_MODIFIED) {
            Some(FileStatusType::Modified)
        } else if status.contains(git2::Status::WT_DELETED) {
            Some(FileStatusType::Deleted)
        } else if status.contains(git2::Status::WT_RENAMED) {
            Some(FileStatusType::Renamed)
        } else if status.contains(git2::Status::CONFLICTED) {
            Some(FileStatusType::Conflicted)
        } else {
            None
        }
    }

    pub fn recent_commits(&self, count: usize) -> Result<Vec<CommitInfo>> {
        let mut revwalk = self.repo.revwalk()?;
        revwalk.push_head()?;
        
        let mut commits = Vec::new();
        
        for (i, oid) in revwalk.enumerate() {
            if i >= count {
                break;
            }
            
            let oid = oid?;
            let commit = self.repo.find_commit(oid)?;
            
            let author = commit.author();
            let timestamp = Local.timestamp_opt(commit.time().seconds(), 0)
                .single()
                .unwrap_or_else(Local::now);
            
            commits.push(CommitInfo {
                hash: oid.to_string(),
                short_hash: format!("{:.8}", oid),
                author: author.name().unwrap_or("Unknown").to_string(),
                message: commit.summary().unwrap_or("").to_string(),
                timestamp,
            });
        }
        
        Ok(commits)
    }

    pub fn remote_status(&self) -> Result<RemoteStatus> {
        let head = self.repo.head()?;
        let local_branch = head.shorthand().unwrap_or("HEAD");
        
        let upstream = match self.repo.branch_upstream_name(head.name().unwrap_or_default()) {
            Ok(name) => name,
            Err(_) => {
                return Ok(RemoteStatus {
                    remote_name: "origin".to_string(),
                    branch_name: local_branch.to_string(),
                    ahead: 0,
                    behind: 0,
                });
            }
        };
        
        let upstream_str = upstream.as_str().unwrap_or_default();
        let parts: Vec<&str> = upstream_str.split('/').collect();
        let remote_name = parts.get(2).unwrap_or(&"origin").to_string();
        
        let local_oid = head.target().context("Failed to get local HEAD")?;
        let upstream_oid = self.repo.refname_to_id(upstream_str)?;
        
        let (ahead, behind) = self.repo.graph_ahead_behind(local_oid, upstream_oid)?;
        
        Ok(RemoteStatus {
            remote_name,
            branch_name: local_branch.to_string(),
            ahead,
            behind,
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}