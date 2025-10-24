use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

// Derived 

#[derive(Debug)]
struct TodoItem {
    task: String,
    done: bool,
}

impl TodoItem {
    fn new(task: String) -> Self {
        TodoItem { task, done: false }
    }
}

fn main() {
    let mut todos: Vec<TodoItem> = load_todos("todos.txt");

    loop {
        println!("\n=== Simple Rust To-Do App ===");
        println!("1. Add task");
        println!("2. List tasks");
        println!("3. Mark task as done");
        println!("4. Save & exit");

        let choice = read_input("Choose an option: ");

        match choice.trim() {
            "1" => {
                let task = read_input("Enter a new task: ");
                todos.push(TodoItem::new(task.trim().to_string()));
                println!("/ Task added!");
            }
            "2" => {
                println!("\nYour tasks:");
                for (i, todo) in todos.iter().enumerate() {
                    let status = if todo.done { "[✔]" } else { "[ ]" };
                    println!("{} {} {}", i + 1, status, todo.task);
                }
            }
            "3" => {
                let idx = read_input("Enter task number to mark as done: ");
                if let Ok(num) = idx.trim().parse::<usize>() {
                    if num > 0 && num <= todos.len() {
                        todos[num - 1].done = true;
                        println!("Task marked as done!");
                    } else {
                        println!("Invalid task number.");
                    }
                }
            }
            "4" => {
                save_todos("todos.txt", &todos);
                println!("💾 Tasks saved. Goodbye!");
                break;
            }
            _ => println!("⚠️ Invalid option, try again."),
        }
    }
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn load_todos(filename: &str) -> Vec<TodoItem> {
    let file = OpenOptions::new().read(true).open(filename);
    let mut todos = Vec::new();

    if let Ok(f) = file {
        let reader = BufReader::new(f);
        for line in reader.lines() {
            if let Ok(l) = line {
                let parts: Vec<&str> = l.split('|').collect();
                if parts.len() == 2 {
                    todos.push(TodoItem {
                        task: parts[0].to_string(),
                        done: parts[1] == "1",
                    });
                }
            }
        }
    }
    todos
}

fn save_todos(filename: &str, todos: &[TodoItem]) {
    let mut file = File::create(filename).unwrap();
    for todo in todos {
        let line = format!("{}|{}\n", todo.task, if todo.done { "1" } else { "0" });
        file.write_all(line.as_bytes()).unwrap();
    }
}
