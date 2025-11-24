use clap::{Parser};
use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};


const TASKS: &str = "tasks.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: usize,
    desc: String,
    done: bool,
}

#[derive(Parser)]
#[command(name = "todo-cli", about = "A simple cli to-do app using clap and serde made in rust.")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Parser)]
enum Commands {
    #[clap(alias = "a")]
    /// Add task(s). Use '' to add a task with spaces.
    Add {
        #[clap(num_args = 1..)]
        desc: Vec<String>,
    },
    #[clap(alias = "ls")]
    /// Lists all tasks. Use -s flag to sort by completed.
    List {
        #[arg(short ='s', long)]
        sort: bool,
    },
    #[clap(alias = "dn")]
    /// Mark task(s) as done or not done.
    Done {
        #[clap(num_args = 1..)]
        id: Vec<usize>,
    },
    #[clap(alias = "rm")]
    /// Remove task(s).
    Remove {
        id: Vec<usize>,
    },
    #[clap(alias = "pg")]
    /// Remove all tasks.
    Purge,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli: Cli = Cli::parse();

    let mut tasks = match load() {
        Ok(tasks) => tasks,
        Err(e) => {
            if e.is::<serde_json::Error>() {
                println!("Could not parse current tasks.json file.");
                std::process::exit(1);
            } else {
                println!("Could not find task.json file, starting fresh.");
                Vec::new()
            }
        }
    };

    match cli.command {
        Commands::Add { desc } => {
            for i in desc {
                add(&mut tasks, i);
            }
        }
        Commands::List { sort } => {
            list(&tasks, sort)?
        }
        Commands::Done { id } => {
            for i in id {
                done(&mut tasks, i)?;
            }
        }
        Commands::Remove { id } => {
            for i in id {
                remove(&mut tasks, i)?;
            }
        }
        Commands::Purge => {
            purge(&mut tasks)?;
        }
    }

    save(tasks)
}


fn load() -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let file = File::open(TASKS)?;
    let reader = BufReader::new(file);
    let t = serde_json::from_reader(reader)?;
    Ok(t)
}

fn save(tasks: Vec<Task>) -> Result<(), Box<dyn std::error::Error>> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(TASKS)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &tasks)?;
    Ok(())
}

fn add(tasks: &mut Vec<Task>, d: String) {
    let next_id: usize = tasks.iter().map(|t: &Task| t.id).max().unwrap_or(0) + 1;
    let task: Task = Task {
        id: next_id,
        desc: d,
        done: false,
    };

    println!("Added task {}.", task.desc);
    tasks.push(task)
}

fn list(tasks: &[Task], sort: bool) -> Result<(), Box<dyn std::error::Error>> {
    if tasks.is_empty() {
        println!("No tasks yet.");
        return Ok(());
    }

    let mut display_tasks: Vec<Task> = tasks.to_vec();

    display_tasks.sort_by(|a, b| {
        if sort { // sort by completed
            let status_ordering = a.done.cmp(&b.done);

            if status_ordering != std::cmp::Ordering::Equal {
                return status_ordering;
            }
        }
        a.id.cmp(&b.id) // default sort by id 
    });

    for mut task in display_tasks { 
        println!(
            "{:<4} {:<30} {}",
            task.id,
            wrap(&mut task.desc),
            if task.done { ":)" } else { ":(" }
        );
    }

    Ok(())
}

fn wrap(desc: &mut str)  -> String {
    // count string length
    let size: usize = desc.len();
    
    // cut to max lenght and break a line for the remains
    if size > 30 {
        let (first, last) = desc.split_at_mut(30);
            let i = format!("{}\n     {}", first, wrap(last));
            i
    } else {
        // count remain string length and append how many spaces needed for the done thing
        let mut spaces = 30 - desc.len();
        let mut desc_with_spaces = desc.to_string();
        while spaces != 0 {
            // append
            desc.to_string();
            desc_with_spaces.push(' ');
            spaces -= 1
        }
        // save in one formatted string
        desc_with_spaces
    }
}

fn done(tasks: &mut [Task], id: usize) -> Result<(), Box<dyn std::error::Error>> {
    for task in tasks.iter_mut() {
        if task.id == id {
            if !task.done {
                task.done = true;
                println!("Marked task {id} as done.");
            } else {
                task.done = false;
                println!("Marked task {id} as not done.");
            }
            return Ok(())
        }
    }
    println!("Task {id} not found.");
    Ok(())
}

fn remove (tasks: &mut Vec<Task>, id: usize) -> Result<(), Box<dyn std::error::Error>> {
    println!("Are you sure you want to delete an uncompleted task? (y/N)");
    let mut i = String::new();
    std::io::stdin().read_line(&mut i)?;
    let i = i.trim().to_lowercase();

    if i == "y" || i == "yes" || i.is_empty() {
        tasks.retain(|task| task.id != id);
        println!("Task(s) removed.")
    } else if i == "n" || i == "no" {
    } else {
        println!("Invalid input.");
    }
    Ok(())
}

fn purge (tasks: &mut Vec<Task>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Are you sure you want to delete all of your tasks? (y/N)");
    let mut i = String::new();
    std::io::stdin().read_line(&mut i)?;
    let i = i.trim().to_lowercase();

    if i == "y" || i == "yes" {
        tasks.clear();
    } else if i == "n" || i == "no" || i.is_empty() {
    } else {
        println!("Invalid input.");
    }
    Ok(())
}