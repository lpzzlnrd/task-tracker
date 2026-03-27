use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

const STATUS_TODO: &str = "todo";
const STATUS_IN_PROGRESS: &str = "in-progress";
const STATUS_DONE: &str = "done";

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum Status {
    Todo,
    InProgress,
    Done,
}

impl Status {
    fn as_str(&self) -> &'static str {
        match self {
            Status::Todo => STATUS_TODO,
            Status::InProgress => STATUS_IN_PROGRESS,
            Status::Done => STATUS_DONE,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Task {
    id: u32,
    description: String,
    status: Status,
    created_at: String,
    updated_at: String,
}

fn tasks_file_path() -> Result<PathBuf, String> {
    let cwd = env::current_dir().map_err(|e| format!("Failed to resolve current directory: {e}"))?;
    Ok(cwd.join("tasks.json"))
}

fn ensure_tasks_file() -> Result<PathBuf, String> {
    let path = tasks_file_path()?;
    if !path.exists() {
        fs::write(&path, "[]\n").map_err(|e| format!("Failed to create tasks.json: {e}"))?;
    }
    Ok(path)
}

fn read_tasks() -> Result<Vec<Task>, String> {
    let path = ensure_tasks_file()?;
    let raw = fs::read_to_string(&path).map_err(|e| format!("Failed to read tasks.json: {e}"))?;
    let value: serde_json::Value =
        serde_json::from_str(&raw).map_err(|e| format!("Invalid JSON in tasks.json: {e}"))?;
    let array = value
        .as_array()
        .ok_or_else(|| "Invalid tasks file format. Expected a JSON array.".to_string())?;
    let tasks: Vec<Task> = serde_json::from_value(serde_json::Value::Array(array.clone()))
        .map_err(|e| format!("Invalid task data in tasks.json: {e}"))?;
    Ok(tasks)
}

fn write_tasks(tasks: &[Task]) -> Result<(), String> {
    let path = ensure_tasks_file()?;
    let json = serde_json::to_string_pretty(tasks)
        .map_err(|e| format!("Failed to serialize tasks: {e}"))?;
    fs::write(path, format!("{json}\n")).map_err(|e| format!("Failed to write tasks.json: {e}"))?;
    Ok(())
}

fn now() -> String {
    Utc::now().to_rfc3339()
}

fn parse_id(raw: &str) -> Result<u32, String> {
    let id: u32 = raw
        .parse()
        .map_err(|_| "Invalid id: must be a positive integer.".to_string())?;
    if id == 0 {
        return Err("Invalid id: must be a positive integer.".to_string());
    }
    Ok(id)
}

fn print_help() {
    println!("Task Tracker CLI");
    println!("Usage:");
    println!("  task-cli add \"Task description\"");
    println!("  task-cli update <id> \"New description\"");
    println!("  task-cli delete <id>");
    println!("  task-cli mark-in-progress <id>");
    println!("  task-cli mark-done <id>");
    println!("  task-cli list");
    println!("  task-cli list done");
    println!("  task-cli list todo");
    println!("  task-cli list \"in-progress\"");
}

fn print_tasks(tasks: &[Task]) {
    if tasks.is_empty() {
        println!("No tasks found.");
        return;
    }
    for task in tasks {
        println!(
            "[{id}] {desc} | status={status} | createdAt={created} | updatedAt={updated}",
            id = task.id,
            desc = task.description,
            status = task.status.as_str(),
            created = task.created_at,
            updated = task.updated_at
        );
    }
}

fn find_task_mut<'a>(tasks: &'a mut [Task], id: u32) -> Result<&'a mut Task, String> {
    tasks
        .iter_mut()
        .find(|task| task.id == id)
        .ok_or_else(|| format!("Task with id {id} does not exist."))
}

fn handle_add(args: &[String]) -> Result<(), String> {
    let description = args.join(" ").trim().to_string();
    if description.is_empty() {
        return Err("Description is required for add.".to_string());
    }
    let mut tasks = read_tasks()?;
    let next_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    let timestamp = now();
    let task = Task {
        id: next_id,
        description,
        status: Status::Todo,
        created_at: timestamp.clone(),
        updated_at: timestamp,
    };
    tasks.push(task);
    write_tasks(&tasks)?;
    println!("Task added successfully (id: {next_id})");
    Ok(())
}

fn handle_update(args: &[String]) -> Result<(), String> {
    if args.len() < 2 {
        return Err("Usage: update <id> \"New description\"".to_string());
    }
    let id = parse_id(&args[0])?;
    let description = args[1..].join(" ").trim().to_string();
    if description.is_empty() {
        return Err("New description is required for update.".to_string());
    }
    let mut tasks = read_tasks()?;
    let task = find_task_mut(&mut tasks, id)?;
    task.description = description;
    task.updated_at = now();
    write_tasks(&tasks)?;
    println!("Task {id} updated successfully");
    Ok(())
}

fn handle_delete(args: &[String]) -> Result<(), String> {
    if args.is_empty() {
        return Err("Usage: delete <id>".to_string());
    }
    let id = parse_id(&args[0])?;
    let tasks = read_tasks()?;
    if !tasks.iter().any(|task| task.id == id) {
        return Err(format!("Task with id {id} does not exist."));
    }
    let filtered: Vec<Task> = tasks.into_iter().filter(|task| task.id != id).collect();
    write_tasks(&filtered)?;
    println!("Task {id} deleted successfully");
    Ok(())
}

fn handle_mark(args: &[String], status: Status) -> Result<(), String> {
    if args.is_empty() {
        return Err("Usage: mark-<status> <id>".to_string());
    }
    let id = parse_id(&args[0])?;
    let mut tasks = read_tasks()?;
    let status_label = {
        let task = find_task_mut(&mut tasks, id)?;
        task.status = status;
        task.updated_at = now();
        task.status.as_str().to_string()
    };
    write_tasks(&tasks)?;
    println!("Task {id} marked as {status_label}");
    Ok(())
}

fn handle_list(args: &[String]) -> Result<(), String> {
    let tasks = read_tasks()?;
    if args.is_empty() {
        print_tasks(&tasks);
        return Ok(());
    }
    let requested = args[0].as_str();
    let filtered: Vec<Task> = match requested {
        STATUS_TODO => tasks.into_iter().filter(|t| matches!(t.status, Status::Todo)).collect(),
        STATUS_IN_PROGRESS => tasks
            .into_iter()
            .filter(|t| matches!(t.status, Status::InProgress))
            .collect(),
        STATUS_DONE => tasks.into_iter().filter(|t| matches!(t.status, Status::Done)).collect(),
        _ => {
            return Err("List filter must be one of: \"todo\", \"in-progress\", \"done\".".to_string())
        }
    };
    print_tasks(&filtered);
    Ok(())
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let command = args.next();
    let remaining: Vec<String> = args.collect();

    match command.as_deref() {
        None | Some("help") | Some("--help") => {
            print_help();
            Ok(())
        }
        Some("add") => handle_add(&remaining),
        Some("update") => handle_update(&remaining),
        Some("delete") => handle_delete(&remaining),
        Some("mark-in-progress") => handle_mark(&remaining, Status::InProgress),
        Some("mark-done") => handle_mark(&remaining, Status::Done),
        Some("list") => handle_list(&remaining),
        Some(other) => Err(format!("Unknown command: {other}")),
    }
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}
