use crate::db;
use rusqlite::Connection;
use std::io::Write;

pub struct App<W: Write> {
    pub db_path: String,
    pub db: Connection,
    pub output: W,
}

impl<W: Write> App<W> {
    pub fn new(db_path: &str, output: W) -> App<W> {
        App {
            db_path: db_path.to_string(),
            db: db::conn(db_path).unwrap(),
            output: output,
        }
    }
}

#[cfg(test)]
#[path = "app_tests.rs"]
mod tests;
