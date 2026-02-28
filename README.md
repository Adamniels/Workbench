# 🧠 WorkBench

> A fast, keyboard-driven Project Launcher & Workspace Manager built in Rust.

WorkBench is a Terminal UI (TUI) application designed to streamline your development workflow.  
It allows you to manage projects, launch editors, start services, and orchestrate workspaces — all from a single keyboard-driven interface.

Built in Rust to explore:

- State management
- Event loops
- Terminal rendering
- Process management
- Clean architecture in CLI applications

---

# 🚀 Why WorkBench?

Modern developers juggle:

- Multiple repositories
- Different tech stacks
- Separate backend/frontend services
- Documentation links
- Dev servers
- Database sessions

Instead of manually:

- `cd` into directories
- Opening editors
- Starting servers
- Switching between tools

WorkBench provides a **single entry point** to manage everything.

---

# ✨ Core Features

## 📁 Project Management

- Centralized project list
- Configurable paths
- Tag-based categorization
- Default actions per project

## 🔎 Fuzzy Search

- Live filtering
- Search by name
- Search by tags
- Command-style filters (e.g., `tag:dotnet`)

## ⚡ Action System

Each project can define custom actions:

- Open in editor
- Start backend
- Start frontend
- Open documentation
- Launch tmux sessions
- Run tests
- Anything shell-based

## 🖥️ TUI Interface

- Multi-panel layout
- Vim-style navigation
- Status bar with key hints
- Live output panel for command logs

## 🧩 Extensible

- Fully configurable via TOML
- Supports templated commands
- Modular Rust architecture

---

# 🏗 Architecture Overview

```
src/
├── main.rs
├── app/
│   ├── state.rs
│   ├── mode.rs
│   └── commands.rs
├── config/
│   ├── loader.rs
│   └── models.rs
├── ui/
│   ├── layout.rs
│   ├── projects_panel.rs
│   ├── actions_panel.rs
│   └── output_panel.rs
├── input/
│   └── keymap.rs
├── runner/
│   └── process.rs
└── search/
    └── fuzzy.rs
```

---

# 🧠 System Design Principles

## 1. Explicit State

All UI state lives in a single `AppState` struct.

```rust
pub struct AppState {
    pub projects: Vec<Project>,
    pub filtered_indices: Vec<usize>,
    pub selected_index: usize,
    pub mode: Mode,
    pub search_query: String,
    pub output_lines: Vec<String>,
    pub last_status: Option<i32>,
}
```

## 2. Event Loop Driven

```
while running:
    read keyboard input
    update state
    render UI
```

## 3. Mode-Based Input Handling

```rust
enum Mode {
    Normal,
    Search,
    ActionPicker,
}
```

---

# ⚙ Configuration

WorkBench reads from:

```
~/.config/workbench/config.toml
```

Example:

```toml
[settings]
editor = "nvim"
terminal = "kitty"
shell = "zsh"

[[projects]]
name = "Outreach"
path = "/Users/adam/dev/outreach"
tags = ["dotnet", "vue", "postgres"]
default_action = "open"

actions = [
  { key = "o", name = "Open", cmd = "cd {{path}} && nvim ." },
  { key = "b", name = "Backend", cmd = "cd {{path}}/backend && dotnet watch" },
  { key = "f", name = "Frontend", cmd = "cd {{path}}/frontend && pnpm dev" },
  { key = "d", name = "Docs", cmd = "open https://notion.so/..." }
]
```

---

# 🔤 Template System

WorkBench replaces placeholders before executing commands:

| Placeholder        | Description          |
| ------------------ | -------------------- |
| `{{path}}`         | Project path         |
| `{{project.name}}` | Project name         |
| `{{tags}}`         | Comma-separated tags |

Example:

```
cd {{path}} && {{settings.editor}} .
```

---

# 🎹 Keybindings

## Normal Mode

| Key     | Action             |
| ------- | ------------------ |
| `j / k` | Move selection     |
| `/`     | Enter search mode  |
| `Enter` | Run default action |
| `a`     | Open action picker |
| `e`     | Open config file   |
| `r`     | Refresh git status |
| `q`     | Quit               |

## Search Mode

| Key     | Action             |
| ------- | ------------------ |
| `Esc`   | Exit search        |
| `Enter` | Select first match |

---

# 🧪 MVP Roadmap

## Milestone 1 — Core

- Load config
- Render project list
- Basic navigation
- Execute default action

## Milestone 2 — Search

- Fuzzy filtering
- Tag search support

## Milestone 3 — Action Picker

- Display project actions
- Run arbitrary commands
- Template substitution

## Milestone 4 — Process Streaming

- Spawn background processes
- Capture stdout/stderr
- Display live logs

## Milestone 5 — Tmux Integration

- Auto-create workspace layouts
- Named sessions per project

---

# 🧰 Crates Used

- `ratatui` — Terminal UI rendering
- `crossterm` — Keyboard input & terminal control
- `serde` — Serialization
- `toml` — Config parsing
- `anyhow` — Error handling
- `regex` — Filtering
- `tokio` (optional) — Async process handling

---

# 🔒 Design Constraints

- No hidden global state
- No blocking UI thread
- Clear separation between:
  - UI
  - Business logic
  - Process execution
- Config-driven behavior

---

# 📦 Installation (Future)

```
cargo install workbench
```

Or locally:

```
cargo build --release
```

---

# 🧠 Future Ideas

- Git status integration
- Recent projects ranking
- Persistent session restore
- Multi-select mode
- Background daemon mode
- Plugin system
- Remote SSH support
- Project health metrics
- Built-in task runner

---

# 🎯 Learning Objectives

This project is designed to teach:

- Rust ownership in real-world apps
- State-driven UI architecture
- Process management
- Structured config parsing
- Clean module boundaries
- Building real tools instead of tutorials

---

# 📜 License

MIT

---

# 👨‍💻 Author

Built as a learning project in Rust to explore:

- Systems programming
- Terminal UI design
- Workflow automation

---

> WorkBench is not just a tool.  
> It is a programmable entry point to your entire development universe.
