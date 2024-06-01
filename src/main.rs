use anyhow::{anyhow, Result};
use clap::Parser;
use parsing::entry::parse_bib_file;
use parsing::entry::Bibliography;
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
    keys: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let bibtex: Bibliography = parse_bib_file(args.bib_file.clone())?.into();

    let mut seen_at_least_one = false;
    if args.keys.is_empty() {
        bibtex
            .entries
            .into_iter()
            .for_each(|b| println!("{}", &args.style.fmt_reference(b)));
        Ok(())
    } else {
        args.keys
            .clone()
            .into_iter()
            .for_each(|b| match bibtex.get_entry(b.clone()) {
                Some(entry) => {
                    println!("{}", &args.style.fmt_reference(entry));
                    seen_at_least_one = true;
                }
                None => eprintln!("No entry for key {} was found, skipping...", b),
            });
        if !seen_at_least_one {
            Err(anyhow!(
                "none of the keys {:?} found in bib file {:?}",
                &args.keys,
                &args.bib_file,
            ))
        } else {
            Ok(())
        }
    }
}
