use chrono::{Local, NaiveDate};
use colored::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};

const SAVE_FILE: &str = "todos.json";

#[derive(Serialize, Deserialize, Clone)]
struct Todo {
    id: usize,
    task: String,
    completed: bool,
    due_date: Option<NaiveDate>,
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
                    println!("{}", format!("✓ Loaded {} tasks from file", todos.len()).green());
                    TodoList { todos, next_id }
                } else {
                    println!("{}", "⚠ Could not parse save file, starting fresh".yellow());
                    TodoList::new()
                }
            }
            Err(_) => {
                println!("{}", "📝 No save file found, starting fresh".cyan());
                TodoList::new()
            }
        }
    }

    fn save(&self) {
        match serde_json::to_string_pretty(&self.todos) {
            Ok(json) => {
                if let Err(e) = fs::write(SAVE_FILE, json) {
                    println!("{}", format!("❌ Failed to save: {}", e).red());
                } else {
                    println!("{}", "💾 Tasks saved!".green());
                }
            }
            Err(e) => println!("{}", format!("❌ Failed to serialize: {}", e).red()),
        }
    }

    fn add(&mut self, task: String, due_date: Option<NaiveDate>) {
        let todo = Todo {
            id: self.next_id,
            task,
            completed: false,
            due_date,
        };
        self.todos.push(todo);
        self.next_id += 1;
        println!("{}", "✓ Task added!".green());
        self.save();
    }

    fn list(&self) {
        if self.todos.is_empty() {
            println!("{}", "No tasks yet. Add one to get started!".yellow());
            return;
        }

        let mut todos = self.todos.clone();
        todos.sort_by(|a, b| {
            match (&a.due_date, &b.due_date) {
                (Some(date_a), Some(date_b)) => date_a.cmp(date_b),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => a.id.cmp(&b.id),
            }
        });

        let today = Local::now().date_naive();

        println!("\n{}", "📋 Your Tasks:".bold().cyan());
        println!("{}", "─────────────────────────────────".cyan());
        for todo in &todos {
            let status = if todo.completed { "✓".green() } else { " ".normal() };
            let due_info = if let Some(date) = todo.due_date {
                let days_diff = (date - today).num_days();
                let formatted_date = date.format("%d-%m-%Y");
                if date < today && !todo.completed {
                    format!(" OVERDUE ({})", formatted_date).red().bold()
                } else if days_diff == 0 {
                    " Due TODAY".yellow().bold()
                } else if days_diff == 1 {
                    " Due tomorrow".yellow()
                } else if days_diff <= 3 {
                    format!(" Due in {} days", days_diff).yellow()
                } else {
                    format!(" (Due: {})", formatted_date).normal()
                }
            } else {
                "".normal()
            };
            
            let task_text = if todo.completed {
                todo.task.dimmed()
            } else {
                todo.task.normal()
            };
            
            println!("[{}] {}. {}{}", status, todo.id, task_text, due_info);
        }
        println!("{}\n", "─────────────────────────────────".cyan());
    }

    fn complete(&mut self, id: usize) {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.completed = true;
            println!("{}", format!("✓ Task {} marked as complete!", id).green());
            self.save();
        } else {
            println!("{}", format!("❌ Task {} not found.", id).red());
        }
    }

    fn delete(&mut self, id: usize) {
        if let Some(pos) = self.todos.iter().position(|t| t.id == id) {
            self.todos.remove(pos);
            println!("{}", format!("✓ Task {} deleted!", id).green());
            self.save();
        } else {
            println!("{}", format!("❌ Task {} not found.", id).red());
        }
    }
}

fn main() {
    let mut todo_list = TodoList::load();
    
    println!("{}", "🚀 Welcome to Rust Todo App!".bold().bright_blue());
    
    loop {
        println!("\n{}", "Commands: [a]dd | [l]ist | [c]omplete | [d]elete | [q]uit".bright_black());
        print!("{} ", ">".cyan().bold());
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
                
                print!("Due date (DD-MM-YYYY) or press Enter to skip: ");
                io::stdout().flush().unwrap();
                let mut date_input = String::new();
                io::stdin().read_line(&mut date_input).unwrap();
                let date_input = date_input.trim();
                
                let due_date = if date_input.is_empty() {
                    None
                } else {
                    match NaiveDate::parse_from_str(date_input, "%d-%m-%Y") {
                        Ok(date) => {
                            let today = Local::now().date_naive();
                            if date < today {
                                println!("{}", "❌ Due date cannot be in the past. Task added without due date.".red());
                                None
                            } else {
                                Some(date)
                            }
                        }
                        Err(_) => {
                            println!("{}", "❌ Invalid date format. Task added without due date.".red());
                            None
                        }
                    }
                };
                
                todo_list.add(task.trim().to_string(), due_date);
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
                    println!("{}", "❌ Invalid ID.".red());
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
                    println!("{}", "❌ Invalid ID.".red());
                }
            }
            "q" | "quit" => {
                println!("{}", "👋 Goodbye!".bright_blue());
                break;
            }
            _ => {
                println!("{}", "❌ Unknown command. Try again!".red());
            }
        }
    }
}