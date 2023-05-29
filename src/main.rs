use std::path::PathBuf;

use anyhow::{Result};

use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;

use tracing::{trace, debug, info, error};
use tracing_log::AsTrace;
use tracing_subscriber::FmtSubscriber;

mod utils;
mod subcommands;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {

    #[command(subcommand)]
    command: Option<Commands>,

    /// Don't do anything to the filesystem, just report what happens
    // #[arg(short, long, action = clap::ArgAction::SetTrue)]
    // dryrun: bool,

    /// Turn debugging information on
    #[command(flatten)]
    verbose: Verbosity,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generates folders in the destination output for years asked
    Generate {
        /// years to generate folders for. Accepts ranges(-) and solo(,): e.g. 1980-2000,2020
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

fn main() -> Result<()> {
    let cli = Cli::parse();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(cli.verbose.log_level_filter().as_trace()) // ignore anything more verbose than what's asked for
        .without_time()
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default log subscriber failed");

    debug!("cli.command: {:#?}", cli.command);
    trace!("cli.verbose: {:#?}", cli.verbose);
    trace!("cli.verbose.log_level_filter: {:#?}", cli.verbose.log_level_filter());
    trace!("cli.verbose.log_level_filter.as_trace: {:#?}", cli.verbose.log_level_filter().as_trace());

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
    };

    // specifics on how to fully customize error display available here:
    // https://docs.rs/anyhow/latest/anyhow/struct.Error.html
    match cmd_result {
        Ok(_cmd_result) => info!("Command completed successfully."),
        Err(cmd_result) => error!("Request failed with this error:\n\n {:?}", cmd_result),
    };

    return Ok(());
}

