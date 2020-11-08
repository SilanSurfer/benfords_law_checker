use csv::Reader;
use log::{error, info, trace, warn};
use std::collections::HashMap;
use std::error::Error;

fn read_file(filename: &str) -> Result<(), Box<dyn Error>> {
    info!("Reading from file {}", filename);
    let mut digit_freq_map = HashMap::new();
    let mut reader = Reader::from_path(filename)?;
    for result in reader.records() {
        match result {
            Ok(record) => {
                if let Some(population) = record.get(1) {
                    trace!("Val: {}", population);
                    population
                        .chars()
                        .next()
                        .filter(char::is_ascii_digit)
                        .and_then(|x| {
                            let freq = digit_freq_map.entry(x).or_insert(1);
                            *freq += 1;
                            trace!("{} == {:?}", x, *freq);
                            Some(*freq)
                        });
                } else {
                    warn!("No population value!");
                }
            }
            Err(err) => {
                error!("Error while reading record!");
                return Err(Box::new(err));
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
