use reqwest::{Client, Error};
mod models;

const ERGAST_BASE_API_URL: &str = "https://ergast.com/api/f1";

#[tokio::main]
pub async fn get_season_data(year: u32) -> Result<(), Error> {

    let season_url = format!("{ERGAST_BASE_API_URL}/{year}.json?limit=100");
    println!("Sending request to {:#?}", season_url);

    let season_data = Client::new()
        .get(season_url)
        .send()
        .await?
        .text()
        .await?;

    println!("{:#?}", season_data);
    Ok(())
}

