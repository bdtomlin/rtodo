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

pub fn run<W: Write>(app: &mut app::App<W>, args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse_from(args);

    match &cli.commands {
        Commands::Add { todo } => {
            let mut todo = Todo::new(todo.to_string());
            app.add_todo(&mut todo)?;
            writeln!(app.output, "{}", todo)?;
        }
        Commands::List => {
            for todo in app.list_todos()? {
                writeln!(app.output, "{}", todo)?;
            }
        }
        Commands::Count => {
            writeln!(app.output, "{}", app.count_todos()?)?;
        }
        Commands::Get { id } => {
            writeln!(app.output, "{}", app.get_todo(*id)?)?;
        }
        Commands::Complete { id } => {
            let mut t = app.get_todo(*id)?;
            app.complete_todo(&mut t)?;
            writeln!(app.output, "Completed")?;
        }
        Commands::Incomplete { id } => {
            let mut t = app.get_todo(*id)?;
            app.incomplete_todo(&mut t)?;
            writeln!(app.output, "Incompleted")?;
        }
        Commands::Delete { id } => {
            let mut t = app.get_todo(*id)?;
            app.delete_todo(&mut t)?;
            writeln!(app.output, "Deleted")?;
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "lib_tests.rs"]
mod tests;
