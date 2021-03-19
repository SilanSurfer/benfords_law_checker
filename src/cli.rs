use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "Benford's Law Checker",
    about = "Application checks if data follow Benford's Law"
)]
pub struct CliArgs {
    /// The path to the file to read
    pub input_file_path: String,
    /// Name of the column's header that will be checked. By default the first column is used.
    #[structopt(short = "i", long)]
    pub input_header: Option<String>,
    /// Verbosity level
    /// -v for debug,
    /// -vv for trace
    #[structopt(short = "v", parse(from_occurrences))]
    pub verbose: u8,
    /// Display graph with presentation of the results.
    #[structopt(short, long)]
    pub graph: bool,
}
