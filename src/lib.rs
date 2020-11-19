use csv::Reader;
use log::{error, info, trace};
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

pub fn read_file(filename: &str) -> Result<Reader<File>, Box<dyn Error>> {
    info!("Reading from file {}", filename);
    let reader = Reader::from_path(filename)?;
    Ok(reader)
}

pub fn get_occurence_map(reader: &mut Reader<File>) -> Result<HashMap<char, u64>, Box<dyn Error>> {
    let mut digit_freq_map = HashMap::new();
    for result in reader.records() {
        match result {
            Ok(record) => {
                get_first_digit_from(&record)
                    .map(|x| update_occurence_in_map(x, &mut digit_freq_map));
            }
            Err(err) => {
                error!("Error while reading record!");
                return Err(Box::new(err));
            }
        }
    }
    Ok(digit_freq_map)
}

pub fn display_digits_frequencies(occurence_map: HashMap<char, u64>) {
    let total: u64 = occurence_map.values().sum();
    let freq_result: BTreeMap<char, f64> = occurence_map
        .into_iter()
        .map(|(digit, val)| (digit, val as f64 / total as f64))
        .collect();
    info!("Frequency map:\n{:.2?}", freq_result);
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

fn update_occurence_in_map(digit: char, hash_map: &mut HashMap<char, u64>) -> u64 {
    let freq = hash_map.entry(digit).or_insert(0);
    *freq += 1;
    trace!("{} == {:?}", digit, *freq);
    *freq
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

    mod update_occurence_in_map {
        use crate::update_occurence_in_map;
        use std::collections::HashMap;

        #[test]
        fn key_doesnt_exist_in_map() {
            let mut digit_freq_map = HashMap::new();
            assert_eq!(1, update_occurence_in_map('2', &mut digit_freq_map));
            assert_eq!(true, digit_freq_map.contains_key(&'2'));
            assert_eq!(Some(&1), digit_freq_map.get(&'2'));
        }
        #[test]
        fn key_exists_in_map() {
            let mut digit_freq_map = HashMap::new();
            digit_freq_map.insert('2', 1);
            assert_eq!(2, update_occurence_in_map('2', &mut digit_freq_map));
            assert_eq!(true, digit_freq_map.contains_key(&'2'));
            assert_eq!(Some(&2), digit_freq_map.get(&'2'));
        }
    }
}
