use anyhow::Result;
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

pub struct FileSystemWatcher {
    watcher: RecommendedWatcher,
    rx: Receiver<notify::Result<Event>>,
}

impl FileSystemWatcher {
    pub fn new() -> Result<Self> {
        let (tx, rx) = channel();
        
        let watcher = RecommendedWatcher::new(
            move |res| {
                let _ = tx.send(res);
            },
            Config::default().with_poll_interval(Duration::from_secs(1)),
        )?;

        Ok(Self { watcher, rx })
    }

    pub fn watch(&mut self, path: &Path) -> Result<()> {
        self.watcher.watch(path, RecursiveMode::Recursive)?;
        Ok(())
    }

    pub fn unwatch(&mut self, path: &Path) -> Result<()> {
        self.watcher.unwatch(path)?;
        Ok(())
    }

    pub fn try_recv_event(&self) -> Option<Event> {
        match self.rx.try_recv() {
            Ok(Ok(event)) => Some(event),
            _ => None,
        }
    }

    pub fn has_git_changes(&self) -> bool {
        while let Some(event) = self.try_recv_event() {
            if Self::is_git_related(&event) {
                return true;
            }
        }
        false
    }

    fn is_git_related(event: &Event) -> bool {
        match &event.kind {
            EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) => {
                event.paths.iter().any(|path| {
                    path.components().any(|c| c.as_os_str() == ".git") ||
                    path.extension().map_or(false, |ext| {
                        ext == "rs" || ext == "toml" || ext == "md"
                    })
                })
            }
            _ => false,
        }
    }
}