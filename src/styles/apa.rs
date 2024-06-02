use std::collections::BTreeMap;

use crate::parsing::{
    entry::BibEntry,
    names::{and_seperated_names, OwnedFullName},
};
use chrono::NaiveDate;
use unicode_segmentation::UnicodeSegmentation;

use super::ThesisKind;

pub fn fmt_reference_apa(entry: BibEntry) -> String {
    let (kind, _key, authors, fields) = entry.into_components();

    match kind {
        crate::parsing::entry::EntryType::Article => fmt_article_apa(authors, fields),
        crate::parsing::entry::EntryType::Book => fmt_book_apa(authors, fields),
        crate::parsing::entry::EntryType::Booklet => fmt_booklet_apa(authors, fields),
        crate::parsing::entry::EntryType::Conference => fmt_conference_apa(authors, fields),
        crate::parsing::entry::EntryType::Inbook => fmt_inbook_apa(authors, fields),
        crate::parsing::entry::EntryType::Incollection => fmt_incollection_apa(authors, fields),
        crate::parsing::entry::EntryType::Inproceedings => fmt_inproceedings_apa(authors, fields),
        crate::parsing::entry::EntryType::Manual => fmt_manual_apa(authors, fields),
        crate::parsing::entry::EntryType::Mastersthesis => {
            fmt_thesis_apa(ThesisKind::Msc, authors, fields)
        }
        crate::parsing::entry::EntryType::Misc => fmt_misc_apa(authors, fields),
        crate::parsing::entry::EntryType::Phdthesis => {
            fmt_thesis_apa(ThesisKind::Phd, authors, fields)
        }
        crate::parsing::entry::EntryType::Proceedings => fmt_proceedings_apa(fields),
        crate::parsing::entry::EntryType::Techreport => fmt_techreport_apa(authors, fields),
        crate::parsing::entry::EntryType::Unpublished => fmt_unpublished_apa(authors, fields),
    }
}

fn fmt_unpublished_apa(authors: Vec<OwnedFullName>, fields: BTreeMap<String, String>) -> String {
    let mut out = String::new();
    let title = fields.get("title").unwrap();
    let year = fields.get("year");
    let month = fields.get("month");
    out.push_str(&fmt_authors_apa(authors));
    out.push(' ');
    out.push_str(&fmt_year_month_apa(year, month, true));
    out.push_str(title);
    out.push_str(".");

    out
}

fn fmt_techreport_apa(authors: Vec<OwnedFullName>, fields: BTreeMap<String, String>) -> String {
    let mut out = String::new();
    let title = fields.get("title").unwrap();
    let year = fields.get("year");
    let month = fields.get("month");
    let number = fields.get("number").unwrap();
    let institution = fields.get("institution").unwrap();
    let address = fields.get("address").unwrap();
    out.push_str(&fmt_authors_apa(authors));
    out.push(' ');
    out.push_str(&fmt_year_month_apa(year, month, true));
    out.push_str(title);
    out.push(' ');
    out.push_str(&format!("(tech. rep. No. {}). ", number));
    out.push_str(&format!("{}. ", institution));
    out.push_str(&format!("{}.", address));

    out
}

fn fmt_proceedings_apa(fields: BTreeMap<String, String>) -> String {
    let mut out = String::new();
    let title = fields.get("title").unwrap();
    let year = fields.get("year");
    let month = fields.get("month");
    let editors_str = fields.get("editor").unwrap();
    let (_tail, edrs) = and_seperated_names(editors_str).unwrap();
    let editor_names: Vec<OwnedFullName> = edrs.into_iter().map(|n| n.into()).collect();
    let volume = fields.get("volume").unwrap();
    let publisher = fields.get("publisher").unwrap();
    out.push_str(&fmt_authors_apa(editor_names));
    out.push_str(" (Eds.). ");
    out.push_str(&fmt_year_month_apa(year, month, true));
    out.push_str(title);
    out.push_str(&format!(" (Vol. {}). ", volume));
    out.push_str(publisher);
    out.push('.');
    out
}

fn fmt_thesis_apa(
    kind: ThesisKind,
    authors: Vec<OwnedFullName>,
    fields: BTreeMap<String, String>,
) -> String {
    let mut out = String::new();
    let title = fields.get("title").unwrap();
    let year = fields.get("year");
    let month = fields.get("month");
    let school = fields.get("school").unwrap();
    out.push_str(&fmt_authors_apa(authors));
    out.push(' ');
    out.push_str(&fmt_year_month_apa(year, month, true));
    out.push_str(title);
    out.push(' ');
    match kind {
        ThesisKind::Phd => out.push_str(&format!("[Doctoral dissertation, {}].", school)),
        ThesisKind::Msc => out.push_str(&format!("[Master's thesis, {}].", school)),
    };
    out
}

fn fmt_misc_apa(authors: Vec<OwnedFullName>, fields: BTreeMap<String, String>) -> String {
    let mut out = String::new();
    let title = fields.get("title").unwrap();
    let year = fields.get("year");
    let month = fields.get("month");
    out.push_str(&fmt_authors_apa(authors));
    out.push_str(". ");
    out.push_str(&fmt_year_month_apa(year, month, true));
    out.push_str(title);
    match fields.get("note") {
        Some(n) => out.push_str(&format!(" [{}]", n)),
        None => (),
    };
    out.push('.');

    out
}

fn fmt_manual_apa(authors: Vec<OwnedFullName>, fields: BTreeMap<String, String>) -> String {
    let mut out = String::new();
    let title = fields.get("title").unwrap();
    let year = fields.get("year");
    let month = fields.get("month");
    let organization = fields.get("organization").unwrap();
    let address = fields.get("address").unwrap();
    out.push_str(&fmt_authors_apa(authors));
    out.push_str(". ");
    out.push_str(&fmt_year_month_apa(year, month, true));
    out.push_str(title);
    out.push_str(". ");
    out.push_str(&format!("{}. ", organization));
    out.push_str(&format!("{}.", address));

    out
}

fn fmt_inproceedings_apa(authors: Vec<OwnedFullName>, fields: BTreeMap<String, String>) -> String {
    let mut out = String::new();
    let title = fields.get("title").unwrap();
    let year = fields.get("year");
    let month = fields.get("month");
    let booktitle = fields.get("booktitle").unwrap();
    let pages = fields.get("pages").unwrap();
    out.push_str(&fmt_authors_apa(authors));
    out.push_str(" ");
    out.push_str(&fmt_year_month_apa(year, month, true));
    out.push_str(title);
    out.push_str(". ");
    out.push_str(booktitle);
    out.push_str(", ");
    out.push_str(pages);
    out.push_str(".");

    out
}

fn fmt_incollection_apa(authors: Vec<OwnedFullName>, fields: BTreeMap<String, String>) -> String {
    let mut out = String::new();
    let title = fields.get("title").unwrap();
    let year = fields.get("year");
    let month = fields.get("month");
    let booktitle = fields.get("booktitle").unwrap();
    let pages = fields.get("pages").unwrap();
    let editors_str = fields.get("editor").unwrap();
    let (_tail, edrs) = and_seperated_names(editors_str).unwrap();
    let editor_names: Vec<OwnedFullName> = edrs.into_iter().map(|n| n.into()).collect();
    let publisher = fields.get("publisher").unwrap();
    out.push_str(&fmt_authors_apa(authors));
    out.push_str(" ");
    out.push_str(&fmt_year_month_apa(year, month, true));
    out.push_str(title);
    out.push_str(". In ");
    out.push_str(&fmt_editors_apa(editor_names));
    out.push_str(" (Eds.), ");
    out.push_str(booktitle);
    out.push_str(&format!(" (pp. {}). ", pages));
    out.push_str(publisher);
    out.push_str(".");

    out
}

fn fmt_inbook_apa(authors: Vec<OwnedFullName>, fields: BTreeMap<String, String>) -> String {
    let mut out = String::new();
    let title = fields.get("title").unwrap();
    let year = fields.get("year");
    let month = fields.get("month");
    let booktitle = fields.get("booktitle").unwrap();
    let publisher = fields.get("publisher").unwrap();
    let pages = fields.get("pages").unwrap();
    out.push_str(&fmt_authors_apa(authors));
    out.push_str(" ");
    out.push_str(&fmt_year_month_apa(year, month, true));
    out.push_str(title);
    out.push_str(". In ");
    out.push_str(booktitle);
    out.push_str(&format!(" (pp. {}). ", pages));
    out.push_str(&publisher);
    out.push_str(".");

    out
}

fn fmt_conference_apa(authors: Vec<OwnedFullName>, fields: BTreeMap<String, String>) -> String {
    let mut out = String::new();
    let title = fields.get("title").unwrap();
    let year = fields.get("year");
    let month = fields.get("month");
    out.push_str(&fmt_authors_apa(authors));
    out.push_str(" ");
    out.push_str(&fmt_year_month_apa(year, month, true));
    out.push_str(title);
    out.push_str(". ");

    out
}

fn fmt_booklet_apa(authors: Vec<OwnedFullName>, fields: BTreeMap<String, String>) -> String {
    let mut out = String::new();
    let title = fields.get("title").unwrap();
    let year = fields.get("year");
    let month = fields.get("month");
    let howpublished = fields.get("howpublished").unwrap();

    out.push_str(&fmt_authors_apa(authors));
    out.push_str(" ");
    out.push_str(title);
    out.push_str(". ");
    out.push_str(howpublished);
    out.push_str(". ");
    out.push_str(&fmt_year_month_apa(year, month, false));

    out
}

fn fmt_book_apa(authors: Vec<OwnedFullName>, fields: BTreeMap<String, String>) -> String {
    let mut out = String::new();
    let title = fields.get("title").unwrap();
    let year = fields.get("year");
    let month = fields.get("month");
    let publisher = fields.get("publisher").unwrap();
    out.push_str(&fmt_authors_apa(authors));
    out.push_str(" ");
    out.push_str(&fmt_year_month_apa(year, month, true));
    out.push_str(title);
    out.push_str(". ");
    out.push_str(&publisher);
    out.push_str(".");

    out
}

fn fmt_year_month_apa(year: Option<&String>, month: Option<&String>, braces: bool) -> String {
    let mut out = String::new();
    if braces {
        out.push_str("(");
    };
    match (year, month) {
        (None, None) => out.push_str("n.d."),
        (None, Some(_)) => out.push_str("n.d."),

        (Some(y), None) => {
            out.push_str(y);
        }
        (Some(y), Some(m)) => {
            // years generally don't get represented as anything other than number so unwrapping here is fine
            let y_parsed = y.parse::<i32>().unwrap();
            let m_parsed = m.parse::<u32>();
            let date_formatted = match m_parsed {
                Ok(m) => {
                    let date = NaiveDate::from_ymd_opt(y_parsed, m, 1).unwrap();
                    date.format("%B").to_string()
                }
                // if it's not a number just capitalise the first letter
                Err(_) => m[0..1].to_uppercase() + &m[1..],
            };
            out.push_str(y);
            out.push_str(", ");
            out.push_str(&date_formatted);
        }
    };
    if braces {
        out.push_str("). ");
    } else {
        out.push_str(".");
    };

    out
}

fn fmt_article_apa(authors: Vec<OwnedFullName>, fields: BTreeMap<String, String>) -> String {
    let title = fields.get("title").unwrap().clone();
    let volume = fields.get("volume").unwrap_or(&String::new()).clone();
    let pages = fields.get("pages");
    let number = fields.get("number").unwrap_or(&String::new()).clone();
    let journal = fields.get("journal").unwrap_or(&String::new()).clone();
    let year = fields.get("year").map(|s| s.to_string());
    let doi = fields.get("doi");
    let mut out = String::new();
    out.push_str(&fmt_authors_apa(authors.clone()));
    out.push(' ');
    out.push_str(&fmt_pub_date_apa(year));
    out.push(' ');
    out.push_str(&fmt_title_apa(title));
    out.push(' ');
    out.push_str(&fmt_journal_apa(journal));
    out.push(' ');
    out.push_str(&fmt_vol_issue_apa(volume, number));
    out.push(' ');
    if let Some(p) = pages {
        out.push_str(&fmt_pages_apa(p));
    };

    if let Some(d) = doi {
        if pages.is_some() {
            out.push(' ');
        }
        out.push_str(d);
    }

    out
}

fn fmt_pages_apa(pages: &String) -> String {
    format!("{}.", pages)
}
fn fmt_journal_apa(journal: String) -> String {
    format!("{},", journal)
}
fn fmt_vol_issue_apa(vol: String, number: String) -> String {
    format!("{} ({}),", vol, number)
}
fn fmt_authors_apa(mut authors: Vec<OwnedFullName>) -> String {
    match &authors.len() {
        0 => String::new(),
        1 => {
            let author = authors.remove(0);
            fmt_single_author_apa(author)
        }
        2 => {
            let author1 = authors.remove(0);
            let author2 = authors.remove(0);
            format!(
                "{}, & {}",
                fmt_single_author_apa(author1),
                fmt_single_author_apa(author2)
            )
        }
        3..=21 => {
            let last_author = authors.remove(authors.len() - 1);
            format!(
                "{}, & {}",
                authors
                    .into_iter()
                    .map(fmt_single_author_apa)
                    .collect::<Vec<String>>()
                    .join(", "),
                fmt_single_author_apa(last_author)
            )
        }
        22.. => {
            let last_author = authors.remove(authors.len() - 1);
            let listed_authors = authors.drain(0..19);
            format!(
                "{},...{}",
                listed_authors
                    .into_iter()
                    .map(fmt_single_author_apa)
                    .collect::<Vec<String>>()
                    .join(", "),
                fmt_single_author_apa(last_author)
            )
        }
    }
}
fn fmt_editors_apa(mut authors: Vec<OwnedFullName>) -> String {
    match &authors.len() {
        0 => String::new(),
        1 => {
            let author = authors.remove(0);
            fmt_single_editor_apa(author)
        }
        2 => {
            let author1 = authors.remove(0);
            let author2 = authors.remove(0);
            format!(
                "{} & {}",
                fmt_single_editor_apa(author1),
                fmt_single_editor_apa(author2)
            )
        }
        3..=21 => {
            let last_author = authors.remove(authors.len() - 1);
            format!(
                "{} & {}",
                authors
                    .into_iter()
                    .map(fmt_single_author_apa)
                    .collect::<Vec<String>>()
                    .join(" "),
                fmt_single_editor_apa(last_author)
            )
        }
        22.. => {
            let last_author = authors.remove(authors.len() - 1);
            let listed_authors = authors.drain(0..19);
            format!(
                "{},...{}",
                listed_authors
                    .into_iter()
                    .map(fmt_single_author_apa)
                    .collect::<Vec<String>>()
                    .join(" "),
                fmt_single_editor_apa(last_author)
            )
        }
    }
}

fn fmt_pub_date_apa(year: Option<String>) -> String {
    format!("({}).", year.unwrap_or_else(|| "n.d.".to_string()))
}
fn fmt_title_apa(title: String) -> String {
    format!("{}.", title)
}

fn fmt_single_author_apa(name: OwnedFullName) -> String {
    let mut out = String::new();
    if !name.last.is_empty() {
        out.push_str(&name.last.join(" "));
    }
    if !name.first.is_empty() {
        if !name.last.is_empty() {
            out.push_str(", ");
        }
        out.push_str(
            &name
                .first
                .iter()
                .map(|n| format!("{}.", n.graphemes(true).next().unwrap()))
                .collect::<Vec<String>>()
                .join(" "),
        )
    };
    out
}
fn fmt_single_editor_apa(name: OwnedFullName) -> String {
    let mut out = String::new();
    if !name.first.is_empty() {
        out.push_str(
            &name
                .first
                .iter()
                .map(|n| format!("{}.", n.graphemes(true).next().unwrap()))
                .collect::<Vec<String>>()
                .join(" "),
        )
    };
    if !name.last.is_empty() {
        if !name.first.is_empty() {
            out.push_str(" ");
        }
        out.push_str(&name.last.join(" "));
    }
    out
}
#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::*;
    use crate::parsing::entry::parse_bib_file;
    use anyhow::Result;

    #[test]
    fn single_author_fmt() -> Result<()> {
        let author = OwnedFullName {
            first: vec!["Ada".to_string(), "Maria".to_string()],
            last: vec!["Lovelace".to_string(), "Augusta".to_string()],
            von: vec![],
            title: vec![],
        };
        let formated = fmt_authors_apa(vec![author]);
        assert_eq!(formated, "Lovelace Augusta, A. M.");

        Ok(())
    }
    #[test]
    fn dual_authors_fmt() -> Result<()> {
        let authors = vec![
            OwnedFullName {
                first: vec!["Ada".to_string(), "Maria".to_string()],
                last: vec!["Lovelace".to_string(), "Augusta".to_string()],
                von: vec![],
                title: vec![],
            },
            OwnedFullName {
                first: vec!["Amalie".to_string(), "Emmy".to_string()],
                last: vec!["Noether".to_string()],
                von: vec![],
                title: vec![],
            },
        ];
        let formated = fmt_authors_apa(authors);
        assert_eq!(formated, "Lovelace Augusta, A. M., & Noether, A. E.");

        Ok(())
    }
    #[test]
    fn triple_authors_fmt() -> Result<()> {
        let authors = vec![
            OwnedFullName {
                first: vec!["Ada".to_string(), "Maria".to_string()],
                last: vec!["Lovelace".to_string(), "Augusta".to_string()],
                von: vec![],
                title: vec![],
            },
            OwnedFullName {
                first: vec!["Amalie".to_string(), "Emmy".to_string()],
                last: vec!["Noether".to_string()],
                von: vec![],
                title: vec![],
            },
            OwnedFullName {
                first: vec!["Sophie".to_string()],
                last: vec!["Germain".to_string()],
                von: vec![],
                title: vec![],
            },
        ];
        let formated = fmt_authors_apa(authors);
        assert_eq!(
            formated,
            "Lovelace Augusta, A. M., Noether, A. E., & Germain, S."
        );

        Ok(())
    }
    #[test]
    fn twenty_four_authors_fmt() -> Result<()> {
        let authors = vec![
            OwnedFullName {
                first: vec!["Ada".to_string(), "Maria".to_string()],
                last: vec!["Lovelace".to_string(), "Augusta".to_string()],
                von: vec![],
                title: vec![],
            },
            OwnedFullName {
                first: vec!["Amalie".to_string(), "Emmy".to_string()],
                last: vec!["Noether".to_string()],
                von: vec![],
                title: vec![],
            },
            OwnedFullName {
                first: vec!["Sophie".to_string()],
                last: vec!["Germain".to_string()],
                von: vec![],
                title: vec![],
            },
            OwnedFullName {
                first: vec!["Sofia".to_string()],
                last: vec!["Kovalevskaya".to_string()],
                von: vec![],
                title: vec![],
            },
            OwnedFullName {
                first: vec!["Dorothy".to_string()],
                last: vec!["Vaughn".to_string()],
                von: vec![],
                title: vec![],
            },
            OwnedFullName {
                first: vec!["Maryam".to_string()],
                last: vec!["Mirzakhani".to_string()],
                von: vec![],
                title: vec![],
            },
            OwnedFullName {
                first: vec!["Ada".to_string(), "Maria".to_string()],
                last: vec!["Lovelace".to_string(), "Augusta".to_string()],
                von: vec![],
                title: vec![],
            },
            OwnedFullName {
                first: vec!["Amalie".to_string(), "Emmy".to_string()],
                last: vec!["Noether".to_string()],
                von: vec![],
                title: vec![],
            },
            OwnedFullName {
                first: vec!["Sophie".to_string()],
                last: vec!["Germain".to_string()],
                von: vec![],
                title: vec![],
            },
            OwnedFullName {
                first: vec!["Sofia".to_string()],
                last: vec!["Kovalevskaya".to_string()],
                von: vec![],
                title: vec![],
            },
            OwnedFullName {
                first: vec!["Dorothy".to_string()],
                last: vec!["Vaughn".to_string()],
                von: vec![],
                title: vec![],
            },
            OwnedFullName {
                first: vec!["Maryam".to_string()],
                last: vec!["Mirzakhani".to_string()],
                von: vec![],
                title: vec![],
            },
            OwnedFullName {
                first: vec!["Ada".to_string(), "Maria".to_string()],
                last: vec!["Lovelace".to_string(), "Augusta".to_string()],
                von: vec![],
                title: vec![],
            },
            OwnedFullName {
                first: vec!["Amalie".to_string(), "Emmy".to_string()],
                last: vec!["Noether".to_string()],
                von: vec![],
                title: vec![],
            },
            OwnedFullName {
                first: vec!["Sophie".to_string()],
                last: vec!["Germain".to_string()],
                von: vec![],
                title: vec![],
            },
            OwnedFullName {
                first: vec!["Sofia".to_string()],
                last: vec!["Kovalevskaya".to_string()],
                von: vec![],
                title: vec![],
            },
            OwnedFullName {
                first: vec!["Dorothy".to_string()],
                last: vec!["Vaughn".to_string()],
                von: vec![],
                title: vec![],
            },
            OwnedFullName {
                first: vec!["Maryam".to_string()],
                last: vec!["Mirzakhani".to_string()],
                von: vec![],
                title: vec![],
            },
            OwnedFullName {
                first: vec!["Ada".to_string(), "Maria".to_string()],
                last: vec!["Lovelace".to_string(), "Augusta".to_string()],
                von: vec![],
                title: vec![],
            },
            OwnedFullName {
                first: vec!["Amalie".to_string(), "Emmy".to_string()],
                last: vec!["Noether".to_string()],
                von: vec![],
                title: vec![],
            },
            OwnedFullName {
                first: vec!["Sophie".to_string()],
                last: vec!["Germain".to_string()],
                von: vec![],
                title: vec![],
            },
            OwnedFullName {
                first: vec!["Sofia".to_string()],
                last: vec!["Kovalevskaya".to_string()],
                von: vec![],
                title: vec![],
            },
            OwnedFullName {
                first: vec!["Dorothy".to_string()],
                last: vec!["Vaughn".to_string()],
                von: vec![],
                title: vec![],
            },
            OwnedFullName {
                first: vec!["Maryam".to_string()],
                last: vec!["Mirzakhani".to_string()],
                von: vec![],
                title: vec![],
            },
        ];
        let formated = fmt_authors_apa(authors);
        assert_eq!(
            formated,
            "Lovelace Augusta, A. M., Noether, A. E., Germain, S., Kovalevskaya, S., Vaughn, D., Mirzakhani, M., Lovelace Augusta, A. M., Noether, A. E., Germain, S., Kovalevskaya, S., Vaughn, D., Mirzakhani, M., Lovelace Augusta, A. M., Noether, A. E., Germain, S., Kovalevskaya, S., Vaughn, D., Mirzakhani, M., Lovelace Augusta, A. M.,...Mirzakhani, M."
        );

        Ok(())
    }

    #[test]
    fn random_forests_formatted_citation() -> Result<()> {
        let key = "breiman2001random";
        let formatted_citation = "Breiman, L. (2001). Random forests. Machine learning, 45 (1), 5-32. https://doi.org/10.1023/a:1010933404324";
        let entries = parse_bib_file(PathBuf::from("cite.bib"))?;
        let entry = entries.into_iter().find(|e| e.key == key).unwrap();
        let citation = fmt_reference_apa(entry);
        assert_eq!(citation, formatted_citation);
        Ok(())
    }
    #[test]
    fn bacterial_formatted_citation() -> Result<()> {
        let key = "10.1093/femsec/fiw174";
        let formatted_citation= "Liao, J., Cao, X., Zhao, L., Wang, J., Gao, Z., Wang, M. C., & Huang, Y. (2016). The importance of neutral and niche processes for bacterial community assembly differs between habitat generalists and specialists. FEMS Microbiology Ecology, 92 (11), https://doi.org/10.1093/femsec/fiw174";
        let entries = parse_bib_file(PathBuf::from("cite.bib"))?;
        let entry = entries.into_iter().find(|e| e.key == key).unwrap();
        let citation = fmt_reference_apa(entry);
        assert_eq!(citation, formatted_citation);
        Ok(())
    }
    #[test]
    fn article_formatted_citation() -> Result<()> {
        let key = "article";
        let formatted_citation= "Cohen, P. J. (1963). The independence of the continuum hypothesis. Proceedings of the National Academy of Sciences, 50 (6), 1143-1148.";
        let entries = parse_bib_file(PathBuf::from("cite.bib"))?;
        let entry = entries.into_iter().find(|e| e.key == key).unwrap();
        let citation = fmt_reference_apa(entry);
        assert_eq!(citation, formatted_citation);
        Ok(())
    }
    #[test]
    fn book_formatted_citation() -> Result<()> {
        let key = "book";
        let formatted_citation= "Susskind, L., & Hrabovsky, G. (2014). Classical mechanics: the theoretical minimum. Penguin Random House.";
        let entries = parse_bib_file(PathBuf::from("cite.bib"))?;
        let entry = entries.into_iter().find(|e| e.key == key).unwrap();
        let citation = fmt_reference_apa(entry);
        assert_eq!(citation, formatted_citation);
        Ok(())
    }
    #[test]
    fn booklet_formatted_citation() -> Result<()> {
        let key = "booklet";
        let formatted_citation= "Swetla, M. Canoe tours in Sweden. Distributed at the Stockholm Tourist Office. 2015, July.";
        let entries = parse_bib_file(PathBuf::from("cite.bib"))?;
        let entry = entries.into_iter().find(|e| e.key == key).unwrap();
        let citation = fmt_reference_apa(entry);
        assert_eq!(citation, formatted_citation);
        Ok(())
    }
    #[test]
    fn inbook_formatted_citation() -> Result<()> {
        let key = "inbook";
        let formatted_citation= "Urry, L. A., Cain, M. L., Wasserman, S. A., Minorsky, P. V., & Reece, J. B. (2016). Photosynthesis. In Campbell biology (pp. 187-221). Pearson.";
        let entries = parse_bib_file(PathBuf::from("cite.bib"))?;
        let entry = entries.into_iter().find(|e| e.key == key).unwrap();
        let citation = fmt_reference_apa(entry);
        assert_eq!(citation, formatted_citation);
        Ok(())
    }
    #[test]
    fn incollection_formatted_citation() -> Result<()> {
        let key = "incollection";
        let formatted_citation= "Shapiro, H. M. (2018). Flow cytometry: The glass is half full. In T. S. Hawley & R. G. Hawley (Eds.), Flow cytometry protocols (pp. 1-10). Springer.";
        let entries = parse_bib_file(PathBuf::from("cite.bib"))?;
        let entry = entries.into_iter().find(|e| e.key == key).unwrap();
        let citation = fmt_reference_apa(entry);
        assert_eq!(citation, formatted_citation);
        Ok(())
    }
    #[test]
    fn inprocedings_formatted_citation() -> Result<()> {
        let key = "inproceedings";
        let formatted_citation= "Holleis, P., Wagner, M., & Koolwaaij, J. (2010). Studying mobile context-aware social services in the wild. Proc. of the 6th Nordic Conf. on Human-Computer Interaction, 207-216.";
        let entries = parse_bib_file(PathBuf::from("cite.bib"))?;
        let entry = entries.into_iter().find(|e| e.key == key).unwrap();
        let citation = fmt_reference_apa(entry);
        assert_eq!(citation, formatted_citation);
        Ok(())
    }
    #[test]
    fn manual_formatted_citation() -> Result<()> {
        let key = "manual";
        let formatted_citation= "R Core Team. (2018). R: A language and environment for statistical computing. R Foundation for Statistical Computing. Vienna, Austria.";
        let entries = parse_bib_file(PathBuf::from("cite.bib"))?;
        let entry = entries.into_iter().find(|e| e.key == key).unwrap();
        let citation = fmt_reference_apa(entry);
        assert_eq!(citation, formatted_citation);
        Ok(())
    }
    #[test]
    fn mastersthesis_formatted_citation() -> Result<()> {
        let key = "mastersthesis";
        let formatted_citation= "Tang, J. (1996, September). Spin structure of the nucleon in the asymptotic limit [Master's thesis, Massachusetts Institute of Technology].";
        let entries = parse_bib_file(PathBuf::from("cite.bib"))?;
        let entry = entries.into_iter().find(|e| e.key == key).unwrap();
        let citation = fmt_reference_apa(entry);
        assert_eq!(citation, formatted_citation);
        Ok(())
    }
    #[test]
    fn misc_formatted_citation() -> Result<()> {
        let key = "misc";
        let formatted_citation =
            "NASA. (2015). Pluto: The 'other' red planet [Accessed: 2018-12-06].";
        let entries = parse_bib_file(PathBuf::from("cite.bib"))?;
        let entry = entries.into_iter().find(|e| e.key == key).unwrap();
        let citation = fmt_reference_apa(entry);
        assert_eq!(citation, formatted_citation);
        Ok(())
    }
    #[test]
    fn phdthesis_formatted_citation() -> Result<()> {
        let key = "phdthesis";
        let formatted_citation= "Rempel, R. C. (1956, June). Relaxation effects for coupled nuclear spins [Doctoral dissertation, Stanford University].";
        let entries = parse_bib_file(PathBuf::from("cite.bib"))?;
        let entry = entries.into_iter().find(|e| e.key == key).unwrap();
        let citation = fmt_reference_apa(entry);
        assert_eq!(citation, formatted_citation);
        Ok(())
    }
    #[test]
    fn proceedings_formatted_citation() -> Result<()> {
        let key = "proceedings";
        let formatted_citation= "Stepney, S., & Verlan, S. (Eds.). (2018). Proceedings of the 17th international conference on computation and natural computation, fontainebleau, france (Vol. 10867). Springer.";
        let entries = parse_bib_file(PathBuf::from("cite.bib"))?;
        let entry = entries.into_iter().find(|e| e.key == key).unwrap();
        let citation = fmt_reference_apa(entry);
        assert_eq!(citation, formatted_citation);
        Ok(())
    }
    #[test]
    fn techreport_formatted_citation() -> Result<()> {
        let key = "techreport";
        let formatted_citation= "Bennett, V., Bowman, K., & Wright, S. (2018, September). Wasatch Solar Project final report (tech. rep. No. DOE-SLC-6903-1). Salt Lake City Corporation. Salt Lake City, UT.";
        let entries = parse_bib_file(PathBuf::from("cite.bib"))?;
        let entry = entries.into_iter().find(|e| e.key == key).unwrap();
        let citation = fmt_reference_apa(entry);
        assert_eq!(citation, formatted_citation);
        Ok(())
    }
    #[test]
    fn unpublished_formatted_citation() -> Result<()> {
        let key = "unpublished";
        let formatted_citation = "Suresh, M. (2006). Evolution: A revised theory.";
        let entries = parse_bib_file(PathBuf::from("cite.bib"))?;
        let entry = entries.into_iter().find(|e| e.key == key).unwrap();
        let citation = fmt_reference_apa(entry);
        assert_eq!(citation, formatted_citation);
        Ok(())
    }
}
