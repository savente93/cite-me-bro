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
    use std::{collections::BTreeMap, path::PathBuf};

    use super::*;
    use crate::parsing::entry::{parse_bib_file, EntryType};
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
        let formatted_citation= "J. Liao, X. Cao, L. Zhao, et al., \"The importance of neutral and niche processes for bacterial community assembly differs between habitat generalists and specialists,\" FEMS Microbiology Ecology, vol. 92, no. 11, fiw174, Aug. 2016, issn: 0168-6496. doi:10.1093/femsec/fiw174. [Online]. Available: https://doi.org/10.1093/femsec/fiw174.";
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
    fn phthesis_formatted_citation() -> Result<()> {
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
        let formatted_citation = "M. Suresh, \"Evolution: A revised theory,\" 2006";
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
        assert_eq!(formated, "A. M. Lovelace Augusta et al.");

        Ok(())
    }
}
