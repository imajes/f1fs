use std::path::PathBuf;

use clap::{Parser, Subcommand};

use tracing::{debug, error, info, warn, Level};
use tracing_subscriber::FmtSubscriber;

mod ergast_client;
mod generate;
mod relocate;
mod watch;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {

    #[command(subcommand)]
    command: Option<Commands>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    verbose: bool,

    /// Don't do anything, just report
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    dryrun: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// generates folders in the destination output
    Generate {
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
}

fn main() {
    // a builder for `FmtSubscriber`.
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // completes the builder.
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

    match subcommand {
        Commands::Generate { ref dest } => generate::subcommand(dest),
        Commands::Relocate { ref src, ref dest } => relocate::subcommand(src, dest),
        Commands::Watch { ref watch, ref dest } => watch::subcommand(watch, dest),
        // _ => panic!("Unrecognized command requested: {:#?}", subcommand),
    }
}
