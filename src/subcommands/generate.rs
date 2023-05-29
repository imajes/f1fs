use std::path::{PathBuf};
use std::fs;

use anyhow::{Context};

use crate::utils::*;
use crate::utils::ergast_client::*;

pub fn subcommand(years: &String, dest: &PathBuf) -> anyhow::Result<()> {
    tracing::info!("Generating folders in {:#?} for the following years: {:#?}", dest.display(), years);

    tracing::debug!("dest folder exists? {:#?}", dest.exists());
    if !dest.exists() {
        fs::create_dir(&dest)?;
    }

    let parsed_years = parse_years(years)?;
    tracing::debug!("parsed_years told us {:#?}", parsed_years);

    for year in parsed_years.iter() {
        let _output = match year {
           year => process_year(year, dest)?,
        };
    };

    return Ok(());
}

fn process_year(year: &usize, dest: &PathBuf) -> anyhow::Result<()> {

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

    return Ok(());
}
