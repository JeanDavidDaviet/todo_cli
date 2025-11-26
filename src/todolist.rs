use chrono::Local;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
    vec,
};

use crate::task::Task;
use crate::{exporter::*, task::PriorityEnum};

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoList {
    pub tasks: Vec<Task>,
    #[serde(skip)]
    pub path: PathBuf,
}

impl TodoList {
    pub fn new(path: &Path) -> Self {
        let todolist = TodoList {
            tasks: vec![],
            path: path.to_path_buf(),
        };
        todolist.save_tasks();
        todolist
    }

    pub fn add_task(&mut self, title: String, priority: Option<PriorityEnum>) {
        let mut last_task_id = 0;
        if let Some(last_task) = self.tasks.last() {
            last_task_id = last_task.id + 1;
        }
        let task = Task {
            id: last_task_id,
            title,
            done: false,
            created_at: Local::now(),
            completed_at: None,
            priority,
        };
        self.tasks.push(task);
        self.save_tasks();
    }

    pub fn remove_task(&mut self, id: i32) {
        if let Some(index) = self.tasks.iter().position(|task| task.id == id) {
            self.tasks.remove(index);
        };
        self.save_tasks();
    }

    pub fn list_tasks(&mut self) {
        for task in self.tasks.iter() {
            task.display();
        }
    }

    pub fn list_completed_tasks(&mut self) {
        for task in self.completed_tasks() {
            task.display();
        }
    }

    pub fn list_pending_tasks(&mut self) {
        for task in self.pending_tasks() {
            task.display();
        }
    }

    pub fn complete_task(&mut self, i: i32) {
        if let Ok(index) = usize::try_from(i - 1)
            && let Some(task) = self.tasks.get_mut(index)
        {
            task.done = true;
            task.completed_at = Some(Local::now())
        }
        self.save_tasks();
    }

    pub fn reset_tasks(&mut self) {
        for task in self.tasks.iter_mut() {
            task.done = false;
            task.completed_at = None;
        }
        self.save_tasks();
    }

    pub fn export_tasks(&self, format: FormatEnum) {
        if fs::exists(&self.path).is_err() {
            fs::write(&self.path, "").unwrap_or_else(|_| {
                panic!("Error creating file {:?}", &self.path);
            });
        }

        let exporter: Box<dyn Exporter> = match format {
            FormatEnum::Json => Box::new(JsonExporter),
            FormatEnum::Csv => Box::new(CsvExporter),
            FormatEnum::Yaml => Box::new(YamlExporter),
            FormatEnum::Markdown => Box::new(MarkdownExporter),
        };

        match exporter.export(self) {
            Ok(_) => (),
            Err(ExportError::SerializationError(msg)) => {
                eprintln!("Serialization failed {}", msg);
            }
            Err(ExportError::IoError(e)) => {
                eprintln!("IO error {}", e);
            }
        }
    }

    pub fn save_tasks(&self) {
        self.export_tasks(FormatEnum::Json);
    }

    pub fn load_tasks(path: PathBuf) -> Self {
        match fs::read_to_string(&path) {
            Ok(content) => {
                let mut todolist: TodoList =
                    serde_json::from_str(&content).unwrap_or_else(|_| TodoList::new(&path));
                todolist.path = path;
                todolist
            }
            Err(_) => TodoList::new(&path),
        }
    }

    pub fn completed_tasks<'a>(&'a self) -> CompletedTasksIter<'a> {
        CompletedTasksIter {
            inner: self.tasks.iter(),
        }
    }

    pub fn pending_tasks<'a>(&'a self) -> PendingTasksIter<'a> {
        PendingTasksIter {
            inner: self.tasks.iter(),
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

pub struct CompletedTasksIter<'a> {
    inner: std::slice::Iter<'a, Task>,
}

impl<'a> Iterator for CompletedTasksIter<'a> {
    type Item = &'a Task;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.by_ref().find(|&task| task.done).map(|v| v as _)
    }
}

pub struct PendingTasksIter<'a> {
    inner: std::slice::Iter<'a, Task>,
}

impl<'a> Iterator for PendingTasksIter<'a> {
    type Item = &'a Task;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.by_ref().find(|&task| !task.done).map(|v| v as _)
    }
}
