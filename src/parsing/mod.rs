#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_import_braces)]
use anyhow::Error;
use biblatex::Pagination;
// lint allows are just while developing, will be removed soon
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while1},
    character::{
        complete::{line_ending, one_of, space0, space1},
        is_space,
    },
    combinator::{eof, map},
    multi::{many1, many_till, separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    AsChar, Err, IResult, Parser,
};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct NameComponent<'a> {
    components: Vec<&'a str>,
}

impl<'a> NameComponent<'a> {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }
    pub fn merge(mut self, other: NameComponent<'a>) -> Self {
        self.components.extend(other.components);
        self
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct FullName<'a> {
    first: NameComponent<'a>,
    last: NameComponent<'a>,
}
impl<'a, 'b: 'a> From<Vec<&'b str>> for NameComponent<'a> {
    fn from(value: Vec<&'b str>) -> Self {
        Self { components: value }
    }
}
impl<'a, 'b: 'a> From<&'b str> for NameComponent<'a> {
    fn from(value: &'b str) -> Self {
        Self {
            components: vec![value],
        }
    }
}
fn whitespace(input: &str) -> IResult<&str, &str> {
    let (tail, whitespace) = take_while(|c| is_space(c as u8))(input)?;
    Ok((tail, whitespace))
}

fn trim0(input: &str) -> IResult<&str, ()> {
    match whitespace(input) {
        Ok((t, _)) => Ok((t, ())),
        Result::Err(_) => Ok((input, ())),
    }
}

fn word(input: &str) -> IResult<&str, &str> {
    let (tail, word) = take_while1(AsChar::is_alpha)(input)?;
    Ok((tail, word))
}

fn space_seperated_words(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(space1, word)(input)
}

fn last_first(input: &str) -> IResult<&str, FullName, nom::error::Error<&str>> {
    let (tail, (last, first)) = separated_pair(
        space_seperated_words,
        delimited(space0, tag(","), space0),
        space_seperated_words,
    )(input)?;
    Ok((
        tail,
        FullName {
            last: last.into(),
            first: first.into(),
        },
    ))
}

fn first_last(input: &str) -> IResult<&str, FullName, nom::error::Error<&str>> {
    let (tail, mut words) = space_seperated_words(input)?;
    let last = words.remove(&words.len() - 1);
    Ok((
        tail,
        FullName {
            first: words.into(),
            last: last.into(),
        },
    ))
}

#[cfg(test)]
mod test {

    use super::*;
    use anyhow::Result;
    #[test]
    fn test_last_first() -> Result<()> {
        let name = "Newton, Isaac";
        let (tail, name) = last_first(name)?;
        assert_eq!(tail, "");
        assert_eq!(
            name,
            FullName {
                first: "Isaac".into(),
                last: "Newton".into()
            }
        );

        Ok(())
    }

    #[test]
    fn test_last_first_first() -> Result<()> {
        let name = "Jackson, Michael Joseph";
        let (tail, name) = last_first(name)?;
        assert_eq!(tail, "");
        assert_eq!(
            name,
            FullName {
                first: vec!["Michael", "Joseph"].into(),
                last: "Jackson".into()
            }
        );

        Ok(())
    }
    #[test]
    fn test_first_last() -> Result<()> {
        let name = "Isaac Newton";
        let (tail, name) = first_last(name)?;
        assert_eq!(tail, "");
        assert_eq!(
            name,
            FullName {
                first: "Isaac".into(),
                last: "Newton".into()
            }
        );

        Ok(())
    }
    #[test]
    fn test_first_first_last() -> Result<()> {
        let name = "Michael Joseph Jackson";
        let (tail, name) = first_last(name)?;
        assert_eq!(tail, "");
        assert_eq!(
            name,
            FullName {
                first: vec!["Michael", "Joseph"].into(),
                last: "Jackson".into()
            }
        );

        Ok(())
    }
    #[test]
    fn test_von_last_initial() -> Result<()> {
        let name = "van      Beethoven ,    L";
        let (tail, name) = last_first(name)?;
        assert_eq!(tail, "");
        assert_eq!(
            name,
            FullName {
                first: vec!["L"].into(),
                last: vec!["van", "Beethoven"].into()
            }
        );

        Ok(())
    }
    #[test]
    fn test_von_last_first() -> Result<()> {
        let name = "van Beethoven, Ludwig";
        let (tail, name) = last_first(name)?;
        assert_eq!(tail, "");
        assert_eq!(
            name,
            FullName {
                first: vec!["Ludwig"].into(),
                last: vec!["van", "Beethoven"].into()
            }
        );

        Ok(())
    }
    #[test]
    fn test_first_von_last() -> Result<()> {
        let name = "Ludwig van Beethoven";
        let (tail, name) = first_last(name)?;
        assert_eq!(tail, "");
        assert_eq!(
            name,
            FullName {
                first: vec!["Ludwig", "van"].into(),
                last: "Beethoven".into()
            }
        );

        Ok(())
    }
    // author = ""
    // author = ""

    //     }
    // Brinch Hansen, Per
    // Charles Louis Xavier Joseph de la Vallee Poussin -> First(Charles Louis Xavier Joseph) von(de la) Last(Vallee Poussin)
    //  "{Barnes and Noble, Inc.}" "{Barnes and} {Noble, Inc.}" "{Barnes} {and} {Noble,} {Inc.}"
    // Ford, Jr., Henry
    // author = "Jackson, Michael J"
    // author = "Jackson, M J"

    // % An example with a suffix
    // author = "Stoner, Jr, Winifred Sackville"

    // % An exmaple with a particle

    // % Corporate names or names of consortia
    // author = "{Barnes and Noble, Inc.}"
    // author = "{FCC H2020 Project}"
    // Donald E. Knuth
    // Frank Mittelbach and Michel Gossens and Johannes Braams and David Carlisle and Chris Rowley
    // author = {{World Health Organisation}}
    // author = {Geert {Van der Plas} and John Doe}
    // King, Jr, Martin Luther
    // Fisher, James AND John Clark
}
