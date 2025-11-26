use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::{exporter::FormatEnum, task::PriorityEnum};

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple task manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    /// Path to the save file
    #[arg(short, long, default_value = "todo.json")]
    pub path: PathBuf,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new task
    Add {
        /// The task title
        title: String,
        /// The task priority
        #[arg(short, long)]
        priority: Option<PriorityEnum>,
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
        id: i32,
    },
    /// Complete a task
    Complete {
        /// The task ID
        id: i32,
    },
    /// Reset all tasks
    Reset,
    /// Export all tasks
    Export {
        /// Choose which format to export to
        #[arg(short, long)]
        format: FormatEnum,
    },
}
