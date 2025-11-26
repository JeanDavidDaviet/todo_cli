use chrono::{DateTime, Local};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, ValueEnum, PartialEq, Debug)]
pub enum PriorityEnum {
    High,
    Medium,
    Low,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub done: bool,
    pub created_at: DateTime<Local>,
    pub completed_at: Option<DateTime<Local>>,
    pub priority: Option<PriorityEnum>,
}

impl Task {
    pub fn display(&self) {
        let priority = match self.priority {
            Some(PriorityEnum::High) => " - Priority high",
            Some(PriorityEnum::Medium) => " - Priority medium",
            Some(PriorityEnum::Low) => " - Priority low",
            None => "",
        };
        if self.done {
            println!(
                "✅ {} - Created on {} - Completed on {}{}",
                self.title,
                self.created_at,
                self.completed_at
                    .map_or("Not completed".to_string(), |dt| dt.to_string()),
                priority,
            );
        } else {
            println!(
                "❌ {} - Created on {}{}",
                self.title, self.created_at, priority
            )
        }
    }
}
