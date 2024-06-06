use std::io::{self, Write};

struct Task {
    description: String,
    completed: bool,
}

impl Task {
    fn new(description: String) -> Task {
        Task {
            description,
            completed: false,
        }
    }

    fn complete(&mut self) {
        self.completed = true;
    }
}

struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    fn new() -> TaskManager {
        TaskManager { tasks: Vec::new() }
    }

    fn add_task(&mut self, description: String) {
        let task = Task::new(description);
        self.tasks.push(task);
    }

    fn list_tasks(&self) {
        for (index, task) in self.tasks.iter().enumerate() {
            println!("{}. [{}] {}", index + 1, if task.completed { "X" } else { " " }, task.description);
        }
    }

    fn complete_task(&mut self, index: usize) -> Result<(), &'static str> {
        if index >= self.tasks.len() {
            return Err("Invalid task index");
        }
        self.tasks[index].complete();
        Ok(())
    }
}

fn main() {
    let mut task_manager = TaskManager::new();
    loop {
        println!("Options:");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Complete Task");
        println!("4. Exit");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: usize = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid option");
                continue;
            }
        };

        match choice {
            1 => {
                print!("Enter task description: ");
                io::stdout().flush().unwrap();
                let mut description = String::new();
                io::stdin().read_line(&mut description).expect("Failed to read line");
                task_manager.add_task(description.trim().to_string());
            }
            2 => {
                println!("Tasks:");
                task_manager.list_tasks();
            }
            3 => {
                print!("Enter task index to mark as complete: ");
                io::stdout().flush().unwrap();
                let mut index_str = String::new();
                io::stdin().read_line(&mut index_str).expect("Failed to read line");
                let index: usize = match index_str.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please enter a valid task index");
                        continue;
                    }
                };
                match task_manager.complete_task(index - 1) {
                    Ok(_) => println!("Task marked as complete"),
                    Err(err) => println!("{}", err),
                }
            }
            4 => break,
            _ => println!("Invalid option"),
        }
    }
}
