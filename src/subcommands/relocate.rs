use std::path::PathBuf;
use std::error::Error;

type RelocateResult<T> = Result<T, Box<dyn Error>>;

pub fn subcommand(src: &PathBuf, dest: &PathBuf) -> RelocateResult<bool> {
    println!("Getting content from {:#?}", &src);
    println!("Sending content to {:#?}", &dest);
    println!("This is happening from within the RELOCATE sub option command");

    return Ok(true);
}