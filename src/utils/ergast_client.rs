use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};

const ERGAST_BASE_API_URL: &str = "https://ergast.com/api/f1";

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventWhen {
    pub date: Option<String>, //DateTime<Utc>,
    pub time: Option<String>, //DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub lat: String,
    pub long: String,
    pub locality: String,
    pub country: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Circuit {
    pub circuit_id: String,
    pub url: String,
    pub circuit_name: String,
    #[serde(rename = "Location")]
    pub location: Location,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Race {
    pub season: String, //i32,
    pub round: String, //i32,
    pub url: String,
    pub race_name: String,
    #[serde(rename = "Circuit")]
    pub circuit: Circuit,
    pub date: String, //DateTime<Utc>,
    pub time: Option<String>, //DateTime<Utc>,
    // these started appearing from 2022 onwards
    #[serde(rename = "FirstPractice")]
    pub first_practice: Option<EventWhen>,
    #[serde(rename = "SecondPractice")]
    pub second_practice: Option<EventWhen>,
    #[serde(rename = "ThirdPractice")]
    pub third_practice: Option<EventWhen>,
    #[serde(rename = "Qualifying")]
    pub qualifying: Option<EventWhen>,
    #[serde(rename = "Sprint")]
    pub sprint: Option<EventWhen>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RaceTable {
    pub season: String, //i32,
    #[serde(rename = "Races")]
    pub races: Vec<Race>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MRData {
    pub xmlns: String,
    pub series: String,
    pub url: String,
    pub limit: String, //i32,
    pub offset: String, //i32,
    pub total: String, //i32,
    #[serde(rename = "RaceTable")]
    pub race_table: RaceTable,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ErgastResponse {
    pub m_r_data: MRData,
}

#[tokio::main]
pub async fn get_season_data(year: &usize) -> Result<RaceTable, Error> {

    let season_url = format!("{ERGAST_BASE_API_URL}/{year}.json?limit=100");
    tracing::info!("Sending request to {:#?}", season_url);

    let ergast_response: ErgastResponse = Client::new()
        .get(season_url)
        .send()
        .await?
        .json()
        .await?;

    // dispense with the stuff we really don't need right now
    let race_data = ergast_response.m_r_data.race_table;

    Ok(race_data)
}
