mod constants;
mod handlers;
mod models;
mod printing;
mod storage;

use std::env;

use handlers::{
    handle_add, handle_add_user, handle_assign, handle_delete, handle_list, handle_list_user,
    handle_list_users, handle_mark, handle_update,
};
use models::Status;
use printing::print_help;

/// Punto de entrada de alto nivel: interpreta comando y delega al handler correcto.
fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let command = args.next();
    let remaining: Vec<String> = args.collect();

    match command.as_deref() {
        None | Some("help") | Some("--help") => {
            print_help();
            Ok(())
        }
        Some("add-user") => handle_add_user(&remaining),
        Some("list-users") => handle_list_users(),
        Some("add") => handle_add(&remaining),
        Some("update") => handle_update(&remaining),
        Some("delete") => handle_delete(&remaining),
        Some("mark-in-progress") => handle_mark(&remaining, Status::InProgress),
        Some("mark-done") => handle_mark(&remaining, Status::Done),
        Some("assign") => handle_assign(&remaining),
        Some("list") => handle_list(&remaining),
        Some("list-user") => handle_list_user(&remaining),
        Some(other) => Err(format!("Unknown command: {other}")),
    }
}

/// Punto de entrada del programa: ejecuta la app y retorna codigo de error en fallos.
fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}
