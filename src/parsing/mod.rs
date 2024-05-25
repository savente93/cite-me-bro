#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_import_braces)]
use anyhow::Error;
use biblatex::Pagination;
// lint allows are just while developing, will be removed soon
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_until, take_while, take_while1},
    character::{
        complete::{char, line_ending, not_line_ending, one_of, space0, space1},
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

fn von_english(input: &str) -> IResult<&str, &str> {
    let (tail, word) = tag_no_case("Of")(input)?;
    Ok((tail, word))
}
fn von_dutch(input: &str) -> IResult<&str, &str> {
    let (tail, word) = alt((
        tag_no_case("Van der"),
        tag_no_case("Van de"),
        tag_no_case("Van"),
    ))(input)?;
    Ok((tail, word))
}
fn von_german(input: &str) -> IResult<&str, &str> {
    let (tail, word) = alt((
        tag_no_case("von der"),
        tag_no_case("von"),
        tag_no_case("zum"),
        tag_no_case("zur"),
        tag_no_case("zu"),
    ))(input)?;
    Ok((tail, word))
}
fn von_french(input: &str) -> IResult<&str, &str> {
    let (tail, word) = alt((
        tag_no_case("Des"),
        // De case is commented because it interferes with cases like della in italian
        // other languages have this case longer down the list, so it's still caught
        // tag_no_case("De"),
        tag_no_case("Du"),
        tag_no_case("D'"),
    ))(input)?;
    Ok((tail, word))
}
fn von_italian(input: &str) -> IResult<&str, &str> {
    let (tail, word) = alt((
        tag_no_case("della"),
        tag_no_case("delle"),
        tag_no_case("del"),
        tag_no_case("dei"),
        tag_no_case("di"),
        tag_no_case("d'"),
    ))(input)?;
    Ok((tail, word))
}
fn von_spanish(input: &str) -> IResult<&str, &str> {
    let (tail, word) = alt((
        tag_no_case("Del"),
        tag_no_case("De los"),
        tag_no_case("De las"),
        tag_no_case("De la"),
        tag_no_case("De"),
    ))(input)?;
    Ok((tail, word))
}
fn von_portuguese(input: &str) -> IResult<&str, &str> {
    let (tail, word) = alt((
        tag_no_case("Dos"),
        tag_no_case("Das"),
        tag_no_case("De"),
        tag_no_case("Do"),
        tag_no_case("Da"),
    ))(input)?;
    Ok((tail, word))
}
fn von_scandinavian(input: &str) -> IResult<&str, &str> {
    let (tail, word) = tag_no_case("Af")(input)?;
    Ok((tail, word))
}
fn von_russian(input: &str) -> IResult<&str, &str> {
    let (tail, word) = alt((tag_no_case("Из"), tag_no_case("Iz")))(input)?;
    Ok((tail, word))
}
fn von_polish(input: &str) -> IResult<&str, &str> {
    let (tail, word) = alt((tag_no_case("Ze"), tag_no_case("Z")))(input)?;
    Ok((tail, word))
}
fn von(input: &str) -> IResult<&str, &str> {
    let (tail, word) = alt((
        von_dutch,
        von_english,
        von_german,
        von_french,
        von_italian,
        von_spanish,
        von_portuguese,
        von_scandinavian,
        von_russian,
        von_polish,
    ))(input)?;
    Ok((tail, word))
}

fn space_seperated_words(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(space1, word)(input)
}

fn brace_quoted_literal(input: &str) -> IResult<&str, &str> {
    delimited(char('{'), take_until("}"), char('}'))(input)
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
    fn test_last_first_init() -> Result<()> {
        let name = "Jackson, Michael J";
        let (tail, name) = last_first(name)?;
        assert_eq!(tail, "");
        assert_eq!(
            name,
            FullName {
                first: vec!["Michael", "J"].into(),
                last: "Jackson".into()
            }
        );

        Ok(())
    }
    #[test]
    fn test_last_init_init() -> Result<()> {
        let name = "Jackson, M J";
        let (tail, name) = last_first(name)?;
        assert_eq!(tail, "");
        assert_eq!(
            name,
            FullName {
                first: vec!["M", "J"].into(),
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
    #[test]
    fn test_quoted_literal() -> Result<()> {
        // author = "{Barnes and Noble, Inc.}"
        for (test, answer) in vec![
            ("{Barnes and Noble, Inc.}", "Barnes and Noble, Inc."),
            ("{FCC H2020 Project}", "FCC H2020 Project"),
            ("{World Health Organisation}", "World Health Organisation"),
        ] {
            let (tail, name) = brace_quoted_literal(test)?;
            assert_eq!(tail, "");
            assert_eq!(name, answer);
        }

        Ok(())
    }
    #[test]
    fn test_von() -> Result<()> {
        // author = "{Barnes and Noble, Inc.}"
        for (test, answer) in vec![
            //english
            ("of", "of"),
            // dutch
            ("van", "van"),
            ("van de", "van de"),
            ("van der", "van der"),
            // german
            ("von", "von"),
            ("von der", "von der"),
            ("zu", "zu"),
            ("zum", "zum"),
            ("zur", "zur"),
            //french
            ("de", "de"),
            ("du", "du"),
            ("des", "des"),
            ("d'", "d'"),
            //italian
            ("di", "di"),
            ("d'", "d'"),
            ("del", "del"),
            ("della", "della"),
            ("dei", "dei"),
            ("delle", "delle"),
            // //spanish
            ("de", "de"),
            ("del", "del"),
            ("de la", "de la"),
            ("de los", "de los"),
            ("de las", "de las"),
            //portugese
            ("de", "de"),
            ("do", "do"),
            ("da", "da"),
            ("dos", "dos"),
            ("das", "das"),
            //scandanavvian
            ("af", "af"),
            //russian
            ("из", "из"),
            ("iz", "iz"),
            // polish
            ("z", "z"),
            ("ze", "ze"),
        ] {
            dbg!(&test);
            let (tail, name) = von(test)?;
            assert_eq!(tail, "");
            assert_eq!(name, answer);
        }

        Ok(())
    }
    // Brinch Hansen, Per
    // Charles Louis Xavier Joseph de la Vallee Poussin -> First(Charles Louis Xavier Joseph) von(de la) Last(Vallee Poussin)
    //  "" "{Barnes and} {Noble, Inc.}" "{Barnes} {and} {Noble,} {Inc.}"
    // Ford, Jr., Henry

    // % An example with a suffix
    // author = "Stoner, Jr, Winifred Sackville"

    // % Corporate names or names of consortia
    // Donald E. Knuth
    // Frank Mittelbach and Michel Gossens and Johannes Braams and David Carlisle and Chris Rowley
    // author = {Geert {Van der Plas} and John Doe}
    // King, Jr, Martin Luther
    // Fisher, James AND John Clark
}
