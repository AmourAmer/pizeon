use super::repos::Repo;
use crate::api::db;
use chrono::Utc;
use json;
use pizeon_client::database::Database;
use pizeon_client::notice::Notice as RawNotice;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use time::OffsetDateTime;

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
                date: (Utc::now() - Duration::from_secs(24 * 60 * 60)).timestamp(),
            },
            signs: vec![String::from("shitty sign"), String::from("more signs")],
            repo: Repo::Junk,
        })
    }
}

#[tauri::command]
pub async fn get_abstract(id: &str) -> Result<Abstract, ()> {
    let db = db().await.unwrap();
    let h = db.load(id).await.unwrap().unwrap();
    let body = json::parse(h.body.as_str()).unwrap();
    Ok(Abstract {
        heading: body["heading"].as_str().unwrap_or("Missing heading").into(),
        date: (h.timestamp).unix_timestamp(),
    })
}

#[tauri::command]
pub async fn send_notice(
    servers: Vec<String>,
    body: String,
    signatures: Vec<String>,
) -> Result<(), ()> {
    for server in servers {
        match server.as_str() {
            "self" | "" => {
                let h: RawNotice = RawNotice::create()
                    .timestamp(OffsetDateTime::now_utc())
                    .body(body.clone())
                    .build()
                    .into();

                let db = db().await.unwrap();
                db.save(&h).await.unwrap();
            }
            _ => (),
        }
    }

    // TODO: actually send to server, of course
    Ok(())
}
