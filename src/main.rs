use anyhow::{anyhow, Result};
use clap::Parser;
use colored::Colorize;
use log::{error, warn};
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
    #[arg(short, long, value_name = "BIB_FILE")]
    bib_file: PathBuf,
    #[arg(short, long, value_enum, default_value_t = ReferenceStyle::IEEE)]
    style: ReferenceStyle,
    keys: Vec<String>,
    #[arg(short, long, value_name = "INPLACE_FILE", conflicts_with = "keys")]
    inplace: Option<PathBuf>,

    #[arg(short, long, default_value_t = false)]
    quiet: bool,

    #[arg(short, long, conflicts_with = "quiet", default_value_t = false)]
    panic: bool,
}

fn main() -> Result<()> {
    colog::init();
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
                None => {
                    if !args.quiet && !args.panic {
                        warn!(
                            "No entry for key {} was found, skipping...",
                            b.bold().yellow()
                        )
                    };
                    if args.panic {
                        error!(
                            "key {:?} found in bib file {:?}, exiting...",
                            b, args.bib_file
                        );
                        std::process::exit(1);
                    };
                }
            });
        if !seen_at_least_one && !args.quiet {
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
