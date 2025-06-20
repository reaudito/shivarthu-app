// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod signing;

use crate::signing::create_account::create_seed;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, create_seed])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
