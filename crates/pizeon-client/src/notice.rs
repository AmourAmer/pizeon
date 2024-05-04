use core::fmt::Formatter;
// use rmp::decode::ValueReadError;
// use rmp::{decode::Bytes, Marker};
// use std::env;
use std::fmt::Display;

// use pizeon_common::record::DecryptedData;
use pizeon_common::utils::uuid_v7;

// use eyre::{bail, eyre, Result};
// use regex::RegexSet;
//
// use crate::utils::get_host_user;
// use crate::{secrets::SECRET_PATTERNS, settings::Settings};
use time::OffsetDateTime;

mod builder;
// pub mod store;
//
// const HISTORY_VERSION: &str = "v0";
// pub const HISTORY_TAG: &str = "history";

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct NoticeId(pub String);

impl Display for NoticeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for NoticeId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

/// Client-side notice entry.
/// TODO very likely to change
///
/// To create a new notice entry, PIG FIXME~ If you see this, plz tell me to fix, examples should
/// be added.
//
// ## Implementation Notes from Atuin(I don't understand)
//
// New fields must should be added to `encryption::{encode, decode}` in a backwards
// compatible way. (eg sensible defaults and updating the nfields parameter)
#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct Notice {
    /// Whether this notice is blocked by rules
    pub blocked: bool,
    /// A server-generated ID, used to identify the entry
    ///
    /// Stored as `notice_id` in the database.
    pub id: NoticeId,
    /// When the notice arrived.
    pub timestamp: OffsetDateTime,
    // /// the url of notice body, commented because failed to use builder
    // pub body_url: Url,
    /// Notice body
    /// PIG FIXME should use specific trait
    pub body: String,
    /// local stored version marks of notice bodies
    pub versions: String,
    /// Timestamp, which is set when the entry is deleted, allowing a soft delete.
    pub deleted_at: Option<OffsetDateTime>,
    /// Timestamp, when the notice expires
    pub expires_at: Option<OffsetDateTime>,
    /// Timestamp
    pub last_changed: Option<OffsetDateTime>,
}

// #[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
// pub struct HistoryStats {
//     /// The command that was ran after this one in the session
//     pub next: Option<History>,
//     ///
//     /// The command that was ran before this one in the session
//     pub previous: Option<History>,
//
//     /// How many times has this command been ran?
//     pub total: u64,
//
//     pub average_duration: u64,
//
//     pub exits: Vec<(i64, i64)>,
//
//     pub day_of_week: Vec<(String, i64)>,
//
//     pub duration_over_time: Vec<(String, i64)>,
// }

impl Notice {
    //     #[allow(clippy::too_many_arguments)]
    fn new(
        blocked: bool,
        timestamp: OffsetDateTime,
        body: String,
        deleted_at: Option<OffsetDateTime>,
        expires_at: Option<OffsetDateTime>,
    ) -> Self {
        Self {
            blocked,
            id: uuid_v7().as_simple().to_string().into(),
            timestamp,
            body,
            versions: String::from(""),
            deleted_at,
            expires_at,
            last_changed: None,
        }
    }

    //     pub fn serialize(&self) -> Result<DecryptedData> {
    //         // This is pretty much the same as what we used for the old history, with one difference -
    //         // it uses integers for timestamps rather than a string format.
    //
    //         use rmp::encode;
    //
    //         let mut output = vec![];
    //
    //         // write the version
    //         encode::write_u16(&mut output, 0)?;
    //         // INFO: ensure this is updated when adding new fields
    //         encode::write_array_len(&mut output, 9)?;
    //
    //         encode::write_str(&mut output, &self.id.0)?;
    //         encode::write_u64(&mut output, self.timestamp.unix_timestamp_nanos() as u64)?;
    //         encode::write_sint(&mut output, self.duration)?;
    //         encode::write_sint(&mut output, self.exit)?;
    //         encode::write_str(&mut output, &self.command)?;
    //         encode::write_str(&mut output, &self.cwd)?;
    //         encode::write_str(&mut output, &self.session)?;
    //         encode::write_str(&mut output, &self.hostname)?;
    //
    //         match self.deleted_at {
    //             Some(d) => encode::write_u64(&mut output, d.unix_timestamp_nanos() as u64)?,
    //             None => encode::write_nil(&mut output)?,
    //         }
    //
    //         Ok(DecryptedData(output))
    //     }
    //
    //     fn deserialize_v0(bytes: &[u8]) -> Result<History> {
    //         use rmp::decode;
    //
    //         fn error_report<E: std::fmt::Debug>(err: E) -> eyre::Report {
    //             eyre!("{err:?}")
    //         }
    //
    //         let mut bytes = Bytes::new(bytes);
    //
    //         let version = decode::read_u16(&mut bytes).map_err(error_report)?;
    //
    //         if version != 0 {
    //             bail!("expected decoding v0 record, found v{version}");
    //         }
    //
    //         let nfields = decode::read_array_len(&mut bytes).map_err(error_report)?;
    //
    //         if nfields != 9 {
    //             bail!("cannot decrypt history from a different version of Pizeon");
    //         }
    //
    //         let bytes = bytes.remaining_slice();
    //         let (id, bytes) = decode::read_str_from_slice(bytes).map_err(error_report)?;
    //
    //         let mut bytes = Bytes::new(bytes);
    //         let timestamp = decode::read_u64(&mut bytes).map_err(error_report)?;
    //         let duration = decode::read_int(&mut bytes).map_err(error_report)?;
    //         let exit = decode::read_int(&mut bytes).map_err(error_report)?;
    //
    //         let bytes = bytes.remaining_slice();
    //         let (command, bytes) = decode::read_str_from_slice(bytes).map_err(error_report)?;
    //         let (cwd, bytes) = decode::read_str_from_slice(bytes).map_err(error_report)?;
    //         let (session, bytes) = decode::read_str_from_slice(bytes).map_err(error_report)?;
    //         let (hostname, bytes) = decode::read_str_from_slice(bytes).map_err(error_report)?;
    //
    //         // if we have more fields, try and get the deleted_at
    //         let mut bytes = Bytes::new(bytes);
    //
    //         let (deleted_at, bytes) = match decode::read_u64(&mut bytes) {
    //             Ok(unix) => (Some(unix), bytes.remaining_slice()),
    //             // we accept null here
    //             Err(ValueReadError::TypeMismatch(Marker::Null)) => (None, bytes.remaining_slice()),
    //             Err(err) => return Err(error_report(err)),
    //         };
    //
    //         if !bytes.is_empty() {
    //             bail!("trailing bytes in encoded history. malformed")
    //         }
    //
    //         Ok(History {
    //             id: id.to_owned().into(),
    //             timestamp: OffsetDateTime::from_unix_timestamp_nanos(timestamp as i128)?,
    //             duration,
    //             exit,
    //             command: command.to_owned(),
    //             cwd: cwd.to_owned(),
    //             session: session.to_owned(),
    //             hostname: hostname.to_owned(),
    //             deleted_at: deleted_at
    //                 .map(|t| OffsetDateTime::from_unix_timestamp_nanos(t as i128))
    //                 .transpose()?,
    //         })
    //     }
    //
    //     pub fn deserialize(bytes: &[u8], version: &str) -> Result<History> {
    //         match version {
    //             HISTORY_VERSION => Self::deserialize_v0(bytes),
    //
    //             _ => bail!("unknown version {version:?}"),
    //         }
    //     }
    //
    //     /// Builder for a history entry that is imported from shell history.
    //     ///
    //     /// The only two required fields are `timestamp` and `command`.
    //     ///
    //     /// ## Examples
    //     /// ```
    //     /// use pizeon_client::history::History;
    //     ///
    //     /// let history: History = History::import()
    //     ///     .timestamp(time::OffsetDateTime::now_utc())
    //     ///     .command("ls -la")
    //     ///     .build()
    //     ///     .into();
    //     /// ```
    //     ///
    //     /// If shell history contains more information, it can be added to the builder:
    //     /// ```
    //     /// use pizeon_client::history::History;
    //     ///
    //     /// let history: History = History::import()
    //     ///     .timestamp(time::OffsetDateTime::now_utc())
    //     ///     .command("ls -la")
    //     ///     .cwd("/home/user")
    //     ///     .exit(0)
    //     ///     .duration(100)
    //     ///     .build()
    //     ///     .into();
    //     /// ```
    //     ///
    //     /// Unknown command or command without timestamp cannot be imported, which
    //     /// is forced at compile time:
    //     ///
    //     /// ```compile_fail
    //     /// use pizeon_client::history::History;
    //     ///
    //     /// // this will not compile because timestamp is missing
    //     /// let history: History = History::import()
    //     ///     .command("ls -la")
    //     ///     .build()
    //     ///     .into();
    //     /// ```
    //     pub fn import() -> builder::HistoryImportedBuilder {
    //         builder::HistoryImported::builder()
    //     }

    /// Builder for a notice entry manually added
    ///
    /// ## Examples
    /// PIG TODO: write eg
    ///
    pub fn create() -> builder::NoticeCreatedBuilder {
        builder::NoticeCreated::builder()
    }

    /// Builder for a notice entry that is imported from the database.
    ///
    /// Removed some compile_fail code from atuin
    pub fn from_db() -> builder::NoticeFromDbBuilder {
        builder::NoticeFromDb::builder()
    }

    //     pub fn success(&self) -> bool {
    //         self.exit == 0 || self.duration == -1
    //     }
    //
    //     pub fn should_save(&self, settings: &Settings) -> bool {
    //         let secret_regex = SECRET_PATTERNS.iter().map(|f| f.1);
    //         let secret_regex = RegexSet::new(secret_regex).expect("Failed to build secrets regex");
    //
    //         !(self.command.starts_with(' ')
    //             || settings.history_filter.is_match(&self.command)
    //             || settings.cwd_filter.is_match(&self.cwd)
    //             || (secret_regex.is_match(&self.command)) && settings.secrets_filter)
    //     }
}
//
// #[cfg(test)]
// mod tests {
//     use regex::RegexSet;
//     use time::macros::datetime;
//
//     use crate::{history::HISTORY_VERSION, settings::Settings};
//
//     use super::History;
//
//     // Test that we don't save history where necessary
//     #[test]
//     fn privacy_test() {
//         let settings = Settings {
//             cwd_filter: RegexSet::new(["^/supasecret"]).unwrap(),
//             history_filter: RegexSet::new(["^psql"]).unwrap(),
//             ..Settings::utc()
//         };
//
//         let normal_command: History = History::capture()
//             .timestamp(time::OffsetDateTime::now_utc())
//             .command("echo foo")
//             .cwd("/")
//             .build()
//             .into();
//
//         let with_space: History = History::capture()
//             .timestamp(time::OffsetDateTime::now_utc())
//             .command(" echo bar")
//             .cwd("/")
//             .build()
//             .into();
//
//         let stripe_key: History = History::capture()
//             .timestamp(time::OffsetDateTime::now_utc())
//             .command("curl foo.com/bar?key=sk_test_1234567890abcdefghijklmnop")
//             .cwd("/")
//             .build()
//             .into();
//
//         let secret_dir: History = History::capture()
//             .timestamp(time::OffsetDateTime::now_utc())
//             .command("echo ohno")
//             .cwd("/supasecret")
//             .build()
//             .into();
//
//         let with_psql: History = History::capture()
//             .timestamp(time::OffsetDateTime::now_utc())
//             .command("psql")
//             .cwd("/supasecret")
//             .build()
//             .into();
//
//         assert!(normal_command.should_save(&settings));
//         assert!(!with_space.should_save(&settings));
//         assert!(!stripe_key.should_save(&settings));
//         assert!(!secret_dir.should_save(&settings));
//         assert!(!with_psql.should_save(&settings));
//     }
//
//     #[test]
//     fn disable_secrets() {
//         let settings = Settings {
//             secrets_filter: false,
//             ..Settings::utc()
//         };
//
//         let stripe_key: History = History::capture()
//             .timestamp(time::OffsetDateTime::now_utc())
//             .command("curl foo.com/bar?key=sk_test_1234567890abcdefghijklmnop")
//             .cwd("/")
//             .build()
//             .into();
//
//         assert!(stripe_key.should_save(&settings));
//     }
//
//     #[test]
//     fn test_serialize_deserialize() {
//         let bytes = [
//             205, 0, 0, 153, 217, 32, 54, 54, 100, 49, 54, 99, 98, 101, 101, 55, 99, 100, 52, 55,
//             53, 51, 56, 101, 53, 99, 53, 98, 56, 98, 52, 52, 101, 57, 48, 48, 54, 101, 207, 23, 99,
//             98, 117, 24, 210, 246, 128, 206, 2, 238, 210, 240, 0, 170, 103, 105, 116, 32, 115, 116,
//             97, 116, 117, 115, 217, 42, 47, 85, 115, 101, 114, 115, 47, 99, 111, 110, 114, 97, 100,
//             46, 108, 117, 100, 103, 97, 116, 101, 47, 68, 111, 99, 117, 109, 101, 110, 116, 115,
//             47, 99, 111, 100, 101, 47, 97, 116, 117, 105, 110, 217, 32, 98, 57, 55, 100, 57, 97,
//             51, 48, 54, 102, 50, 55, 52, 52, 55, 51, 97, 50, 48, 51, 100, 50, 101, 98, 97, 52, 49,
//             102, 57, 52, 53, 55, 187, 102, 118, 102, 103, 57, 51, 54, 99, 48, 107, 112, 102, 58,
//             99, 111, 110, 114, 97, 100, 46, 108, 117, 100, 103, 97, 116, 101, 192,
//         ];
//
//         let history = History {
//             id: "66d16cbee7cd47538e5c5b8b44e9006e".to_owned().into(),
//             timestamp: datetime!(2023-05-28 18:35:40.633872 +00:00),
//             duration: 49206000,
//             exit: 0,
//             command: "git status".to_owned(),
//             cwd: "/Users/conrad.ludgate/Documents/code/pizeon".to_owned(),
//             session: "b97d9a306f274473a203d2eba41f9457".to_owned(),
//             hostname: "fvfg936c0kpf:conrad.ludgate".to_owned(),
//             deleted_at: None,
//         };
//
//         let serialized = history.serialize().expect("failed to serialize history");
//         assert_eq!(serialized.0, bytes);
//
//         let deserialized = History::deserialize(&serialized.0, HISTORY_VERSION)
//             .expect("failed to deserialize history");
//         assert_eq!(history, deserialized);
//
//         // test the snapshot too
//         let deserialized =
//             History::deserialize(&bytes, HISTORY_VERSION).expect("failed to deserialize history");
//         assert_eq!(history, deserialized);
//     }
//
//     #[test]
//     fn test_serialize_deserialize_deleted() {
//         let history = History {
//             id: "66d16cbee7cd47538e5c5b8b44e9006e".to_owned().into(),
//             timestamp: datetime!(2023-05-28 18:35:40.633872 +00:00),
//             duration: 49206000,
//             exit: 0,
//             command: "git status".to_owned(),
//             cwd: "/Users/conrad.ludgate/Documents/code/pizeon".to_owned(),
//             session: "b97d9a306f274473a203d2eba41f9457".to_owned(),
//             hostname: "fvfg936c0kpf:conrad.ludgate".to_owned(),
//             deleted_at: Some(datetime!(2023-11-19 20:18 +00:00)),
//         };
//
//         let serialized = history.serialize().expect("failed to serialize history");
//
//         let deserialized = History::deserialize(&serialized.0, HISTORY_VERSION)
//             .expect("failed to deserialize history");
//
//         assert_eq!(history, deserialized);
//     }
//
//     #[test]
//     fn test_serialize_deserialize_version() {
//         // v0
//         let bytes_v0 = [
//             205, 0, 0, 153, 217, 32, 54, 54, 100, 49, 54, 99, 98, 101, 101, 55, 99, 100, 52, 55,
//             53, 51, 56, 101, 53, 99, 53, 98, 56, 98, 52, 52, 101, 57, 48, 48, 54, 101, 207, 23, 99,
//             98, 117, 24, 210, 246, 128, 206, 2, 238, 210, 240, 0, 170, 103, 105, 116, 32, 115, 116,
//             97, 116, 117, 115, 217, 42, 47, 85, 115, 101, 114, 115, 47, 99, 111, 110, 114, 97, 100,
//             46, 108, 117, 100, 103, 97, 116, 101, 47, 68, 111, 99, 117, 109, 101, 110, 116, 115,
//             47, 99, 111, 100, 101, 47, 97, 116, 117, 105, 110, 217, 32, 98, 57, 55, 100, 57, 97,
//             51, 48, 54, 102, 50, 55, 52, 52, 55, 51, 97, 50, 48, 51, 100, 50, 101, 98, 97, 52, 49,
//             102, 57, 52, 53, 55, 187, 102, 118, 102, 103, 57, 51, 54, 99, 48, 107, 112, 102, 58,
//             99, 111, 110, 114, 97, 100, 46, 108, 117, 100, 103, 97, 116, 101, 192,
//         ];
//
//         // some other version
//         let bytes_v1 = [
//             205, 1, 0, 153, 217, 32, 54, 54, 100, 49, 54, 99, 98, 101, 101, 55, 99, 100, 52, 55,
//             53, 51, 56, 101, 53, 99, 53, 98, 56, 98, 52, 52, 101, 57, 48, 48, 54, 101, 207, 23, 99,
//             98, 117, 24, 210, 246, 128, 206, 2, 238, 210, 240, 0, 170, 103, 105, 116, 32, 115, 116,
//             97, 116, 117, 115, 217, 42, 47, 85, 115, 101, 114, 115, 47, 99, 111, 110, 114, 97, 100,
//             46, 108, 117, 100, 103, 97, 116, 101, 47, 68, 111, 99, 117, 109, 101, 110, 116, 115,
//             47, 99, 111, 100, 101, 47, 97, 116, 117, 105, 110, 217, 32, 98, 57, 55, 100, 57, 97,
//             51, 48, 54, 102, 50, 55, 52, 52, 55, 51, 97, 50, 48, 51, 100, 50, 101, 98, 97, 52, 49,
//             102, 57, 52, 53, 55, 187, 102, 118, 102, 103, 57, 51, 54, 99, 48, 107, 112, 102, 58,
//             99, 111, 110, 114, 97, 100, 46, 108, 117, 100, 103, 97, 116, 101, 192,
//         ];
//
//         let deserialized = History::deserialize(&bytes_v0, HISTORY_VERSION);
//         assert!(deserialized.is_ok());
//
//         let deserialized = History::deserialize(&bytes_v1, HISTORY_VERSION);
//         assert!(deserialized.is_err());
//     }
// }
