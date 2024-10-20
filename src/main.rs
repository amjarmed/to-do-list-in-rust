
use clap::{Parser, Subcommand};
use std::fs::OpenOptions;
use std::io::{self, Read, Write};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(name = "ToDo CLI")]
#[command(about = "A simple To-Do List Command Line Tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { task: String },
    List,
    Done { index: usize },
    Remove { index: usize },
}



fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { task } => add_task(task),
        Commands::List => list_tasks(),
        Commands::Done { index } => mark_done(*index),
        Commands::Remove { index } => remove_task(*index),
    }
}

#[derive(Serialize, Deserialize)]
struct Task {
    description: String,
    done: bool,
}

const FILE_NAME: &str = "todo.json";

fn add_task(description: &str) {
    let mut tasks = load_tasks();
    tasks.push(Task {
        description: description.to_string(),
        done: false,
    });
    save_tasks(&tasks);
    println!("Added task: {}", description);
}

fn list_tasks() {
    let tasks = load_tasks();
    for (i, task) in tasks.iter().enumerate() {
        let status = if task.done { "[x]" } else { "[ ]" };
        println!("{}: {} {}", i + 1, status, task.description);
    }
}

fn mark_done(index: usize) {
    let mut tasks = load_tasks();
    if index == 0 || index > tasks.len() {
        println!("Invalid task index");
        return;
    }
    tasks[index - 1].done = true;
    save_tasks(&tasks);
    println!("Task {} marked as done", index);
}

fn remove_task(index: usize) {
    let mut tasks = load_tasks();
    if index == 0 || index > tasks.len() {
        println!("Invalid task index");
        return;
    }
    tasks.remove(index - 1);
    save_tasks(&tasks);
    println!("Removed task {}", index);
}

fn load_tasks() -> Vec<Task> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(FILE_NAME)
        .expect("Unable to open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");

    if contents.is_empty() {
        Vec::new()
    } else {
        serde_json::from_str(&contents).expect("Unable to parse JSON")
    }
}

fn save_tasks(tasks: &Vec<Task>) {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(FILE_NAME)
        .expect("Unable to open file");
    let json = serde_json::to_string(tasks).expect("Unable to serialize tasks");
    file.write_all(json.as_bytes()).expect("Unable to write to file");
}