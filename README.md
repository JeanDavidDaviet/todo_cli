# Todo CLI

A simple command-line task manager written in Rust.

## Features

- Add tasks
- List tasks (all, completed, or pending)
- Complete tasks
- Remove tasks
- Reset all tasks
- Persistent storage in JSON format using filesystem
- Differents storage formats (JSON only for now)

## Installation

```bash
cargo build --release
```

## Usage

### Add a task
```bash
todo add "Task title"
```

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

### Custom storage path
```bash
todo --path /path/to/file.json list
```

### Custom format storage
```bash
todo --format json list
```

Default storage path is `todo.json` in the current directory.

## License

MIT

## TODO

- [x] dynamic path
- [ ] anyhow ou thiserror
- [ ] tests
- [ ] modules
- [ ] TUI
- [ ] Add priorities (high/medium/low)
- [ ] Add categories/tags
- [ ] Allow editing task title
- [ ] Undo system (keep history of modifications)
- [ ] Export to different formats using custom trait:
    - [x] JSON, 
    - [] CSV, 
    - [] YAML, 
    - [] Markdown
- [ ] Stats: number of tasks completed this week, completion rate
- [ ] Sub-tasks (recursive structure)
- [ ] Proper error handling: replace expect() with proper error handling
