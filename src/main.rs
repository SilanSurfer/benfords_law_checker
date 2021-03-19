use log::info;
use std::error::Error;
use structopt::StructOpt;

use benfords_law_checker::{display_digits_frequencies, get_occurence_map, read_file};

mod cli;
mod logger;

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::CliArgs::from_args();
    logger::configure_logger(args.verbose);

    let mut reader = read_file(&args.input_file_path)?;
    let occurence_map = get_occurence_map(&mut reader, args.input_header)?;
    info!(
        "Digit frequencies: {}",
        display_digits_frequencies(occurence_map, args.graph)
    );
    Ok(())
}
