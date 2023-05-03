use std::path::PathBuf;
use std::error::Error;
use std::fs;

// use number_range::{NumberRange, NumberRangeOptions};
use number_range::{NumberRangeOptions};

// use crate::utils::ergast_client::*;

use simple_error::SimpleError;

pub type GenerateResult<T> = Result<T, Box<dyn Error>>;

// fn get_race_data() {
//     let race_data = match get_season_data(2022) {
//         Ok(rv) => rv,
//         Err(e) => panic!("Problem getting data from ergast: {:#?}", e),
//     };
//
//     tracing::info!("{:#?}", race_data[0]);
// }

pub fn subcommand(years: &String, dest: &PathBuf) -> GenerateResult<bool> {
    tracing::info!("This is happening from within the GENERATE sub option command");

    let parsed_years = parse_years(years)?;
    tracing::debug!("parsed_years told us {:#?}", parsed_years);

    tracing::debug!("Sending content to root folder {:#?}", dest);
    tracing::debug!("dest folder exists? {:#?}", dest.exists());

    if !dest.exists() {
        fs::create_dir(&dest)?;
    }

    // for year in parsed_years.iter() {
    //     match year {
    //         process_year(year);
    //     }
    // }

    // get_race_data();

    return Ok(true);
}

fn parse_years(years: &String) -> GenerateResult<Vec<usize>>{
    // let parsed_years = NumberRange::<i64>::default().parse_str(&years)?; //.collect();
    let year_range = NumberRangeOptions::<usize>::new()
        .with_list_sep(',')
        .with_range_sep('-')
        .parse(&years)
        .map_err(|_| SimpleError::new("[GENERATE] Couldn't parse the years given."))?;

    tracing::debug!("Year parsing gave us range of {:#?}", year_range);

    return Ok(year_range.collect::<Vec<usize>>());
}
