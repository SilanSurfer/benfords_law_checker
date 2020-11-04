use csv::Reader;
use std::error::Error;
use log::{info, trace, warn, error};

fn read_file(filename: &str) -> Result <(), Box<dyn Error>> {
    info!("Reading from file {}", filename);
    let mut reader = Reader::from_path(filename)?;
    for result in reader.records() {
        match result {
            Ok(record) => {
                if let Some(population) = record.get(1) {
                    trace!("Val: {}", population);
                } else {
                    warn!("No population value!");
                }
            }
            Err(err) => {
                error!("Error while reading record!");
                return Err(Box::new(err))
            }
        }
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
