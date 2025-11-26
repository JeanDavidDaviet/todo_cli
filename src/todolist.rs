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
        let mut last_task_id = 1;
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

    pub fn remove_task(&mut self, i: i32) {
        if let Ok(index) = usize::try_from(i - 1)
            && let Some(_) = self.tasks.get_mut(index)
        {
            self.tasks.remove(index);
        }
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

#[cfg(test)]
mod tests {
    use tempfile::NamedTempFile;
    use super::*;

    #[test]
    fn test_new_todolist_is_empty() {
        let path = NamedTempFile::new().unwrap().path().to_path_buf();
        let todolist = TodoList::new(&path);
        assert_eq!(todolist.tasks.len(), 0);
        assert_eq!(&todolist.path, &path);
    }

    #[test]
    fn test_add_task_increase_length() {
        let path = NamedTempFile::new().unwrap().path().to_path_buf();
        let mut todolist = TodoList::new(&path);
        todolist.add_task("task 1".to_string(), None);
        assert_eq!(todolist.tasks.len(), 1);
        todolist.add_task("task 2".to_string(), None);
        assert_eq!(todolist.tasks.len(), 2);
    }

    #[test]
    fn test_remove_task_decrease_length() {
        let path = NamedTempFile::new().unwrap().path().to_path_buf();
        let mut todolist = TodoList::new(&path);
        todolist.add_task("task 1".to_string(), None);
        todolist.add_task("task 2".to_string(), None);
        todolist.remove_task(2);
        assert_eq!(todolist.tasks.len(), 1);
    }

    #[test]
    fn test_add_tasks_with_priority() {
        let path = NamedTempFile::new().unwrap().path().to_path_buf();
        let mut todolist = TodoList::new(&path);
        todolist.add_task("task 1".to_string(), Some(PriorityEnum::High));
        todolist.add_task("task 2".to_string(), Some(PriorityEnum::Medium));
        todolist.add_task("task 2".to_string(), Some(PriorityEnum::Low));
        todolist.add_task("task 2".to_string(), None);
        assert_eq!(
            todolist.tasks.get(0).unwrap().priority,
            Some(PriorityEnum::High)
        );
        assert_eq!(
            todolist.tasks.get(1).unwrap().priority,
            Some(PriorityEnum::Medium)
        );
        assert_eq!(
            todolist.tasks.get(2).unwrap().priority,
            Some(PriorityEnum::Low)
        );
        assert_eq!(todolist.tasks.get(3).unwrap().priority, None);
    }

    #[test]
    fn test_task_ids_are_incremented() {
        let path = NamedTempFile::new().unwrap().path().to_path_buf();
        let mut todolist = TodoList::new(&path);
        todolist.add_task("task 1".to_string(), None);
        assert_eq!(todolist.tasks.get(0).unwrap().id, 1);
        todolist.add_task("task 2".to_string(), None);
        assert_eq!(todolist.tasks.get(1).unwrap().id, 2);
    }

    #[test]
    fn test_complete_task_changes_status() {
        let path = NamedTempFile::new().unwrap().path().to_path_buf();
        let mut todolist = TodoList::new(&path);
        todolist.add_task("task 1".to_string(), None);
        assert_eq!(todolist.tasks.get(0).unwrap().done, false);
        todolist.complete_task(1);
        assert_eq!(todolist.tasks.get(0).unwrap().done, true);
    }

    #[test]
    fn test_complete_task_changes_completed_at() {
        let path = NamedTempFile::new().unwrap().path().to_path_buf();
        let mut todolist = TodoList::new(&path);
        todolist.add_task("task 1".to_string(), None);
        assert_eq!(todolist.tasks.get(0).unwrap().completed_at, None);
        todolist.complete_task(1);
        assert_ne!(todolist.tasks.get(0).unwrap().completed_at, None);
    }

    #[test]
    fn test_completed_task_iterator() {
        let path = NamedTempFile::new().unwrap().path().to_path_buf();
        let mut todolist = TodoList::new(&path);
        todolist.add_task("task 1".to_string(), None);
        todolist.add_task("task 2".to_string(), None);
        todolist.add_task("task 3".to_string(), None);
        let completed_tasks: Vec<&Task> = todolist.completed_tasks().collect();
        assert_eq!(completed_tasks.len(), 0);
        todolist.complete_task(2);
        todolist.complete_task(3);
        let completed_tasks: Vec<&Task> = todolist.completed_tasks().collect();
        assert_eq!(completed_tasks.len(), 2);
    }

    #[test]
    fn test_pending_task_iterator() {
        let path = NamedTempFile::new().unwrap().path().to_path_buf();
        let mut todolist = TodoList::new(&path);
        todolist.add_task("task 1".to_string(), None);
        todolist.add_task("task 2".to_string(), None);
        todolist.add_task("task 3".to_string(), None);
        let pending_tasks: Vec<&Task> = todolist.pending_tasks().collect();
        assert_eq!(pending_tasks.len(), 3);
        todolist.complete_task(2);
        todolist.complete_task(3);
        let pending_tasks: Vec<&Task> = todolist.pending_tasks().collect();
        assert_eq!(pending_tasks.len(), 1);
    }

    #[test]
    fn test_reset_all_tasks() {
        let path = NamedTempFile::new().unwrap().path().to_path_buf();
        let mut todolist = TodoList::new(&path);
        todolist.add_task("task 1".to_string(), None);
        todolist.add_task("task 2".to_string(), None);
        todolist.add_task("task 3".to_string(), None);
        todolist.complete_task(1);
        todolist.complete_task(2);
        todolist.reset_tasks();
        let pending_tasks: Vec<&Task> = todolist.pending_tasks().collect();
        assert_eq!(pending_tasks.len(), 3);
    }

    #[test]
    fn test_save_and_load() {
        let path = NamedTempFile::new().unwrap().path().to_path_buf();
        let mut todolist = TodoList::new(&path);
        todolist.add_task("task 1".to_string(), None);
        todolist.add_task("task 2".to_string(), Some(PriorityEnum::High));
        todolist.add_task("task 3".to_string(), None);
        todolist.complete_task(2);
        todolist.save_tasks();

        let content = fs::read_to_string(&path).unwrap();
        let loaded: TodoList = serde_json::from_str(&content).unwrap();

        assert_eq!(loaded.tasks.len(), 3);
        assert_eq!(loaded.tasks[0].title, "task 1");
        assert_eq!(loaded.tasks[0].done, false);
        assert_eq!(loaded.tasks[1].title, "task 2");
        assert_eq!(loaded.tasks[1].done, true);
        assert_eq!(loaded.tasks[1].priority, Some(PriorityEnum::High));
        assert_ne!(loaded.tasks[1].completed_at, None);
        assert_eq!(loaded.tasks[2].title, "task 3");
        assert_eq!(loaded.tasks[2].done, false);
    }
}
