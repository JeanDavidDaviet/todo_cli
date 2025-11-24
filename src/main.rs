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

#[derive(Serialize, Deserialize, Debug)]
struct TodoList {
    tasks: Vec<Task>,
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
            if task.done {
                println!("✅ {} - Créée le {} - Complétée le {}", task.title, task.created_at, task.completed_at.map_or("Non complétée".to_string(), |dt| dt.to_string()));
            } else {
                println!("❌ {} - Créée le {}", task.title, task.created_at)
            }
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

    fn pending_tasks(&self) -> PendingTasksIter {
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
    List,
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
    let mut todo_list = TodoList { tasks: vec![] };
    todo_list.add_task("coucou".to_string());
    todo_list.add_task("hello".to_string());
    todo_list.complete_task(1);

    // for task in &todo_list {
    //     println!("{}", task.title);
    // }

    for task in todo_list.pending_tasks() {
        println!("{}", task.title);
    }
    // let cli = Cli::parse();
    // let mut todolist = TodoList::load();
    // match cli.command {
    //     Commands::Add { title } => {
    //         todolist.add_task(title);
    //         todolist.list_tasks();
    //     },
    //     Commands::Remove { id } => {
    //         todolist.remove_task(id);
    //         todolist.list_tasks();
    //     },
    //     Commands::Complete { id } => {
    //         todolist.complete_task(id);
    //         todolist.list_tasks();
    //     }
    //     Commands::List => {
    //         todolist.list_tasks();
    //     }
    //     Commands::Reset => {
    //         todolist.reset_tasks();
    //         todolist.list_tasks();
    //     }
    // }
}
