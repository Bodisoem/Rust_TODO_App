# 📋 Rust Todo App

A beautiful, colorful command-line todo application written in Rust. Keep track of your tasks with due dates, automatic sorting, and persistent storage.

## ✨ Features

- ✅ **Add, List, Complete, and Delete** tasks
- 📅 **Due Dates** with DD-MM-YYYY format
- 🎨 **Color-Coded Display**
  - 🔴 Red for overdue tasks
  - 🟡 Yellow for upcoming tasks (within 3 days)
  - 🟢 Green for completed tasks
- 💾 **Automatic Saving** to JSON file
- 🔄 **Auto-Sort** by due date (soonest first)
- ⚠️ **Smart Warnings** for tasks due today or tomorrow
- 🚫 **Past Date Validation** - prevents setting due dates in the past

## 🚀 Getting Started

### Prerequisites

- Rust and Cargo installed ([Install Rust](https://www.rust-lang.org/tools/install))
- A terminal that supports ANSI colors

### Installation

1. Clone or download this repository
2. Navigate to the project directory:
   ```bash
   cd todo_app
   ```
3. Build and run:
   ```bash
   cargo run
   ```

## 📖 Usage

### Commands

Once the app is running, use these commands:

- `a` or `add` - Add a new task
- `l` or `list` - List all tasks
- `c` or `complete` - Mark a task as complete
- `d` or `delete` - Delete a task
- `q` or `quit` - Exit the app

### Adding a Task

```
> a
Enter task: Buy groceries
Due date (DD-MM-YYYY) or press Enter to skip: 15-02-2026
✓ Task added!
💾 Tasks saved!
```

### Viewing Tasks

```
> l

📋 Your Tasks:
─────────────────────────────────
[✓] 1. Completed task
[ ] 2. Buy groceries Due TODAY
[ ] 3. Submit report Due in 2 days
[ ] 4. Call dentist (Due: 20-02-2026)
─────────────────────────────────
```

### Completing a Task

```
> c
Enter task ID to complete: 2
✓ Task 2 marked as complete!
💾 Tasks saved!
```

### Deleting a Task

```
> d
Enter task ID to delete: 3
✓ Task 3 deleted!
💾 Tasks saved!
```

## 🛠️ Technical Details

### Built With

- **Rust** - Programming language
- **serde** - Serialization framework
- **serde_json** - JSON support
- **chrono** - Date and time handling
- **colored** - Terminal colors

### Data Storage

Tasks are automatically saved to `todos.json` in the project directory. The file is created automatically on first run.

### Project Structure

```
todo_app/
├── Cargo.toml          # Project dependencies
├── src/
│   └── main.rs         # Main application code
├── todos.json          # Auto-generated task storage
└── README.md           # This file
```

## 🎯 Learning Objectives

This project demonstrates:

- **Structs and Methods** - Organizing data with `Todo` and `TodoList`
- **File I/O** - Reading and writing JSON files
- **Serialization** - Using Serde for data persistence
- **Date Handling** - Working with dates using Chrono
- **Error Handling** - Using `Result`, `Option`, and `match`
- **Pattern Matching** - Command handling with `match`
- **Traits** - Using the `colored` crate's color traits
- **Sorting** - Custom sorting logic for tasks
- **Cargo** - Dependency management and project structure

## 🎨 Color Legend

- **🔴 Red** - Error messages and overdue tasks
- **🟡 Yellow** - Warnings and tasks due within 3 days
- **🟢 Green** - Success messages and completed tasks
- **🔵 Blue/Cyan** - Headers and UI elements
- **⚫ Dimmed** - Completed task text

## 📝 Future Enhancements

Some ideas for extending this app:

- [ ] Add task priorities (high, medium, low)
- [ ] Filter tasks by completion status
- [ ] Add task categories/tags
- [ ] Export tasks to different formats
- [ ] Add recurring tasks
- [ ] Search functionality
- [ ] Edit existing tasks

## 🤝 Contributing

Feel free to fork this project and add your own features! This is a learning project designed to help understand Rust fundamentals.

## 📄 License

This project is open source and available for learning purposes.

## 🙏 Acknowledgments

Built as a learning project to explore Rust programming concepts including:
- Ownership and borrowing
- Pattern matching
- Error handling
- Working with external crates
- Building CLI applications

---

**Happy task managing! 🎉**
