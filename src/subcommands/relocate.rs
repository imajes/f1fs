use std::path::PathBuf;
use std::error::Error;
use std::fs;

use anyhow::{Context, Result};

use crate::utils::*;

type RelocateResult<T> = Result<T, Box<dyn Error>>;

pub fn subcommand(years: &String, src: &PathBuf, dest: &PathBuf) -> RelocateResult<bool> {
    tracing::info!("Relocating files from {:#?} to {:#?}", src.display(), dest.display());

    // years - because it's nicer to keep that truth consistent
    let parsed_years = parse_years(years)?;
    tracing::debug!("parsed_years told us {:#?}", parsed_years);

    for year in parsed_years.iter() {
        let _output = match year {
            year => process_year(year, src, dest)?,
        };
    };

    return Ok(true);
}

fn process_year(year: &usize, src: &PathBuf, dest: &PathBuf) -> RelocateResult<bool> {

    let src_folder = src.join(year.to_string());
    tracing::trace!("Looking for files in {:#?}", src_folder);

    if src_folder.is_dir() {
        for file in fs::read_dir(src_folder)? {
            let file = file?;
            _rv = process_file(file)?;
        }
    }

    return Ok(true);
}

fn process_file(src: &DirEntry, dest: &PathBuf) -> RelocateResult<bool> {
    let file_path = src.path();
    tracing::trace!("found file at {:#?}", file_path);

}
