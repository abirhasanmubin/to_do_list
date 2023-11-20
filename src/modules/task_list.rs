use crate::modules::task::Task;

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
        let task = Task::new(title, description);
        self.tasks.push(task);
        self.total += 1;
    }
    pub fn get_task(&mut self, index: usize) -> &Task {
        let task = self.tasks.get(index).expect("Error index is out of bound");
        task
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
}
