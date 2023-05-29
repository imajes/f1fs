use anyhow::{Context, Result};
use number_range::NumberRangeOptions;

pub mod ergast_client;

pub fn parse_years(years: &String) -> Result<Vec<usize>, anyhow::Error>{
    let year_range = NumberRangeOptions::<usize>::new()
        .with_list_sep(',')
        .with_range_sep('-')
        .parse(&years)
        .with_context(|| format!("Couldn't parse the years given ({:#?})", years))?;

    tracing::debug!("Year parsing gave us range of {:#?}", year_range);

    return Ok(year_range.collect::<Vec<usize>>());
}
