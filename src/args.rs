use crate::{logger, CliResult};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(
        name = "init",
        about = "Initializes a new project configuration file and OpenAPI spec"
    )]
    Init {
        #[structopt(help = "", name = "name")]
        name: String,
    },
    #[structopt(
        name = "sync",
        about = "Syncs CDD projects using language-specific adaptors"
    )]
    Sync,
}

/// Compiler driven development cli tool
#[derive(StructOpt, Debug)]
#[structopt(name = "cdd")]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: u8,

    /// Optionally specifies a config file location.
    #[structopt(help = "Config file location", parse(from_os_str), name = "config")]
    config: Option<PathBuf>,
}

pub fn run() -> CliResult<()> {
    let opt = Opt::from_args();
    let _ = logger::start_logger(opt.verbose, false);

    match opt.cmd {
        Command::Init { name } => crate::commands::init(&name),
        Command::Sync => crate::commands::sync(),
    }
}
