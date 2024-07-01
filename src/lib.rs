#![allow(dead_code)]
pub mod formaters;
pub mod ops;
pub mod parsing;
pub mod styles;
pub mod utils;

pub static VERSION: &str = "1.0";

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum Format {
    #[default]
    Plain,
    Markdown,
    Html,
}
