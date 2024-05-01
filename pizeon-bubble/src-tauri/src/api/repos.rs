use super::{db, notices::Repo};
use pizeon_client::database::Database;

#[tauri::command]
pub async fn get_bill(repo: Repo) -> Vec<String> {
    let db = db().await.unwrap();
    let notices = db.list(None, false).await;
    if repo == Repo::Blocked {
        vec![String::from("1"), String::from("3")]
    } else {
        vec![String::from("5")]
    }
}
