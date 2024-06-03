use core::fmt;
use std::{collections::BTreeMap, fmt::Debug};

use anyhow::Result;
// lint allows are just while developing, will be removed soon
use nom::{
    branch::{alt, permutation},
    bytes::complete::{tag, tag_no_case, take_till, take_till1, take_until},
    character::{
        complete::{char, line_ending, multispace0},
        is_space,
    },
    multi::{many0, many1, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};
use parse_hyperlinks::take_until_unbalanced;

use super::names::{and_seperated_names, OwnedFullName};
pub type EntrySubComponents<'a> = (EntryType, &'a str, Vec<(&'a str, &'a str)>);

#[derive(Debug, PartialEq, Eq, Clone)]
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

/// Searches the input for a \cite tag
///
pub fn citation(input: &str) -> IResult<&str, &str> {
    preceded(
        tag("\\cite"),
        delimited(char('{'), take_until("}"), char('}')),
    )(input)
}
/// gives back tail, text consumed
pub fn next_citation(input: &str) -> IResult<&str, (&str, &str)> {
    let (tail, unmodified) = take_until("\\cite")(input)?;
    let (tail, citation_key) = citation(tail)?;

    Ok((tail, (unmodified, citation_key)))
}
pub fn all_citations(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    many1(next_citation)(input)
}

#[derive(PartialEq, Eq, Clone)]
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

impl Debug for BibEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}({})", self.key, self.kind)?;
        writeln!(f, "  - Authors:")?;
        for auth in self.authors.iter() {
            writeln!(f, "    - {:?}", auth)?;
        }
        for (k, v) in self.fields.iter() {
            writeln!(f, "  - {} = {}", k, v)?;
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
    delimited(tag("\""), take_till(|c| c == '"'), tag("\""))(input)
}
fn unquoted_field(input: &str) -> IResult<&str, &str> {
    let (tail, val) = take_till(|c| ",}".contains(c))(input)?;
    let (_, val_stripped) = delimited(
        many0(permutation((multispace0, line_ending))),
        take_till(|c| is_space(c as u8)),
        many0(permutation((multispace0, line_ending))),
    )(val)?;

    Ok((tail, val_stripped))
}

fn field(input: &str) -> IResult<&str, (&str, &str)> {
    let (tail, (kind, value)) = separated_pair(
        field_type,
        delimited(multispace0, tag("="), multispace0),
        alt((brace_quoted_field, quote_quoted_field, unquoted_field)),
    )(input)?;
    Ok((tail, (kind, value)))
}
fn fields(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    let (tail, fields) = separated_list1(
        alt((
            delimited(multispace0, tag(","), multispace0),
            preceded(multispace0, line_ending),
        )),
        delimited(multispace0, field, multispace0),
    )(input)?;

    Ok((tail, fields))
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

pub fn entry(input: &str) -> IResult<&str, EntrySubComponents> {
    let (tail, kind) = entry_kind(input)?;
    let (tail, content) = entry_content(tail)?;
    let (rest_of_content, key) = entry_key(content)?;

    let (_, fields) = fields(rest_of_content)?;
    let (tail, _) = multispace0(tail)?;
    Ok((tail, (kind, key, fields)))
}

#[cfg(test)]
mod test {

    use super::*;
    use anyhow::Result;

    #[test]
    fn test_quoted() -> Result<()> {
        let input = "\"Shapiro, Howard M.\"";
        let (tail, field) = quote_quoted_field(input)?;
        assert_eq!(tail, "");
        assert_eq!(field, "Shapiro, Howard M.");
        Ok(())
    }

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
        let (tail, content) = entry_content(dummy_entry)?;
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

    #[test]
    fn test_key_parsing() -> Result<()> {
        let key = "10.1093/femsec/fiw174,";
        let (tail, content) = entry_key(key)?;
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
    fn unquoted_year_comma() -> Result<()> {
        let input = "year    = 1956   ,  \n";
        let (tail, (kind, content)) = field(input)?;
        assert_eq!(tail, ",  \n");
        assert_eq!(kind, "year");
        assert_eq!(content, "1956");

        Ok(())
    }
    #[test]
    fn unquoted_year_no_comma() -> Result<()> {
        let input = "year    = 1956   \n}";
        let (tail, (kind, content)) = field(input)?;
        assert_eq!(tail, "}");
        assert_eq!(kind, "year");
        assert_eq!(content, "1956");

        Ok(())
    }
    #[test]
    fn unquoted_field_last() -> Result<()> {
        let input = "month   = jun \n}";
        let (tail, (kind, content)) = field(input)?;
        assert_eq!(tail, "}");
        assert_eq!(kind, "month");
        assert_eq!(content, "jun");

        Ok(())
    }
    #[test]
    fn brace_quoted_field() -> Result<()> {
        let input = "month   = {jun}, month   = {jun}";
        let (tail, (kind, content)) = field(input)?;
        assert_eq!(tail, ", month   = {jun}");
        assert_eq!(kind, "month");
        assert_eq!(content, "jun");

        Ok(())
    }

    #[test]
    fn text_citation_simple() -> Result<()> {
        let input = "         asd;lkfjwliefjxcajvnasledifm; sei help I'm stuck in a sub factory! asdoifmwae;va \\cite{book}.lkfjwliefjxcajvnasledifm";
        let (tail, (unmodified, citation_key)) = next_citation(input)?;

        assert_eq!(tail, ".lkfjwliefjxcajvnasledifm");
        assert_eq!(unmodified,"         asd;lkfjwliefjxcajvnasledifm; sei help I'm stuck in a sub factory! asdoifmwae;va ");
        assert_eq!(citation_key, "book");
        Ok(())
    }
}
