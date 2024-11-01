use colored::*;
use rand::Rng;
use std::io::{self, Write};

#[derive(Debug)]
enum Status {
    Pending,
    Completed,
}

#[derive(Debug)]
struct Task {
    title: String,
    status: Status,
}

impl Task {
    fn new(title: String) -> Self {
        Self { title, status: Status::Pending }
    }
    fn complete(&mut self) { self.status = Status::Completed; }
    fn display(&self) {
        let status = match self.status {
            Status::Pending => "Pending".yellow(),
            Status::Completed => "Completed".green(),
        };
        println!("{} - [{}]", self.title, status);
    }
}

fn random_message<'a>(quotes: &'a [&'a str]) -> &'a str {
    quotes[rand::thread_rng().gen_range(0..quotes.len())]
}

fn main() {
    let mut tasks = Vec::new();
    println!("{}", "Welcome to Mayor's To-Do!".bold().cyan());
    let add_msgs = ["You can do it! 🚀", "Keep pushing forward! 💪", "One task at a time! 🌟"];
    let complete_msgs = ["Great job! 🎉", "One down, many to go! 🌈", "Another task bites the dust! 🕺"];

    loop {
        println!("\n1. Add task\n2. Complete task\n3. View tasks\n4. Exit");
        print!("Enter choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        match choice.trim() {
            "1" => {
                print!("Enter task title: ");
                io::stdout().flush().unwrap();
                let mut title = String::new();
                io::stdin().read_line(&mut title).expect("Failed to read line");
                tasks.push(Task::new(title.trim().to_string()));
                println!("{}", random_message(&add_msgs).bold().green());
            }
            "2" => {
                print!("Enter task number to complete: ");
                io::stdout().flush().unwrap();
                let mut num = String::new();
                io::stdin().read_line(&mut num).expect("Failed to read line");
                if let Ok(index) = num.trim().parse::<usize>() {
                    if index > 0 && index <= tasks.len() {
                        tasks[index - 1].complete();
                        println!("{}", random_message(&complete_msgs).bold().green());
                    } else {
                        println!("{}", "Invalid task number.".bold().red());
                    }
                }
            }
            "3" => {
                println!("\nYour Tasks:");
                for (i, task) in tasks.iter().enumerate() {
                    print!("{}: ", i + 1);
                    task.display();
                }
            }
            "4" => {
                println!("{}", "Goodbye!".bold().cyan());
                break;
            }
            _ => println!("{}", "Invalid choice, please try again.".bold().red()),
        }
    }
}
