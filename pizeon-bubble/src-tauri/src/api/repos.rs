use super::db;
use pizeon_client::{database::Database, notice::Notice};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use time::OffsetDateTime;

#[derive(PartialEq, Serialize, Deserialize)]
pub enum Repo {
    Fresh,
    Blocked,
    Fridge,
    Junk,
}

pub fn which_repo(notice: &Notice) -> Option<Repo> {
    if let Some(deleted_at) = notice.deleted_at {
        let ddl = OffsetDateTime::now_utc() + Duration::from_secs(24 * 60 * 60);
        if deleted_at > ddl {
            return Some(Repo::Junk);
        } else {
            return None;
        }
    }
    if notice.blocked {
        return Some(Repo::Blocked);
    }
    if notice.expires_at.is_some() {
        return Some(Repo::Fresh);
    }
    Some(Repo::Fridge)
}

#[tauri::command]
pub async fn get_bill(repo: Repo) -> Vec<String> {
    let db = db().await.unwrap();
    db.list(None, true)
        .await
        .unwrap()
        .iter()
        .filter(|notice| match which_repo(notice) {
            Some(r) => r == repo,
            None => false,
        })
        .map(|notice| notice.id.0.clone())
        .collect()
    // FIXME: Did I write actually Repo filter already? Or some enhancement?
}

#[tauri::command]
pub async fn move_notice(id: String, repo: Repo) {
    let db = db().await.unwrap();
    let Some(h) = db.load(&id).await.unwrap() else {
        // warn!("history entry is missing"); // atuin warns so.
        return;
    };

    match repo {
        Repo::Junk => db.delete(h).await.unwrap(),
        Repo::Fridge => db.freeze(h).await.unwrap(),
        Repo::Fresh => db.recover(h).await.unwrap(),
        _ => (),
    }
}
