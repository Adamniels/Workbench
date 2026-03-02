use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    widgets::{Block, Borders, List, ListItem, ListState},
};
use std::io::stdout;

mod config;

fn main() -> Result<()> {
    // Load config
    let config = config::load()?;

    // Set up terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // UI state
    let mut selected: usize = 0;
    let mut list_state = ListState::default();

    // Ensure selected is valid for current list
    if config.projects.is_empty() {
        list_state.select(None);
    } else {
        list_state.select(Some(selected));
    }

    // Event loop
    loop {
        // Draw UI
        terminal.draw(|frame| {
            let items: Vec<ListItem> = config
                .projects
                .iter()
                .map(|p| {
                    let tags = p.tags.join(", ");
                    ListItem::new(format!("{} [{}]", p.name, tags))
                })
                .collect();

            let list = List::new(items)
                .block(Block::default().title("Projects").borders(Borders::ALL))
                .highlight_symbol(" > ");

            let area = frame.area();
            frame.render_stateful_widget(list, area, &mut list_state);
        })?;

        // Handle input
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                break;
            }

            if config.projects.is_empty() {
                list_state.select(None);
                continue;
            }

            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('j') {
                let max = config.projects.len();
                if selected < max - 1 {
                    selected += 1;
                    list_state.select(Some(selected));
                }
            } else if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('k') {
                if selected > 0 {
                    selected -= 1;
                    list_state.select(Some(selected));
                }
            }
        }
    }

    // Clean up
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
