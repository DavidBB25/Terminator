
# Terminator: terminate your tasks

#### Video Demo:  [click me](https://www.youtube.com/watch?v=ni1oDOPLVBQ)

A fast and minimalist to-do CLI app made in rust with clap and serde.


## Demo

```
$ todo a 'buy groceries' draw
Added task buy groceries.
Added task draw.

$ todo ls
1    buy groceries                  :(
2    draw                           :(

$ todo dn 1
Marked task 1 as done.

$ todo ls -s
2    draw                           :(
1    buy groceries                  :)

$ todo rm 2
Are you sure you want to delete an uncompleted task? (y/N)
$ y
Task(s) removed.
```
## Installation

Install Terminator with cargo

```bash
  cargo build --release
  sudo install -t /usr/bin ./target/release/todo
```
    
## Usage/Examples

Terminator is designed to be simple to use. All commands follow the todo <COMMAND> [ARGUMENTS] structure.

## a <DESCRIPTION>

Adds a new task to your list. If your task description contains spaces, you must enclose it in quotes.

### Syntax

```
todo a 'task description here'
```

### Example

```
$ todo a 'buy groceries'
Added task buy groceries.
```

## ls [FLAGS]

Displays all of your current tasks. By default, tasks are listed by ID. 

### Flags:
- -s : Sorts task by their completion status, showing incomplete tasks first

### Syntax

```
todo ls
```

### Example

```
# List tasks by id
$ todo ls

# List incomplete tasks first, then complete tasks
$ todo list -s
```
## dn <TASK_ID>

Marks an existing task as complete. You must provide the numerical ID of the task you wish to complete.

### Syntax

```
todo dn <TASK_ID>
```

### Example

```
$ todo dn 1
Marked task 1 as done.
```

## rm <TASK_ID>

Permanently removes a task from your list. Warns you if you prompt to remove an uncompleted task.

### Syntax

```
todo rm <TASK_ID>
```

### Example

```
$ todo rm 1
Task(s) removed.
```

## pg

Remove all of your tasks.

### Syntax

```
todo -h
```

### Example

```
$ todo pg
Are you sure you want to delete all of your tasks? (y/N)
```

## help

Lists all of the available commands.

### Syntax

```
todo -h
```

### Example

```
$ todo -h
A simple cli to-do app using clap and serde made in rust.

Usage: todo <COMMAND>

Commands:
  add     Add task(s). Use '' to add a task with spaces
  list    Lists all tasks. Use -s flag to sort by completed
  done    Mark task(s) as done or not done
  remove  Remove task(s)
  purge   Remove all tasks
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```
## Data Storage

Terminator stores all tasks in a file named tasks.json in the same directory it's installed. Thanks to the json format all data is human readable and also makes so that you can easily view, edit or backup the file.