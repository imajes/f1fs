use std::path::PathBuf;

pub fn subcommand(watch: &PathBuf, dest: &PathBuf) -> () {  //Result<Void, Error> {
    println!("Watching content in {:#?}", &watch);
    println!("Sending content to {:#?}", &dest);
    println!("This is happening from within the WATCH sub option command");
}
