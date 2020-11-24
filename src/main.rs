use log::info;
use std::error::Error;
use structopt::StructOpt;

use benfords_law_checker::{display_digits_frequencies, get_occurence_map, read_file};

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
}

fn configure_logger(verbose: u8) {
    use chrono::Local;
    use env_logger::fmt::Color;
    use env_logger::Builder;
    use log::LevelFilter;
    use std::io::Write;

    let level_filter = match verbose {
        1 => LevelFilter::Debug,
        2 => LevelFilter::Trace,
        _ => LevelFilter::Info,
    };

    let mut builder = Builder::from_default_env();
    builder
        .format(|buf, record| {
            let mut level_style = buf.style();
            let mut args_style = buf.style();
            match record.level() {
                log::Level::Error => {
                    level_style.set_color(Color::Red).set_bold(true);
                    args_style.set_color(Color::Red);
                }
                log::Level::Warn => {
                    level_style.set_color(Color::Yellow);
                    args_style.set_color(Color::Red);
                }
                log::Level::Info => {
                    level_style.set_color(Color::Green);
                }
                _ => {}
            }
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%H:%M:%S"),
                level_style.value(record.level()),
                args_style.value(record.args())
            )
        })
        .filter(None, level_filter)
        .init();
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = CliArgs::from_args();
    configure_logger(args.verbose);

    let mut reader = read_file(&args.input_file_path)?;
    let occurence_map = get_occurence_map(&mut reader, args.input_header)?;
    info!(
        "Digit frequencies: {}",
        display_digits_frequencies(occurence_map)
    );
    Ok(())
}
