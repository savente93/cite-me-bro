use std::{collections::HashMap, fmt::Debug};

use anyhow::Error;
use biblatex::Pagination;
// lint allows are just while developing, will be removed soon
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_till1, take_until, take_while, take_while1},
    character::{
        complete::{char, line_ending, not_line_ending, one_of, space0, space1},
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

fn entry_kind(input: &str) -> IResult<&str, &str> {
    preceded(tag("@"), entry_type)(input)
}

fn entry_content(input: &str) -> IResult<&str, &str> {
    delimited(tag("{"), take_until_unbalanced('{', '}'), tag("}"))(input)
}

fn entry_key(input: &str) -> IResult<&str, &str> {
    terminated(take_till1(|c| c == ','), char(','))(input)
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

    #[test]
    fn test_key_parsing() -> Result<()> {
        let key = "10.1093/femsec/fiw174,";
        let (tail, content) = entry_key(&key)?;
        assert_eq!(tail, "");
        assert_eq!(content, "10.1093/femsec/fiw174");

        Ok(())
    }
}
