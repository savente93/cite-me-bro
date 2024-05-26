use core::fmt;
use std::{
    collections::{hash_map, BTreeMap},
    fmt::Debug,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Error, Result};
// lint allows are just while developing, will be removed soon
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_till1, take_until, take_while, take_while1},
    character::{
        complete::{char, line_ending, multispace0, not_line_ending, one_of, space0, space1},
        is_space,
    },
    combinator::{all_consuming, eof, map, opt, recognize, verify},
    multi::{many1, many_till, separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    AsChar, Err, IResult, Parser,
};
use nom_supreme::error::ErrorTree;
use parse_hyperlinks::take_until_unbalanced;

use super::names::{self, and_seperated_names, FullName, OwnedFullName};

#[derive(Debug, PartialEq, Eq)]
pub enum EntryType {
    Article,
    Book,
    Booklet,
    Conference,
    Inbook,
    Incollection,
    Inproceedings,
    Manual,
    Mastersthesis,
    Misc,
    Phdthesis,
    Proceedings,
    Techreport,
    Unpublished,
}

impl fmt::Display for EntryType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EntryType::Article => write!(f, "Article"),
            EntryType::Book => write!(f, "Book"),
            EntryType::Booklet => write!(f, "Booklet"),
            EntryType::Conference => write!(f, "Conference"),
            EntryType::Inbook => write!(f, "Inbook"),
            EntryType::Incollection => write!(f, "Incollection"),
            EntryType::Inproceedings => write!(f, "Inproceedings"),
            EntryType::Manual => write!(f, "Manual"),
            EntryType::Mastersthesis => write!(f, "Mastersthesis"),
            EntryType::Misc => write!(f, "Misc"),
            EntryType::Phdthesis => write!(f, "Phdthesis"),
            EntryType::Proceedings => write!(f, "Proceedings"),
            EntryType::Techreport => write!(f, "Techreport"),
            EntryType::Unpublished => write!(f, "Unpublished"),
        }
    }
}

impl TryFrom<&str> for EntryType {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "article" => Ok(EntryType::Article),
            "booklet" => Ok(EntryType::Booklet),
            "conference" => Ok(EntryType::Conference),
            "inbook" => Ok(EntryType::Inbook),
            "book" => Ok(EntryType::Book),
            "incollection" => Ok(EntryType::Incollection),
            "inproceedings" => Ok(EntryType::Inproceedings),
            "manual" => Ok(EntryType::Manual),
            "mastersthesis" => Ok(EntryType::Mastersthesis),
            "misc" => Ok(EntryType::Misc),
            "phdthesis" => Ok(EntryType::Phdthesis),
            "proceedings" => Ok(EntryType::Proceedings),
            "techreport" => Ok(EntryType::Techreport),
            "unpublished" => Ok(EntryType::Unpublished),
            _ => Err("unknown kind"),
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct BibEntry {
    pub kind: EntryType,
    pub key: String,
    // authors have a special data set and are also included in μ-almost all entries
    // so it get's special treatment
    pub authors: Vec<OwnedFullName>,
    pub fields: BTreeMap<String, String>,
}

impl BibEntry {
    pub fn into_components(
        self,
    ) -> (
        EntryType,
        String,
        Vec<OwnedFullName>,
        BTreeMap<String, String>,
    ) {
        (self.kind, self.key, self.authors, self.fields)
    }
}

impl<'a> Debug for BibEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})\n", self.key, self.kind)?;
        write!(f, "  - Authors:\n")?;
        for auth in self.authors.iter() {
            write!(f, "    - {:?}\n", auth)?;
        }
        for (k, v) in self.fields.iter() {
            write!(f, "  - {} = {}\n", k, v)?;
        }

        Ok(())
    }
}

impl<'a> From<(EntryType, &'a str, Vec<(&'a str, &'a str)>)> for BibEntry {
    fn from(value: (EntryType, &'a str, Vec<(&'a str, &'a str)>)) -> Self {
        let mut fields = BTreeMap::new();

        for (k, v) in value.2 {
            // authors get special treatment
            fields.insert(String::from(k), String::from(v));
        }

        let authors: Vec<OwnedFullName> = match fields.remove_entry("author") {
            Some((_k, v)) => {
                let (_tail, auth) = and_seperated_names(&v).unwrap();
                auth.into_iter().map(|n| n.into()).collect()
            }
            None => vec![],
        };

        Self {
            kind: value.0,
            key: String::from(value.1),
            authors,
            fields,
        }
    }
}

fn entry_type(input: &str) -> IResult<&str, EntryType> {
    let (tail, t) = alt((
        tag_no_case("article"),
        tag_no_case("booklet"),
        tag_no_case("conference"),
        tag_no_case("inbook"),
        tag_no_case("book"),
        tag_no_case("incollection"),
        tag_no_case("inproceedings"),
        tag_no_case("manual"),
        tag_no_case("mastersthesis"),
        tag_no_case("misc"),
        tag_no_case("phdthesis"),
        tag_no_case("proceedings"),
        tag_no_case("techreport"),
        tag_no_case("unpublished"),
    ))(input)?;
    let t = EntryType::try_from(t).expect("Unknown entry type");
    Ok((tail, t))
}

fn field_type(input: &str) -> IResult<&str, &str> {
    alt((
        alt((
            tag_no_case("address"),
            tag_no_case("annote"),
            tag_no_case("author"),
            tag_no_case("booktitle"),
            tag_no_case("chapter"),
            tag_no_case("edition"),
            tag_no_case("editor"),
        )),
        tag_no_case("howpublished"),
        tag_no_case("institution"),
        tag_no_case("journal"),
        tag_no_case("month"),
        tag_no_case("note"),
        tag_no_case("number"),
        tag_no_case("organization"),
        tag_no_case("pages"),
        tag_no_case("publisher"),
        tag_no_case("school"),
        tag_no_case("series"),
        tag_no_case("title"),
        tag_no_case("type"),
        tag_no_case("volume"),
        tag_no_case("year"),
        tag_no_case("doi"),
        tag_no_case("issn"),
        tag_no_case("isbn"),
        tag_no_case("url"),
    ))(input)
}

fn brace_quoted_field(input: &str) -> IResult<&str, &str> {
    delimited(tag("{"), take_until_unbalanced('{', '}'), tag("}"))(input)
}
fn quote_quoted_field(input: &str) -> IResult<&str, &str> {
    delimited(tag("\""), take_until_unbalanced('"', '"'), tag("\""))(input)
}
fn unquoted_field(input: &str) -> IResult<&str, &str> {
    take_until(",")(input)
}

fn fields(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    separated_list1(
        alt((tag(","), line_ending)),
        delimited(
            multispace0,
            separated_pair(
                field_type,
                delimited(multispace0, tag("="), multispace0),
                alt((brace_quoted_field, quote_quoted_field, unquoted_field)),
            ),
            multispace0,
        ),
    )(input)
}

fn entry_kind(input: &str) -> IResult<&str, EntryType> {
    preceded(multispace0, preceded(tag("@"), entry_type))(input)
}

fn entry_content(input: &str) -> IResult<&str, &str> {
    delimited(tag("{"), take_until_unbalanced('{', '}'), tag("}"))(input)
}

fn entry_key(input: &str) -> IResult<&str, &str> {
    terminated(take_till1(|c| c == ','), char(','))(input)
}

fn entry(input: &str) -> IResult<&str, (EntryType, &str, Vec<(&str, &str)>)> {
    let (tail, kind) = entry_kind(input)?;
    let (tail, content) = entry_content(tail)?;
    let (rest_of_content, key) = entry_key(content)?;

    let (_, fields) = fields(rest_of_content)?;
    let (tail, _) = multispace0(tail)?;
    Ok((tail, (kind, key, fields)))
}

fn parse_bib_file(path: PathBuf) -> Result<Vec<BibEntry>> {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");

    let (_tail, entries): (&str, Vec<(EntryType, &str, Vec<(&str, &str)>)>) =
        all_consuming(many1(entry))(&contents).unwrap();
    dbg!(&entries);
    dbg!(&_tail);
    let entry_vec: Vec<BibEntry> = entries.into_iter().map(|t| t.into()).collect();
    Ok(entry_vec)
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::dict;

    use super::entry_type;
    use super::*;
    use anyhow::Result;

    #[test]
    fn parse_entry_types() -> Result<()> {
        for (test, expected) in vec![
            ("@book", EntryType::Book),
            ("@booklet", EntryType::Booklet),
            ("@conference", EntryType::Conference),
            ("@inbook", EntryType::Inbook),
            ("@incollection", EntryType::Incollection),
            ("@inproceedings", EntryType::Inproceedings),
            ("@manual", EntryType::Manual),
            ("@mastersthesis", EntryType::Mastersthesis),
            ("@misc", EntryType::Misc),
            ("@phdthesis", EntryType::Phdthesis),
            ("@proceedings", EntryType::Proceedings),
            ("@techreport", EntryType::Techreport),
            ("@unpublished", EntryType::Unpublished),
        ] {
            let (tail, kind) = entry_kind(test)?;
            assert_eq!(tail, "");
            assert_eq!(kind, expected);
        }
        Ok(())
    }

    #[test]
    fn parse_multiple_entry() -> Result<()> {
        let entries = "@misc{foo,
        title = {blurb},
        }
        @misc{bar,
        title = {d}
        }";
        let (tail, (kind, key, fields)) = entry(entries)?;
        assert_eq!(
            tail,
            "@misc{bar,
        title = {d}
        }"
        );

        assert_eq!(kind, EntryType::Misc);
        assert_eq!(key, "foo");
        assert_eq!(fields, vec![("title", "blurb")]);
        Ok(())
    }
    #[test]
    fn parse_dummy_entry() -> Result<()> {
        let dummy_entry = "{asdf,
        foo = {bar}
        baz = {
            multi
            line
            content
        }}";
        let (tail, content) = entry_content(&dummy_entry)?;
        assert_eq!(tail, "");
        assert_eq!(
            content,
            "asdf,
        foo = {bar}
        baz = {
            multi
            line
            content
        }"
        );

        Ok(())
    }

    fn parse_dummy_fields() -> Result<()> {
        let dummy_content = "
        foo = {bar}
        baz = {
            multi
            line
            content
        }
        asdf = \"whatever\"";
        let (tail, fields) = fields(&dummy_content)?;
        assert_eq!(tail, "");
        // weird spacing needs to be maintained to get the pased content to line up
        assert_eq!(
            fields,
            vec![
                ("foo", "bar"),
                (
                    "bas",
                    "            multi
            line
            content
"
                ),
                ("asdf", "whatever"),
                ("foo", "bar"),
            ]
        );
        Ok(())
    }

    #[test]
    fn test_key_parsing() -> Result<()> {
        let key = "10.1093/femsec/fiw174,";
        let (tail, content) = entry_key(&key)?;
        assert_eq!(tail, "");
        assert_eq!(content, "10.1093/femsec/fiw174");

        Ok(())
    }

    #[test]
    fn test_field_type_parsing() -> Result<()> {
        for (test, expected) in vec![
            ("address", "address"),
            ("annote", "annote"),
            ("author", "author"),
            ("booktitle", "booktitle"),
            ("chapter", "chapter"),
            ("edition", "edition"),
            ("editor", "editor"),
            ("howpublished", "howpublished"),
            ("institution", "institution"),
            ("journal", "journal"),
            ("month", "month"),
            ("note", "note"),
            ("number", "number"),
            ("organization", "organization"),
            ("pages", "pages"),
            ("publisher", "publisher"),
            ("school", "school"),
            ("series", "series"),
            ("title", "title"),
            ("type", "type"),
            ("volume", "volume"),
            ("year", "year"),
            ("doi", "doi"),
            ("issn", "issn"),
            ("isbn", "isbn"),
            ("url", "url"),
        ] {
            let (tail, field) = field_type(test)?;
            assert_eq!(tail, "");
            assert_eq!(field, expected);
        }

        Ok(())
    }
    #[test]
    fn test_bib_file_parse() -> Result<()> {
        let path = PathBuf::from_str("cite.bib")?;
        let entries = parse_bib_file(path)?;
        assert_eq!(
            entries[0],
            BibEntry {
                kind: EntryType::Article,
                key: String::from("breiman2001random"),
                authors: vec![OwnedFullName {
                    first: vec!["Leo".to_string()],
                    last: vec!["Breiman".to_string()],
                    von: Vec::new(),
                    title: Vec::new()
                }],
                fields: dict!(
                "title".to_string() => "Random forests".to_string() ,
                "journal".to_string() => "Machine learning".to_string() ,
                "volume".to_string() => "45".to_string() ,
                "pages".to_string() => "5-32".to_string() ,
                "year".to_string() => "2001".to_string() ,
                "publisher".to_string() => "Springer".to_string(),
                "doi".to_string() => "https://doi.org/10.1023/A:1010933404324".to_string()
                              ),
            }
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
                fields: dict!(
                "title".to_string() => "The importance of neutral and niche processes for bacterial community assembly differs between habitat generalists and specialists".to_string(),
                "journal".to_string() => "FEMS Microbiology Ecology".to_string(),
                "volume".to_string() => "92".to_string(),
                "number".to_string() => "11".to_string(),
                "pages".to_string() => "fiw174".to_string(),
                "year".to_string() => "2016".to_string(),
                "month".to_string() => "08".to_string(),
                "issn".to_string() => "0168-6496".to_string(),
                "doi".to_string() => "10.1093/femsec/fiw174".to_string(),
                "url".to_string() => "https://doi.org/10.1093/femsec/fiw174".to_string()
                          ),
            }
        );
        Ok(())
    }
}
