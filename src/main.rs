use clap::Parser;
use parsing::entry::parse_bib_file;
use std::path::PathBuf;
use styles::ReferenceStyle;

mod parsing;
pub mod styles;
pub mod utils;

#[derive(Parser)]
#[command(
    name = "file_reader",
    version = "1.0",
    about = "formats bibtex entries to stdout"
)]
struct Args {
    /// The path to the file to read
    #[arg(short, long, value_name = "FILE")]
    bib_file: PathBuf,
    #[arg(short, long, value_enum, default_value_t = ReferenceStyle::IEEE)]
    style: ReferenceStyle,
}

fn main() {
    let args = Args::parse();

    let bibtex = parse_bib_file(args.bib_file).unwrap();

    bibtex.into_iter().for_each(|b| {
        println!("{}", &args.style.fmt_reference(b));
    });
}
