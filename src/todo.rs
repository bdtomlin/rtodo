use crate::app::App;
use rusqlite::{Result, params};
use std::io::Write;

/// A sruct to represent a todo with id, text, and done
pub struct Todo {
    pub id: i64,
    pub text: String,
    pub done: bool,
}

impl Todo {
    pub fn new(text: String) -> Self {
        Self {
            id: 0,
            text,
            done: false,
        }
    }

    pub fn add<W: Write>(app: &App<W>, todo: &mut Todo) -> Result<()> {
        let mut stmt = app.db.prepare("INSERT INTO todos (text) VALUES (?)")?;
        let id = stmt.insert(params![todo.text])?;
        todo.id = id;
        Ok(())
    }

    pub fn get<W: Write>(app: &App<W>, id: i64) -> Result<Todo> {
        let mut stmt = app.db.prepare("SELECT * FROM todos where id = (?)")?;
        let todo = stmt.query_one(params![id], |row| {
            Ok(Todo {
                id: row.get(0)?,
                text: row.get(1)?,
                done: row.get(2).unwrap_or(false),
            })
        })?;
        Ok(todo)
    }

    pub fn list<W: Write>(app: &App<W>) -> Result<Vec<Todo>> {
        let mut stmt = app.db.prepare("SELECT id, text, done FROM todos")?;
        let rows = stmt.query_map([], |row| {
            Ok(Todo {
                id: row.get(0)?,
                text: row.get(1)?,
                done: row.get(2).unwrap_or(false),
            })
        })?;
        rows.collect()
    }

    pub fn count<W: Write>(app: &App<W>) -> Result<i64> {
        let mut stmt = app.db.prepare("SELECT count(id) FROM todos")?;
        let count = stmt.query_one([], |row| Ok(row.get(0)?))?;
        Ok(count)
    }

    pub fn delete<W: Write>(self: Self, app: &App<W>) -> Result<usize> {
        let mut stmt = app.db.prepare("DELETE FROM todos where id = (?)")?;
        stmt.execute(params![self.id])
    }

    pub fn complete<W: Write>(self: &mut Self, app: &App<W>) -> Result<usize> {
        let mut stmt = app
            .db
            .prepare("UPDATE todos set done = true where id = (?)")?;
        stmt.execute(params![self.id])?;
        self.done = true;
        Ok(1)
    }

    pub fn incomplete<W: Write>(self: &mut Self, app: &App<W>) -> Result<usize> {
        let mut stmt = app
            .db
            .prepare("UPDATE todos set done = false where id = (?)")?;
        stmt.execute(params![self.id])?;
        self.done = false;
        Ok(1)
    }
}

impl std::fmt::Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id: {}, text: {}, done: {}",
            self.id, self.text, self.done
        )
    }
}

#[cfg(test)]
#[path = "todo_tests.rs"]
mod tests;
