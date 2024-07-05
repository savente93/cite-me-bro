use anyhow::{anyhow, Result};
use cite_me_bro::{ops::bibligraphy::Bibliography, styles::ReferenceStyle, Format};
use clap::{crate_version, Parser};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "cite-me-bro",
    version = crate_version!(),
    about = "formats bibtex entries to stdout"
)]
struct Args {
    /// the bib file containing the reference information
    #[arg(short, long, value_name = "BIB_FILE")]
    bib_files: Vec<PathBuf>,

    /// the reference style in which to print the references
    #[arg(short, long, value_enum, default_value_t = ReferenceStyle::IEEE)]
    style: ReferenceStyle,

    /// the format in which to print the references
    #[arg(short, long, value_enum, default_value_t = Format::Plain)]
    format: Format,

    /// the keys of the references to print. If none are provided all references will be printed
    keys: Vec<String>,

    /// Instead of printing citations to stdout, replace instances of \cite{key}
    /// in INPLACE_FILE with the corresponding reference
    #[arg(short, long, value_name = "INPLACE_FILE", conflicts_with = "keys")]
    inplace_file: Option<PathBuf>,

    #[arg(long, default_value_t = false)]
    fail_fast: bool,

    /// Do not pring warnings when citation keys are not found
    /// does nothing if no keys are provided
    #[arg(short, long, default_value_t = false)]
    quiet: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format_target(false)
        .format_timestamp(None)
        .init();
    let mut bibliography = Bibliography::default();

    for p in args.bib_files.clone() {
        let tmp_bib = Bibliography::from_file(p)?;
        bibliography.merge(tmp_bib);
    }

    if let Some(inplace_path) = args.inplace_file {
        bibliography.expand_file_citations_inplace(
            inplace_path,
            args.style,
            args.format,
            args.fail_fast,
        )?;
        Ok(())
    } else if args.keys.is_empty() {
        bibliography
            .fmt_entries(args.style, args.format)
            .into_iter()
            .for_each(|f| println!("{}", f));
        Ok(())
    } else {
        let (formatted, unknown_keys) = bibliography.fmt_entries_filtered(
            args.style,
            args.format,
            args.keys.clone(),
            args.fail_fast,
        )?;
        if formatted.is_empty() && !args.quiet {
            Err(anyhow!(
                "none of the keys {:?} found in bib file(s) {:?}",
                &args.keys,
                &args
                    .bib_files
                    .clone()
                    .into_iter()
                    .map(|e| e.display().to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ))
        } else {
            formatted.into_iter().for_each(|f| println!("{}", f));
            if !args.quiet {
                unknown_keys
                    .into_iter()
                    .for_each(|k| eprintln!("No entry for key {} was found, skipping...", k));
            }

            Ok(())
        }
    }
}
