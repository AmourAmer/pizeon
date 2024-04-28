// use clap::Subcommand;
// use eyre::Result;
// 
// use pizeon_client::{record::sqlite_store::SqliteStore, settings::Settings};
// 
// mod alias;
// mod var;
// 
// #[derive(Subcommand, Debug)]
// #[command(infer_subcommands = true)]
// pub enum Cmd {
//     /// Manage shell aliases with Pizeon
//     #[command(subcommand)]
//     Alias(alias::Cmd),
// 
//     /// Manage shell and environment variables with Pizeon
//     #[command(subcommand)]
//     Var(var::Cmd),
// }
// 
// impl Cmd {
//     pub async fn run(self, settings: &Settings, store: SqliteStore) -> Result<()> {
//         match self {
//             Self::Alias(cmd) => cmd.run(settings, store).await,
//             Self::Var(cmd) => cmd.run(settings, store).await,
//         }
//     }
// }
