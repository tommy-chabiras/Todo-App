mod db;
mod commands;
use db::Db;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	let db = Db::new("todos.db").unwrap();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(db)
        .invoke_handler(tauri::generate_handler![commands::greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
