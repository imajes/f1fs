use std::path::PathBuf;
use std::error::Error;

type RelocateResult<T> = Result<T, Box<dyn Error>>;

pub fn subcommand(years: &String, src: &PathBuf, dest: &PathBuf) -> RelocateResult<bool> {
    println!("Getting years from {:#?}", &years);
    println!("Getting content from {:#?}", &src);
    println!("Sending content to {:#?}", &dest);
    println!("This is happening from within the RELOCATE sub option command");

    return Ok(true);
}
