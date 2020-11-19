use std::error::Error;

use benfords_law_checker::{display_digits_frequencies, get_occurence_map, read_file};

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logger
    env_logger::init();

    let filename = String::from("population_by_country_2020.csv");
    let mut reader = read_file(&filename)?;
    let occurence_map = get_occurence_map(&mut reader)?;
    display_digits_frequencies(occurence_map);
    Ok(())
}
