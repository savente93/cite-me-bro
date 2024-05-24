use clap::Parser;
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

use nom::{
    bytes::complete::{tag, take_while},
    character::complete::{space0, space1},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};

// Helper function to parse individual words
fn parse_word(input: &str) -> IResult<&str, &str> {
    take_while(|c: char| c.is_alphabetic())(input)
}

// Helper function to parse words separated by spaces into a vector
fn parse_words_vec(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(space1, parse_word)(input)
}

// Main parser function
fn parse_line(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    all_consuming(separated_pair(
        parse_words_vec,
        delimited(space0, tag(","), space0),
        parse_words_vec,
    ))(input)
}

fn main() {
    let test_str = "word word , word word";
    match parse_line(test_str) {
        Ok((remaining, (segment1, segment2))) => {
            println!("Segment 1: {:?}", segment1);
            println!("Segment 2: {:?}", segment2);
            println!("Remaining: '{}'", remaining);
        }
        Err(err) => println!("Error: {:?}", err),
    }
}

// fn main() -> io::Result<()> {
// let args = Args::parse();

// let mut file = File::open(&args.bib_file)?;
// let mut contents = String::new();
// file.read_to_string(&mut contents)?;
// let bibtex = Bibliography::parse(&contents).unwrap();

// println!("{:?}", &args.style.fmt(&bibtex));

//     Ok(())
// }
