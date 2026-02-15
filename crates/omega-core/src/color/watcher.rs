//! Theme file watcher for hot-reloading support.
//!
//! Monitors the user theme directory for changes and notifies the application
//! when themes are created, modified, or deleted.

use crate::color::loader::ThemeLoader;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, channel};
use std::time::Duration;

/// Notifies the application of theme file system events.
pub enum ThemeEvent {
    /// A theme file was modified or created.
    Updated(PathBuf),
    /// A theme file was deleted.
    Deleted(PathBuf),
}

/// Watches the user theme directory for changes.
pub struct ThemeWatcher {
    watcher: RecommendedWatcher,
    receiver: Receiver<notify::Result<Event>>,
}

impl ThemeWatcher {
    /// Creates a new watcher for the user theme directory.
    pub fn new() -> notify::Result<Self> {
        let (tx, rx) = channel();

        let watcher = RecommendedWatcher::new(
            tx,
            Config::default().with_poll_interval(Duration::from_millis(500)),
        )?;

        Ok(Self { watcher, receiver: rx })
    }

    /// Starts watching the user theme directory.
    pub fn watch_user_dir(&mut self) -> notify::Result<()> {
        if let Some(dir) = ThemeLoader::user_theme_dir() {
            // Ensure directory exists first
            let _ = ThemeLoader::ensure_user_dir();
            self.watcher.watch(&dir, RecursiveMode::NonRecursive)?;
        }
        Ok(())
    }

    /// Non-blocking check for new theme events.
    pub fn poll_event(&self) -> Option<ThemeEvent> {
        if let Ok(Ok(event)) = self.receiver.try_recv() {
            match event.kind {
                notify::EventKind::Modify(_) | notify::EventKind::Create(_) => {
                    if let Some(path) = event.paths.first()
                        && path.extension().is_some_and(|ext| ext == "toml")
                    {
                        return Some(ThemeEvent::Updated(path.clone()));
                    }
                }
                notify::EventKind::Remove(_) => {
                    if let Some(path) = event.paths.first()
                        && path.extension().is_some_and(|ext| ext == "toml")
                    {
                        return Some(ThemeEvent::Deleted(path.clone()));
                    }
                }
                _ => {}
            }
        }
        None
    }
}
