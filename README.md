# Todo CLI

A simple command-line task manager written in Rust.

## Features

- Add tasks with optional priorities (high, medium, low)
- List tasks (all, completed, or pending)
- Complete tasks
- Remove tasks
- Reset all tasks
- Export tasks to different formats (JSON, CSV, YAML, Markdown)
- Persistent storage in JSON format using filesystem
- Different storage formats (JSON only for now)

## Installation

```bash
cargo build --release
```

## Usage

### Add a task
```bash
todo add "Task title"
```

### Add a task with priority
```bash
todo add "Task title" --priority high
# Or use short form
todo add "Task title" -p high
```

Available priorities: `high`, `medium`, `low`

### List all tasks
```bash
todo list
```

### List completed tasks only
```bash
todo list --completed
```

### List pending tasks only
```bash
todo list --pending
```

### Complete a task
```bash
todo complete <index>
```

### Remove a task
```bash
todo remove <index>
```

### Reset all tasks
```bash
todo reset
```

### Export tasks
```bash
# Export to JSON (default)
todo export output.json

# Export to CSV
todo export output.csv --format csv

# Export to YAML
todo export output.yaml --format yaml

# Export to Markdown
todo export output.md --format markdown

# Use short form for format
todo export output.csv -f csv
```

### Custom storage path
```bash
todo --path /path/to/file.json list
```

### Custom format storage
```bash
todo --format json list
# Or use short form
todo -f json list
```

Default storage path is `todo.json` in the current directory.

## License

MIT

## TODO

- [x] dynamic path
- [ ] anyhow ou thiserror
- [x] tests
- [x] modules
- [ ] TUI
- [x] Add priorities (high/medium/low)
- [ ] Add categories/tags
- [ ] Allow editing task title
- [ ] Undo system (keep history of modifications)
- [x] Export to different formats using custom trait:
    - [x] JSON, 
    - [x] CSV, 
    - [x] YAML, 
    - [x] Markdown
- [ ] Stats: number of tasks completed this week, completion rate
- [ ] Sub-tasks (recursive structure)
- [ ] Proper error handling: replace expect() with proper error handling
