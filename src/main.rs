use log::info;
use std::error::Error;
use structopt::StructOpt;

use benfords_law_checker::{display_digits_frequencies, get_occurence_map, read_file};

mod logger;

#[derive(StructOpt)]
#[structopt(
    name = "Benford's Law Checker",
    about = "Application checks if data follow Benford's Law"
)]
struct CliArgs {
    /// The path to the file to read
    input_file_path: String,
    /// Name of the column's header that will be checked. By default first column is used.
    #[structopt(short = "i", long)]
    input_header: Option<String>,
    /// Verbosity level
    /// -v for debug,
    /// -vv for trace
    #[structopt(short = "v", parse(from_occurrences))]
    verbose: u8,
    /// Display graph with presentation of the results.
    #[structopt(short, long)]
    graph: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = CliArgs::from_args();
    logger::configure_logger(args.verbose);

    let mut reader = read_file(&args.input_file_path)?;
    let occurence_map = get_occurence_map(&mut reader, args.input_header)?;
    info!(
        "Digit frequencies: {}",
        display_digits_frequencies(occurence_map, args.graph)
    );
    Ok(())
}
