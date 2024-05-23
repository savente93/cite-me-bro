use clap::Parser;
use std::io;
use std::path::PathBuf;
// use styles::CitationStyles;

mod parsing;
// mod styles;
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
    // #[arg(short, long, value_enum)]
    // style: CitationStyles,
}

fn main() -> io::Result<()> {
    // let args = Args::parse();

    // let mut file = File::open(&args.bib_file)?;
    // let mut contents = String::new();
    // file.read_to_string(&mut contents)?;
    // let bibtex = Bibliography::parse(&contents).unwrap();

    // println!("{:?}", &args.style.fmt(&bibtex));

    Ok(())
}
