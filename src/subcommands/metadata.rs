use std::path::PathBuf;
use std::error::Error;

type MetaDataResult<T> = Result<T, Box<dyn Error>>;

pub fn subcommand(kind: &String, years: &String, dest: &PathBuf) -> MetaDataResult<bool> {
    println!("looking for type {:#?}", &kind);
    println!("Watching content for years {:#?}", &years);
    println!("Sending content to {:#?}", &dest);
    println!("This is happening from within the METADATA sub option command");

    return Ok(true);
}
