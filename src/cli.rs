use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "Application checks if data follow Benford's Law", long_about = None)]
pub struct CliArgs {
    /// The path to the file to read
    pub input_file_path: String,
    /// Name of the column's header that will be checked. By default the first column is used.
    #[arg(short, long)]
    pub input_header: Option<String>,
    /// Verbosity level
    /// -v for debug,
    /// -vv for trace
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
    /// Name of the file where graph will be saved.
    #[arg(short, long)]
    pub graph: Option<String>,
}
