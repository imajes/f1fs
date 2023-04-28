use std::path::PathBuf;
use super::ergast_client::*;


fn get_race_data() {
    let race_data = match get_season_data(2022) {
        Ok(rv) => rv,
        Err(e) => panic!("Problem doing the thing: {:#?}", e),
    };

    println!("{:#?}", race_data[0]);
}

pub fn subcommand(dest: &PathBuf) -> () {  //Result<Void, Error> {
    println!("Sending content to {:#?}", &dest);
    println!("This is happening from within the GENERATE sub option command");

    get_race_data();
}
