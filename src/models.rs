use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub lat: f64;
    pub long: f64;
    pub locality: String;
    pub country: String;
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Circuit {
    pub circuit_id: String;
    pub url: String;
    pub circuit_name: String;
    pub location: Location;
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Race {
    pub season: i32;
    pub round: i32;
    pub url: String;
    pub race_name: String;
    pub circuit: Circuit;
    pub date: String;
    pub time: String;
}
