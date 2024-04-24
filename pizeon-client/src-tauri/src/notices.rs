use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Notice {
    heading: String,
    body: String,
    date: i64,
}
#[derive(PartialEq, Serialize, Deserialize)]
pub enum Repo {
    Fresh,
    Unwelcomed,
    Fridge,
    Junk,
}
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub fn get_notice(repo: Repo, id: &str) -> (Notice, Vec<String>) {
    if id == "1" && repo == Repo::Fresh {
        (
            Notice {
                heading: String::from("hi"),
                body: String::from("join us"),
                date: Utc::now().timestamp(),
            },
            vec![String::from("fake sign")],
        )
    } else {
        (
            Notice {
                heading: String::from("hell"),
                body: String::from("Dark lord will consume you"),
                date: (Utc::now() - Duration::days(1)).timestamp(),
            },
            vec![String::from("shitty sign"), String::from("more signs")],
        )
    }
}
