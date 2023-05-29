use std::path::PathBuf;

pub fn subcommand(kind: &String, years: &String, dest: &PathBuf) -> anyhow::Result<()> {
    println!("looking for type {:#?}", &kind);
    println!("Watching content for years {:#?}", &years);
    println!("Sending content to {:#?}", &dest);
    println!("This is happening from within the METADATA sub option command");

    return Ok(());
}
