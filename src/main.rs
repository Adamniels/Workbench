use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};
use std::io::stdout;

mod config;

fn main() -> Result<()> {
    // Ladda config
    let config = config::load()?;

    // Sätt upp terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Event loop
    loop {
        // Rita UI
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
                .block(Block::default().title("Projects").borders(Borders::ALL));

            frame.render_widget(list, frame.area());
        })?;

        // Vänta på tangent
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    // Städa upp
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}