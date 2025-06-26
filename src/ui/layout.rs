use anyhow::Result;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::git::status::FileStatusType;
use crate::ui::app::{App, TabType};

pub fn draw_ui(f: &mut Frame, app: &App) -> Result<()> {
    let size = f.area();
    
    // Main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(0),     // Content
            Constraint::Length(2),  // Footer
        ])
        .split(size);

    draw_header(f, chunks[0], app)?;
    draw_content(f, chunks[1], app)?;
    draw_footer(f, chunks[2]);

    Ok(())
}

fn draw_header(f: &mut Frame, area: Rect, app: &App) -> Result<()> {
    let branch = app.repo().current_branch().unwrap_or_else(|_| "Unknown".to_string());
    let remote_status = app.repo().remote_status().ok();
    let path = app.repo().path().display().to_string();
    
    let mut header_text = vec![
        Span::raw(" gittop - "),
        Span::styled("Repository: ", Style::default().fg(Color::Gray)),
        Span::styled(&path, Style::default().fg(Color::White)),
        Span::raw(" - "),
        Span::styled("Branch: ", Style::default().fg(Color::Gray)),
        Span::styled(&branch, Style::default().fg(Color::Green)),
    ];

    if let Some(remote) = remote_status {
        if remote.ahead > 0 || remote.behind > 0 {
            header_text.push(Span::raw(" - "));
            header_text.push(Span::styled(
                format!("[↑{} ↓{}]", remote.ahead, remote.behind),
                Style::default().fg(Color::Yellow),
            ));
        }
    }

    let header = Paragraph::new(Line::from(header_text))
        .style(Style::default().bg(Color::DarkGray))
        .block(Block::default().borders(Borders::BOTTOM));

    f.render_widget(header, area);
    Ok(())
}

fn draw_content(f: &mut Frame, area: Rect, app: &App) -> Result<()> {
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    match app.current_tab() {
        TabType::Status => {
            draw_status_view(f, content_chunks[0], app)?;
            draw_recent_commits(f, content_chunks[1], app)?;
        }
        TabType::Commits => {
            draw_recent_commits(f, area, app)?;
        }
    }

    Ok(())
}

fn draw_status_view(f: &mut Frame, area: Rect, app: &App) -> Result<()> {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ])
        .split(area);

    // Staged changes
    let staged_files = app.repo().staged_files().unwrap_or_default();
    let staged_items: Vec<ListItem> = staged_files
        .iter()
        .map(|file| {
            let symbol = get_status_symbol(&file.status);
            let color = get_status_color(&file.status);
            ListItem::new(format!("{} {}", symbol, file.path.display()))
                .style(Style::default().fg(color))
        })
        .collect();

    let staged_list = List::new(staged_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" Staged Changes ({}) ", staged_files.len()))
                .title_style(Style::default().fg(Color::Green)),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_widget(staged_list, chunks[0]);

    // Unstaged changes
    let unstaged_files = app.repo().unstaged_files().unwrap_or_default();
    let unstaged_items: Vec<ListItem> = unstaged_files
        .iter()
        .map(|file| {
            let symbol = get_status_symbol(&file.status);
            let color = get_status_color(&file.status);
            ListItem::new(format!("{} {}", symbol, file.path.display()))
                .style(Style::default().fg(color))
        })
        .collect();

    let unstaged_list = List::new(unstaged_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" Unstaged Changes ({}) ", unstaged_files.len()))
                .title_style(Style::default().fg(Color::Yellow)),
        );

    f.render_widget(unstaged_list, chunks[1]);

    // Status summary
    let branch = app.repo().current_branch().unwrap_or_else(|_| "Unknown".to_string());
    let remote_status = app.repo().remote_status().ok();
    
    let mut status_text = vec![
        Line::from(format!("On branch: {}", branch)),
    ];

    if let Some(remote) = remote_status {
        if remote.is_up_to_date() {
            status_text.push(Line::from("Your branch is up to date"));
        } else {
            if remote.ahead > 0 {
                status_text.push(Line::from(format!(
                    "Your branch is ahead by {} commit(s)",
                    remote.ahead
                )));
            }
            if remote.behind > 0 {
                status_text.push(Line::from(format!(
                    "Your branch is behind by {} commit(s)",
                    remote.behind
                )));
            }
        }
    }

    let status_summary = Paragraph::new(status_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Status ")
                .title_style(Style::default().fg(Color::Cyan)),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(status_summary, chunks[2]);

    Ok(())
}

fn draw_recent_commits(f: &mut Frame, area: Rect, app: &App) -> Result<()> {
    let commits = app.repo().recent_commits(10).unwrap_or_default();
    
    let commit_items: Vec<ListItem> = commits
        .iter()
        .map(|commit| {
            let time_str = commit.timestamp.format("%Y-%m-%d %H:%M").to_string();
            ListItem::new(vec![
                Line::from(vec![
                    Span::styled(&commit.short_hash, Style::default().fg(Color::Yellow)),
                    Span::raw(" "),
                    Span::raw(&commit.message),
                ]),
                Line::from(vec![
                    Span::raw("  "),
                    Span::styled(&commit.author, Style::default().fg(Color::Green)),
                    Span::raw(" - "),
                    Span::styled(time_str, Style::default().fg(Color::Gray)),
                ]),
            ])
        })
        .collect();

    let commits_list = List::new(commit_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Recent Commits ")
                .title_style(Style::default().fg(Color::Magenta)),
        );

    f.render_widget(commits_list, area);
    Ok(())
}

fn draw_footer(f: &mut Frame, area: Rect) {
    let footer_text = Line::from(vec![
        Span::raw("Press "),
        Span::styled("'q'", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw(" to quit, "),
        Span::styled("'r'", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw(" to refresh, "),
        Span::styled("'Tab'", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw(" to switch tabs"),
    ]);

    let footer = Paragraph::new(footer_text)
        .style(Style::default().bg(Color::DarkGray))
        .alignment(Alignment::Center);

    f.render_widget(footer, area);
}

fn get_status_symbol(status: &FileStatusType) -> &'static str {
    match status {
        FileStatusType::Added => "+",
        FileStatusType::Modified => "M",
        FileStatusType::Deleted => "-",
        FileStatusType::Renamed => "R",
        FileStatusType::Untracked => "?",
        FileStatusType::Conflicted => "!",
    }
}

fn get_status_color(status: &FileStatusType) -> Color {
    match status {
        FileStatusType::Added => Color::Green,
        FileStatusType::Modified => Color::Yellow,
        FileStatusType::Deleted => Color::Red,
        FileStatusType::Renamed => Color::Blue,
        FileStatusType::Untracked => Color::Gray,
        FileStatusType::Conflicted => Color::Red,
    }
}