// #![warn(clippy::pedantic, clippy::nursery)]
// #![allow(clippy::use_self, clippy::missing_const_for_fn)] // not 100% reliable
//
use clap::Parser;
use eyre::Result;

use command::PizeonCmd;

mod command;
//
// #[cfg(feature = "sync")]
// mod sync;
//
const VERSION: &str = env!("CARGO_PKG_VERSION");
// const SHA: &str = env!("GIT_HASH");

static HELP_TEMPLATE: &str = "\
{before-help}{name} {version}
{author}
{about}

{usage-heading}
  {usage}

{all-args}{after-help}";

/// Free notifier
#[derive(Parser)]
#[command(
    author = "TODO TODO@TODO",
    version = VERSION,
    help_template(HELP_TEMPLATE),
)]
struct Pizeon {
    #[command(subcommand)]
    pizeon: PizeonCmd,
}

impl Pizeon {
    fn run(self) -> Result<()> {
        self.pizeon.run()
    }
}

fn main() -> Result<()> {
    Pizeon::parse().run()
}
