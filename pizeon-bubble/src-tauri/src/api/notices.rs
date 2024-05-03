use super::repos::{which_repo, Repo};
use crate::api::db;
use json;
use pizeon_client::database::Database;
use pizeon_client::notice::Notice as RawNotice;
use serde::{Deserialize, Serialize};
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

#[tauri::command]
pub async fn get_notice(id: &str) -> Result<Meal, ()> {
    let db = db().await.unwrap();
    let h = db.load(id).await.unwrap().unwrap();
    let body = json::parse(h.body.as_str()).unwrap();
    Ok(Meal {
        repo: which_repo(&h),
        notice: Notice {
            heading: body["heading"]
                .as_str()
                .unwrap_or("Missing heading!")
                .into(),
            body: h.body,
            date: (h.timestamp).unix_timestamp(),
        },
        signs: vec![],
    })
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
            "self" | "localhost" => {
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
