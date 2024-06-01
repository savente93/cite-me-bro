use std::collections::BTreeMap;

use crate::parsing::{
    entry::BibEntry,
    names::{and_seperated_names, OwnedFullName},
};
use chrono::prelude::*;
use unicode_segmentation::UnicodeSegmentation;

pub fn fmt_reference_ieee(entry: BibEntry) -> String {
    let (kind, _key, authors, fields) = entry.into_components();

    match kind {
        crate::parsing::entry::EntryType::Article => fmt_article_ieee(authors, fields),
        crate::parsing::entry::EntryType::Book => fmt_book_ieee(authors, fields),
        crate::parsing::entry::EntryType::Booklet => fmt_booklet_ieee(authors, fields),
        crate::parsing::entry::EntryType::Conference => fmt_conference_ieee(authors, fields),
        crate::parsing::entry::EntryType::Inbook => fmt_inbook_ieee(authors, fields),
        crate::parsing::entry::EntryType::Incollection => fmt_incollection_ieee(authors, fields),
        crate::parsing::entry::EntryType::Inproceedings => fmt_inprocedings_ieee(authors, fields),
        crate::parsing::entry::EntryType::Manual => fmt_manual_ieee(authors, fields),
        crate::parsing::entry::EntryType::Mastersthesis => {
            fmt_thesis_ieee(ThesisKind::Msc, authors, fields)
        }
        crate::parsing::entry::EntryType::Misc => fmt_misc_ieee(authors, fields),
        crate::parsing::entry::EntryType::Phdthesis => {
            fmt_thesis_ieee(ThesisKind::Phd, authors, fields)
        }
        // full proceedings don't have authors, only editors, hence only passing fields
        crate::parsing::entry::EntryType::Proceedings => fmt_procedings_ieee(fields),
        crate::parsing::entry::EntryType::Techreport => fmt_tech_report_ieee(authors, fields),
        crate::parsing::entry::EntryType::Unpublished => fmt_unpublished_ieee(authors, fields),
    }
}

enum ThesisKind {
    Phd,
    Msc,
}

fn fmt_book_ieee(authors: Vec<OwnedFullName>, fields: BTreeMap<String, String>) -> String {
    let mut out = String::new();
    let title = fields.get("title").unwrap_or(&String::new()).clone();
    let publisher = fields.get("publisher").unwrap_or(&String::new()).clone();
    let address = fields.get("address").unwrap_or(&String::new()).clone();
    let year = fields.get("year").unwrap_or(&String::new()).clone();
    out.push_str(&fmt_authors_ieee(authors.clone()));
    out.push_str(", ");
    out.push_str(&title);
    out.push_str(". ");
    out.push_str(&address);
    out.push_str(": ");
    out.push_str(&publisher);
    out.push_str(", ");
    out.push_str(&year);
    out.push('.');
    out
}
fn fmt_booklet_ieee(authors: Vec<OwnedFullName>, fields: BTreeMap<String, String>) -> String {
    let mut out = String::new();
    let title = fields.get("title").unwrap_or(&String::new()).clone();
    let howpublished = fields.get("howpublished").unwrap_or(&String::new()).clone();
    let year = fields.get("year");
    let month = fields.get("month");
    out.push_str(&fmt_authors_ieee(authors.clone()));
    out.push_str(", ");
    out.push_str(&title);
    out.push_str(", ");
    out.push_str(&howpublished);
    out.push(',');
    out.push_str(&fmt_year_month(year, month));
    out.push('.');
    out
}
fn fmt_conference_ieee(authors: Vec<OwnedFullName>, fields: BTreeMap<String, String>) -> String {
    let mut out = String::new();
    let title = fields.get("title").unwrap_or(&String::new()).clone();
    let book_title = fields.get("booktitle").unwrap_or(&String::new()).clone();
    let series = fields.get("series").unwrap_or(&String::new()).clone();
    let publisher = fields.get("publisher").unwrap_or(&String::new()).clone();
    let address = fields.get("address").unwrap_or(&String::new()).clone();
    let pages = fields.get("pages").unwrap_or(&String::new()).clone();
    let year = fields.get("year").unwrap_or(&String::new()).clone();
    out.push_str(&fmt_authors_ieee(authors.clone()));
    out.push_str(", ");
    out.push_str(&fmt_title_ieee(title));
    out.push_str(" in ");
    out.push_str(&book_title);
    out.push_str(", ser. ");
    out.push_str(&series);
    out.push_str(", ");
    out.push_str(&address);
    out.push_str(": ");
    out.push_str(&publisher);
    out.push_str(", ");
    out.push_str(&year);
    out.push_str(", pp. ");
    out.push_str(&pages);
    out.push('.');
    out
}
fn fmt_inbook_ieee(authors: Vec<OwnedFullName>, fields: BTreeMap<String, String>) -> String {
    let mut out = String::new();
    let title = fields.get("title").unwrap_or(&String::new()).clone();
    let book_title = fields.get("booktitle").unwrap_or(&String::new()).clone();
    let series = fields.get("series").unwrap_or(&String::new()).clone();
    let publisher = fields.get("publisher").unwrap_or(&String::new()).clone();
    let address = fields.get("address").unwrap_or(&String::new()).clone();
    let pages = fields.get("pages").unwrap_or(&String::new()).clone();
    let year = fields.get("year").unwrap_or(&String::new()).clone();
    out.push_str(&fmt_authors_ieee(authors.clone()));
    out.push_str(", ");
    out.push_str(&fmt_title_ieee(title));
    out.push_str(" in ");
    out.push_str(&book_title);
    out.push_str(", ser. ");
    out.push_str(&series);
    out.push_str(", ");
    out.push_str(&address);
    out.push_str(": ");
    out.push_str(&publisher);
    out.push_str(", ");
    out.push_str(&year);
    out.push_str(", pp. ");
    out.push_str(&pages);
    out.push('.');
    out
}
fn fmt_incollection_ieee(
    authors: Vec<OwnedFullName>,
    mut fields: BTreeMap<String, String>,
) -> String {
    let mut out = String::new();
    let title = fields.get("title").unwrap_or(&String::new()).clone();
    let book_title = fields.get("booktitle").unwrap_or(&String::new()).clone();
    let publisher = fields.get("publisher").unwrap_or(&String::new()).clone();
    let address = fields.get("address").unwrap_or(&String::new()).clone();
    let pages = fields.get("pages").unwrap_or(&String::new()).clone();
    let year = fields.get("year").unwrap_or(&String::new()).clone();
    out.push_str(&fmt_authors_ieee(authors.clone()));
    out.push_str(", ");
    out.push_str(&fmt_title_ieee(title));
    out.push_str(" in ");
    out.push_str(&book_title);
    out.push_str(", ");
    let editors_str = fields.entry("editor".to_string()).or_default();
    let (_tail, edrs) = and_seperated_names(editors_str).unwrap();
    let editor_names: Vec<OwnedFullName> = edrs.into_iter().map(|n| n.into()).collect();
    out.push_str(&fmt_authors_ieee(editor_names.clone()));
    out.push_str(", Eds., ");
    out.push_str(&address);
    out.push_str(": ");
    out.push_str(&publisher);
    out.push_str(", ");
    out.push_str(&year);
    out.push_str(", pp. ");
    out.push_str(&pages);
    out.push('.');
    out
}
fn fmt_manual_ieee(authors: Vec<OwnedFullName>, fields: BTreeMap<String, String>) -> String {
    let mut out = String::new();
    let title = fields.get("title").unwrap_or(&String::new()).clone();
    let organization = fields.get("organization").unwrap_or(&String::new()).clone();
    let address = fields.get("address").unwrap_or(&String::new()).clone();
    let year = fields.get("year").unwrap_or(&String::new()).clone();
    out.push_str(&fmt_authors_ieee(authors.clone()));
    out.push_str(", ");
    out.push_str(&title);
    out.push_str(", ");
    out.push_str(&organization);
    out.push_str(", ");
    out.push_str(&address);
    out.push_str(", ");
    out.push_str(&year);
    out.push('.');
    out
}
fn fmt_inprocedings_ieee(authors: Vec<OwnedFullName>, fields: BTreeMap<String, String>) -> String {
    let mut out = String::new();
    let title = fields.get("title").unwrap_or(&String::new()).clone();
    let book_title = fields.get("booktitle").unwrap_or(&String::new()).clone();
    let series = fields.get("series").unwrap_or(&String::new()).clone();
    let publisher = fields.get("publisher").unwrap_or(&String::new()).clone();
    let address = fields.get("address").unwrap_or(&String::new()).clone();
    let pages = fields.get("pages").unwrap_or(&String::new()).clone();
    let year = fields.get("year").unwrap_or(&String::new()).clone();
    out.push_str(&fmt_authors_ieee(authors.clone()));
    out.push_str(", ");
    out.push_str(&fmt_title_ieee(title));
    out.push_str(" in ");
    out.push_str(&book_title);
    out.push_str(", ser. ");
    out.push_str(&series);
    out.push_str(", ");
    out.push_str(&address);
    out.push_str(": ");
    out.push_str(&publisher);
    out.push_str(", ");
    out.push_str(&year);
    out.push_str(", pp. ");
    out.push_str(&pages);
    out.push('.');

    out
}
fn fmt_procedings_ieee(mut fields: BTreeMap<String, String>) -> String {
    // J. K. Author, “Title of paper,” presented at the Abbreviated Name of Conf., City of Conf., Abbrev. State, Country, Month and day(s), year, Paper number
    let mut out = String::new();
    let editors_str = fields.entry("editor".to_string()).or_default();
    let (_tail, edrs) = and_seperated_names(editors_str).unwrap();
    let editor_names: Vec<OwnedFullName> = edrs.into_iter().map(|n| n.into()).collect();
    let title = fields.get("title").unwrap_or(&String::new()).clone();
    let volume = fields.get("volume").unwrap_or(&String::new()).clone();
    let series = fields.get("series").unwrap_or(&String::new()).clone();
    let address = fields.get("address").unwrap_or(&String::new()).clone();
    let publisher = fields.get("publisher").unwrap_or(&String::new()).clone();
    let year = fields.get("year").unwrap_or(&String::new()).clone();
    out.push_str(&fmt_authors_ieee(editor_names.clone()));
    out.push_str(", Eds., ");
    out.push_str(&title);
    out.push_str(&format!(", vol. {}, ", volume));
    out.push_str(&series);
    out.push_str(", ");
    out.push_str(&address);
    out.push_str(": ");
    out.push_str(&publisher);
    out.push_str(", ");
    out.push_str(&year);

    out
}
fn fmt_unpublished_ieee(authors: Vec<OwnedFullName>, fields: BTreeMap<String, String>) -> String {
    let mut out = String::new();
    let title = fields.get("title").unwrap_or(&String::new()).clone();
    out.push_str(&fmt_authors_ieee(authors.clone()));
    out.push_str(", ");
    out.push_str(&fmt_title_ieee(title));
    out.push_str(" unpublished.");

    out
}
fn fmt_tech_report_ieee(authors: Vec<OwnedFullName>, fields: BTreeMap<String, String>) -> String {
    let title = fields.get("title").unwrap_or(&String::new()).clone();
    let institution = fields.get("institution").unwrap_or(&String::new()).clone();
    let address = fields.get("address").unwrap_or(&String::new()).clone();

    let number = fields.get("number").unwrap_or(&String::new()).clone();
    let year = fields.get("year");
    let month = fields.get("month");
    let mut out = String::new();
    out.push_str(&fmt_authors_ieee(authors.clone()));
    out.push_str(", ");
    out.push_str(&fmt_title_ieee(title));
    out.push(' ');
    out.push_str(&institution);
    out.push_str(", ");
    out.push_str(&address);
    out.push_str(", Tech. Rep. ");
    out.push_str(&number);
    out.push(',');
    out.push_str(&fmt_year_month(year, month));
    out.push('.');

    out
}

fn fmt_year_month(year: Option<&String>, month: Option<&String>) -> String {
    let mut out = String::new();
    match (year, month) {
        (None, None) => (),
        (None, Some(_)) => (),

        (Some(y), None) => {
            out.push(' ');
            out.push_str(y);
        }
        (Some(y), Some(m)) => {
            out.push(' ');
            // years generally don't get represented as anything other than number so unwrapping here is fine
            let y_parsed = y.parse::<i32>().unwrap();
            let m_parsed = m.parse::<u32>();
            let date_formatted = match m_parsed {
                Ok(m) => {
                    let date = NaiveDate::from_ymd_opt(y_parsed, m, 1).unwrap();
                    date.format("%b").to_string()
                }
                // if it's not a number just capitalise the first letter
                Err(_) => m[0..1].to_uppercase() + &m[1..],
            };
            out.push_str(&date_formatted);
            out.push_str(". ");
            out.push_str(y);
        }
    };

    out
}

fn fmt_article_ieee(authors: Vec<OwnedFullName>, fields: BTreeMap<String, String>) -> String {
    let title = fields.get("title").unwrap_or(&String::new()).clone();
    let volume = fields.get("volume").unwrap_or(&String::new()).clone();
    let pages = fields.get("pages");
    let journal = fields.get("journal").unwrap_or(&String::new()).clone();
    let number = fields.get("number").unwrap_or(&String::new()).clone();
    let year = fields.get("year");
    let month = fields.get("month");
    let doi = fields.get("doi");
    let issn = fields.get("issn");
    let url = fields.get("url");
    let mut out = String::new();
    out.push_str(&fmt_authors_ieee(authors.clone()));
    out.push_str(", ");
    out.push_str(&fmt_title_ieee(title));
    out.push(' ');
    out.push_str(&format!("{}, vol. {}, no. {}", journal, &volume, &number));
    out.push(',');
    if let Some(p) = pages {
        out.push_str(" pp. ");
        out.push_str(p);
        out.push(',');
    }
    out.push_str(&fmt_year_month(year, month));

    if let Some(i) = issn {
        out.push(',');
        out.push_str(" issn: ");
        out.push_str(i);
    };

    if let Some(d) = doi {
        out.push('.');
        out.push_str(" doi: ");
        out.push_str(d);
        out.push('.');
    };

    if let Some(u) = url {
        out.push_str(" [Online]. Available: ");
        out.push_str(u);
        out.push('.');
    };

    if url.is_none() && doi.is_none() && issn.is_none() {
        out.push('.')
    }

    out
}

fn fmt_thesis_ieee(
    theis_kind: ThesisKind,
    authors: Vec<OwnedFullName>,
    fields: BTreeMap<String, String>,
) -> String {
    let title = fields.get("title").unwrap_or(&String::new()).clone();
    let year = fields.get("year");
    let month = fields.get("month");
    let school = fields.get("school");
    let address = fields.get("address");
    let mut out = String::new();
    out.push_str(&fmt_authors_ieee(authors.clone()));
    out.push_str(", ");
    out.push_str(&fmt_title_ieee(title));
    out.push(' ');
    out.push_str(match theis_kind {
        ThesisKind::Phd => "Ph.D. dissertation, ",
        ThesisKind::Msc => "M.S. thesis, ",
    });
    if let Some(s) = school {
        out.push_str(s);
        out.push(',');
    }
    if let Some(a) = address {
        out.push(' ');
        out.push_str(a);
        out.push(',');
    }
    out.push_str(&fmt_year_month(year, month));
    out.push('.');

    out
}

fn fmt_misc_ieee(authors: Vec<OwnedFullName>, fields: BTreeMap<String, String>) -> String {
    let title = fields.get("title").unwrap_or(&String::new()).clone();
    let year = fields.get("year");
    let howpublished = fields.get("howpublished");
    let note = fields.get("note");
    let mut out = String::new();
    dbg!(&authors);
    out.push_str(&fmt_authors_ieee(authors.clone()));
    out.push_str(", ");
    out.push_str(&title);

    if let Some(u) = howpublished {
        out.push_str(", ");
        out.push_str(u);
    };
    if let Some(n) = note {
        out.push_str(", ");
        out.push_str(n);
    };
    if let Some(y) = year {
        out.push_str(", ");
        out.push_str(y);
        out.push('.');
    };

    out
}

fn fmt_single_author_ieee(name: OwnedFullName) -> String {
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
            out.push(' ');
        }
        out.push_str(&name.last.join(" "));
    }
    out
}

fn fmt_authors_ieee(mut authors: Vec<OwnedFullName>) -> String {
    match &authors.len() {
        0 => String::new(),
        1 => {
            let author = authors.remove(0);
            fmt_single_author_ieee(author)
        }
        2 => {
            let author1 = authors.remove(0);
            let author2 = authors.remove(0);
            format!(
                "{} and {}",
                fmt_single_author_ieee(author1),
                fmt_single_author_ieee(author2)
            )
        }
        3..=6 => {
            let last_author = authors.remove(authors.len() - 1);
            format!(
                "{}, and {}",
                authors
                    .into_iter()
                    .map(fmt_single_author_ieee)
                    .collect::<Vec<String>>()
                    .join(", "),
                fmt_single_author_ieee(last_author)
            )
        }
        7.. => {
            let first_three_authors = authors.drain(0..3);
            format!(
                "{}, et al.",
                (first_three_authors
                    .into_iter()
                    .map(fmt_single_author_ieee)
                    .collect::<Vec<String>>()
                    .join(", "))
            )
        }
    }
}

fn fmt_title_ieee(title: String) -> String {
    format!("\"{},\"", title)
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::*;
    use crate::parsing::entry::parse_bib_file;
    use anyhow::Result;

    #[test]
    fn random_forests_formatted_citation() -> Result<()> {
        let key = "breiman2001random";
        let formatted_citation = "L. Breiman, \"Random forests,\" Machine learning, vol. 45, no. 1, pp. 5-32, 2001. doi: https://doi.org/10.1023/a:1010933404324.";
        let entries = parse_bib_file(PathBuf::from("cite.bib"))?;
        let entry = entries.into_iter().find(|e| e.key == key).unwrap();
        let citation = fmt_reference_ieee(entry);
        assert_eq!(citation, formatted_citation);
        Ok(())
    }
    #[test]
    fn bacterial_formatted_citation() -> Result<()> {
        let key = "10.1093/femsec/fiw174";
        let formatted_citation= "J. Liao, X. Cao, L. Zhao, et al., \"The importance of neutral and niche processes for bacterial community assembly differs between habitat generalists and specialists,\" FEMS Microbiology Ecology, vol. 92, no. 11, Aug. 2016, issn: 0168-6496. doi: https://doi.org/10.1093/femsec/fiw174. [Online]. Available: https://doi.org/10.1093/femsec/fiw174.";
        let entries = parse_bib_file(PathBuf::from("cite.bib"))?;
        let entry = entries.into_iter().find(|e| e.key == key).unwrap();
        let citation = fmt_reference_ieee(entry);
        assert_eq!(citation, formatted_citation);
        Ok(())
    }
    #[test]
    fn article_formatted_citation() -> Result<()> {
        let key = "article";
        let formatted_citation= "P. J. Cohen, \"The independence of the continuum hypothesis,\" Proceedings of the National Academy of Sciences, vol. 50, no. 6, pp. 1143-1148, 1963.";
        let entries = parse_bib_file(PathBuf::from("cite.bib"))?;
        let entry = entries.into_iter().find(|e| e.key == key).unwrap();
        let citation = fmt_reference_ieee(entry);
        assert_eq!(citation, formatted_citation);
        Ok(())
    }
    #[test]
    fn book_formatted_citation() -> Result<()> {
        let key = "book";
        let formatted_citation= "L. Susskind and G. Hrabovsky, Classical mechanics: the theoretical minimum. New York, NY: Penguin Random House, 2014.";
        let entries = parse_bib_file(PathBuf::from("cite.bib"))?;
        let entry = entries.into_iter().find(|e| e.key == key).unwrap();
        let citation = fmt_reference_ieee(entry);
        assert_eq!(citation, formatted_citation);
        Ok(())
    }
    #[test]
    fn booklet_formatted_citation() -> Result<()> {
        let key = "booklet";
        let formatted_citation= "M. Swetla, Canoe tours in Sweden, Distributed at the Stockholm Tourist Office, Jul. 2015.";
        let entries = parse_bib_file(PathBuf::from("cite.bib"))?;
        let entry = entries.into_iter().find(|e| e.key == key).unwrap();
        let citation = fmt_reference_ieee(entry);
        assert_eq!(citation, formatted_citation);
        Ok(())
    }
    #[test]
    fn inbook_formatted_citation() -> Result<()> {
        let key = "inbook";
        let formatted_citation= "L. A. Urry, M. L. Cain, S. A. Wasserman, P. V. Minorsky, and J. B. Reece, \"Photosynthesis,\" in Campbell Biology. New York, NY: Pearson, 2016, pp. 187-221.";
        let entries = parse_bib_file(PathBuf::from("cite.bib"))?;
        let entry = entries.into_iter().find(|e| e.key == key).unwrap();
        let citation = fmt_reference_ieee(entry);
        assert_eq!(citation, formatted_citation);
        Ok(())
    }
    #[test]
    fn incollection_formatted_citation() -> Result<()> {
        let key = "incollection";
        let formatted_citation= "H. M. Shapiro, \"Flow cytometry: The glass is half full,\" in Flow Cytometry Protocols, T. S. Hawley and R. G. Hawley, Eds., New York, NY: Springer, 2018, pp. 1-10.";
        let entries = parse_bib_file(PathBuf::from("cite.bib"))?;
        let entry = entries.into_iter().find(|e| e.key == key).unwrap();
        let citation = fmt_reference_ieee(entry);
        assert_eq!(citation, formatted_citation);
        Ok(())
    }
    #[test]
    fn inprocedings_formatted_citation() -> Result<()> {
        let key = "inproceedings";
        let formatted_citation= "P. Holleis, M. Wagner, and J. Koolwaaij, \"Studying mobile context-aware social services in the wild,\" in Proc. of the 6th Nordic Conf. on Human-Computer Interaction, ser. NordiCHI, New York, NY: ACM, 2010, pp. 207-216.";
        let entries = parse_bib_file(PathBuf::from("cite.bib"))?;
        let entry = entries.into_iter().find(|e| e.key == key).unwrap();
        let citation = fmt_reference_ieee(entry);
        assert_eq!(citation, formatted_citation);
        Ok(())
    }
    #[test]
    fn manual_formatted_citation() -> Result<()> {
        let key = "manual";
        let formatted_citation= "R Core Team, R: A language and environment for statistical computing, R Foundation for Statistical Computing, Vienna, Austria, 2018.";
        let entries = parse_bib_file(PathBuf::from("cite.bib"))?;
        let entry = entries.into_iter().find(|e| e.key == key).unwrap();
        let citation = fmt_reference_ieee(entry);
        assert_eq!(citation, formatted_citation);
        Ok(())
    }
    #[test]
    fn mastersthesis_formatted_citation() -> Result<()> {
        let key = "mastersthesis";
        let formatted_citation= "J. Tang, \"Spin structure of the nucleon in the asymptotic limit,\" M.S. thesis, Massachusetts Institute of Technology, Cambridge, MA, Sep. 1996.";
        let entries = parse_bib_file(PathBuf::from("cite.bib"))?;
        let entry = entries.into_iter().find(|e| e.key == key).unwrap();
        let citation = fmt_reference_ieee(entry);
        assert_eq!(citation, formatted_citation);
        Ok(())
    }
    #[test]
    fn misc_formatted_citation() -> Result<()> {
        let key = "misc";
        let formatted_citation= "NASA, Pluto: The 'other' red planet, https://www.nasa.gov/nh/pluto-the-other-red-planet, Accessed: 2018-12-06, 2015.";
        let entries = parse_bib_file(PathBuf::from("cite.bib"))?;
        let entry = entries.into_iter().find(|e| e.key == key).unwrap();
        let citation = fmt_reference_ieee(entry);
        assert_eq!(citation, formatted_citation);
        Ok(())
    }
    #[test]
    fn phdthesis_formatted_citation() -> Result<()> {
        let key = "phdthesis";
        let formatted_citation= "R. C. Rempel, \"Relaxation effects for coupled nuclear spins,\" Ph.D. dissertation, Stanford University, Stanford, CA, Jun. 1956.";
        let entries = parse_bib_file(PathBuf::from("cite.bib"))?;
        let entry = entries.into_iter().find(|e| e.key == key).unwrap();
        let citation = fmt_reference_ieee(entry);
        assert_eq!(citation, formatted_citation);
        Ok(())
    }
    #[test]
    fn proceedings_formatted_citation() -> Result<()> {
        let key = "proceedings";
        let formatted_citation= "S. Stepney and S. Verlan, Eds., Proceedings of the 17th International Conference on Computation and Natural Computation, Fontainebleau, France, vol. 10867, Lecture Notes in Computer Science, Cham, Switzerland: Springer, 2018";
        let entries = parse_bib_file(PathBuf::from("cite.bib"))?;
        let entry = entries.into_iter().find(|e| e.key == key).unwrap();
        let citation = fmt_reference_ieee(entry);
        assert_eq!(citation, formatted_citation);
        Ok(())
    }
    #[test]
    fn techreport_formatted_citation() -> Result<()> {
        let key = "techreport";
        let formatted_citation= "V. Bennett, K. Bowman, and S. Wright, \"Wasatch Solar Project final report,\" Salt Lake City Corporation, Salt Lake City, UT, Tech. Rep. DOE-SLC-6903-1, Sep. 2018.";
        let entries = parse_bib_file(PathBuf::from("cite.bib"))?;
        let entry = entries.into_iter().find(|e| e.key == key).unwrap();
        let citation = fmt_reference_ieee(entry);
        assert_eq!(citation, formatted_citation);
        Ok(())
    }
    #[test]
    fn unpublished_formatted_citation() -> Result<()> {
        let key = "unpublished";
        let formatted_citation = "M. Suresh, \"Evolution: A revised theory,\" unpublished.";
        let entries = parse_bib_file(PathBuf::from("cite.bib"))?;
        let entry = entries.into_iter().find(|e| e.key == key).unwrap();
        let citation = fmt_reference_ieee(entry);
        assert_eq!(citation, formatted_citation);
        Ok(())
    }
    #[test]
    fn single_author_fmt() -> Result<()> {
        let author = OwnedFullName {
            first: vec!["Ada".to_string(), "Maria".to_string()],
            last: vec!["Lovelace".to_string(), "Augusta".to_string()],
            von: vec![],
            title: vec![],
        };
        let formated = fmt_authors_ieee(vec![author]);
        assert_eq!(formated, "A. M. Lovelace Augusta");

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
        let formated = fmt_authors_ieee(authors);
        assert_eq!(formated, "A. M. Lovelace Augusta and A. E. Noether");

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
        let formated = fmt_authors_ieee(authors);
        assert_eq!(
            formated,
            "A. M. Lovelace Augusta, A. E. Noether, and S. Germain"
        );

        Ok(())
    }
    #[test]
    fn six_authors_fmt() -> Result<()> {
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
        ];
        let formated = fmt_authors_ieee(authors);
        assert_eq!(
            formated,
            "A. M. Lovelace Augusta, A. E. Noether, S. Germain, S. Kovalevskaya, D. Vaughn, and M. Mirzakhani"
        );

        Ok(())
    }
    #[test]
    fn seven_plus_authors_fmt() -> Result<()> {
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
                first: vec!["Mae".to_string(), "Carol".to_string()],
                last: vec!["Jemison".to_string()],
                von: vec![],
                title: vec![],
            },
            OwnedFullName {
                first: vec!["Julia".to_string()],
                last: vec!["Robinson".to_string()],
                von: vec![],
                title: vec![],
            },
            OwnedFullName {
                first: vec!["Katherine".to_string()],
                last: vec!["Johnson".to_string()],
                von: vec![],
                title: vec![],
            },
        ];
        let formated = fmt_authors_ieee(authors);
        assert_eq!(
            formated,
            "A. M. Lovelace Augusta, A. E. Noether, S. Germain, et al."
        );

        Ok(())
    }
}
