use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};

const SAVE_FILE: &str = "todos.json";

#[derive(Serialize, Deserialize)]
struct Todo {
    id: usize,
    task: String,
    completed: bool,
}

struct TodoList {
    todos: Vec<Todo>,
    next_id: usize,
}

impl TodoList {
    fn new() -> Self {
        TodoList {
            todos: Vec::new(),
            next_id: 1,
        }
    }

    fn load() -> Self {
        match fs::read_to_string(SAVE_FILE) {
            Ok(contents) => {
                if let Ok(todos) = serde_json::from_str::<Vec<Todo>>(&contents) {
                    let next_id = todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;
                    println!("✓ Loaded {} tasks from file", todos.len());
                    TodoList { todos, next_id }
                } else {
                    println!("⚠ Could not parse save file, starting fresh");
                    TodoList::new()
                }
            }
            Err(_) => {
                println!("📝 No save file found, starting fresh");
                TodoList::new()
            }
        }
    }

    fn save(&self) {
        match serde_json::to_string_pretty(&self.todos) {
            Ok(json) => {
                if let Err(e) = fs::write(SAVE_FILE, json) {
                    println!("❌ Failed to save: {}", e);
                } else {
                    println!("💾 Tasks saved!");
                }
            }
            Err(e) => println!("❌ Failed to serialize: {}", e),
        }
    }

    fn add(&mut self, task: String) {
        let todo = Todo {
            id: self.next_id,
            task,
            completed: false,
        };
        self.todos.push(todo);
        self.next_id += 1;
        println!("✓ Task added!");
        self.save();
    }

    fn list(&self) {
        if self.todos.is_empty() {
            println!("No tasks yet. Add one to get started!");
            return;
        }

        println!("\n📋 Your Tasks:");
        println!("─────────────────────────────────");
        for todo in &self.todos {
            let status = if todo.completed { "✓" } else { " " };
            println!("[{}] {}. {}", status, todo.id, todo.task);
        }
        println!("─────────────────────────────────\n");
    }

    fn complete(&mut self, id: usize) {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.completed = true;
            println!("✓ Task {} marked as complete!", id);
            self.save();
        } else {
            println!("❌ Task {} not found.", id);
        }
    }

    fn delete(&mut self, id: usize) {
        if let Some(pos) = self.todos.iter().position(|t| t.id == id) {
            self.todos.remove(pos);
            println!("✓ Task {} deleted!", id);
            self.save();
        } else {
            println!("❌ Task {} not found.", id);
        }
    }
}

fn main() {
    let mut todo_list = TodoList::load();
    
    println!("🚀 Welcome to Rust Todo App!");
    
    loop {
        println!("\nCommands: [a]dd | [l]ist | [c]omplete | [d]elete | [q]uit");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "a" | "add" => {
                print!("Enter task: ");
                io::stdout().flush().unwrap();
                let mut task = String::new();
                io::stdin().read_line(&mut task).unwrap();
                todo_list.add(task.trim().to_string());
            }
            "l" | "list" => {
                todo_list.list();
            }
            "c" | "complete" => {
                print!("Enter task ID to complete: ");
                io::stdout().flush().unwrap();
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).unwrap();
                if let Ok(id) = id_input.trim().parse::<usize>() {
                    todo_list.complete(id);
                } else {
                    println!("❌ Invalid ID.");
                }
            }
            "d" | "delete" => {
                print!("Enter task ID to delete: ");
                io::stdout().flush().unwrap();
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).unwrap();
                if let Ok(id) = id_input.trim().parse::<usize>() {
                    todo_list.delete(id);
                } else {
                    println!("❌ Invalid ID.");
                }
            }
            "q" | "quit" => {
                println!("👋 Goodbye!");
                break;
            }
            _ => {
                println!("❌ Unknown command. Try again!");
            }
        }
    }
}