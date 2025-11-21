use rusqlite::{Connection, Result};
use std::sync::Mutex;

pub struct Db {
    pub conn: Mutex<Connection>,
}

impl Db {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        let db = Self {
            conn: Mutex::new(conn),
        };
        db.conn.lock().unwrap().execute(
            "CREATE TABLE IF NOT EXISTS todos (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                description TEXT NOT NULL,
				start_date DATETIME DEFAULT CURRENT_TIMESTAMP,
    			end_date DATETIME,
				created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
				completed BOOLEAN NOT NULL DEFAULT 0
            )",
            [],
        )?;

        Ok(db)
    }

    pub fn add_todo(
        &self,
        title: &str,
        description: &str,
        date: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.conn.lock().unwrap().execute(
            "INSERT INTO todos (title, description, end_date) VALUES (?1, ?2, ?3)",
            (title, description, date),
        )?;
        Ok(())
    }

    pub fn get_todos(&self) -> Result<Vec<(i32, String, String, String)>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, title, description, end_date FROM todos")?;
        let rows = stmt
            .query_map([], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(rows)
    }

    pub fn delete_todo(&self, id: i32) -> Result<(), Box<dyn std::error::Error>> {
        self.conn
            .lock()
            .unwrap()
            .execute("DELETE FROM todos WHERE id = ?1", [id])?;
        Ok(())
    }

    pub fn update_todo(&self, id: i32, title: &str, description: &str, date: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.conn.lock().unwrap().execute(
            "UPDATE todos SET title = ?1, description = ?2 end_date = ?3 WHERE id = ?4",
            (title, description, date, id),
        )?;
        Ok(())
    }
}
