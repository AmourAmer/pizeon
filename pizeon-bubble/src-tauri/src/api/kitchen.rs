// FIXME:
#[tauri::command]
pub async fn whoami() -> Result<String, ()> {
    Ok("Pigeon".into())
}

// FIXME:
#[tauri::command]
pub async fn get_servers(role: Option<String>) -> Result<Vec<String>, ()> {
    match role {
        Some(role) => Ok(vec![role]),
        None => Ok(vec![whoami().await?]),
    }
}
