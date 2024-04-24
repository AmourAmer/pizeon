use super::notices::Repo;

#[tauri::command]
pub fn get_repo(repo: Repo) -> Vec<String> {
    vec![String::from("1"), String::from("3")]
}
