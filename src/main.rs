use log::info;
use std::error::Error;

use benfords_law_checker::{display_digits_frequencies, get_occurence_map, read_file};

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
    configure_logger();

    let filename = String::from("population_by_country_2020.csv");
    let mut reader = read_file(&filename)?;
    let occurence_map = get_occurence_map(&mut reader)?;
    info!(
        "Digit frequencies: {}",
        display_digits_frequencies(occurence_map)
    );
    Ok(())
}
