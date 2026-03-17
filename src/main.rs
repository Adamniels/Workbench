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

enum Focus {
    ProjectList,
    // SearchInput,
}

enum Action {
    Quit,
    MoveDown,
    MoveUp,
    Enter,
    FocusProjects,
    // FocusSearch,
    TypeChar(char),
    Backspace,
}

struct App {
    focus: Focus,
    project_list: ProjectListState,
    running: bool,
}

impl App {
    fn new(config: config::Config) -> Self {
        Self {
            focus: Focus::ProjectList,
            project_list: ProjectListState::new(config.projects),
            running: true,
        }
    }
    fn handle_action(&mut self, action: Action) {
        match action {
            Action::Quit => {
                self.running = false;
            }

            // Not sure how to navigate between projects and search input yet, so just switch focus for now
            Action::FocusProjects => {
                self.focus = Focus::ProjectList;
            }

            // Action::FocusSearch => {
            //     self.focus = Focus::SearchInput;
            // }
            _ => match self.focus {
                Focus::ProjectList => self.project_list.handle_action(action),
            },
        }
    }
}

struct ProjectListState {
    projects: Vec<config::Project>,
    state: ListState,
}
impl ProjectListState {
    fn new(projects: Vec<config::Project>) -> Self {
        let mut state = ListState::default();
        if projects.is_empty() {
            state.select(None);
        } else {
            state.select(Some(0));
        }
        Self { projects, state }
    }

    fn next(&mut self) {
        if self.projects.is_empty() {
            self.state.select(None);
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.projects.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        if self.projects.is_empty() {
            self.state.select(None);
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.projects.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    fn handle_action(&mut self, action: Action) {
        match action {
            Action::MoveDown => self.next(),
            Action::MoveUp => self.previous(),
            _ => {}
        }
    }
}

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
    let mut project_list_state = ProjectListState::new(config.projects.clone());

    // Event loop
    loop {
        // Draw UI
        terminal.draw(|frame| {
            let items: Vec<ListItem> = project_list_state
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
            frame.render_stateful_widget(list, area, &mut project_list_state.state);
        })?;

        // Handle input
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                break;
            }
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('j') {
                project_list_state.next();
            } else if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('k') {
                project_list_state.previous();
            }
        }
    }

    // Clean up
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
