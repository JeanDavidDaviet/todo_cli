mod cli;
mod exporter;
mod task;
mod todolist;

use clap::Parser;

use crate::{
    cli::{Cli, Commands},
    todolist::TodoList,
};

fn main() {
    let cli = Cli::parse();
    let mut todolist = TodoList::load_tasks(cli.path);
    match cli.command {
        Commands::Add { title } => {
            todolist.add_task(title);
            todolist.list_tasks();
        }
        Commands::Remove { id } => {
            todolist.remove_task(id);
            todolist.list_tasks();
        }
        Commands::Complete { id } => {
            todolist.complete_task(id);
            todolist.list_tasks();
        }
        Commands::List { completed, pending } => {
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
        Commands::Export { format } => {
            todolist.export_tasks(format);
        }
    }
}
