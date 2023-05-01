extern crate simple_error;

use std::error::Error;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use clap_verbosity_flag::{Verbosity, InfoLevel};

use tracing::{debug, error, info, warn, Level};
use tracing_subscriber::FmtSubscriber;

mod utils;
mod subcommands;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {

    #[command(subcommand)]
    command: Option<Commands>,

    /// Turn debugging information on
    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,
    // #[arg(short, long, action = clap::ArgAction::SetTrue)]
    // verbose: bool,

    /// Don't do anything, just report
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    dryrun: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// generates folders in the destination output
    Generate {
        /// years to generate folders for
        #[arg(short, long, value_name="PATH")]
        years: String,

        /// dest path to move files to
        #[arg(short, long, value_name="PATH")]
        dest: PathBuf,
    },

    Relocate {
        /// source path to move files from
        #[arg(short, long, value_name="PATH")]
        src: PathBuf,

        /// dest path to move files to
        #[arg(short, long, value_name="PATH")]
        dest: PathBuf,
    },

    Watch {
        /// source path to move files from
        #[arg(short, long, value_name="PATH")]
        watch: PathBuf,

        /// dest path to move files to
        #[arg(short, long, value_name="PATH")]
        dest: PathBuf,
    },

    MetaData {
        /// type of metadata to locate/generate
        #[arg(short, long, value_name="TYPE")]
        kind: String,

        /// years to generate folders for
        #[arg(short, long, value_name="PATH")]
        years: String,

        /// dest path to move files to
        #[arg(short, long, value_name="PATH")]
        dest: PathBuf,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let cli = Cli::parse();
    debug!("cli.command: {:#?}", cli.command);

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
