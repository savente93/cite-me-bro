use std::collections::BTreeMap;

use apa::ApaStylizer;
use ieee::IeeeStylizer;

use crate::{
    formaters::plain::PlainTextFormatter,
    parsing::{
        entry::{BibEntry, EntryType},
        names::OwnedFullName,
    },
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
            ReferenceStyle::IEEE => {
                IeeeStylizer::<PlainTextFormatter>::default().fmt_reference(entry)
            }
            ReferenceStyle::APA => {
                ApaStylizer::<PlainTextFormatter>::default().fmt_reference(entry)
            }
        }
    }
}
pub trait Stylizer {
    //required
    fn fmt_unpublished(
        &self,
        authors: Vec<OwnedFullName>,
        fields: BTreeMap<String, String>,
    ) -> String;
    fn fmt_techreport(
        &self,
        authors: Vec<OwnedFullName>,
        fields: BTreeMap<String, String>,
    ) -> String;
    fn fmt_proceedings(&self, fields: BTreeMap<String, String>) -> String;
    fn fmt_thesis(
        &self,
        kind: ThesisKind,
        authors: Vec<OwnedFullName>,
        fields: BTreeMap<String, String>,
    ) -> String;
    fn fmt_misc(&self, authors: Vec<OwnedFullName>, fields: BTreeMap<String, String>) -> String;
    fn fmt_manual(&self, authors: Vec<OwnedFullName>, fields: BTreeMap<String, String>) -> String;
    fn fmt_inproceedings(
        &self,
        authors: Vec<OwnedFullName>,
        fields: BTreeMap<String, String>,
    ) -> String;
    fn fmt_incollection(
        &self,
        authors: Vec<OwnedFullName>,
        fields: BTreeMap<String, String>,
    ) -> String;
    fn fmt_inbook(&self, authors: Vec<OwnedFullName>, fields: BTreeMap<String, String>) -> String;
    fn fmt_conference(
        &self,
        authors: Vec<OwnedFullName>,
        fields: BTreeMap<String, String>,
    ) -> String;
    fn fmt_booklet(&self, authors: Vec<OwnedFullName>, fields: BTreeMap<String, String>) -> String;
    fn fmt_book(&self, authors: Vec<OwnedFullName>, fields: BTreeMap<String, String>) -> String;
    fn fmt_article(&self, authors: Vec<OwnedFullName>, fields: BTreeMap<String, String>) -> String;
    fn fmt_year_month(&self, year: Option<&String>, month: Option<&String>) -> String;
    fn fmt_authors(&self, authors: Vec<OwnedFullName>) -> String;
    // provided
    fn fmt_reference(&self, entry: BibEntry) -> String {
        let (kind, _key, authors, fields) = entry.into_components();

        match kind {
            EntryType::Article => Self::fmt_article(self, authors, fields),
            EntryType::Book => Self::fmt_book(self, authors, fields),
            EntryType::Booklet => Self::fmt_booklet(self, authors, fields),
            EntryType::Conference => Self::fmt_conference(self, authors, fields),
            EntryType::Inbook => Self::fmt_inbook(self, authors, fields),
            EntryType::Incollection => Self::fmt_incollection(self, authors, fields),
            EntryType::Inproceedings => Self::fmt_inproceedings(self, authors, fields),
            EntryType::Manual => Self::fmt_manual(self, authors, fields),
            EntryType::Mastersthesis => Self::fmt_thesis(self, ThesisKind::Msc, authors, fields),
            EntryType::Misc => Self::fmt_misc(self, authors, fields),
            EntryType::Phdthesis => Self::fmt_thesis(self, ThesisKind::Phd, authors, fields),
            EntryType::Proceedings => Self::fmt_proceedings(self, fields),
            EntryType::Techreport => Self::fmt_techreport(self, authors, fields),
            EntryType::Unpublished => Self::fmt_unpublished(self, authors, fields),
        }
    }
}
