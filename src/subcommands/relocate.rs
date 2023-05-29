use std::path::PathBuf;
use std::fs;

use anyhow::{bail};
use glob::glob;

use crate::utils::*;
//use crate::utils::ergast_client::*;

pub fn subcommand(src: &PathBuf, dest: &PathBuf) -> anyhow::Result<()> {

    tracing::info!("Moving files from {:#?} to {:#?}", src.display(), dest.display());

    tracing::debug!("dest folder exists? {:#?}", dest.exists());
    if !dest.exists() {
        fs::create_dir(&dest)?;
    }

    tracing::debug!("src folder exists? {:#?}", src.exists());
    if !src.exists() {
       bail!("Source folder for relocation [{}] is missing!", src.display());
    }

    let file_glob = format!("{}/*", src.display());

    for e in glob(&file_glob).expect("Failed to read glob pattern") {

        let item = e.unwrap();

        tracing::debug!("Found item {:#?}", item.display());

        if item.is_dir() {
            tracing::debug!("Item is a folder...");
            handle_folder(&item);
        } else {
            handle_file(&item);
        }

    // let parsed_years = parse_years(years)?;
    // tracing::debug!("parsed_years told us {:#?}", parsed_years);
    //
    // for year in parsed_years.iter() {
    //     let _output = match year {
    //        year => process_year(year, dest)?,
    //     };
    // };
    //


    return Ok(());
}
// fn process_year(year: &usize, src: &PathBuf, dest: &PathBuf) -> RelocateResult<bool> {
//
//     let src_folder = src.join(year.to_string());
//     tracing::trace!("Looking for files in {:#?}", src_folder);
//
//     if src_folder.is_dir() {
//         for file in fs::read_dir(src_folder)? {
//             let file = file?;
//             _rv = process_file(file)?;
//         }
//     }
//
//     return Ok(true);
// }
//
// fn process_file(src: &DirEntry, dest: &PathBuf) -> RelocateResult<bool> {
//     let file_path = src.path();
//     tracing::trace!("found file at {:#?}", file_path);
//
// }

fn handle_folder(src: &Path, dest: &PathBuf) -> anyhow::Result<()> {
    
}

