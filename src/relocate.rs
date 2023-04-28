use std::path::PathBuf;

pub fn subcommand(src: &PathBuf, dest: &PathBuf) -> () {  //Result<Void, Error> {
    println!("Getting content from {:#?}", &src);
    println!("Sending content to {:#?}", &dest);
    println!("This is happening from within the RELOCATE sub option command");
}
