use super::db;
use eyre::Result;
use pizeon_client::{database::Database, notice::Notice};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize)]
pub enum Repo {
    Fresh,
    Blocked,
    Fridge,
    Junk,
}

#[tauri::command]
pub async fn get_bill(repo: Repo) -> Vec<String> {
    // I don't know why I wrote this inside fn.
    fn which_repo(notice: &Notice) -> Repo {
        if notice.deleted_at.is_some() {
            return Repo::Junk;
        }
        if notice.blocked {
            return Repo::Blocked;
        }
        Repo::Fresh
        // TODO: Fridge
    }

    let db = db().await.unwrap();
    db.list(None, false)
        .await
        .unwrap()
        .iter()
        .filter(|notice| which_repo(notice) == repo)
        .map(|notice| notice.id.0.clone())
        .collect()
    // FIXME: Did I write actually Repo filter already? Or some enhancement?
}

#[tauri::command]
pub async fn move_notice(_id: &str, repo: Repo) -> Result<()> {
    let db = db().await.unwrap();
    let Some(mut h) = db.load(id).await? else {
        // warn!("history entry is missing"); // atuin warns so.
        return Ok(());
    };

    Ok(())
}
