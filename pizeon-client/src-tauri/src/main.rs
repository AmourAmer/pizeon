// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde::Serialize;

#[derive(Serialize)]
struct Notice {
    heading: String,
}
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_notice() -> Notice {
    Notice {
        heading: String::from("hi"),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_notice])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
