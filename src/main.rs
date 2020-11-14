use csv::Reader;
use log::{error, info, trace};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

fn get_freq_map(reader: &mut Reader<File>) -> Result<HashMap<char, u64>, Box<dyn Error>> {
    let mut digit_freq_map = HashMap::new();
    for result in reader.records() {
        match result {
            Ok(record) => {
                get_first_digit_from(&record)
                        .map(|x| {
                            let freq = digit_freq_map.entry(x).or_insert(1);
                            *freq += 1;
                            trace!("{} == {:?}", x, *freq);
                            *freq
                        });
            }
            Err(err) => {
                error!("Error while reading record!");
                return Err(Box::new(err));
            }
        }
    }
    Ok(digit_freq_map)
}

fn get_first_digit_from(record: &csv::StringRecord) -> Option<char> {
    match record.get(1) {
        Some(val) => {
            trace!("Parsing value: {}", val);
            val.chars().next().filter(|c| c.is_ascii_digit() && *c != '0')},
        None => None,
    }
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
    let freq_map = get_freq_map(&mut reader)?;
    info!("Frequency map:\n{:?}", freq_map);
    Ok(())
}
