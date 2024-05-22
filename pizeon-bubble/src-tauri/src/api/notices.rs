use super::repos::{which_repo, Repo};
use crate::api::db;
use json;
use pizeon_client::database::Database;
use pizeon_client::notice::Notice as RawNotice;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Serialize, Deserialize)]
pub struct Notice {
    title: String,
    bare_body: String,
    date: i64,
}
#[derive(Serialize)]
pub struct Abstract {
    title: String,
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
        repo: which_repo(&h).unwrap_or(Repo::Junk),
        notice: Notice {
            title: body["title"].as_str().unwrap_or("Missing title!").into(),
            bare_body: h.body,
            date: (h.timestamp).unix_timestamp(),
        },
        signs: vec![],
        // FIXME: verify keys
    })
}

#[tauri::command]
pub async fn get_abstract(id: &str) -> Result<Abstract, ()> {
    let db = db().await.unwrap();
    let h = db.load(id).await.unwrap().unwrap();
    let body = json::parse(h.body.as_str()).unwrap();
    Ok(Abstract {
        title: body["title"].as_str().unwrap_or("Missing title").into(),
        date: (h.timestamp).unix_timestamp(),
    })
}

#[tauri::command]
pub async fn send_notice(
    destinations: Vec<String>,
    body: String,
    signatures: Vec<String>,
) -> Result<(), ()> {
    for destination in destinations {
        match destination.as_str() {
            "self" | "localhost" => {
                let h: RawNotice = RawNotice::create()
                    .timestamp(OffsetDateTime::now_utc())
                    // TODO: substitute variables
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
