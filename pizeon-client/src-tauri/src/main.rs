// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod storeroom;
use storeroom::get_notice;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_notice])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
