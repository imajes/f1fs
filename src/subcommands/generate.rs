use std::path::PathBuf;
use number_range::NumberRange;

use crate::utils::ergast_client::*;

use std::error::Error;
type GenerateResult<T> = Result<T, Box<dyn Error>>;

fn get_race_data() {
    let race_data = match get_season_data(2022) {
        Ok(rv) => rv,
        Err(e) => panic!("Problem doing the thing: {:#?}", e),
    };

    tracing::debug!("{:#?}", race_data[0]);
}

pub fn subcommand(years: &String, dest: &PathBuf) -> GenerateResult<bool> {

    let parsed_years = NumberRange::<i64>::default().parse_str(&years)?; //.collect();

    tracing::info!("Year parsing gave us {:#?}", parsed_years);
    tracing::info!("Sending content to {:#?}", &dest);
    tracing::info!("This is happening from within the GENERATE sub option command");

    get_race_data();

    return Ok(true);
}
