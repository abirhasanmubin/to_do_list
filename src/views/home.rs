use crate::utils::input;

// todo: add current in progress tasks here

#[derive(Debug, Clone)]
pub enum HomeInputOptions {
    TaskList,
    AddTask,
    Exit,
    Invalid,
}

pub fn home() {
    println!("TODO APP");
    println!();
    println!("1. View All Tasks");
    println!("2. Add Task");
    println!("0. Exit");
    let input = input::take_input("Please enter ");
    let inputted_option = match validate_input(&input) {
        Ok(option) => option,
        Err(err) => {
            eprintln!("Error: {}", err);
            // Return a default value or handle the error in your application logic
            // For example, returning a default option:
            HomeInputOptions::Invalid
        }
    };
    println!("{:?}", inputted_option);
}

fn validate_input(input: &str) -> Result<HomeInputOptions, &str> {
    let trimmed_input = input.trim();
    if trimmed_input.is_empty() {
        return Err("No input provided");
    }

    let split_input: Vec<&str> = trimmed_input.split_whitespace().collect();
    if split_input.is_empty() {
        return Err("No input provided");
    }

    match split_input[0] {
        "1" => Ok(HomeInputOptions::TaskList),
        "2" => Ok(HomeInputOptions::AddTask),
        "0" => Ok(HomeInputOptions::Exit),
        _ => Err("Invalid input"),
    }
}
