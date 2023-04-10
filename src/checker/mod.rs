use csv::Reader;
use error::CheckerError;
use log::{debug, error, info, trace};
use std::collections::BTreeMap;
use std::fs::File;

mod error;
mod graph;

pub fn run(
    input_file: &str,
    header: Option<String>,
    render_graph: Option<String>,
) -> Result<(), CheckerError> {
    let mut reader = read_file(input_file)?;
    let occurence_map = get_occurence_map(&mut reader, header)?;
    info!(
        "Digit frequencies: {}",
        display_digits_frequencies(&occurence_map)
    );

    if let Some(graph_name) = render_graph {
        display_graph(occurence_map, graph_name)?;
    }

    Ok(())
}

fn display_graph(
    occurence_map: BTreeMap<char, u64>,
    graph_name: String,
) -> Result<(), error::CheckerError> {
    use std::path::Path;

    let graph_name_path = Path::new(&graph_name);
    if graph_name_path.extension().is_none() {
        return Err(CheckerError::GraphOutputFileError);
    }
    info!("Saving plot in {}", &graph_name_path.display());

    let sorted_vals = sort_values_by_digit_asc(occurence_map);

    debug!("Sorted vals:\n{:?}", sorted_vals);

    let graph = graph::Graph::new(
        graph_name_path,
        String::from("Benford's Law"),
        String::from("Number"),
        String::from("Count"),
    );

    graph.draw(sorted_vals)
}

fn sort_values_by_digit_asc(occurence_map: BTreeMap<char, u64>) -> Vec<u64> {
    occurence_map.values().cloned().collect::<Vec<u64>>()
}

fn read_file(filename: &str) -> Result<Reader<File>, error::CheckerError> {
    info!("Reading from file {}", filename);
    Reader::from_path(filename).map_err(error::CheckerError::IoError)
}

fn get_occurence_map(
    reader: &mut Reader<File>,
    input_header: Option<String>,
) -> Result<BTreeMap<char, u64>, error::CheckerError> {
    debug!("Counting digit occurences");
    let mut header_index: usize = 1;
    if let Some(name) = input_header {
        header_index = get_header_index(reader, name)?;
    }
    let mut digit_freq_map = BTreeMap::new();
    if reader.records().next().is_none() {
        return Err(error::CheckerError::EmptySource);
    }
    for result in reader.records() {
        match result {
            Ok(record) => {
                get_first_digit_from(&record, header_index)
                    .map(|x| update_occurence_in_map(x, &mut digit_freq_map));
            }
            Err(err) => {
                error!("Error while reading record!");
                return Err(error::CheckerError::CsvError(err));
            }
        }
    }
    Ok(digit_freq_map)
}

fn display_digits_frequencies(occurence_map: &BTreeMap<char, u64>) -> String {
    debug!("Displaying digit frequencies");
    let total: u64 = occurence_map.values().sum();
    let freq_result: BTreeMap<char, f64> = occurence_map
        .iter()
        .map(|(digit, val)| (*digit, *val as f64 / total as f64))
        .collect();
    format!("{:.2?}", freq_result)
}

fn get_header_index(
    reader: &mut Reader<File>,
    header_name: String,
) -> Result<usize, error::CheckerError> {
    let headers = reader.headers().map_err(error::CheckerError::CsvError)?;
    if headers.is_empty() {
        Err(error::CheckerError::NoHeaders)
    } else {
        headers
            .iter()
            .position(|x| x == header_name)
            .ok_or(error::CheckerError::NoHeaderName(header_name))
    }
}

fn get_first_digit_from(record: &csv::StringRecord, index: usize) -> Option<char> {
    match record.get(index) {
        Some(val) => {
            trace!("Parsing value: {}", val);
            val.chars()
                .next()
                .filter(|c| c.is_ascii_digit() && *c != '0')
        }
        None => None,
    }
}

fn update_occurence_in_map(digit: char, hash_map: &mut BTreeMap<char, u64>) -> u64 {
    let freq = hash_map.entry(digit).or_insert(0);
    *freq += 1;
    trace!("{} == {:?}", digit, *freq);
    *freq
}

#[cfg(test)]
mod tests {
    mod get_first_digit_from {
        use crate::checker::get_first_digit_from;
        use csv::StringRecord;

        #[test]
        fn record_ok() {
            let record = StringRecord::from(vec!["test", "1243"]);
            assert_eq!(Some('1'), get_first_digit_from(&record, 1));
        }
        #[test]
        fn record_contains_zero_as_first_digit() {
            let record = StringRecord::from(vec!["test", "0243"]);
            assert_eq!(None, get_first_digit_from(&record, 1));
        }
        #[test]
        fn record_contains_not_ascii_digit_at_first_plae() {
            let record = StringRecord::from(vec!["test", "q243"]);
            assert_eq!(None, get_first_digit_from(&record, 1));
        }
        #[test]
        fn record_contains_only_one_element() {
            let record = StringRecord::from(vec!["123"]);
            assert_eq!(None, get_first_digit_from(&record, 1));
        }
    }

    mod update_occurence_in_map {
        use crate::checker::update_occurence_in_map;
        use std::collections::BTreeMap;

        #[test]
        fn key_doesnt_exist_in_map() {
            let mut digit_freq_map = BTreeMap::new();
            assert_eq!(1, update_occurence_in_map('2', &mut digit_freq_map));
            assert_eq!(true, digit_freq_map.contains_key(&'2'));
            assert_eq!(Some(&1), digit_freq_map.get(&'2'));
        }
        #[test]
        fn key_exists_in_map() {
            let mut digit_freq_map = BTreeMap::new();
            digit_freq_map.insert('2', 1);
            assert_eq!(2, update_occurence_in_map('2', &mut digit_freq_map));
            assert_eq!(true, digit_freq_map.contains_key(&'2'));
            assert_eq!(Some(&2), digit_freq_map.get(&'2'));
        }
    }

    mod display_digits_frequencies {
        use crate::checker::display_digits_frequencies;
        use std::collections::BTreeMap;

        #[test]
        fn display_empty_result() {
            let digit_occurence_map: BTreeMap<char, u64> = BTreeMap::new();
            assert_eq!("{}", display_digits_frequencies(&digit_occurence_map));
        }

        #[test]
        fn display_normal_result() {
            let mut digit_occurence_map = BTreeMap::new();
            digit_occurence_map.insert('1', 5);
            digit_occurence_map.insert('2', 10);
            assert_eq!(
                "{'1': 0.33, '2': 0.67}",
                display_digits_frequencies(&digit_occurence_map)
            );
        }
    }
}
