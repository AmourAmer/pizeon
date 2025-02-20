use std::path::PathBuf;

use clap::Subcommand;
use eyre::{Result, WrapErr};

use env_logger::Builder;
use pizeon_client::{database::Sqlite, record::sqlite_store::SqliteStore, settings::Settings};

// #[cfg(feature = "sync")]
// mod sync;
//
// #[cfg(feature = "sync")]
// mod account;
//
// mod default_config;
// mod doctor;
// mod dotfiles;
mod notice;
// mod import;
// mod info;
// mod init;
// mod kv;
// mod search;
// mod stats;
// mod store;

#[derive(Subcommand, Debug)]
#[command(infer_subcommands = true)]
pub enum Cmd {
    /// Manipulates notices
    #[command(subcommand)]
    Notice(notice::Cmd),
    //
    // /// Import shell history from file
    // #[command(subcommand)]
    // Import(import::Cmd),
    //
    // /// Calculate statistics for your history
    // Stats(stats::Cmd),
    //
    // /// Interactive history search
    // Search(search::Cmd),
    //
    // #[cfg(feature = "sync")]
    // #[command(flatten)]
    // Sync(sync::Cmd),
    //
    // /// Manage your sync account
    // #[cfg(feature = "sync")]
    // Account(account::Cmd),
    //
    // /// Get or set small key-value pairs
    // #[command(subcommand)]
    // Kv(kv::Cmd),
    //
    // /// Manage the pizeon data store
    // #[command(subcommand)]
    // Store(store::Cmd),
    //
    // /// Manage your dotfiles with Pizeon
    // #[command(subcommand)]
    // Dotfiles(dotfiles::Cmd),
    //
    // /// Print Pizeon's shell init script
    // #[command()]
    // Init(init::Cmd),
    //
    // /// Information about dotfiles locations and ENV vars
    // #[command()]
    // Info,
    //
    // /// Run the doctor to check for common issues
    // #[command()]
    // Doctor,
    //
    // /// Print example configuration
    // #[command()]
    // DefaultConfig,
}

impl Cmd {
    pub fn run(self) -> Result<()> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        let settings = Settings::new().wrap_err("could not load client settings")?;
        let res = runtime.block_on(self.run_inner(settings));

        runtime.shutdown_timeout(std::time::Duration::from_millis(50));

        res
    }

    async fn run_inner(self, mut settings: Settings) -> Result<()> {
        Builder::new()
            .filter_level(log::LevelFilter::Off)
            .filter_module("sqlx_sqlite::regexp", log::LevelFilter::Off)
            .parse_env("PIZEON_LOG")
            .init();

        tracing::trace!(command = ?self, "client command");

        let db_path = PathBuf::from(settings.db_path.as_str());
        let record_store_path = PathBuf::from(settings.record_store_path.as_str());

        let db = Sqlite::new(db_path, settings.local_timeout).await?;
        let sqlite_store = SqliteStore::new(record_store_path, settings.local_timeout).await?;

        match self {
            Self::Notice(notice) => notice.run(&settings, &db, sqlite_store).await,
            // Self::Import(import) => import.run(&db).await,
            // Self::Stats(stats) => stats.run(&db, &settings).await,
            // Self::Search(search) => search.run(db, &mut settings, sqlite_store).await,
            //
            // #[cfg(feature = "sync")]
            // Self::Sync(sync) => sync.run(settings, &db, sqlite_store).await,
            //
            // #[cfg(feature = "sync")]
            // Self::Account(account) => account.run(settings, sqlite_store).await,
            //
            // Self::Kv(kv) => kv.run(&settings, &sqlite_store).await,
            //
            // Self::Store(store) => store.run(&settings, &db, sqlite_store).await,
            //
            // Self::Dotfiles(dotfiles) => dotfiles.run(&settings, sqlite_store).await,
            //
            // Self::Init(init) => init.run(&settings).await,
            //
            // Self::Info => {
            //     info::run(&settings);
            //     Ok(())
            // }
            //
            // Self::Doctor => doctor::run(&settings),
            //
            // Self::DefaultConfig => {
            //     default_config::run();
            //     Ok(())
            // }
        }
    }
}
