use clap::ValueEnum;
use std::fs;

use crate::todolist::TodoList;

#[derive(Clone, ValueEnum)]
pub enum FormatEnum {
    Json,
    Csv,
    Yaml,
    Markdown,
}

pub trait Exporter {
    fn export(&self, todolist: &TodoList) -> Result<(), ExportError>;
}

pub enum ExportError {
    SerializationError(String),
    IoError(std::io::Error),
}

pub struct JsonExporter;

impl Exporter for JsonExporter {
    fn export(&self, todolist: &TodoList) -> Result<(), ExportError> {
        let json = serde_json::to_string_pretty(todolist)
            .map_err(|e| ExportError::SerializationError(e.to_string()))?;
        fs::write(&todolist.path, json).map_err(ExportError::IoError)?;
        Ok(())
    }
}

pub struct CsvExporter;

impl Exporter for CsvExporter {
    fn export(&self, todolist: &TodoList) -> Result<(), ExportError> {
        let mut csv = csv::Writer::from_path(todolist.path.with_extension("csv"))
            .map_err(|e| ExportError::SerializationError(e.to_string()))?;
        for task in todolist.tasks.iter() {
            csv.serialize(task)
                .map_err(|e| ExportError::SerializationError(e.to_string()))?;
        }
        csv.flush().map_err(ExportError::IoError)?;
        Ok(())
    }
}

pub struct YamlExporter;

impl Exporter for YamlExporter {
    fn export(&self, todolist: &TodoList) -> Result<(), ExportError> {
        let yaml = serde_yml::to_string(todolist)
            .map_err(|e| ExportError::SerializationError(e.to_string()))?;
        fs::write(todolist.path.with_extension("yaml"), yaml).map_err(ExportError::IoError)?;
        Ok(())
    }
}

pub struct MarkdownExporter;

impl Exporter for MarkdownExporter {
    fn export(&self, todolist: &TodoList) -> Result<(), ExportError> {
        let mut markdown = String::new();
        for task in &todolist.tasks {
            markdown.push_str("- [");
            if !task.done {
                markdown.push('x');
            }
            markdown.push_str("] ");
            markdown.push_str(&task.title);
            markdown.push_str(&format!(" - Created at {}", task.created_at));
            if let Some(completed) = task.completed_at {
                markdown.push_str(&format!(" - Completed at {}", completed));
            }
            markdown.push('\n');
        }
        fs::write(todolist.path.with_extension("md"), markdown).map_err(ExportError::IoError)?;
        Ok(())
    }
}
