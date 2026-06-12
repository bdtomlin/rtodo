//! rtodo is a command line todo application
//! Author: Bryan Tomlin
use clap::{Parser, Subcommand};

mod app;
mod db;
mod todo;
use crate::todo::Todo;
use std::io::Write;

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add a new todo
    Add { todo: String },
    /// List all todos
    List,
    /// Count todos
    Count,
    /// Get a todo by id
    Get { id: i64 },
    /// Delete a todo by id
    Delete { id: i64 },
    /// Complete a todo by id
    Complete { id: i64 },
    /// Incomplete a todo by id
    Incomplete { id: i64 },
}

pub fn get_app<W: Write>(db_path: &str, output: W) -> app::App<W> {
    app::App::new(db_path, output)
}

/// Run rtodo. Currently passing in a writer for testing
/// but consider using the assert_command crate for better cli testing
pub fn run<W: Write>(app: &mut app::App<W>, args: Vec<String>) {
    let cli = Cli::parse_from(args);

    match &cli.commands {
        Commands::Add { todo } => {
            let mut todo = Todo::new(todo.to_string());
            app.add_todo(&mut todo).unwrap();
            writeln!(app.output, "{}", todo).unwrap();
        }
        Commands::List {} => {
            for todo in app.list_todos().unwrap() {
                writeln!(app.output, "{}", todo).unwrap();
            }
        }
        Commands::Count {} => {
            let count = app.count_todos().unwrap();
            writeln!(app.output, "{}", count).unwrap();
        }
        Commands::Get { id } => {
            let t = app.get_todo(*id).unwrap();
            writeln!(app.output, "{}", t).unwrap();
        }
        Commands::Complete { id } => match app.get_todo(*id) {
            Ok(mut t) => match app.complete_todo(&mut t) {
                Ok(_) => writeln!(app.output, "Completed").unwrap(),
                Err(e) => writeln!(app.output, "Error: {}", e).unwrap(),
            },
            Err(e) => writeln!(app.output, "Error: {}", e).unwrap(),
        },
        Commands::Incomplete { id } => match app.get_todo(*id) {
            Ok(mut t) => match app.incomplete_todo(&mut t) {
                Ok(_) => writeln!(app.output, "Incompleted").unwrap(),
                Err(e) => writeln!(app.output, "Error: {}", e).unwrap(),
            },
            Err(e) => writeln!(app.output, "Error: {}", e).unwrap(),
        },
        Commands::Delete { id } => match app.get_todo(*id) {
            Ok(mut t) => match app.delete_todo(&mut t) {
                Ok(_) => writeln!(app.output, "Deleted").unwrap(),
                Err(e) => writeln!(app.output, "Error: {}", e).unwrap(),
            },
            Err(e) => writeln!(app.output, "Error: {}", e).unwrap(),
        },
    }
}

#[cfg(test)]
#[path = "lib_tests.rs"]
mod tests;
