use csv::Reader;
use std::error::Error;

fn read_file(filename: &str) -> Result <(), Box<dyn Error>> {
    let mut reader = Reader::from_path(filename)?;
    for result in reader.records() {
        let line = result?;
        println!("{:?}", line);
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = String::from("population_by_country_2020.csv");
    read_file(&filename)?;
    Ok(())
}
