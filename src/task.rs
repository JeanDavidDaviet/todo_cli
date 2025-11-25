use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub done: bool,
    pub created_at: DateTime<Local>,
    pub completed_at: Option<DateTime<Local>>,
}

impl Task {
    pub fn display(&self) {
        if self.done {
            println!(
                "✅ {} - Created on {} - Completed on {}",
                self.title,
                self.created_at,
                self.completed_at
                    .map_or("Not completed".to_string(), |dt| dt.to_string())
            );
        } else {
            println!("❌ {} - Created on {}", self.title, self.created_at)
        }
    }
}
