// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod notices;
use notices::get_notice;
mod repos;
use repos::get_ids;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_notice, get_ids])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
