// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
use api::{
    notices::{get_abstract, get_notice, send_notice},
    repos::{get_bill, move_notice},
};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_abstract,
            get_notice,
            get_bill,
            move_notice,
            send_notice,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
