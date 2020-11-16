use csv::Reader;
use log::{error, info, trace};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

pub fn read_file(filename: &str) -> Result<Reader<File>, Box<dyn Error>> {
    info!("Reading from file {}", filename);
    let reader = Reader::from_path(filename)?;
    Ok(reader)
}

pub fn get_freq_map(reader: &mut Reader<File>) -> Result<HashMap<char, u64>, Box<dyn Error>> {
    let mut digit_freq_map = HashMap::new();
    for result in reader.records() {
        match result {
            Ok(record) => {
                get_first_digit_from(&record).map(|x| update_freq_in_map(x, &mut digit_freq_map));
            }
            Err(err) => {
                error!("Error while reading record!");
                return Err(Box::new(err));
            }
        }
    }
    Ok(digit_freq_map)
}

fn update_freq_in_map(digit: char, hash_map: &mut HashMap<char, u64>) -> u64 {
    let freq = hash_map.entry(digit).or_insert(1);
    *freq += 1;
    trace!("{} == {:?}", digit, *freq);
    *freq
}

fn get_first_digit_from(record: &csv::StringRecord) -> Option<char> {
    match record.get(1) {
        Some(val) => {
            trace!("Parsing value: {}", val);
            val.chars()
                .next()
                .filter(|c| c.is_ascii_digit() && *c != '0')
        }
        None => None,
    }
}

#[cfg(test)]
mod tests {
    mod get_first_digit_from {
        use crate::get_first_digit_from;
        use csv::StringRecord;

        #[test]
        fn record_ok() {
            let record = StringRecord::from(vec!["test", "1243"]);
            assert_eq!(Some('1'), get_first_digit_from(&record));
        }
        #[test]
        fn record_contains_zero_as_first_digit() {
            let record = StringRecord::from(vec!["test", "0243"]);
            assert_eq!(None, get_first_digit_from(&record));
        }
        #[test]
        fn record_contains_not_ascii_digit_at_first_plae() {
            let record = StringRecord::from(vec!["test", "q243"]);
            assert_eq!(None, get_first_digit_from(&record));
        }
        #[test]
        fn record_contains_only_one_element() {
            let record = StringRecord::from(vec!["123"]);
            assert_eq!(None, get_first_digit_from(&record));
        }

    }
}
