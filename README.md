# clitdl
Terminal Todo Application (built in Rust)

# Terminal Todo Application Documentation

## User Guide

### Installation

1. Ensure you have Rust installed on your system

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clone the repository or create a new directory

3. Build the application:

   ```bash
   cargo build --release
   ```

4. Run the application:

   ```bash
   cargo run --release
   ```

### Features

- Full-screen terminal user interface
- Keyboard-driven interaction
- Todo list with completion status
- Visual feedback for selected items
- Input mode for adding new todos

### Controls

| Key         | Action                                    |
| ----------- | ----------------------------------------- |
| `e`         | Enter editing mode to add new todos       |
| `Esc`       | Exit editing mode                         |
| `Enter`     | Save new todo (in editing mode)           |
| `j` or `↓`  | Move selection down                       |
| `k` or `↑`  | Move selection up                         |
| `Space`     | Toggle completion status of selected todo |
| `Backspace` | Delete selected todo                      |
| `q`         | Quit application                          |

### User Interface

The application interface is divided into three main sections:

1. Help Bar (top) - Shows available commands
2. Todo List (middle) - Displays all todos with completion status
3. Input Box (bottom) - For entering new todos

## Developer Documentation

### Project Structure

```
td/
├── Cargo.toml
└── src/
    └── main.rs
```

### Dependencies

- `crossterm` (v0.27) - Terminal manipulation and input handling
- `ratatui` (v0.24) - Terminal user interface framework
- `unicode-width` (v0.1.11) - Unicode string width calculations

### Key Components

#### Data Structures

1. `InputMode` Enum

```rust
enum InputMode {
    Normal,    // For navigation and todo manipulation
    Editing,   // For adding new todos
}
```

2. `Todo` Struct

```rust
struct Todo {
    title: String,      // The todo text
    completed: bool,    // Completion status
}
```

3. `App` Struct

```rust
struct App {
    todos: Vec<Todo>,           // List of todos
    input: String,              // Current input buffer
    input_mode: InputMode,      // Current input mode
    selected_index: Option<usize>, // Currently selected todo
}
```

### Core Functions

#### `main()`

- Initializes the terminal
- Sets up raw mode and alternate screen
- Creates the application instance
- Handles cleanup on exit

#### `run_app()`

- Main application loop
- Handles event processing
- Updates application state
- Manages terminal drawing

#### `ui()`

- Renders the user interface
- Manages layout
- Handles widget styling and placement

### UI Layout

The interface uses a vertical layout with three sections:

```
┌─────────────────────┐
│      Help Bar       │ <- 3 lines
├─────────────────────┤
│                     │
│     Todo List       │ <- Flexible height
│                     │
├─────────────────────┤
│     Input Box       │ <- 3 lines
└─────────────────────┘
```

### Events and State Management

- Event handling is done through crossterm's event system
- State updates are managed through the App struct
- UI updates are triggered after each state change

### Adding New Features

#### Adding a New Command

1. Add the key binding in the `run_app()` function:

```rust
match app.input_mode {
    InputMode::Normal => match key.code {
        KeyCode::Char('x') => {
            // Your new command logic here
        },
        // ...
    },
    // ...
}
```

2. Update the help message in `ui()`:

```rust
let (msg, style) = match app.input_mode {
    InputMode::Normal => (
        vec![Line::from(vec![
            // ... Add your new command to the help text
        ])],
        Style::default(),
    ),
    // ...
};
```

#### Adding New Todo Properties

1. Update the `Todo` struct:

```rust
struct Todo {
    title: String,
    completed: bool,
    // Add new field
    priority: Priority,
}
```

2. Update the rendering in the `ui()` function:

```rust
ListItem::new(content).style(Style::default().fg(
    match todo.priority {
        Priority::High => Color::Red,
        Priority::Normal => Color::White,
        Priority::Low => Color::Gray,
    }
))
```

### Best Practices

1. Error Handling
   - Use `Result` for operations that can fail
   - Properly clean up terminal state in error cases
   - Provide meaningful error messages

2. State Management
   - Keep all state in the `App` struct
   - Use immutable references where possible
   - Update state atomically

3. UI Design
   - Follow terminal UI conventions
   - Provide clear visual feedback
   - Keep the interface responsive

4. Code Organization
   - Separate concerns (UI, state, events)
   - Use meaningful variable names
   - Comment complex logic

### Testing

Add tests to ensure reliability:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todo_completion() {
        let mut todo = Todo {
            title: "Test todo".to_string(),
            completed: false,
        };
        assert!(!todo.completed);
        todo.completed = true;
        assert!(todo.completed);
    }
}
```

### Future Improvements

1. Persistence
   - Add file storage for todos
   - Implement import/export functionality
   - Add configuration file support

2. Features
   - Todo categories/tags
   - Due dates
   - Priority levels
   - Search functionality

3. UI Enhancements
   - Color themes
   - Custom key bindings
   - Status bar
   - Multiple views/panels

4. Performance
   - Optimize rendering for large lists
   - Implement pagination
   - Add caching for frequently accessed data
