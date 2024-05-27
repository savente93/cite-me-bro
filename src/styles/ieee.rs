use crate::parsing::{entry::BibEntry, names::OwnedFullName};
use unicode_segmentation::UnicodeSegmentation;

pub fn fmt_reference_ieee(entry: BibEntry) -> String {
    let (_kind, _key, authors, fields) = entry.into_components();

    let title = fields.get("title").unwrap_or(&String::new()).clone();
    let volume = fields.get("volume").unwrap_or(&String::new()).clone();
    let pages = fields.get("pages").unwrap_or(&String::new()).clone();
    let journal = fields.get("journal").unwrap_or(&String::new()).clone();
    let year = fields.get("year").unwrap_or(&String::new()).clone();
    let doi = fields.get("doi").unwrap_or(&String::new()).clone();

    format!(
        "{}, {} {}, vol. {}, pp. {}, {}, doi: {}.",
        fmt_authors_ieee(authors.clone()),
        fmt_title_ieee(title),
        journal,
        volume,
        pages,
        year,
        doi
    )
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

fn fmt_title_ieee(title: String) -> String {
    format!("\"{},\"", title)
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use super::*;
    use crate::parsing::entry::EntryType;
    use anyhow::Result;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref FORMATTED_CITATIONS: BTreeMap<&'static str, &'static str> = {
            let mut m = BTreeMap::new();
            m.insert("breiman2001random","L. Breiman, \"Random forests,\" Machine learning, vol. 45, no. 1, pp. 5–32, 2001. doi: https://doi.org/10.1023/a:1010933404324.");
            m.insert("10.1093/femsec/fiw174", "J. Liao, X. Cao, L. Zhao, et al., \"The importance of neutral and niche processes for bacterial community assembly differs between habitat generalists and specialists,\" FEMS Microbiology Ecology, vol. 92, no. 11, fiw174, Aug. 2016, issn: 0168-6496. doi:10.1093/femsec/fiw174. [Online]. Available: https://doi.org/10.1093/femsec/fiw174.");
            m.insert("article", "P. J. Cohen, \"The independence of the continuum hypothesis,\" Proceedings of the National Academy of Sciences, vol. 50, no. 6, pp. 1143–1148, 1963.");
            m.insert("book", "L. Susskind and G. Hrabovsky, Classical mechanics: the theoretical minimum. New York, NY: Penguin Random House, 2014.");
            m.insert("booklet", "M. Swetla, Canoe tours in Sweden, Distributed at the Stockholm Tourist Office, Jul. 2015.");
            m.insert("inbook", "L. A. Urry, M. L. Cain, S. A. Wasserman, P. V. Minorsky, and J. B. Reece, \"Photosynthesis,\" in Campbell Biology. New York, NY: Pearson, 2016, pp. 187–221.");
            m.insert("incollection", "H. M. Shapiro, \"Flow cytometry: The glass is half full,\" in Flow Cytometry Protocols, T. S. Hawley and R. G. Hawley, Eds., New York, NY: Springer, 2018, pp. 1–10.");
            m.insert("inproceedings", "P. Holleis, M. Wagner, and J. Koolwaaij, \"Studying mobile context-aware social services in the wild,\" in Proc. of the 6th Nordic Conf. on Human-Computer Interaction, ser. NordiCHI, New York, NY: ACM, 2010, pp. 207–216.");
            m.insert("manual", "R Core Team, R: A language and environment for statistical computing, R Foundation for Statistical Computing, Vienna, Austria, 2018.");
            m.insert("mastersthesis", "J. Tang, \"Spin structure of the nucleon in the asymptotic limit,\" M.S. thesis, Massachusetts Institute of Technology, Cambridge, MA, Sep. 1996.");
            m.insert("misc", "NASA, Pluto: The 'other' red planet, https://www.nasa.gov/nh/pluto- the-other-red-planet, Accessed: 2018-12-06, 2015.");
            m.insert("phdthesis", "R. C. Rempel, \"Relaxation effects for coupled nuclear spins,\" Ph.D. dissertation, Stanford University, Stanford, CA, Jun. 1956.");
            m.insert("proceedings", "S. Stepney and S. Verlan, Eds., Proceedings of the 17th International Conference on Computation and Natural Computation, Fontainebleau, France, vol. 10867, Lecture Notes in Computer Science, Cham, Switzerland: Springer, 2018");
            m.insert("techreport", "V. Bennett, K. Bowman, and S. Wright, \"Wasatch Solar Project final report,\" Salt Lake City Corporation, Salt Lake City, UT, Tech. Rep. DOE- SLC-6903-1, Sep. 2018.");
            m.insert(
                "unpublished",
                "M. Suresh, \"Evolution: A revised theory,\" 2006",
            );

            m
        };
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
        let formated = fmt_authors_ieee(authors);
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
        let formated = fmt_authors_ieee(authors);
        assert_eq!(formated, "A. M. Lovelace Augusta et al.");

        Ok(())
    }

    #[test]
    fn test_random_forests_against_externally_generated() -> Result<()> {
        let mut fields = BTreeMap::new();

        fields.insert("journal".to_string(), "Machine learning".to_string());
        fields.insert("pages".to_string(), "5-32".to_string());
        fields.insert("publisher".to_string(), "Springer".to_string());
        fields.insert("title".to_string(), "Random forests".to_string());
        fields.insert("volume".to_string(), "45".to_string());
        fields.insert("year".to_string(), "2001".to_string());
        fields.insert(
            "doi".to_string(),
            "https://doi.org/10.1023/a:1010933404324".to_string(),
        );

        let expected = "L. Breiman, \"Random forests,\" Machine learning, vol. 45, pp. 5-32, 2001, doi: https://doi.org/10.1023/a:1010933404324.".to_string();
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
        let answer = fmt_reference_ieee(entry);

        assert_eq!(answer, expected);

        Ok(())
    }
}
