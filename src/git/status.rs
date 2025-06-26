use std::path::PathBuf;
use chrono::{DateTime, Local};

#[derive(Debug, Clone, PartialEq)]
pub enum FileStatusType {
    Added,
    Modified,
    Deleted,
    Renamed,
    Untracked,
    Conflicted,
}

#[derive(Debug, Clone)]
pub struct FileStatus {
    pub path: PathBuf,
    pub status: FileStatusType,
    pub staged: bool,
}

#[derive(Debug, Clone)]
pub struct CommitInfo {
    pub hash: String,
    pub short_hash: String,
    pub author: String,
    pub message: String,
    pub timestamp: DateTime<Local>,
}

#[derive(Debug, Clone)]
pub struct RemoteStatus {
    pub remote_name: String,
    pub branch_name: String,
    pub ahead: usize,
    pub behind: usize,
}

impl RemoteStatus {
    pub fn is_up_to_date(&self) -> bool {
        self.ahead == 0 && self.behind == 0
    }
}