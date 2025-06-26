use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use anyhow::Result;
use std::time::Duration;

pub enum AppEvent {
    KeyPress(KeyEvent),
    FileSystemChange,
    Tick,
    Quit,
}

pub struct EventHandler {
    tick_rate: Duration,
}

impl EventHandler {
    pub fn new(tick_rate: Duration) -> Self {
        Self { tick_rate }
    }

    pub fn next(&self) -> Result<AppEvent> {
        if crossterm::event::poll(self.tick_rate)? {
            match crossterm::event::read()? {
                Event::Key(key) => Ok(AppEvent::KeyPress(key)),
                _ => Ok(AppEvent::Tick),
            }
        } else {
            Ok(AppEvent::Tick)
        }
    }
}

pub fn should_quit(key: &KeyEvent) -> bool {
    matches!(
        key,
        KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::NONE,
            ..
        } | KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }
    )
}

pub fn should_refresh(key: &KeyEvent) -> bool {
    matches!(
        key,
        KeyEvent {
            code: KeyCode::Char('r'),
            modifiers: KeyModifiers::NONE,
            ..
        }
    )
}