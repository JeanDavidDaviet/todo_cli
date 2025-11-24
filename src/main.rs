use std::{fs, path::PathBuf, vec};
use clap::{Parser, Subcommand};
use chrono::{DateTime, Local};


use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: i32,
    title: String,
    done: bool,
    created_at: DateTime<Local>,
    completed_at: Option<DateTime<Local>>,
}

impl Task {
    fn display(&self) {
        if self.done {
            println!("✅ {} - Created on {} - Completed on {}", self.title, self.created_at, self.completed_at.map_or("Not completed".to_string(), |dt| dt.to_string()));
        } else {
            println!("❌ {} - Created on {}", self.title, self.created_at)
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct TodoList {
    tasks: Vec<Task>,
    #[serde(skip_serializing)]
    path: PathBuf,
}

struct CompletedTasksIter<'a> {
    inner: std::slice::Iter<'a, Task>
}
struct PendingTasksIter<'a> {
    inner: std::slice::Iter<'a, Task>
}

impl TodoList {
    fn new(path: &PathBuf) -> Self {
        let todolist = TodoList { 
            tasks: vec![],
            path: path.to_path_buf()
        };
        todolist.save();
        todolist
    }

    fn add_task(&mut self, title: String) {
        let mut last_task_id = 0;
        if let Some(last_task) = self.tasks.last() {
            last_task_id = last_task.id + 1;
        }
        let task = Task {
            id: last_task_id,
            title: title,
            done: false,
            created_at: Local::now(),
            completed_at: None,
        };
        self.tasks.push(task);
        self.save();
    }

    fn remove_task(&mut self, id: i32) {
        if let Some(index) = self.tasks.iter().position(|task| task.id == id) {
            self.tasks.remove(index);
        };
        self.save();
    }

    fn list_tasks(&mut self) {
        for task in self.tasks.iter() {
            task.display();
        }
    }

    fn list_completed_tasks(&mut self) {
        for task in self.completed_tasks() {
            task.display();
        }
    }

    fn list_pending_tasks(&mut self) {
        for task in self.pending_tasks() {
            task.display();
        }
    }

    fn complete_task(&mut self, i: i32) {
        if let Ok(index) =usize::try_from(i - 1) {
            if let Some(task) = self.tasks.get_mut(index) {
                task.done = true;
                task.completed_at = Some(Local::now())
            }
        }
        self.save();
    }

    fn reset_tasks(&mut self) {
        for task in self.tasks.iter_mut() {
            task.done = false;
            task.completed_at = None;
        }
        self.save();
    }

    fn save(&self) {
        if let Err(_) = fs::exists(&self.path) {
            fs::write(&self.path, "").unwrap_or_else(|_| {
                panic!("Error creating file {:?}", &self.path);
            });
        }
        let json = serde_json::to_string_pretty(&self).expect("Serialization error");
        fs::write(&self.path, json).expect("File write error");
    }

    fn load(path: PathBuf) -> Self {
        match fs::read_to_string(&path) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| TodoList::new(&path)),
            Err(_) => TodoList::new(&path),
        }
    }

    fn completed_tasks<'a> (&'a self) -> CompletedTasksIter<'a> {
        CompletedTasksIter { 
            inner: self.tasks.iter()
        }
    }

    fn pending_tasks<'a> (&'a self) -> PendingTasksIter<'a> {
        PendingTasksIter { 
            inner: self.tasks.iter()
        }
    }
}

impl<'a> IntoIterator for &'a TodoList {
    type Item = &'a Task;
    type IntoIter = std::slice::Iter<'a, Task>;
    fn into_iter(self) -> Self::IntoIter {
        self.tasks.iter()
    }
}

impl<'a> Iterator for PendingTasksIter<'a> {
    type Item = &'a Task;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(task) = self.inner.next() {
            if !task.done {
                return Some(task)
            }
        }
        None
    }
}

impl<'a> Iterator for CompletedTasksIter<'a> {
    type Item = &'a Task;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(task) = self.inner.next() {
            if task.done {
                return Some(task)
            }
        }
        None
    }
}

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple task manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    /// Path to the save file
    #[arg(short, long, default_value = "todo.json")]
    path: PathBuf,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add {
        /// The task title
        title: String
    },
    /// List all tasks
    List {
        /// Display only completed tasks
        #[arg(long)]
        completed: bool,

        /// Display only pending tasks
        #[arg(long)]
        pending: bool,
    },
    /// Remove a task
    Remove {
        /// The task ID
        id: i32
    },
    /// Complete a task
    Complete {
        /// The task ID
        id: i32
    },
    /// Reset all tasks
    Reset,
}

fn main() {    
    let cli = Cli::parse();
    let mut todolist = TodoList::load(cli.path);
    match cli.command {
        Commands::Add { title } => {
            todolist.add_task(title);
            todolist.list_tasks();
        },
        Commands::Remove { id } => {
            todolist.remove_task(id);
            todolist.list_tasks();
        },
        Commands::Complete { id } => {
            todolist.complete_task(id);
            todolist.list_tasks();
        }
        Commands::List { completed, pending} => {
            if completed {
                todolist.list_completed_tasks();
            } else if pending {
                todolist.list_pending_tasks();
            } else {
                todolist.list_tasks();
            }
        }
        Commands::Reset => {
            todolist.reset_tasks();
            todolist.list_tasks();
        }
    }
}
