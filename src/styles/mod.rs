use std::collections::BTreeMap;

use apa::ApaStylizer;
use ieee::fmt_reference_ieee;

use crate::parsing::{
    entry::{BibEntry, EntryType},
    names::OwnedFullName,
};

pub mod apa;
pub mod ieee;

#[derive(Debug, Default, Clone, PartialEq, Eq, clap::ValueEnum)]
pub enum ReferenceStyle {
    #[default]
    IEEE,
    APA,
}

pub enum ThesisKind {
    Phd,
    Msc,
}

impl ReferenceStyle {
    pub fn fmt_reference(&self, entry: BibEntry) -> String {
        match self {
            ReferenceStyle::IEEE => fmt_reference_ieee(entry),
            ReferenceStyle::APA => ApaStylizer::fmt_reference(entry),
        }
    }
}
pub trait Stylizer {
    //required
    fn fmt_unpublished(authors: Vec<OwnedFullName>, fields: BTreeMap<String, String>) -> String;
    fn fmt_techreport(authors: Vec<OwnedFullName>, fields: BTreeMap<String, String>) -> String;
    fn fmt_proceedings(fields: BTreeMap<String, String>) -> String;
    fn fmt_thesis(
        kind: ThesisKind,
        authors: Vec<OwnedFullName>,
        fields: BTreeMap<String, String>,
    ) -> String;
    fn fmt_misc(authors: Vec<OwnedFullName>, fields: BTreeMap<String, String>) -> String;
    fn fmt_manual(authors: Vec<OwnedFullName>, fields: BTreeMap<String, String>) -> String;
    fn fmt_inproceedings(authors: Vec<OwnedFullName>, fields: BTreeMap<String, String>) -> String;
    fn fmt_incollection(authors: Vec<OwnedFullName>, fields: BTreeMap<String, String>) -> String;
    fn fmt_inbook(authors: Vec<OwnedFullName>, fields: BTreeMap<String, String>) -> String;
    fn fmt_conference(authors: Vec<OwnedFullName>, fields: BTreeMap<String, String>) -> String;
    fn fmt_booklet(authors: Vec<OwnedFullName>, fields: BTreeMap<String, String>) -> String;
    fn fmt_book(authors: Vec<OwnedFullName>, fields: BTreeMap<String, String>) -> String;
    fn fmt_year_month(year: Option<&String>, month: Option<&String>, braces: bool) -> String;
    fn fmt_article(authors: Vec<OwnedFullName>, fields: BTreeMap<String, String>) -> String;
    fn fmt_authors(authors: Vec<OwnedFullName>) -> String;
    // provided
    fn fmt_reference(entry: BibEntry) -> String {
        let (kind, _key, authors, fields) = entry.into_components();

        match kind {
            EntryType::Article => Self::fmt_article(authors, fields),
            EntryType::Book => Self::fmt_book(authors, fields),
            EntryType::Booklet => Self::fmt_booklet(authors, fields),
            EntryType::Conference => Self::fmt_conference(authors, fields),
            EntryType::Inbook => Self::fmt_inbook(authors, fields),
            EntryType::Incollection => Self::fmt_incollection(authors, fields),
            EntryType::Inproceedings => Self::fmt_inproceedings(authors, fields),
            EntryType::Manual => Self::fmt_manual(authors, fields),
            EntryType::Mastersthesis => Self::fmt_thesis(ThesisKind::Msc, authors, fields),
            EntryType::Misc => Self::fmt_misc(authors, fields),
            EntryType::Phdthesis => Self::fmt_thesis(ThesisKind::Phd, authors, fields),
            EntryType::Proceedings => Self::fmt_proceedings(fields),
            EntryType::Techreport => Self::fmt_techreport(authors, fields),
            EntryType::Unpublished => Self::fmt_unpublished(authors, fields),
        }
    }
}
