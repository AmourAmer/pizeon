// FIXME:
#[tauri::command]
pub async fn whoami() -> Result<String, ()> {
    Ok("Pigeon".into())
}

// FIXME:
#[tauri::command]
pub async fn get_servers(role: String) -> Result<Vec<String>, ()> {
    Ok(vec![role])
}
