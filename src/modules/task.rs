use std::fmt::{Display, Formatter, Result};

use chrono::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub enum TaskStatus {
    Open,
    InProgress,
    Done,
}

impl Display for TaskStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            TaskStatus::Open => write!(f, "OPEN"),
            TaskStatus::InProgress => write!(f, "IN PROGRESS"),
            TaskStatus::Done => write!(f, "DONE"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Task {
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    status: TaskStatus,
}

impl Task {
    pub fn new(title: &str, description: &str) -> Task {
        Task {
            title: String::from(title),
            description: String::from(description),
            status: TaskStatus::Open,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
    pub fn update_status(&mut self, status: TaskStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }
    pub fn update_task_title(&mut self, title: String) {
        self.title = title;
        self.updated_at = Utc::now();
    }
    pub fn update_task_description(&mut self, description: String) {
        self.description = description;
        self.updated_at = Utc::now();
    }
    pub fn get_status(&self) -> &TaskStatus {
        &self.status
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "title: {} -> {} | created: {}, updated: {}",
            self.title,
            self.status,
            self.created_at.format("%b %d, %Y %I:%M %P"),
            self.updated_at.format("%x %I:%M %P"),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_task() {
        let task = Task::new("first task", "test task");

        println!("{}", task.to_string());
        assert_eq!(task.title, String::from("first task"));
        assert_eq!(task.description, String::from("test task"));
        assert_eq!(task.status, TaskStatus::Open);
    }
}
