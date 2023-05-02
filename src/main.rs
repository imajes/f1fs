extern crate simple_error;

use std::error::Error;
use std::path::PathBuf;
use std::str::FromStr;

use clap::{Parser, Subcommand};
use clap_verbosity_flag::{Verbosity, InfoLevel};

use tracing::{debug, info, error, warn, Level};
use tracing_subscriber::FmtSubscriber;

mod utils;
mod subcommands;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {

    #[command(subcommand)]
    command: Option<Commands>,

    /// Don't do anything to the filesystem, just report what happens
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    dryrun: bool,

    /// Turn debugging information on
    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generates folders in the destination output for years asked
    Generate {
        /// years to generate folders for
        #[arg(short, long, value_name="PATH")]
        years: String,

        /// destination path to move files to
        #[arg(short, long, value_name="PATH")]
        dest: PathBuf,
    },

    /// Moves source files into the correct destination folders
    Relocate {
        /// source path to move files from
        #[arg(short, long, value_name="PATH")]
        src: PathBuf,

        /// destination path to move files to
        #[arg(short, long, value_name="PATH")]
        dest: PathBuf,
    },

    /// Check an incoming folder and run the relocation steps for new media
    Watch {
        /// source path to move files from
        #[arg(short, long, value_name="PATH")]
        watch: PathBuf,

        /// destination path to move files to
        #[arg(short, long, value_name="PATH")]
        dest: PathBuf,
    },

    /// Gather metadata/assets for a given season/race/event
    MetaData {
        /// type of metadata to locate/generate
        #[arg(short, long, value_name="TYPE")]
        kind: String,

        /// years to generate folders for
        #[arg(short, long, value_name="PATH")]
        years: String,

        /// destination path to move files to
        #[arg(short, long, value_name="PATH")]
        dest: PathBuf,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // Handle verbosity in a log filter that's friendly to us all
    let tracing_level_filter = Level::from_str(&cli.verbose.to_string())
        .expect("unable to determine verbosity");

    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing_level_filter) // ignore anything more verbose than what's asked for
        .without_time()
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default log subscriber failed");


    warn!("cli.command: {:#?}", cli.command);
    debug!("cli.verbose log_level_filter: {:#?}", cli.verbose.log_level_filter());
    debug!("cli.verbose get: {:#?}", cli.verbose.log_level());
    debug!("tracing_level_filter maybe: {:#?}", tracing_level_filter);

    let subcommand = match cli.command {
        Some(subcommand) => subcommand,
        None => {
            error!("You must enter a subcommand!");
            std::process::exit(1);
        },
    };

    let cmd_result = match subcommand {
        Commands::Generate { ref years, ref dest } => subcommands::generate::subcommand(years, dest),
        Commands::Relocate { ref src, ref dest } => subcommands::relocate::subcommand(src, dest),
        Commands::Watch { ref watch, ref dest } => subcommands::watch::subcommand(watch, dest),
        Commands::MetaData { ref kind, ref years, ref dest } => subcommands::metadata::subcommand(kind, years, dest),
        // _ => panic!("Unrecognized command requested: {:#?}", subcommand),
    };

    info!("Command result is {:#?}", cmd_result);

    return Ok(());
}
