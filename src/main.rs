use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use std::{error::Error, io};
use unicode_width::UnicodeWidthStr;
use serde::{Deserialize, Serialize};
use std::fs;
enum InputMode {
    Normal,
    Editing,
    TaskEditing,
}

#[derive(Serialize, Deserialize)]
struct Todo {
    title: String,
    completed: bool,
}

struct App {
    todos: Vec<Todo>,
    input: String,
    input_mode: InputMode,
    selected_index: Option<usize>,
    editing_task_index: Option<usize>,
}

impl Default for App {
    fn default() -> App {
        App {
            todos: App::load_todos(),
            input: String::new(),
            input_mode: InputMode::Normal,
            selected_index: None,
            editing_task_index: None,
        }
    }
}

impl App {
    fn save_todos(&self) -> io::Result<()> {
        let json = serde_json::to_string(&self.todos)?;
        fs::write("todos.json", json)?;
        Ok(())
    }

    fn load_todos() -> Vec<Todo> {
        match fs::read_to_string("todos.json") {
            Ok(json) => serde_json::from_str(&json).unwrap_or_default(),
            Err(_) => Vec::new(),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App {
        todos: App::load_todos(),
        input: String::new(),
        input_mode: InputMode::Normal,
        selected_index: None,
        editing_task_index: None,
    };
    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('e') => {
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Enter => {
                        if let Some(index) = app.selected_index {
                            if index < app.todos.len() {
                                app.input = app.todos[index].title.clone();
                                app.editing_task_index = Some(index);
                                app.input_mode = InputMode::TaskEditing;
                            }
                        }
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    KeyCode::Char('j') | KeyCode::Down => {
                        if let Some(index) = app.selected_index {
                            if index < app.todos.len().saturating_sub(1) {
                                app.selected_index = Some(index + 1);
                            }
                        } else {
                            app.selected_index = Some(0);
                        }
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        if let Some(index) = app.selected_index {
                            if index > 0 {
                                app.selected_index = Some(index - 1);
                            }
                        }
                    }
                    KeyCode::Char(' ') => {
                        if let Some(index) = app.selected_index {
                            if index < app.todos.len() {
                                app.todos[index].completed = !app.todos[index].completed;
                                app.save_todos()?;
                            }
                        }
                    }
                    KeyCode::Delete | KeyCode::Backspace => {
                        if let Some(index) = app.selected_index {
                            if index < app.todos.len() {
                                app.todos.remove(index);
                                app.save_todos()?;
                                if app.todos.is_empty() {
                                    app.selected_index = None;
                                } else if index == app.todos.len() {
                                    app.selected_index = Some(index - 1);
                                }
                            }
                        }
                    }
                    _ => {}
                },
                InputMode::TaskEditing => match key.code {
                    KeyCode::Enter => {
                        if let Some(index) = app.editing_task_index {
                            if !app.input.is_empty() {
                                app.todos[index].title = app.input.drain(..).collect();
                                app.save_todos()?;
                            }
                            app.editing_task_index = None;
                            app.input_mode = InputMode::Normal;
                        }
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Esc => {
                        app.input.clear();
                        app.editing_task_index = None;
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        if !app.input.is_empty() {
                            app.todos.push(Todo {
                                title: app.input.drain(..).collect(),
                                completed: false,
                            });
                            app.save_todos()?;
                            if app.selected_index.is_none() {
                                app.selected_index = Some(0);
                            }
                        }
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
            }
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.size());

    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Line::from(vec![
                    Span::raw("Press "),
                    Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to exit, "),
                    Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to start editing, "),
                    Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to edit selected task."),
                ]),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::TaskEditing => (
            vec![
                Line::from(vec![
                    Span::raw("Press "),
                    Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to cancel, "),
                    Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to save changes"),
                ]),
            ],
            Style::default(),
        ),
        InputMode::Editing => (
            vec![
                Line::from(vec![
                    Span::raw("Press "),
                    Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to stop editing, "),
                    Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to record the message"),
                ]),
            ],
            Style::default(),
        ),
    };

    let help_message = Paragraph::new(msg)
        .style(style)
        .block(Block::default().borders(Borders::ALL).title("Help"));
    f.render_widget(help_message, chunks[0]);

    let input = Paragraph::new(vec![Line::from(app.input.as_str())])
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
            InputMode::TaskEditing => Style::default().fg(Color::Green),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, chunks[2]);

    let todos: Vec<ListItem> = app
        .todos
        .iter()
        .enumerate()
        .map(|(i, todo)| {
            let content = vec![Line::from(vec![
                Span::styled(
                    if todo.completed {
                        "▣ "
                    } else {
                        "□ "
                    },
                    Style::default().fg(if todo.completed {
                        Color::Green
                    } else {
                        Color::White
                    }),
                ),
                Span::raw(&todo.title),
            ])];
            ListItem::new(content).style(Style::default().fg(if Some(i) == app.selected_index {
                Color::Yellow
            } else {
                Color::White
            }))
        })
        .collect();

    let todos = List::new(todos)
        .block(Block::default().borders(Borders::ALL).title("To-Do List"))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        );

    f.render_widget(todos, chunks[1]);
    
    match app.input_mode {
        InputMode::Normal => {}
        InputMode::TaskEditing | InputMode::Editing => {
            f.set_cursor(
                chunks[2].x + app.input.width() as u16 + 1,
                chunks[2].y + 1,
            )
        }
    }
}
