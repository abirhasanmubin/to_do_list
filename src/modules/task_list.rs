use crate::modules::task::{Task, TaskStatus};

#[derive(Debug)]
pub struct TaskList {
    pub tasks: Vec<Task>,
    pub total: i32,
}

impl TaskList {
    pub fn new() -> TaskList {
        TaskList {
            tasks: Vec::new(),
            total: 0,
        }
    }
    pub fn add_task(&mut self, title: &str, description: &str) {
        let mut task = Task::new(title, description);
        if self.total % 2 == 0 {
            task.update_status(TaskStatus::InProgress);
        }
        self.tasks.push(task);
        self.total += 1;
    }
    pub fn get_task(&mut self, index: usize) -> &Task {
        let task = self.tasks.get(index).expect("Error index is out of bound");
        task
    }
    pub fn get_in_progress_task(&self) -> Vec<Task> {
        let in_progress_tasks: Vec<Task> = self
            .tasks
            .iter()
            .filter(|&task| *task.get_status() == TaskStatus::InProgress)
            .map(|task_ref| (*task_ref).clone())
            .collect();
        println!("{:?}", in_progress_tasks);
        in_progress_tasks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn initialize_task_list() -> TaskList {
        let mut task_list = TaskList::new();
        task_list.add_task("first task", "test task");
        task_list
    }

    #[test]
    fn create_task_list() {
        let task_list = initialize_task_list();
        assert_eq!(task_list.total, 1);
    }

    #[test]
    fn add_task_to_task_list() {
        let task_list = initialize_task_list();
        assert_eq!(task_list.total, 1);
    }

    #[test]
    fn get_task() {
        let mut task_list = initialize_task_list();
        let task = task_list.get_task(0);
        assert_eq!(task.title, "first task");
        assert_eq!(task.description, "test task");
        assert_eq!(task.to_string(), "title: first task -> OPEN");
    }

    #[test]
    fn check_filter() {
        let mut task_list = initialize_task_list();
        let filtered_list = task_list.get_in_progress_task();
        println!("{:?}", filtered_list);
    }
}
