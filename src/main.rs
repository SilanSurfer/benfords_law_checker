use csv::Reader;
use log::{error, info, trace, warn};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

fn get_freq_map(reader: &mut Reader<File>) -> Result<HashMap<char, u64>, Box<dyn Error>> {
    let mut digit_freq_map = HashMap::new();
    for result in reader.records() {
        match result {
            Ok(record) => {
                if let Some(population) = record.get(1) {
                    trace!("Val: {}", population);
                    population
                        .chars()
                        .next()
                        .filter(char::is_ascii_digit)
                        .map(|x| {
                            let freq = digit_freq_map.entry(x).or_insert(1);
                            *freq += 1;
                            trace!("{} == {:?}", x, *freq);
                            *freq
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
    Ok(digit_freq_map)
}

fn read_file(filename: &str) -> Result<Reader<File>, Box<dyn Error>> {
    info!("Reading from file {}", filename);
    let reader = Reader::from_path(filename)?;
    Ok(reader)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logger
    env_logger::init();

    let filename = String::from("population_by_country_2020.csv");
    let mut reader = read_file(&filename)?;
    get_freq_map(&mut reader)?;
    Ok(())
}
