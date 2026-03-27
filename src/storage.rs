use chrono::Utc;
use std::env;
use std::fs;
use std::path::PathBuf;

use crate::models::{Task, User};

/// Construye la ruta absoluta al archivo tasks.json en el directorio actual.
fn tasks_file_path() -> Result<PathBuf, String> {
    let cwd = env::current_dir().map_err(|e| format!("Failed to resolve current directory: {e}"))?;
    Ok(cwd.join("tasks.json"))
}

/// Construye la ruta absoluta al archivo users.json en el directorio actual.
fn users_file_path() -> Result<PathBuf, String> {
    let cwd = env::current_dir().map_err(|e| format!("Failed to resolve current directory: {e}"))?;
    Ok(cwd.join("users.json"))
}

/// Garantiza que tasks.json exista; si no existe, lo crea como arreglo vacio.
fn ensure_tasks_file() -> Result<PathBuf, String> {
    let path = tasks_file_path()?;
    if !path.exists() {
        fs::write(&path, "[]\n").map_err(|e| format!("Failed to create tasks.json: {e}"))?;
    }
    Ok(path)
}

/// Garantiza que users.json exista; si no existe, lo crea como arreglo vacio.
fn ensure_users_file() -> Result<PathBuf, String> {
    let path = users_file_path()?;
    if !path.exists() {
        fs::write(&path, "[]\n").map_err(|e| format!("Failed to create users.json: {e}"))?;
    }
    Ok(path)
}

/// Lee tareas desde tasks.json y valida que el contenido sea un arreglo valido.
pub fn read_tasks() -> Result<Vec<Task>, String> {
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

/// Lee usuarios desde users.json y valida que el contenido sea un arreglo valido.
pub fn read_users() -> Result<Vec<User>, String> {
    let path = ensure_users_file()?;
    let raw = fs::read_to_string(&path).map_err(|e| format!("Failed to read users.json: {e}"))?;
    let value: serde_json::Value =
        serde_json::from_str(&raw).map_err(|e| format!("Invalid JSON in users.json: {e}"))?;
    let array = value
        .as_array()
        .ok_or_else(|| "Invalid users file format. Expected a JSON array.".to_string())?;
    let users: Vec<User> = serde_json::from_value(serde_json::Value::Array(array.clone()))
        .map_err(|e| format!("Invalid user data in users.json: {e}"))?;
    Ok(users)
}

/// Escribe la lista completa de tareas en tasks.json con formato legible.
pub fn write_tasks(tasks: &[Task]) -> Result<(), String> {
    let path = ensure_tasks_file()?;
    let json = serde_json::to_string_pretty(tasks)
        .map_err(|e| format!("Failed to serialize tasks: {e}"))?;
    fs::write(path, format!("{json}\n")).map_err(|e| format!("Failed to write tasks.json: {e}"))?;
    Ok(())
}

/// Escribe la lista completa de usuarios en users.json con formato legible.
pub fn write_users(users: &[User]) -> Result<(), String> {
    let path = ensure_users_file()?;
    let json = serde_json::to_string_pretty(users)
        .map_err(|e| format!("Failed to serialize users: {e}"))?;
    fs::write(path, format!("{json}\n")).map_err(|e| format!("Failed to write users.json: {e}"))?;
    Ok(())
}

/// Genera timestamp UTC en formato RFC 3339.
pub fn now() -> String {
    Utc::now().to_rfc3339()
}

/// Convierte y valida un id recibido por CLI.
pub fn parse_id(raw: &str) -> Result<u32, String> {
    let id: u32 = raw
        .parse()
        .map_err(|_| "Invalid id: must be a positive integer.".to_string())?;
    if id == 0 {
        return Err("Invalid id: must be a positive integer.".to_string());
    }
    Ok(id)
}