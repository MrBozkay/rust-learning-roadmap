use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    fn load(filename: &str) -> io::Result<Self> {
        if !Path::new(filename).exists() {
            return Ok(Self::new());
        }
        let content = fs::read_to_string(filename)?;
        let list = serde_json::from_str(&content)?;
        Ok(list)
    }

    fn save(&self, filename: &str) -> io::Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        let mut file = File::create(filename)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    fn add(&mut self, description: String) {
        let id = if let Some(last) = self.tasks.last() {
            last.id + 1
        } else {
            1
        };
        let task = Task {
            id,
            description,
            completed: false,
        };
        self.tasks.push(task);
        println!("Task added: {}", id);
    }

    fn list(&self) {
        if self.tasks.is_empty() {
            println!("No tasks found.");
            return;
        }
        for task in &self.tasks {
            let status = if task.completed { "[x]" } else { "[ ]" };
            println!("{} {} - {}", task.id, status, task.description);
        }
    }

    fn complete(&mut self, id: usize) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            return true;
        }
        false
    }

    fn delete(&mut self, id: usize) -> bool {
        if let Some(pos) = self.tasks.iter().position(|t| t.id == id) {
            self.tasks.remove(pos);
            return true;
        }
        false
    }
}

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple CLI to-do list", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add {
        /// The task description
        description: String,
    },
    /// List all tasks
    List,
    /// Complete a task by ID
    Complete {
        /// The ID of the task to complete
        id: usize,
    },
    /// Delete a task by ID
    Delete {
        /// The ID of the task to delete
        id: usize,
    },
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let filename = "tasks.json";
    let mut todo_list = TodoList::load(filename)?;

    match &cli.command {
        Commands::Add { description } => {
            todo_list.add(description.clone());
            todo_list.save(filename)?;
        }
        Commands::List => {
            todo_list.list();
        }
        Commands::Complete { id } => {
            if todo_list.complete(*id) {
                println!("Task {} completed.", id);
                todo_list.save(filename)?;
            } else {
                println!("Task {} not found.", id);
            }
        }
        Commands::Delete { id } => {
            if todo_list.delete(*id) {
                println!("Task {} deleted.", id);
                todo_list.save(filename)?;
            } else {
                println!("Task {} not found.", id);
            }
        }
    }

    Ok(())
}
