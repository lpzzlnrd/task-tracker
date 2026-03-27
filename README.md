# task-tracker

Simple CLI task tracker project based on roadmap.sh task-tracker requirements.

## requirements covered

- Add, update, and delete tasks
- Mark task as in-progress or done
- List all tasks
- List only done tasks
- List only todo tasks
- List only in-progress tasks
- Persist tasks to JSON file in current directory
- Create JSON file automatically if it does not exist
- Handle invalid input with clear error messages

## runtime

- Node.js (no external libraries used)

## file storage

- The app stores data in `tasks.json` in the current working directory.
- If `tasks.json` is missing, the app creates it with an empty array.

## usage

Run commands with positional arguments:

```bash
node task-cli.js add "Buy groceries"
node task-cli.js update 1 "Buy groceries and cook dinner"
node task-cli.js delete 1
node task-cli.js mark-in-progress 2
node task-cli.js mark-done 2
node task-cli.js list
node task-cli.js list done
node task-cli.js list todo
node task-cli.js list "in-progress"
```

## task json shape

Each task object has:

- `id` (number)
- `description` (string)
- `status` (`todo`, `in-progress`, `done`)
- `createdAt` (ISO datetime string)
- `updatedAt` (ISO datetime string)

## notes

- Use quotes for descriptions with spaces.
- Errors return exit code `1`.
- All implementation comments are inline in `task-cli.js`.
