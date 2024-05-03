use std::{
    fmt::{self, Display},
    io::{self, IsTerminal, Write},
    time::Duration,
};

use clap::Subcommand;
use eyre::{Context, Result};
use pizeon_common::utils::{self, Escapable as _};
use runtime_format::{FormatKey, FormatKeyError, ParseSegment, ParsedFmt};

use pizeon_client::{
    database::{
        // current_context,
        Database,
    },
    // encryption,
    notice::{
        // store::HistoryStore,
        Notice,
    },
    record::sqlite_store::SqliteStore,
    settings::{
        // FilterMode::{Directory, Global, Session},
        Settings,
        // Timezone,
    },
};

// #[cfg(feature = "sync")]
// use pizeon_client::{record, sync};
//
// use log::{debug, warn};
use time::{macros::format_description, OffsetDateTime};
//
// use super::search::format_duration_into;

#[derive(Subcommand, Debug)]
#[command(infer_subcommands = true)]
pub enum Cmd {
    /// Add a notice
    Add { notice: Vec<String> },
    //
    // /// Finishes a new command in the history (adds time, exit code)
    // End {
    //     id: String,
    //     #[arg(long, short)]
    //     exit: i64,
    //     #[arg(long, short)]
    //     duration: Option<u64>,
    // },
    //
    /// List all notices
    List {
        // TODO postponed problem to front-end
        // #[arg(long, short)]
        // server: String, // PIG TODO maybe some more specific type

        // If someone want cli use of pizeon, impl this.
        // But I don't know how to set arg in atuin 18.2.0
        // #[arg(long)]
        // human: bool,
        //
        /// Terminate the output with a null, for better multiline support
        #[arg(long)]
        print0: bool,

        // PIG FIXME copied from atuin, slim chance it can be used later. Can be
        // deleted after 2025
        // #[arg(long, short, default_value = "true")]
        // // accept no value
        // #[arg(num_args(0..=1), default_missing_value("true"))]
        // // accept a value
        // #[arg(action = clap::ArgAction::Set)]
        // reverse: bool,
        //
        /// Available variables: {command}, {directory}, {duration}, {user}, {host}, {exit} and {time}.
        /// Example: --format "{time} - [{duration}] - {directory}$\t{command}"
        #[arg(long, short)]
        format: Option<String>,
    },
    //
    // /// Get the last command ran
    // Last {
    //     #[arg(long)]
    //     human: bool,
    //
    //     /// Show only the text of the command
    //     #[arg(long)]
    //     cmd_only: bool,
    //
    //     /// Display the command time in another timezone other than the configured default.
    //     ///
    //     /// This option takes one of the following kinds of values:
    //     /// - the special value "local" (or "l") which refers to the system time zone
    //     /// - an offset from UTC (e.g. "+9", "-2:30")
    //     #[arg(long, visible_alias = "tz")]
    //     timezone: Option<Timezone>,
    //
    //     /// Available variables: {command}, {directory}, {duration}, {user}, {host} and {time}.
    //     /// Example: --format "{time} - [{duration}] - {directory}$\t{command}"
    //     #[arg(long, short)]
    //     format: Option<String>,
    // },
    //
    // InitStore,
    //
    // /// Delete history entries matching the configured exclusion filters
    // Prune {
    //     /// List matching history lines without performing the actual deletion.
    //     #[arg(short = 'n', long)]
    //     dry_run: bool,
    // },
}

// #[derive(Clone, Copy, Debug)]
// pub enum ListMode {
//     Human,
//     CmdOnly,
//     Regular,
// }
//
// impl ListMode {
//     pub const fn from_flags(human: bool, cmd_only: bool) -> Self {
//         if human {
//             ListMode::Human
//         } else if cmd_only {
//             ListMode::CmdOnly
//         } else {
//             ListMode::Regular
//         }
//     }
// }

#[allow(clippy::cast_sign_loss)]
pub fn print_list(h: &[Notice], format: Option<&str>, print0: bool) {
    let w = std::io::stdout();
    let mut w = w.lock();

    // PIG TODO: add timestamp to this and settings, also for `fmt` below
    let fmt_str = format.unwrap_or("{id}\t{body}").replace("\\t", "\t");

    let parsed_fmt = parse_fmt(&fmt_str);

    let iterator = Box::new(h.iter()) as Box<dyn Iterator<Item = &Notice>>;

    let entry_terminator = if print0 { "\0" } else { "\n" };
    let flush_each_line = print0;

    for notice in iterator {
        let fh = FmtNotice {
            notice,
            output_format: OutputFormat::for_output(&w),
        };
        let args = parsed_fmt.with_args(&fh);
        let write = write!(w, "{args}{entry_terminator}");
        if let Err(err) = args.status() {
            eprintln!("ERROR: history output failed with: {err}");
            std::process::exit(1);
        }
        check_for_write_errors(write);
        if flush_each_line {
            check_for_write_errors(w.flush());
        }
    }

    if !flush_each_line {
        check_for_write_errors(w.flush());
    }
}

fn check_for_write_errors(write: Result<(), io::Error>) {
    if let Err(err) = write {
        // Ignore broken pipe (issue #626)
        if err.kind() != io::ErrorKind::BrokenPipe {
            eprintln!("ERROR: History output failed with the following error: {err}");
            std::process::exit(1);
        }
    }
}

/// Type wrapper around `Notice` with formatting settings.
#[derive(Clone, Copy, Debug)]
struct FmtNotice<'a> {
    notice: &'a Notice,
    output_format: OutputFormat,
}

#[derive(Clone, Copy, Debug)]
enum OutputFormat {
    Literal,
    Escaped,
}
impl OutputFormat {
    fn for_output<O: IsTerminal>(out: &O) -> Self {
        if out.is_terminal() {
            Self::Escaped
        } else {
            Self::Literal
        }
    }
}

/// defines how to format the notice
impl FormatKey for FmtNotice<'_> {
    #[allow(clippy::cast_sign_loss)]
    fn fmt(&self, key: &str, f: &mut fmt::Formatter<'_>) -> Result<(), FormatKeyError> {
        match key {
            "id" => f.write_str(&self.notice.id.0)?,
            "body" => match self.output_format {
                OutputFormat::Literal => f.write_str(self.notice.body.trim()),
                OutputFormat::Escaped => f.write_str(&self.notice.body.trim().escape_control()),
            }?,
            _ => return Err(FormatKeyError::UnknownKey),
        }
        Ok(())
    }
}

fn parse_fmt(format: &str) -> ParsedFmt {
    match ParsedFmt::new(format) {
        Ok(fmt) => fmt,
        Err(err) => {
            eprintln!("ERROR: Notice formatting failed with the following error: {err}");
            println!("If your formatting string contains curly braces (eg: {{var}}) you need to escape them this way: {{{{var}}.");
            std::process::exit(1)
        }
    }
}

impl Cmd {
    // #[allow(clippy::too_many_lines, clippy::cast_possible_truncation)]
    async fn handle_add(db: &impl Database, settings: &Settings, notice: &[String]) -> Result<()> {
        let body = notice.join(" ");

        let h: Notice = Notice::create()
            .timestamp(OffsetDateTime::now_utc())
            .body(body)
            .build()
            .into();

        db.save(&h).await?;

        Ok(())
    }

    //     #[allow(unused_variables)]
    //     async fn handle_end(
    //         db: &impl Database,
    //         store: SqliteStore,
    //         history_store: HistoryStore,
    //         settings: &Settings,
    //         id: &str,
    //         exit: i64,
    //         duration: Option<u64>,
    //     ) -> Result<()> {
    //         if id.trim() == "" {
    //             return Ok(());
    //         }
    //
    //         let Some(mut h) = db.load(id).await? else {
    //             warn!("history entry is missing");
    //             return Ok(());
    //         };
    //
    //         if h.duration > 0 {
    //             debug!("cannot end history - already has duration");
    //
    //             // returning OK as this can occur if someone Ctrl-c a prompt
    //             return Ok(());
    //         }
    //
    //         if !settings.store_failed && h.exit != 0 {
    //             debug!("history has non-zero exit code, and store_failed is false");
    //
    //             // the history has already been inserted half complete. remove it
    //             db.delete(h).await?;
    //
    //             return Ok(());
    //         }
    //
    //         h.exit = exit;
    //         h.duration = match duration {
    //             Some(value) => i64::try_from(value).context("command took over 292 years")?,
    //             None => i64::try_from((OffsetDateTime::now_utc() - h.timestamp).whole_nanoseconds())
    //                 .context("command took over 292 years")?,
    //         };
    //
    //         db.update(&h).await?;
    //         history_store.push(h).await?;
    //
    //         if settings.should_sync()? {
    //             #[cfg(feature = "sync")]
    //             {
    //                 if settings.sync.records {
    //                     let (_, downloaded) = record::sync::sync(settings, &store).await?;
    //                     Settings::save_sync_time()?;
    //
    //                     crate::sync::build(settings, &store, db, Some(&downloaded)).await?;
    //                 } else {
    //                     debug!("running periodic background sync");
    //                     sync::sync(settings, false, db).await?;
    //                 }
    //             }
    //             #[cfg(not(feature = "sync"))]
    //             debug!("not compiled with sync support");
    //         } else {
    //             debug!("sync disabled! not syncing");
    //         }
    //
    //         Ok(())
    //     }

    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::fn_params_excessive_bools)]
    async fn handle_list(
        db: &impl Database,
        settings: &Settings,
        format: Option<String>,
        include_deleted: bool,
        print0: bool,
    ) -> Result<()> {
        let notices = db.list(None, include_deleted).await?;

        print_list(
            &notices,
            match format {
                None => Some(settings.notice_format.as_str()),
                _ => format.as_deref(),
            },
            print0,
        );

        Ok(())
    }

    //     async fn handle_prune(
    //         db: &impl Database,
    //         settings: &Settings,
    //         store: SqliteStore,
    //         context: pizeon_client::database::Context,
    //         dry_run: bool,
    //     ) -> Result<()> {
    //         // Grab all executed commands and filter them using History::should_save.
    //         // We could iterate or paginate here if memory usage becomes an issue.
    //         let matches: Vec<History> = db
    //             .list(&[Global], &context, None, false, false)
    //             .await?
    //             .into_iter()
    //             .filter(|h| !h.should_save(settings))
    //             .collect();
    //
    //         match matches.len() {
    //             0 => {
    //                 println!("No entries to prune.");
    //                 return Ok(());
    //             }
    //             1 => println!("Found 1 entry to prune."),
    //             n => println!("Found {n} entries to prune."),
    //         }
    //
    //         if dry_run {
    //             print_list(
    //                 &matches,
    //                 ListMode::Human,
    //                 Some(settings.history_format.as_str()),
    //                 false,
    //                 false,
    //                 settings.timezone,
    //             );
    //         } else {
    //             let encryption_key: [u8; 32] = encryption::load_key(settings)
    //                 .context("could not load encryption key")?
    //                 .into();
    //             let host_id = Settings::host_id().expect("failed to get host_id");
    //             let history_store = HistoryStore::new(store.clone(), host_id, encryption_key);
    //
    //             for entry in matches {
    //                 eprintln!("deleting {}", entry.id);
    //                 if settings.sync.records {
    //                     let (id, _) = history_store.delete(entry.id.clone()).await?;
    //                     history_store.incremental_build(db, &[id]).await?;
    //                 } else {
    //                     db.delete(entry.clone()).await?;
    //                 }
    //             }
    //         }
    //         Ok(())
    //     }

    pub async fn run(
        self,
        settings: &Settings,
        db: &impl Database,
        store: SqliteStore,
    ) -> Result<()> {
        // let encryption_key: [u8; 32] = encryption::load_key(settings)
        //     .context("could not load encryption key")?
        //     .into();
        //
        // let host_id = Settings::host_id().expect("failed to get host_id");
        // let history_store = HistoryStore::new(store.clone(), host_id, encryption_key);

        match self {
            Self::Add { notice } => Self::handle_add(db, settings, &notice).await,
            // Self::End { id, exit, duration } => {
            //     Self::handle_end(db, store, history_store, settings, &id, exit, duration).await
            // }
            Self::List { print0, format } => {
                Self::handle_list(db, settings, format, false, print0).await
            } //
              // Self::Last {
              //     human,
              //     cmd_only,
              //     timezone,
              //     format,
              // } => {
              //     let last = db.last().await?;
              //     let last = last.as_ref().map(std::slice::from_ref).unwrap_or_default();
              //     let tz = timezone.unwrap_or(settings.timezone);
              //     print_list(
              //         last,
              //         ListMode::from_flags(human, cmd_only),
              //         match format {
              //             None => Some(settings.history_format.as_str()),
              //             _ => format.as_deref(),
              //         },
              //         false,
              //         true,
              //         tz,
              //     );
              //
              //     Ok(())
              // }
              //
              // Self::InitStore => history_store.init_store(db).await,
              //
              // Self::Prune { dry_run } => {
              //     Self::handle_prune(db, settings, store, context, dry_run).await
              // }
        }
    }
}
