use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub end_date: String,
    pub completed: bool,
}

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
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO todos (title, description, end_date) VALUES (?1, ?2, ?3)",
            (title, description, date),
        )?;
        let id = conn.last_insert_rowid();
        Ok(id)
    }

    pub fn get_todo(&self, id: i64) -> Result<Todo> {
        self.conn.lock().unwrap().query_row(
            "SELECT id, title, description, completed, end_date 
         FROM todos WHERE id = ?1",
            [id],
            |row| {
                Ok(Todo {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    description: row.get(2)?,
                    completed: row.get(3)?,
                    end_date: row.get(4)?,
                })
            },
        )
    }

    pub fn get_todos(&self) -> Result<Vec<Todo>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt =
            conn.prepare("SELECT id, title, description, completed, end_date FROM todos")?;
        let rows = stmt
            .query_map([], |row| {
                Ok(Todo {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    description: row.get(2)?,
                    completed: row.get(3)?,
                    end_date: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(rows)
    }

    pub fn delete_todo(&self, id: i64) -> Result<(), Box<dyn std::error::Error>> {
        self.conn
            .lock()
            .unwrap()
            .execute("DELETE FROM todos WHERE id = ?1", [id])?;
        Ok(())
    }

    pub fn update_todo(&self, todo: Todo) -> Result<(), Box<dyn std::error::Error>> {
		self.conn.lock().unwrap().execute(
            "UPDATE todos SET title = ?1, description = ?2, completed = ?3, end_date = ?4 WHERE id = ?5",
            (todo.title, todo.description, todo.completed, todo.end_date, todo.id),
        )?;
        Ok(())
    }
}
