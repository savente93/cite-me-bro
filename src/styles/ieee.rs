use crate::parsing::{entry::BibEntry, names::OwnedFullName};
use unicode_segmentation::UnicodeSegmentation;

pub fn fmt_reference_ieee(entry: BibEntry) -> String {
    todo!();
}

fn fmt_single_author_ieee(name: OwnedFullName) -> String {
    format!(
        "{} {}",
        name.first
            .iter()
            .map(|n| format!("{}.", n.graphemes(true).next().unwrap()))
            .collect::<Vec<String>>()
            .join(" "),
        name.last.join(" ")
    )
}

fn fmt_author_ieee(mut authors: Vec<OwnedFullName>) -> String {
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
                "{} and {}",
                authors
                    .into_iter()
                    .map(fmt_single_author_ieee)
                    .collect::<Vec<String>>()
                    .join(", "),
                fmt_single_author_ieee(last_author)
            )
        }
        7.. => {
            let author = authors.remove(0);
            format!("{} et al.", fmt_single_author_ieee(author))
        }
    }
}

fn fmt_title_ieee(entry: BibEntry) -> String {
    format!("\"{}\"", entry.fields.get("title").unwrap())
}

pub fn fmt_entry(entry: BibEntry) -> String {
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    #[test]
    fn single_author_fmt() -> Result<()> {
        let author = OwnedFullName {
            first: vec!["Ada".to_string(), "Maria".to_string()],
            last: vec!["Lovelace".to_string(), "Augusta".to_string()],
            von: vec![],
            title: vec![],
        };
        let formated = fmt_author_ieee(vec![author]);
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
        let formated = fmt_author_ieee(authors);
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
        let formated = fmt_author_ieee(authors);
        assert_eq!(
            formated,
            "A. M. Lovelace Augusta, A. E. Noether and S. Germain"
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
        let formated = fmt_author_ieee(authors);
        assert_eq!(
            formated,
            "A. M. Lovelace Augusta, A. E. Noether, S. Germain, S. Kovalevskaya, D. Vaughn and M. Mirzakhani"
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
        let formated = fmt_author_ieee(authors);
        assert_eq!(formated, "A. M. Lovelace Augusta et al.");

        Ok(())
    }
}
