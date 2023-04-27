use reqwest::{Client, Error};

const ERGAST_BASE_API_URL: &str = "https://ergast.com/api/f1";

use serde::{Deserialize, Serialize};
use chrono::prelude::*;


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventWhen {
    pub date: DateTime<Utc>,
    pub time: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub lat: f64,
    pub long: f64,
    pub locality: String,
    pub country: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Circuit {
    pub circuit_id: String,
    pub url: String,
    pub circuit_name: String,
    pub location: Location,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Race {
    pub season: i32,
    pub round: i32,
    pub url: String,
    pub race_name: String,
    pub circuit: Circuit,
    pub date: DateTime<Utc>,
    pub time: DateTime<Utc>,
    pub first_practice: EventWhen,
    pub second_practice: EventWhen,
    pub third_practice: EventWhen,
    pub qualifying: EventWhen,
    pub sprint: EventWhen,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RaceSeason {
    pub races: Vec<Race>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RaceTable {
    pub season: i32,
    pub races: RaceSeason,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MRData {
    pub xmlns: String,
    pub series: String,
    pub url: String,
    pub limit: i32,
    pub offset: i32,
    pub total: i32,
    pub race_table: RaceTable;
}

#[tokio::main]
pub async fn get_season_data(year: u32) -> Result<(), Error> {

    let season_url = format!("{ERGAST_BASE_API_URL}/{year}.json?limit=100");
    println!("Sending request to {:#?}", season_url);

    let season_data: models::MRData = Client::new()
        .get(season_url)
        .send()
        .await?
        .text()
        .await?;

    println!("{:#?}", season_data);
    Ok(())
}

