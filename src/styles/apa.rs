use crate::parsing::{entry::BibEntry, names::OwnedFullName};
use unicode_segmentation::UnicodeSegmentation;

pub fn fmt_reference_apa(entry: BibEntry) -> String {
    let (_kind, _key, authors, fields) = entry.into_components();

    let title = fields.get("title").unwrap().clone();
    let volume = fields.get("volume").unwrap_or(&String::new()).clone();
    let pages = fields.get("pages").unwrap_or(&String::new()).clone();
    let number = fields.get("number").unwrap_or(&String::new()).clone();
    let journal = fields.get("journal").unwrap_or(&String::new()).clone();
    let year = fields.get("year").map(|s| s.to_string());
    let doi = fields.get("doi").unwrap_or(&String::new()).clone();
    format!(
        "{} {} {} {} {} {} {}",
        fmt_authors_apa(authors),
        fmt_pub_date_apa(year),
        fmt_title_apa(title),
        fmt_journal_apa(journal),
        fmt_vol_issue_apa(volume, number),
        fmt_pages_apa(pages),
        fmt_doi_apa(doi),
    )
}

fn fmt_pages_apa(pages: String) -> String {
    format!("{}.", pages)
}
fn fmt_doi_apa(doi: String) -> String {
    format!("{}", doi)
}
fn fmt_journal_apa(journal: String) -> String {
    format!("{},", journal)
}
fn fmt_vol_issue_apa(vol: String, number: String) -> String {
    format!("{}({}),", vol, number)
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
                "{} & {}",
                fmt_single_author_apa(author1),
                fmt_single_author_apa(author2)
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

fn fmt_pub_date_apa(year: Option<String>) -> String {
    format!("({}).", year.unwrap_or_else(|| "n.d.".to_string()))
}
fn fmt_title_apa(title: String) -> String {
    format!("{}.", title)
}

fn fmt_single_author_apa(name: OwnedFullName) -> String {
    format!(
        "{}, {}",
        name.last.join(" "),
        name.first
            .iter()
            .map(|n| format!("{}.", n.graphemes(true).next().unwrap()))
            .collect::<Vec<String>>()
            .join(" "),
    )
}
#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use super::*;
    use crate::parsing::entry::EntryType;
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
        assert_eq!(formated, "Lovelace Augusta, A. M. & Noether, A. E.");

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
            "Lovelace Augusta, A. M., Noether, A. E. & Germain, S."
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
    fn test_random_forests_against_externally_generated() -> Result<()> {
        let mut fields = BTreeMap::new();

        fields.insert("journal".to_string(), "Machine learning".to_string());
        fields.insert("pages".to_string(), "5-32".to_string());
        fields.insert("number".to_string(), "1".to_string());
        fields.insert(
            "doi".to_string(),
            "https://doi.org/10.1023/a:1010933404324".to_string(),
        );
        fields.insert("title".to_string(), "Random forests".to_string());
        fields.insert("volume".to_string(), "45".to_string());
        fields.insert("year".to_string(), "2001".to_string());

        let expected = "Breiman, L. (2001). Random forests. Machine learning, 45(1), 5-32. https://doi.org/10.1023/a:1010933404324".to_string();
        let entry = BibEntry {
            kind: EntryType::Article,
            key: "breiman2001random".to_string(),
            authors: vec![OwnedFullName {
                first: vec!["Leo".to_string()],
                last: vec!["Breiman".to_string()],
                von: vec![],
                title: vec![],
            }],
            fields,
        };
        let answer = fmt_reference_apa(entry);

        assert_eq!(answer, expected);

        Ok(())
    }
}
