use super::{db, notices::Repo};
use pizeon_client::{database::Database, notice::Notice};

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
