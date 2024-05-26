use std::{collections::HashMap, fmt::Debug};

use anyhow::Error;
use biblatex::{Entry, Pagination};
// lint allows are just while developing, will be removed soon
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_till1, take_until, take_while, take_while1},
    character::{
        complete::{char, line_ending, multispace0, not_line_ending, one_of, space0, space1},
        is_space,
    },
    combinator::{eof, map, recognize, verify},
    multi::{many1, many_till, separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    AsChar, Err, IResult, Parser,
};
use nom_supreme::error::ErrorTree;
use parse_hyperlinks::take_until_unbalanced;

#[derive(Debug)]
pub enum EntryType {
    Book,
    Article,
}

#[derive(Debug)]
pub struct BibEntry<'a> {
    kind: EntryType,
    key: &'a str,
    fields: HashMap<&'a str, &'a str>,
}

fn entry_type(input: &str) -> IResult<&str, &str> {
    alt((
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
    ))(input)
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
fn field(input: &str) -> IResult<&str, (&str, &str)> {
    delimited(
        multispace0,
        terminated(
            separated_pair(
                field_type,
                delimited(multispace0, tag("="), multispace0),
                alt((brace_quoted_field, quote_quoted_field, unquoted_field)),
            ),
            tag(","),
        ),
        multispace0,
    )(input)
}

fn fields(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    many1(field)(input)
}

fn entry_kind(input: &str) -> IResult<&str, &str> {
    preceded(tag("@"), entry_type)(input)
}

fn entry_content(input: &str) -> IResult<&str, &str> {
    delimited(tag("{"), take_until_unbalanced('{', '}'), tag("}"))(input)
}

fn entry_key(input: &str) -> IResult<&str, &str> {
    terminated(take_till1(|c| c == ','), char(','))(input)
}

fn entry(input: &str) -> IResult<&str, (&str, &str, Vec<(&str, &str)>)> {
    let (tail, kind) = entry_kind(input)?;
    let (tail, content) = entry_content(tail)?;
    let (rest_of_content, key) = entry_key(content)?;

    let (_, fields) = fields(rest_of_content)?;
    Ok((tail, (kind, key, fields)))
}

#[cfg(test)]
mod test {
    use super::entry_type;
    use super::*;
    use anyhow::Result;

    #[test]
    fn parse_entry_types() -> Result<()> {
        for (test, expected) in vec![
            ("@book", "book"),
            ("@booklet", "booklet"),
            ("@conference", "conference"),
            ("@inbook", "inbook"),
            ("@incollection", "incollection"),
            ("@inproceedings", "inproceedings"),
            ("@manual", "manual"),
            ("@mastersthesis", "mastersthesis"),
            ("@misc", "misc"),
            ("@phdthesis", "phdthesis"),
            ("@proceedings", "proceedings"),
            ("@techreport", "techreport"),
            ("@unpublished", "unpublished"),
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
            "
        @misc{bar,
        title = {d}
        }"
        );

        assert_eq!(kind, "misc");
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
}
