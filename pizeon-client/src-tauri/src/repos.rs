use super::notices::Repo;

#[tauri::command]
pub fn get_ids(repo: Repo) -> Vec<String> {
    vec![String::from("1"), String::from("3")]
}
