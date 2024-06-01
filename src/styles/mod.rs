use self::apa::fmt_reference_apa;
use crate::parsing::entry::BibEntry;
use crate::styles::ieee::fmt_reference_ieee;

pub mod apa;
pub mod ieee;

#[derive(Debug, Default, Clone, PartialEq, Eq, clap::ValueEnum)]
pub enum ReferenceStyle {
    #[default]
    IEEE,
    APA,
}

impl ReferenceStyle {
    pub fn fmt_reference(&self, entry: BibEntry) -> String {
        match self {
            ReferenceStyle::IEEE => fmt_reference_ieee(entry),
            ReferenceStyle::APA => fmt_reference_apa(entry),
        }
    }
}
