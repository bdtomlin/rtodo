use rusqlite::{Connection, Result};

pub fn conn(db_path: &str) -> Result<Connection> {
    let conn = Connection::open(db_path)?;

    ensure_table(&conn)?;

    Ok(conn)
}

fn ensure_table(conn: &Connection) -> Result<usize> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos (
            id   INTEGER PRIMARY KEY,
            text TEXT NOT NULL,
            done BOOLEAN NOT NULL DEFAULT false
        )",
        (),
    )
}

#[cfg(test)]
#[path = "db_tests.rs"]
mod tests;
