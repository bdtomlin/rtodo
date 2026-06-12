use crate::db;
use crate::todo::Todo;
use rusqlite::{Connection, Result, params};
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

    pub fn add_todo(&self, todo: &mut Todo) -> Result<()> {
        let mut stmt = self.db.prepare("INSERT INTO todos (text) VALUES (?)")?;
        let id = stmt.insert(params![todo.text])?;
        todo.id = id;
        Ok(())
    }

    pub fn get_todo(&self, id: i64) -> Result<Todo> {
        let mut stmt = self.db.prepare("SELECT * FROM todos where id = (?)")?;
        let todo = stmt.query_one(params![id], |row| {
            Ok(Todo {
                id: row.get(0)?,
                text: row.get(1)?,
                done: row.get(2).unwrap_or(false),
            })
        })?;
        Ok(todo)
    }

    pub fn list_todos(&self) -> Result<Vec<Todo>> {
        let mut stmt = self.db.prepare("SELECT id, text, done FROM todos")?;
        let todos = stmt.query_map(params![], |row| {
            Ok(Todo {
                id: row.get(0)?,
                text: row.get(1)?,
                done: row.get(2).unwrap_or(false),
            })
        })?;
        Ok(todos.collect::<Result<Vec<Todo>>>()?)
    }

    pub fn count_todos(&self) -> Result<i64> {
        let mut stmt = self.db.prepare("SELECT count(id) FROM todos")?;
        let count = stmt.query_row(params![], |row| row.get(0))?;
        Ok(count)
    }

    pub fn delete_todo(&self, todo: &mut Todo) -> Result<usize> {
        let mut stmt = self.db.prepare("DELETE FROM todos where id = (?)")?;
        stmt.execute(params![todo.id])
    }

    pub fn complete_todo(&self, todo: &mut Todo) -> Result<usize> {
        let mut stmt = self
            .db
            .prepare("UPDATE todos set done = true where id = (?)")?;
        stmt.execute(params![todo.id])?;
        todo.done = true;
        Ok(1)
    }

    pub fn incomplete_todo(&self, todo: &mut Todo) -> Result<usize> {
        let mut stmt = self
            .db
            .prepare("UPDATE todos set done = false where id = (?)")?;
        stmt.execute(params![todo.id])?;
        todo.done = false;
        Ok(1)
    }
}

#[cfg(test)]
#[path = "app_tests.rs"]
mod tests;
