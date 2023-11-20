use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct Task {
    pub title: String,
    pub description: String,
    is_started: bool,
    is_running: bool,
    is_completed: bool,
}

impl Task {
    pub fn new(title: &str, description: &str) -> Task {
        Task {
            title: String::from(title),
            description: String::from(description),
            is_started: false,
            is_running: false,
            is_completed: false,
        }
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "title: {}", self.title)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_task() {
        let task = Task::new("first task", "test task");

        assert_eq!(task.title, String::from("first task"));
        assert_eq!(task.description, String::from("test task"));
        assert_eq!(task.is_started, false);
        assert_eq!(task.is_running, false);
        assert_eq!(task.is_completed, false);
        assert_eq!(task.to_string(), "title: first task");
    }
}
