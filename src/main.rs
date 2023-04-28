use std::path::PathBuf;

use clap::{Parser, Subcommand};

// mod ergast_client;

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
        dest: Option<PathBuf>,
    },

    Relocate {
        /// source path to move files from
        #[arg(short, long, value_name="PATH")]
        src: Option<PathBuf>,

        /// dest path to move files to
        #[arg(short, long, value_name="PATH")]
        dest: Option<PathBuf>,
    },

    Watch {
        /// source path to move files from
        #[arg(short, long, value_name="PATH")]
        watch: Option<PathBuf>,

        /// dest path to move files to
        #[arg(short, long, value_name="PATH")]
        dest: Option<PathBuf>,
    },
}

// fn get_race_data() {
//     let race_data = match ergast_client::get_season_data(2022) {
//         Ok(rv) => rv,
//         Err(e) => panic!("Problem doing the thing: {:#?}", e),
//     };
//
//     println!("{:#?}", race_data[0]);
// }

fn main() {

    let cli = Cli::parse();
    println!("cli.command: {:#?}", cli.command);

    let subcommand = cli.command.as_ref();

    // if subcommand == None {
    //     println!("Help requested");
    //     return;
    // }
    //
    println!("subcommand is {:#?}", subcommand);

    match subcommand {
        Some(Generate) => println!("This calls the generate sub option command"),
        Some(Relocate) => println!("This calls the relocate sub option command"),
        Some(Watch) => println!("This calls the watch sub option command"),
        None => println!("None - Shouldn't happen here"),
        _ => panic!("Unrecognized command requested: {:#?}", subcommand),
    }

    // get_race_data();
}
