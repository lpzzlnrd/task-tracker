#!/usr/bin/env node

const fs = require("fs");
const path = require("path");

// This is the JSON file that stores all tasks in the current working directory.
const TASKS_FILE = path.join(process.cwd(), "tasks.json");

// These are the allowed status values for each task.
const STATUS = {
  TODO: "todo",
  IN_PROGRESS: "in-progress",
  DONE: "done",
};

// This function ensures the JSON file exists and contains a valid array.
function ensureTasksFile() {
  if (!fs.existsSync(TASKS_FILE)) {
    fs.writeFileSync(TASKS_FILE, "[]\n", "utf8");
  }
}

// This function reads tasks safely from disk.
function readTasks() {
  ensureTasksFile();
  const raw = fs.readFileSync(TASKS_FILE, "utf8");
  const parsed = JSON.parse(raw);
  if (!Array.isArray(parsed)) {
    throw new Error("Invalid tasks file format. Expected a JSON array.");
  }
  return parsed;
}

// This function writes tasks back to disk in a readable JSON format.
function writeTasks(tasks) {
  fs.writeFileSync(TASKS_FILE, `${JSON.stringify(tasks, null, 2)}\n`, "utf8");
}

// This function creates a timestamp string in ISO format.
function now() {
  return new Date().toISOString();
}

// This function parses and validates a task id argument.
function parseId(value) {
  const id = Number(value);
  if (!Number.isInteger(id) || id <= 0) {
    throw new Error("Task id must be a positive integer.");
  }
  return id;
}

// This function prints all supported commands.
function printHelp() {
  console.log("Task Tracker CLI");
  console.log("Usage:");
  console.log('  node task-cli.js add "Task description"');
  console.log('  node task-cli.js update <id> "New description"');
  console.log("  node task-cli.js delete <id>");
  console.log("  node task-cli.js mark-in-progress <id>");
  console.log("  node task-cli.js mark-done <id>");
  console.log("  node task-cli.js list");
  console.log("  node task-cli.js list done");
  console.log("  node task-cli.js list todo");
  console.log('  node task-cli.js list "in-progress"');
}

// This function formats and prints tasks as plain text.
function printTasks(tasks) {
  if (tasks.length === 0) {
    console.log("No tasks found.");
    return;
  }
  tasks.forEach((task) => {
    console.log(
      `[${task.id}] ${task.description} | status=${task.status} | createdAt=${task.createdAt} | updatedAt=${task.updatedAt}`
    );
  });
}

// This function finds a task by id and throws if the task does not exist.
function findTaskById(tasks, id) {
  const task = tasks.find((item) => item.id === id);
  if (!task) {
    throw new Error(`Task with id ${id} does not exist.`);
  }
  return task;
}

// This function handles the add command.
function handleAdd(args) {
  const description = args[0];
  if (!description || description.trim() === "") {
    throw new Error("Description is required for add.");
  }
  const tasks = readTasks();
  const nextId = tasks.reduce((max, t) => Math.max(max, t.id), 0) + 1;
  const task = {
    id: nextId,
    description: description.trim(),
    status: STATUS.TODO,
    createdAt: now(),
    updatedAt: now(),
  };
  tasks.push(task);
  writeTasks(tasks);
  console.log(`Task added successfully (id: ${task.id})`);
}

// This function handles the update command.
function handleUpdate(args) {
  const id = parseId(args[0]);
  const description = args[1];
  if (!description || description.trim() === "") {
    throw new Error("New description is required for update.");
  }
  const tasks = readTasks();
  const task = findTaskById(tasks, id);
  task.description = description.trim();
  task.updatedAt = now();
  writeTasks(tasks);
  console.log(`Task ${id} updated successfully`);
}

// This function handles the delete command.
function handleDelete(args) {
  const id = parseId(args[0]);
  const tasks = readTasks();
  const exists = tasks.some((task) => task.id === id);
  if (!exists) {
    throw new Error(`Task with id ${id} does not exist.`);
  }
  const filtered = tasks.filter((task) => task.id !== id);
  writeTasks(filtered);
  console.log(`Task ${id} deleted successfully`);
}

// This function handles status update commands.
function handleMark(args, status) {
  const id = parseId(args[0]);
  const tasks = readTasks();
  const task = findTaskById(tasks, id);
  task.status = status;
  task.updatedAt = now();
  writeTasks(tasks);
  console.log(`Task ${id} marked as ${status}`);
}

// This function handles listing commands with optional status filter.
function handleList(args) {
  const requestedStatus = args[0];
  const tasks = readTasks();
  if (!requestedStatus) {
    printTasks(tasks);
    return;
  }
  if (!Object.values(STATUS).includes(requestedStatus)) {
    throw new Error('List filter must be one of: "todo", "in-progress", "done".');
  }
  const filtered = tasks.filter((task) => task.status === requestedStatus);
  printTasks(filtered);
}

// This function routes CLI arguments to command handlers.
function main() {
  const args = process.argv.slice(2);
  const command = args[0];
  const commandArgs = args.slice(1);

  if (!command || command === "help" || command === "--help") {
    printHelp();
    return;
  }

  switch (command) {
    case "add":
      handleAdd(commandArgs);
      return;
    case "update":
      handleUpdate(commandArgs);
      return;
    case "delete":
      handleDelete(commandArgs);
      return;
    case "mark-in-progress":
      handleMark(commandArgs, STATUS.IN_PROGRESS);
      return;
    case "mark-done":
      handleMark(commandArgs, STATUS.DONE);
      return;
    case "list":
      handleList(commandArgs);
      return;
    default:
      throw new Error(`Unknown command: ${command}`);
  }
}

try {
  main();
} catch (error) {
  console.error(`Error: ${error.message}`);
  process.exit(1);
}
