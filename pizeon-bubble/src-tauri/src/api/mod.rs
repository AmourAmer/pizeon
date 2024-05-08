use eyre::{Result, WrapErr};
use std::path::PathBuf;
pub mod kitchen;
pub mod notices;
pub mod repos;
use pizeon_client::{database::Sqlite, record::sqlite_store::SqliteStore, settings::Settings};

// FIXME:
async fn settings() -> Result<Settings> {
    Settings::new().wrap_err("could not load client settings")
}

// TODO: maybe sometimes use cached value?
async fn db() -> Result<Sqlite> {
    let settings = settings().await.unwrap();
    let db_path = PathBuf::from(settings.db_path.as_str());

    let db = Sqlite::new(db_path, settings.local_timeout).await?;
    Ok(db)
}
