use log::info;
use std::error::Error;
use structopt::StructOpt;

use benfords_law_checker::{display_digits_frequencies, get_occurence_map, read_file};

#[derive(StructOpt)]
#[structopt(name = "cli args", about = "Structure for keeping all CLI arguments")]
struct CliArgs {
    /// The path to the file to read
    input_file_path: String,
}

fn configure_logger() {
    use chrono::Local;
    use env_logger::fmt::Color;
    use env_logger::Builder;
    use log::LevelFilter;
    use std::io::Write;

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
        .filter(None, LevelFilter::Info)
        .init();
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = CliArgs::from_args();
    configure_logger();

    let mut reader = read_file(&args.input_file_path)?;
    let occurence_map = get_occurence_map(&mut reader)?;
    info!(
        "Digit frequencies: {}",
        display_digits_frequencies(occurence_map)
    );
    Ok(())
}
