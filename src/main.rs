use csv::Reader;
use std::error::Error;
use log::{info, trace};

fn read_file(filename: &str) -> Result <(), Box<dyn Error>> {
    info!("Reading from file {}", filename);
    let mut reader = Reader::from_path(filename)?;
    for result in reader.records() {
        let line = result?;
        trace!("{:?}", line);
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logger
    env_logger::init();

    let filename = String::from("population_by_country_2020.csv");
    read_file(&filename)?;
    Ok(())
}
