use std::collections::HashMap;

use crate::models::{Task, User};

/// Muestra ayuda de uso y comandos disponibles.
pub fn print_help() {
    println!("Task Tracker CLI de Leo");
    println!("Usos Disponibles:");
    println!("  task-cli add-user \"User name\"");
    println!("  task-cli list-users");
    println!("  task-cli add <user_id> \"Task description\"");
    println!("  task-cli update <id> \"New description\"");
    println!("  task-cli delete <id>");
    println!("  task-cli mark-in-progress <id>");
    println!("  task-cli mark-done <id>");
    println!("  task-cli assign <task_id> <user_id>");
    println!("  task-cli list");
    println!("  task-cli list done");
    println!("  task-cli list todo");
    println!("  task-cli list \"in-progress\"");
    println!("  task-cli list-user <user_id>");
    println!("  task-cli list-user <user_id> done");
}

/// Imprime tareas en formato tabulado con columnas dinamicas.
pub fn print_tasks(tasks: &[Task], users: &[User]) {
    if tasks.is_empty() {
        println!("No tasks found.");
        return;
    }

    let user_names: HashMap<u32, &str> = users.iter().map(|u| (u.id, u.name.as_str())).collect();

    let id_header = "ID";
    let desc_header = "DESCRIPTION";
    let owner_header = "OWNER";
    let user_id_header = "USER_ID";
    let status_header = "STATUS";
    let created_header = "CREATED_AT";
    let updated_header = "UPDATED_AT";

    let mut id_width = id_header.len();
    let mut desc_width = desc_header.len();
    let mut owner_width = owner_header.len();
    let mut user_id_width = user_id_header.len();
    let mut status_width = status_header.len();
    let mut created_width = created_header.len();
    let mut updated_width = updated_header.len();

    for task in tasks {
        let owner = user_names.get(&task.user_id).copied().unwrap_or("unassigned");
        id_width = id_width.max(task.id.to_string().len());
        desc_width = desc_width.max(task.description.len());
        owner_width = owner_width.max(owner.len());
        user_id_width = user_id_width.max(task.user_id.to_string().len());
        status_width = status_width.max(task.status.as_str().len());
        created_width = created_width.max(task.created_at.len());
        updated_width = updated_width.max(task.updated_at.len());
    }

    let separator = format!(
        "{:-<id_width$}-+-{:-<desc_width$}-+-{:-<owner_width$}-+-{:-<user_id_width$}-+-{:-<status_width$}-+-{:-<created_width$}-+-{:-<updated_width$}",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        id_width = id_width,
        desc_width = desc_width,
        owner_width = owner_width,
        user_id_width = user_id_width,
        status_width = status_width,
        created_width = created_width,
        updated_width = updated_width
    );

    println!(
        "{:<id_width$} | {:<desc_width$} | {:<owner_width$} | {:<user_id_width$} | {:<status_width$} | {:<created_width$} | {:<updated_width$}",
        id_header,
        desc_header,
        owner_header,
        user_id_header,
        status_header,
        created_header,
        updated_header,
        id_width = id_width,
        desc_width = desc_width,
        owner_width = owner_width,
        user_id_width = user_id_width,
        status_width = status_width,
        created_width = created_width,
        updated_width = updated_width
    );
    println!("{separator}");

    for task in tasks {
        let owner = user_names.get(&task.user_id).copied().unwrap_or("unassigned");
        println!(
            "{:<id_width$} | {:<desc_width$} | {:<owner_width$} | {:<user_id_width$} | {:<status_width$} | {:<created_width$} | {:<updated_width$}",
            task.id,
            task.description,
            owner,
            task.user_id,
            task.status.as_str(),
            task.created_at,
            task.updated_at,
            id_width = id_width,
            desc_width = desc_width,
            owner_width = owner_width,
            user_id_width = user_id_width,
            status_width = status_width,
            created_width = created_width,
            updated_width = updated_width
        );
    }
}

/// Imprime usuarios en formato tabulado.
pub fn print_users(users: &[User]) {
    if users.is_empty() {
        println!("No users found. Use add-user to register one.");
        return;
    }

    let id_header = "ID";
    let name_header = "NAME";
    let created_header = "CREATED_AT";

    let mut id_width = id_header.len();
    let mut name_width = name_header.len();
    let mut created_width = created_header.len();

    for user in users {
        id_width = id_width.max(user.id.to_string().len());
        name_width = name_width.max(user.name.len());
        created_width = created_width.max(user.created_at.len());
    }

    let separator = format!(
        "{:-<id_width$}-+-{:-<name_width$}-+-{:-<created_width$}",
        "",
        "",
        "",
        id_width = id_width,
        name_width = name_width,
        created_width = created_width
    );

    println!(
        "{:<id_width$} | {:<name_width$} | {:<created_width$}",
        id_header,
        name_header,
        created_header,
        id_width = id_width,
        name_width = name_width,
        created_width = created_width
    );
    println!("{separator}");

    for user in users {
        println!(
            "{:<id_width$} | {:<name_width$} | {:<created_width$}",
            user.id,
            user.name,
            user.created_at,
            id_width = id_width,
            name_width = name_width,
            created_width = created_width
        );
    }
}