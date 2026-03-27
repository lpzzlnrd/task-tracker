use crate::constants::{STATUS_DONE, STATUS_IN_PROGRESS, STATUS_TODO};
use crate::models::{Status, Task, User};
use crate::printing::{print_tasks, print_users};
use crate::storage::{now, parse_id, read_tasks, read_users, write_tasks, write_users};

/// Busca una tarea por id y devuelve referencia mutable para poder editarla.
fn find_task_mut(tasks: &mut [Task], id: u32) -> Result<&mut Task, String> {
    tasks
        .iter_mut()
        .find(|task| task.id == id)
        .ok_or_else(|| format!("Task with id {id} does not exist."))
}

/// Busca un usuario por id.
fn find_user(users: &[User], id: u32) -> Result<&User, String> {
    users
        .iter()
        .find(|user| user.id == id)
        .ok_or_else(|| format!("User with id {id} does not exist."))
}

/// Convierte string de estado a enum para filtros opcionales.
fn parse_status_filter(raw: &str) -> Result<Status, String> {
    match raw {
        STATUS_TODO => Ok(Status::Todo),
        STATUS_IN_PROGRESS => Ok(Status::InProgress),
        STATUS_DONE => Ok(Status::Done),
        _ => Err("List filter must be one of: \"todo\", \"in-progress\", \"done\".".to_string()),
    }
}

/// Registra un nuevo usuario para asignar tareas.
pub fn handle_add_user(args: &[String]) -> Result<(), String> {
    let name = args.join(" ").trim().to_string();
    if name.is_empty() {
        return Err("Usage: add-user \"User name\"".to_string());
    }
    let mut users = read_users()?;
    if users.iter().any(|u| u.name.eq_ignore_ascii_case(&name)) {
        return Err(format!("User \"{name}\" already exists."));
    }
    let next_id = users.iter().map(|u| u.id).max().unwrap_or(0) + 1;
    users.push(User {
        id: next_id,
        name,
        created_at: now(),
    });
    write_users(&users)?;
    println!("User added successfully (id: {next_id})");
    Ok(())
}

/// Lista los usuarios registrados.
pub fn handle_list_users() -> Result<(), String> {
    let users = read_users()?;
    print_users(&users);
    Ok(())
}

/// Crea una nueva tarea con estado inicial todo.
pub fn handle_add(args: &[String]) -> Result<(), String> {
    if args.len() < 2 {
        return Err("Usage: add <user_id> \"Task description\"".to_string());
    }
    let user_id = parse_id(&args[0])?;
    let description = args[1..].join(" ").trim().to_string();
    if description.is_empty() {
        return Err("Task description is required for add.".to_string());
    }
    let users = read_users()?;
    find_user(&users, user_id)?;
    let mut tasks = read_tasks()?;
    let next_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    let timestamp = now();
    let task = Task {
        id: next_id,
        description,
        user_id,
        status: Status::Todo,
        created_at: timestamp.clone(),
        updated_at: timestamp,
    };
    tasks.push(task);
    write_tasks(&tasks)?;
    println!("Task added successfully (id: {next_id})");
    Ok(())
}

/// Actualiza la descripcion de una tarea existente.
pub fn handle_update(args: &[String]) -> Result<(), String> {
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

/// Elimina una tarea por id.
pub fn handle_delete(args: &[String]) -> Result<(), String> {
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

/// Cambia el estado de una tarea a in-progress o done.
pub fn handle_mark(args: &[String], status: Status) -> Result<(), String> {
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

/// Reasigna una tarea existente a otro usuario.
pub fn handle_assign(args: &[String]) -> Result<(), String> {
    if args.len() < 2 {
        return Err("Usage: assign <task_id> <user_id>".to_string());
    }
    let task_id = parse_id(&args[0])?;
    let user_id = parse_id(&args[1])?;
    let users = read_users()?;
    find_user(&users, user_id)?;

    let mut tasks = read_tasks()?;
    let task = find_task_mut(&mut tasks, task_id)?;
    task.user_id = user_id;
    task.updated_at = now();
    write_tasks(&tasks)?;
    println!("Task {task_id} assigned to user {user_id}");
    Ok(())
}

/// Lista todas las tareas o solo las que coinciden con un estado.
pub fn handle_list(args: &[String]) -> Result<(), String> {
    let tasks = read_tasks()?;
    let users = read_users()?;
    if args.is_empty() {
        print_tasks(&tasks, &users);
        return Ok(());
    }
    let requested = parse_status_filter(args[0].as_str())?;
    let filtered: Vec<Task> = tasks
        .into_iter()
        .filter(|t| match requested {
            Status::Todo => matches!(t.status, Status::Todo),
            Status::InProgress => matches!(t.status, Status::InProgress),
            Status::Done => matches!(t.status, Status::Done),
        })
        .collect();
    print_tasks(&filtered, &users);
    Ok(())
}

/// Lista tareas de un usuario especifico, con filtro de estado opcional.
pub fn handle_list_user(args: &[String]) -> Result<(), String> {
    if args.is_empty() {
        return Err("Usage: list-user <user_id> [todo|in-progress|done]".to_string());
    }
    let user_id = parse_id(&args[0])?;
    let users = read_users()?;
    find_user(&users, user_id)?;

    let mut filtered: Vec<Task> = read_tasks()?
        .into_iter()
        .filter(|t| t.user_id == user_id)
        .collect();

    if args.len() > 1 {
        let requested = parse_status_filter(args[1].as_str())?;
        filtered = filtered
            .into_iter()
            .filter(|t| match requested {
                Status::Todo => matches!(t.status, Status::Todo),
                Status::InProgress => matches!(t.status, Status::InProgress),
                Status::Done => matches!(t.status, Status::Done),
            })
            .collect();
    }

    print_tasks(&filtered, &users);
    Ok(())
}