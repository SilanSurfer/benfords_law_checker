use log::info;
use std::error::Error;

use benfords_law_checker::{get_freq_map, read_file};

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logger
    env_logger::init();

    let filename = String::from("population_by_country_2020.csv");
    let mut reader = read_file(&filename)?;
    let freq_map = get_freq_map(&mut reader)?;
    info!("Frequency map:\n{:?}", freq_map);
    Ok(())
}
