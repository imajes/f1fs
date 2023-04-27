extern crate getopts;
use getopts::Options;
use std::env;

mod ergast_client;

fn get_race_data() {
    let race_data = match ergast_client::get_season_data(2022) {
        Ok(rv) => rv,
        Err(e) => panic!("Problem doing the thing: {:?}", e),
    };

    println!("{:#?}", race_data[0]);
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] METHOD", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();

    // mode/method [create folders, move files, observe new files]

    opts.optflag("f", "folders", "Create the hierarchy of folders for plex");
    opts.optflag("m", "move", "Move the --src files into the --dest folders");
    opts.optflag("w", "watch", "Watch a folder for incoming downloads and put them in the correct place");

    opts.optopt("c", "create", "Path to create folders in", "PATH");
    opts.optopt("s", "src", "Path to search/source files from", "PATH");
    opts.optopt("d", "dest", "Path to move files to", "PATH");

    opts.optflag("", "debug", "Print debugging messages");
    opts.optflag("n", "dry-run", "Don't execute any changes, just report");
    opts.optflag("h", "help", "print this help menu");

    // parse matches from opt
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{:#?}", f.to_string()) }
    };

    if matches.opt_present("d") {
        // debugging on
    }

    if matches.opt_present("n") {
        // dry run present
    }

    if matches.opt_present("h") {
        // print help/usage
        print_usage(&program, opts);
        return;
    }

    // track paths
    let _dest = matches.opt_str("d");
    let _src = matches.opt_str("s");
    let _create = matches.opt_str("c");

    println!("{:#?}", matches.free);

    let _input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    get_race_data();
}
