//! # Cite me Bro!
//!
//! Cite me bro! (CMB) \cite{book}
//!

use anyhow::anyhow;
pub mod formaters;
pub mod ops;
pub mod parsing;
pub mod styles;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum Format {
    #[default]
    Plain,
    Markdown,
    Html,
}

impl TryFrom<&str> for Format {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "plain" => Ok(Format::Plain),
            "markdown" => Ok(Format::Markdown),
            "html" => Ok(Format::Html),
            _ => Err(anyhow!("invalid format")),
        }
    }
}
