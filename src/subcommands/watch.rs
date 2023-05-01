use std::path::PathBuf;
use std::error::Error;

type WatchResult<T> = Result<T, Box<dyn Error>>;

pub fn subcommand(watch: &PathBuf, dest: &PathBuf) -> WatchResult<bool> {
    println!("Watching content in {:#?}", &watch);
    println!("Sending content to {:#?}", &dest);
    println!("This is happening from within the WATCH sub option command");

    return Ok(true);
}
