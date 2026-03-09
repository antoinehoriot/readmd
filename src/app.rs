use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::prelude::*;
use ratatui::widgets::Paragraph;
use ratatui::Terminal;

use crate::state::{AppState, Focus};
use crate::style;
use crate::widgets::{content, file_browser, toc};

pub fn run(state: &mut AppState) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = event_loop(&mut terminal, state);

    // Always tear down, even on error.
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn event_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    state: &mut AppState,
) -> io::Result<()> {
    loop {
        let content_height = terminal.size()?.height.saturating_sub(3); // borders (2) + status bar (1)

        terminal.draw(|f| draw(f, state))?;

        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                handle_key(key, state, content_height);
            }
        }

        if state.should_quit {
            return Ok(());
        }
    }
}

fn draw(f: &mut Frame, state: &mut AppState) {
    let size = f.area();

    // Vertical split: main area + status bar.
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(size);

    let main_area = vertical[0];
    let status_area = vertical[1];

    // Horizontal split: sidebar + TOC + content.
    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(state.sidebar_width),
            Constraint::Length(25),
            Constraint::Min(0),
        ])
        .split(main_area);

    let sidebar_area = horizontal[0];
    let toc_area = horizontal[1];
    let content_area = horizontal[2];

    file_browser::render_file_browser(
        f,
        sidebar_area,
        &state.tree,
        state.tree_selected,
        &mut state.tree_scroll,
        state.focus == Focus::FileTree,
    );

    toc::render_toc(
        f,
        toc_area,
        &state.toc_headings,
        state.toc_selected,
        &mut state.toc_scroll,
        state.focus == Focus::Toc,
    );

    content::render_content(
        f,
        content_area,
        &state.content_lines,
        state.content_scroll,
        state.current_file.as_deref(),
        state.focus == Focus::Content,
    );

    // Status bar.
    let file_path = state
        .current_file
        .as_ref()
        .map(|p| p.display().to_string())
        .unwrap_or_default();

    let hints = "Tab:switch  q:quit  j/k:scroll  Enter:open";
    let gap = status_area
        .width
        .saturating_sub(file_path.len() as u16 + hints.len() as u16);
    let status_text = format!("{}{:>width$}", file_path, hints, width = (gap as usize + hints.len()));

    let status_bar = Paragraph::new(status_text).style(style::status_bar());
    f.render_widget(status_bar, status_area);
}

fn handle_key(key: KeyEvent, state: &mut AppState, content_height: u16) {
    // Global keys.
    match key.code {
        KeyCode::Char('q') => {
            state.quit();
            return;
        }
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            state.quit();
            return;
        }
        KeyCode::Tab => {
            state.toggle_focus();
            return;
        }
        _ => {}
    }

    // Focus-specific keys.
    match state.focus {
        Focus::FileTree => match key.code {
            KeyCode::Up | KeyCode::Char('k') => state.tree_up(),
            KeyCode::Down | KeyCode::Char('j') => state.tree_down(),
            KeyCode::Enter | KeyCode::Right => state.tree_enter(),
            KeyCode::Left => state.tree_collapse(),
            _ => {}
        },
        Focus::Toc => match key.code {
            KeyCode::Up | KeyCode::Char('k') => state.toc_up(),
            KeyCode::Down | KeyCode::Char('j') => state.toc_down(),
            KeyCode::Enter => state.toc_enter(),
            _ => {}
        },
        Focus::Content => match key.code {
            KeyCode::Up | KeyCode::Char('k') => state.content_up(),
            KeyCode::Down | KeyCode::Char('j') => state.content_down(),
            KeyCode::PageUp => state.content_page_up(content_height as usize),
            KeyCode::PageDown => state.content_page_down(content_height as usize),
            KeyCode::Home | KeyCode::Char('g') => state.content_home(),
            KeyCode::End | KeyCode::Char('G') => state.content_end(),
            _ => {}
        },
    }
}
