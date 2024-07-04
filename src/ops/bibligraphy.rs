use anyhow::Result;
use log::warn;
use std::{
    fs::{self, read_to_string, File},
    io::Write,
    path::PathBuf,
};

use nom::{combinator::all_consuming, multi::many1};

use crate::{styles::ReferenceStyle, Format};

use crate::parsing::entry::{all_citations, entry, BibEntry, EntrySubComponents};

#[derive(Default)]
pub struct Bibliography {
    entries: Vec<BibEntry>,
}

impl Bibliography {
    pub fn get_entry(&self, key: String) -> Option<BibEntry> {
        self.entries.iter().find(|&e| e.key == key).cloned()
    }

    pub fn fmt_entries(self, style: ReferenceStyle, format: Format) -> Vec<String> {
        self.entries
            .into_iter()
            .map(|b| style.fmt_reference(b, format))
            .collect()
    }
    pub fn has_key(&self, key: &String) -> bool {
        self.entries.iter().any(|e| &e.key == key)
    }
    pub fn fmt_entries_filtered(
        self,
        style: ReferenceStyle,
        format: Format,
        keys: Vec<String>,
    ) -> (Vec<String>, Vec<String>) {
        let (known_keys, unknown_keys): (Vec<String>, Vec<String>) =
            keys.into_iter().partition(|e| self.has_key(e));
        let formatted: Vec<String> = known_keys
            .into_iter()
            .map(|b| style.fmt_reference(self.get_entry(b).unwrap(), format))
            .collect();
        (formatted, unknown_keys)
    }

    pub fn expand_file_citations_inplace(
        &self,
        path: PathBuf,
        style: ReferenceStyle,
        format: Format,
        fail_fast: bool
    ) -> Result<()> {
        let mut contents = read_to_string(&path)?;
        contents = self.expand_citations_in_string(&mut contents, style, format, fail_fast)?;
        let mut file = File::create(&path)?;
        file.write_all(contents.as_bytes()).unwrap();
        Ok(())
    }

    pub fn expand_citations_in_string(
        &self,
        contents: &str,
        style: ReferenceStyle,
        format: Format,
        fail_fast: bool,
    ) -> Result<String> {
        let (tail, segments) = all_citations(contents).unwrap();
        let mut acc =
            segments
                .into_iter()
                .try_fold(String::new(), |mut acc, (unmodified, citation_key)| {
                    acc.push_str(unmodified);

                    match self
                        .get_entry(citation_key.to_string())
                        .and_then(|entry| Some(style.fmt_reference(entry, format)))
                    {
                        Some(formatted) => {
                            acc.push_str(&formatted);
                            Ok(acc)
                        }
                        None => {
                            if fail_fast {
                                Err(anyhow::Error::msg(format!("key {} was not present in any of the bib files", &citation_key)))
                            } else {
                                warn!("Key {} in text was not found, skipping...", &citation_key);
                                acc.push_str("\\cite{");
                                acc.push_str(citation_key);
                                acc.push('}');
                                Ok(acc)
                            }
                        },
                    }
                })?;

        acc.push_str(tail);

        Ok(acc)
    }

    pub fn from_file(path: PathBuf) -> Result<Self> {
        if !&path.exists() {
            return Err(anyhow::Error::msg(format!(
                "File {} does not exist",
                path.display()
            )));
        }
        let contents = fs::read_to_string(path)?;

        let (_tail, entries): (&str, Vec<EntrySubComponents>) =
            all_consuming(many1(entry))(&contents).unwrap();
        let entry_vec: Vec<BibEntry> = entries.into_iter().map(|t| t.into()).collect();
        Ok(entry_vec.into())
    }

    pub fn from_files(path: Vec<PathBuf>) -> Result<Self> {
        let mut out = Self::default();
        let results = path
            .into_iter()
            .map(|p| {
                let new_bib = Bibliography::from_file(p)?;
                out.merge(new_bib);
                Ok(())
            })
            .collect::<Result<Vec<()>, anyhow::Error>>();
        match results {
            Ok(_) => Ok(out),
            Err(e) => Err(e),
        }
    }

    /// merge the two bibliographies by consuming the other.
    /// currently citation conflicts are not handles yet.
    pub fn merge(&mut self, other: Bibliography) -> &mut Self {
        self.entries.extend(other.entries);
        self
    }
}

impl From<Vec<BibEntry>> for Bibliography {
    fn from(value: Vec<BibEntry>) -> Self {
        Self { entries: value }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::collections::BTreeMap;
    use std::{path::PathBuf, str::FromStr};

    use anyhow::Result;
    // lint allows are just while developing, will be removed soon

    use crate::parsing::{entry::EntryType, names::OwnedFullName};

    #[test]
    fn test_bib_file_parse() -> Result<()> {
        let path = PathBuf::from_str("cite.bib")?;
        let entries = Bibliography::from_file(path)?.entries;
        let mut dict = BTreeMap::new();
        dict.insert("title".to_string(), "Random forests".to_string());
        dict.insert("journal".to_string(), "Machine learning".to_string());
        dict.insert("volume".to_string(), "45".to_string());
        dict.insert("number".to_string(), "1".to_string());
        dict.insert("pages".to_string(), "5-32".to_string());
        dict.insert("year".to_string(), "2001".to_string());
        dict.insert("publisher".to_string(), "Springer".to_string());
        dict.insert(
            "doi".to_string(),
            "https://doi.org/10.1023/a:1010933404324".to_string(),
        );
        assert_eq!(
            entries[0],
            BibEntry {
                kind: EntryType::Article,
                key: String::from("breiman2001"),
                authors: vec![OwnedFullName {
                    first: vec!["Leo".to_string()],
                    last: vec!["Breiman".to_string()],
                    von: Vec::new(),
                    title: Vec::new()
                }],
                fields: dict,
            }
        );
        let mut dict = BTreeMap::new();
        dict.insert("title".to_string() , "The importance of neutral and niche processes for bacterial community assembly differs between habitat generalists and specialists".to_string());
        dict.insert(
            "journal".to_string(),
            "FEMS Microbiology Ecology".to_string(),
        );
        dict.insert("volume".to_string(), "92".to_string());
        dict.insert("number".to_string(), "11".to_string());
        dict.insert("year".to_string(), "2016".to_string());
        dict.insert("month".to_string(), "08".to_string());
        dict.insert("issn".to_string(), "0168-6496".to_string());
        dict.insert(
            "doi".to_string(),
            "https://doi.org/10.1093/femsec/fiw174".to_string(),
        );
        dict.insert(
            "url".to_string(),
            "https://doi.org/10.1093/femsec/fiw174".to_string(),
        );
        assert_eq!(
            entries[1],
            BibEntry {
                kind: EntryType::Article,
                key: String::from("10.1093/femsec/fiw174"),
                authors: vec![
                    OwnedFullName {
                        first: vec!["Jingqiu".to_string()],
                        last: vec!["Liao".to_string()],
                        von: Vec::new(),
                        title: Vec::new()
                    },
                    OwnedFullName {
                        first: vec!["Xiaofeng".to_string()],
                        last: vec!["Cao".to_string()],
                        von: Vec::new(),
                        title: Vec::new()
                    },
                    OwnedFullName {
                        first: vec!["Lei".to_string()],
                        last: vec!["Zhao".to_string()],
                        von: Vec::new(),
                        title: Vec::new()
                    },
                    OwnedFullName {
                        first: vec!["Jie".to_string()],
                        last: vec!["Wang".to_string()],
                        von: Vec::new(),
                        title: Vec::new()
                    },
                    OwnedFullName {
                        first: vec!["Zhe".to_string()],
                        last: vec!["Gao".to_string()],
                        von: Vec::new(),
                        title: Vec::new()
                    },
                    OwnedFullName {
                        first: vec!["Michael".to_string(), "Cai".to_string()],
                        last: vec!["Wang".to_string()],
                        von: Vec::new(),

                        title: Vec::new()
                    },
                    OwnedFullName {
                        first: vec!["Yi".to_string()],
                        last: vec!["Huang".to_string()],
                        von: Vec::new(),
                        title: Vec::new()
                    },
                ],
                fields: dict,
            }
        );
        Ok(())
    }
}
