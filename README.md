# task-tracker (Rust)

CLI para gestionar tareas, implementado en Rust, basado en los requisitos del proyecto task-tracker de roadmap.sh.

## funcionalidades cubiertas

- Registro de multiples usuarios (personas)
- Listado de usuarios registrados
- Agregar, actualizar y eliminar tareas
- Asignar tareas a usuarios
- Marcar tareas como `in-progress` o `done`
- Listar todas las tareas
- Filtrar tareas por estado: `todo`, `in-progress`, `done`
- Listar tareas por usuario (con filtro opcional de estado)
- Persistencia en archivo JSON (`tasks.json`) en el directorio actual
- Persistencia de usuarios en archivo JSON (`users.json`) en el directorio actual
- Creacion automatica de `tasks.json` si no existe
- Creacion automatica de `users.json` si no existe
- Manejo de errores con mensajes claros y codigo de salida `1`

## requisitos

- Rust (toolchain estable)
- Cargo

Puedes verificarlo con:

```bash
cargo --version
rustc --version
```

## ejecucion

Desde la raiz del proyecto:

```bash
cargo run -- --help
```

Nota: el doble guion `--` separa opciones de Cargo de los argumentos del programa.

## uso de comandos

```bash
cargo run -- add-user "Leo"
cargo run -- add-user "Ana"
cargo run -- list-users

cargo run -- add 1 "Comprar viveres"
cargo run -- add 2 "Preparar presentacion"
cargo run -- update 1 "Comprar viveres y cocinar"
cargo run -- delete 1
cargo run -- mark-in-progress 2
cargo run -- mark-done 2
cargo run -- assign 2 1
cargo run -- list
cargo run -- list done
cargo run -- list todo
cargo run -- list "in-progress"
cargo run -- list-user 1
cargo run -- list-user 2 done
```

Tambien puedes ejecutar el binario compilado directamente:

```bash
./target/debug/task-cli.exe add "Comprar viveres"
./target/debug/task-cli.exe list
```

En PowerShell (Windows):

```powershell
.\target\debug\task-cli.exe add-user "Leo"
.\target\debug\task-cli.exe add 1 "Comprar viveres"
.\target\debug\task-cli.exe list
```

## formato de `tasks.json`

Cada tarea se guarda con esta estructura:

- `id`: numero entero positivo
- `description`: texto de la tarea
- `userId`: id del usuario propietario de la tarea
- `status`: `todo` | `in-progress` | `done`
- `createdAt`: fecha/hora en formato ISO 8601
- `updatedAt`: fecha/hora en formato ISO 8601

Ejemplo:

```json
[
	{
		"id": 1,
		"description": "Estudiar Rust",
		"userId": 1,
		"status": "todo",
		"createdAt": "2026-03-27T21:00:00Z",
		"updatedAt": "2026-03-27T21:00:00Z"
	}
]
```

## formato de `users.json`

Cada usuario se guarda con esta estructura:

- `id`: numero entero positivo
- `name`: nombre de la persona
- `createdAt`: fecha/hora en formato ISO 8601

Ejemplo:

```json
[
	{
		"id": 1,
		"name": "Leo",
		"createdAt": "2026-03-27T21:00:00Z"
	},
	{
		"id": 2,
		"name": "Ana",
		"createdAt": "2026-03-27T21:01:00Z"
	}
]
```

## pruebas

Pruebas automaticas:

```bash
cargo test
```

Prueba manual recomendada:

```bash
cargo run -- add-user "Usuario Demo"
cargo run -- add 1 "Tarea de prueba"
cargo run -- list
cargo run -- mark-done 1
cargo run -- list done
cargo run -- delete 1
```

## errores comunes

- Si ejecutas `task-cli ...` y sale "command not found", usa `cargo run -- ...` o el binario local en `target/debug`.
- Si pasas opciones de ayuda a Cargo en lugar del programa, usa `cargo run -- --help`.
