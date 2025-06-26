use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::path::PathBuf;
use std::time::Duration;

use crate::git::GitRepository;
use crate::ui::events::{should_quit, should_refresh, AppEvent, EventHandler};
use crate::ui::layout::draw_ui;

pub enum TabType {
    Status,
    Commits,
}

pub struct App {
    repo: GitRepository,
    should_quit: bool,
    current_tab: TabType,
    last_error: Option<String>,
}

impl App {
    pub fn new(repo_path: PathBuf) -> Result<Self> {
        let repo = GitRepository::open(&repo_path)?;
        
        Ok(Self {
            repo,
            should_quit: false,
            current_tab: TabType::Status,
            last_error: None,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let event_handler = EventHandler::new(Duration::from_millis(100));
        let res = self.run_loop(&mut terminal, event_handler);

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;

        res
    }

    fn run_loop<B: ratatui::backend::Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
        event_handler: EventHandler,
    ) -> Result<()> {
        while !self.should_quit {
            self.draw(terminal)?;
            
            match event_handler.next()? {
                AppEvent::KeyPress(key) => {
                    if should_quit(&key) {
                        self.should_quit = true;
                    } else if should_refresh(&key) {
                        self.update()?;
                    } else {
                        self.handle_key_event(key)?;
                    }
                }
                AppEvent::Tick => {
                    // Future: Auto-refresh logic
                }
                _ => {}
            }
        }
        
        Ok(())
    }

    fn draw<B: ratatui::backend::Backend>(&self, terminal: &mut Terminal<B>) -> Result<()> {
        terminal.draw(|f| {
            if let Err(e) = draw_ui(f, self) {
                eprintln!("Failed to draw UI: {}", e);
            }
        })?;
        Ok(())
    }

    pub fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) -> Result<()> {
        use crossterm::event::KeyCode;
        
        match key.code {
            KeyCode::Tab => {
                self.current_tab = match self.current_tab {
                    TabType::Status => TabType::Commits,
                    TabType::Commits => TabType::Status,
                };
            }
            _ => {}
        }
        
        Ok(())
    }

    pub fn update(&mut self) -> Result<()> {
        self.last_error = None;
        Ok(())
    }

    pub fn repo(&self) -> &GitRepository {
        &self.repo
    }

    pub fn current_tab(&self) -> &TabType {
        &self.current_tab
    }

    pub fn last_error(&self) -> Option<&str> {
        self.last_error.as_deref()
    }

    pub fn set_error(&mut self, error: String) {
        self.last_error = Some(error);
    }
}