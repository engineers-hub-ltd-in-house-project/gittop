pub mod git;
pub mod ui;
pub mod watcher;

pub use git::{GitRepository, FileStatus, CommitInfo, RemoteStatus};
pub use ui::App;
pub use watcher::FileSystemWatcher;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");