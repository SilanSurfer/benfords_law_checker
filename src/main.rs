use std::error::Error;
use structopt::StructOpt;

mod checker;
mod cli;
mod logger;

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::CliArgs::from_args();
    logger::configure_logger(args.verbose);

    if let Err(e) = checker::run(&args.input_file_path, args.input_header, args.graph) {
        log::error!("Error while running application:\n{}", e);
    }

    Ok(())
}
