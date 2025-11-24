use std::{fs, vec};
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
            println!("✅ {} - Créée le {} - Complétée le {}", self.title, self.created_at, self.completed_at.map_or("Non complétée".to_string(), |dt| dt.to_string()));
        } else {
            println!("❌ {} - Créée le {}", self.title, self.created_at)
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct TodoList {
    tasks: Vec<Task>,
}

struct CompletedTasksIter<'a> {
    inner: std::slice::Iter<'a, Task>
}
struct PendingTasksIter<'a> {
    inner: std::slice::Iter<'a, Task>
}

impl TodoList {
    fn new() -> Self {
        let todolist = TodoList { tasks: vec![] };
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
        if let Err(_) = fs::exists("json.json") {
            fs::write("json.json", "").expect("Erreur création fichier json.json");
        }
        let json = serde_json::to_string_pretty(&self).expect("Erreur sérialisation");
        fs::write("json.json", json).expect("Erreur écriture fichier");
    }

    fn load() -> Self {
        match fs::read_to_string("json.json") {
            Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| TodoList::new()),
            Err(_) => TodoList::new(),
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
#[command(about = "Un gestionnaire de tâches simple", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Ajouter une nouvelle tâche
    Add {
        /// Le titre de la tâche
        title: String
    },
    /// Lister toutes les tâches
    List {
        /// Afficher seulement les tâches complétées
        #[arg(long)]
        completed: bool,

        /// Afficher seulement les tâches en cours
        #[arg(long)]
        pending: bool,
    },
    /// Supprimer une tâche
    Remove {
        /// L'ID de la tâche
        id: i32
    },
    /// Compléter une tâche
    Complete {
        /// L'ID de la tâche
        id: i32
    },
    /// Remettre à zéro toutes les tâches
    Reset,
}

fn main() {    
    let cli = Cli::parse();
    let mut todolist = TodoList::load();
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
