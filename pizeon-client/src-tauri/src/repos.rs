use super::notices::Repo;

#[tauri::command]
pub fn get_bill(repo: Repo) -> Vec<String> {
    if repo == Repo::Unwelcomed {
        vec![String::from("1"), String::from("3")]
    } else {
        vec![String::from("5")]
    }
}
