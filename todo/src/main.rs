use clap::{App, Arg};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: u32,
    content: String,
    completed: bool,
}

// Save tasks to a file
fn save_tasks(tasks: &Vec<Task>) -> Result<(), std::io::Error> {
    let serialized = serde_json::to_string(&tasks)?;
    std::fs::write("task.json", serialized)?;
    Ok(())
}

// Load tasks from a file
fn load_tasks() -> Result<Vec<Task>, std::io::Error> {
    if let Ok(data) = std::fs::read_to_string("task.json") {
        let tasks = serde_json::from_str(&data)?;
        Ok(tasks)
    } else {
        Ok(Vec::new())
    }
}

fn add_task(tasks: &mut Vec<Task>, content: &str) {
    let new_task = Task {
        id: tasks.len() as u32 + 1,
        content: content.to_string(),
        completed: false,
    };
    tasks.push(new_task);
}

// Function to mark a task as completed
fn complete_task(tasks: &mut Vec<Task>, id: u32) {
    for task in tasks.iter_mut() {
        if task.id == id {
            if task.completed==false
            {
                task.completed = true;
            }
            else{
                task.completed = false;
            }
        }
    }
}

// Function to list all tasks
fn list_tasks(tasks: &Vec<Task>) {
    for task in tasks.iter() {
        let status = if task.completed { "Completed" } else { "Not Completed" };
        println!("ID: {}, Content: {}, Status: {}", task.id, task.content, status);
    }
}

// function to del task
fn del_tasks(tasks: &mut Vec<Task>, id: u32){
    for task in tasks.iter(){
        if task.id == id
        {
            
        }
    }
}

fn main() {
    let mut tasks: Vec<Task> = load_tasks().unwrap_or_else(|_| Vec::new());

    let matches = App::new("ToDo CLI")
        .version("1.0")
        .author("Ayush Bansal")
        .about("Manage your tasks")
        .arg(Arg::with_name("add")
            .short("a")
            .long("add")
            .value_name("TASK")
            .help("Adds a task to your to-do list")
            .takes_value(true))
        .arg(Arg::with_name("status")
            .short("s")
            .long("status")
            .value_name("ID")
            .help("Mark a task as completed")
            .takes_value(true))
        .arg(Arg::with_name("list")
            .short("l")
            .long("list")
            .help("List all tasks in your to-do list"))
        .get_matches();

    if let Some(task) = matches.value_of("add") {
        add_task(&mut tasks, task);
        save_tasks(&tasks).expect("Error saving tasks");
    }

    if let Some(id_str) = matches.value_of("status") {
        if let Ok(id) = id_str.parse::<u32>() {
            complete_task(&mut tasks, id);
            save_tasks(&tasks).expect("Error saving tasks");
        } else {
            eprintln!("Invalid task ID. Please provide a valid integer.");
        }
    }

    if matches.is_present("list") {
        list_tasks(&tasks);
    }
}
