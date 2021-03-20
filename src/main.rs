use std::error::Error;
use structopt::StructOpt;

use benfords_law_checker::run;

mod cli;
mod logger;

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::CliArgs::from_args();
    logger::configure_logger(args.verbose);

    run(&args.input_file_path, args.input_header, args.graph);

    Ok(())
}
