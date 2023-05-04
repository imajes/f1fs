use std::path::{PathBuf};
use std::error::Error;
use std::fs;

use number_range::NumberRangeOptions;
use anyhow::{Context, Result};

use crate::utils::ergast_client::*;

pub type GenerateResult<T> = Result<T, Box<dyn Error>>;

pub fn subcommand(years: &String, dest: &PathBuf) -> GenerateResult<bool> {
    tracing::info!("Generating folders in {:#?} for the following years: {:#?}", dest.display(), years);

    let parsed_years = parse_years(years)?;
    tracing::debug!("parsed_years told us {:#?}", parsed_years);

    tracing::debug!("dest folder exists? {:#?}", dest.exists());
    if !dest.exists() {
        fs::create_dir(&dest)?;
    }

    for year in parsed_years.iter() {
        let _output = match year {
           year => process_year(year, dest)?,
        };
    };

    return Ok(true);
}

fn process_year(year: &usize, dest: &PathBuf) -> GenerateResult<bool> {

    let year_folder = format!("Season {}", year);
    let year_path = dest.join(year_folder);
    tracing::debug!("Season path will be: {:#?}", year_path);

    if !year_path.exists() {
        tracing::trace!("Creating missing season folder");
        fs::create_dir(&year_path)
            .context("Couldn't create the folder due to OS error")?;
    }

    let race_data = get_season_data(year)
        .context("Couldn't get season data from the ergast API.")?;

    tracing::info!("Processing year {}. Creating missing folders for {} races.", year, race_data.races.len());

    for race in race_data.races.iter() {
        tracing::trace!("Race data: {:#?}", race);
        let race_folder = format!("{} - {}", &race.round, &race.race_name);
        let race_path = year_path.join(race_folder);
        tracing::trace!("Race path will be: {:#?}", race_path);

        if !race_path.exists() {
            tracing::trace!("Creating missing race path folder");
            fs::create_dir(&race_path)
                .context("Couldn't create the folder due to OS error")?;
        }
    }

    return Ok(true);
}

fn parse_years(years: &String) -> GenerateResult<Vec<usize>>{
    // let parsed_years = NumberRange::<i64>::default().parse_str(&years)?; //.collect();
    let year_range = NumberRangeOptions::<usize>::new()
        .with_list_sep(',')
        .with_range_sep('-')
        .parse(&years)
        .with_context(|| format!("Couldn't parse the years given ({:#?})", years))?;

    tracing::debug!("Year parsing gave us range of {:#?}", year_range);

    return Ok(year_range.collect::<Vec<usize>>());
}
