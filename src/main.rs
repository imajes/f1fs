mod ergast_client;

fn main() {
    let sd = ergast_client::get_season_data(2022);

    match sd {
        Ok(race_data) => race_data,
        Err(e) => panic!("Problem doing the thing: {:?}", e),
    };

    println!("{:#?}", race_data);
}
