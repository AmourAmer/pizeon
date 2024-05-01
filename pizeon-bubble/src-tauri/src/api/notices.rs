use super::repos::Repo;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Notice {
    heading: String,
    body: String,
    date: i64,
}
#[derive(Serialize)]
pub struct Abstract {
    heading: String,
    date: i64,
}
#[derive(Serialize, Deserialize)]
pub struct Meal {
    notice: Notice,
    signs: Vec<String>,
    repo: Repo,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub async fn get_notice(id: &str) -> Result<Meal, ()> {
    if id == "1" {
        Ok(Meal {
            notice: Notice {
                heading: String::from("hi"),
                body: String::from("join us"),
                date: Utc::now().timestamp(),
            },
            signs: vec![String::from("fake sign")],
            repo: Repo::Fresh,
        })
    } else {
        // Should use cachedValues
        std::thread::sleep(std::time::Duration::from_millis(2383));
        Ok(Meal {
            notice: Notice {
                heading: String::from("hell"),
                body: String::from("Dark lord will consume you"),
                date: (Utc::now() - Duration::days(1)).timestamp(),
            },
            signs: vec![String::from("shitty sign"), String::from("more signs")],
            repo: Repo::Junk,
        })
    }
}

#[tauri::command]
pub fn get_abstract(repo: Repo, id: &str) -> Abstract {
    if id == "1" && repo == Repo::Fresh {
        Abstract {
            heading: String::from("hi"),
            date: Utc::now().timestamp(),
        }
    } else {
        Abstract {
            heading: String::from("hell"),
            date: (Utc::now() - Duration::days(1)).timestamp(),
        }
    }
}
