use super::{db, notices::Repo};
use pizeon_client::database::Database;

#[tauri::command]
pub async fn get_bill(repo: Repo) -> Vec<String> {
    let db = db().await.unwrap();
    db.list(None, false)
        .await
        .unwrap()
        .iter()
        .map(|notice| notice.id.0.clone())
        .collect()
    // FIXME: of course
}
