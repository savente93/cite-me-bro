//! # Cite me Bro!
//! 
//! Cite me bro! (CMB)
//! 
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
