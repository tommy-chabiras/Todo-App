#![allow(dead_code)]

use tauri::State;
use crate::db::{Db, Todo};



#[tauri::command]
pub fn add_todo(db: State<Db>, title: &str, description: &str, date: &str) -> Result<Todo, String> {
  let id: i64 = db.add_todo(title, description, date).map_err(|e| e.to_string())?;
  db.get_todo(id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_todo(db: State<Db>, id: i64) -> Result<(), String> {
  db.delete_todo(id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_todos(db: State<Db>) -> Result<Vec<Todo>, String> {
  db.get_todos().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_todo(db: State<Db>, todo: Todo) -> Result<(), String> {
  db.update_todo(todo).map_err(|e| e.to_string())
}
