#![allow(dead_code)]
pub mod formaters;
pub mod ops;
pub mod parsing;
pub mod styles;
pub mod utils;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum Format {
    #[default]
    Plain,
    Markdown,
    Html,
}
