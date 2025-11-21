#![allow(dead_code)]

use tauri::State;
use crate::db::Db;


#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub fn add_todo(db: State<Db>, title: &str, description: &str, date: &str) -> Result<(), String> {
  db.add_todo(title, description, date).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_todo(db: State<Db>, id: i32) -> Result<(), String> {
  db.delete_todo(id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_todos(db: State<Db>) -> Result<Vec<(i32, String, String, String)>, String> {
  db.get_todos().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_todo(db: State<Db>, id: i32, title: &str, description: &str, date: &str) -> Result<(), String> {
  db.update_todo(id, title, description, date).map_err(|e| e.to_string())
}
