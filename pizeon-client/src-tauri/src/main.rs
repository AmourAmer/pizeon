// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use chrono::Utc;
use serde::Serialize;

#[derive(Serialize)]
struct Notice {
    heading: String,
    body: String,
    date: i64,
}
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_notice() -> (Notice, Vec<String>) {
    (
        Notice {
            heading: String::from("hi"),
            body: String::from("join us"),
            date: Utc::now().timestamp(),
        },
        vec![String::from("fake sign")],
    )
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_notice])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
