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
        complete::{line_ending, one_of},
        is_space,
    },
    combinator::map,
    multi::{many1, separated_list0, separated_list1},
    sequence::{delimited, pair, separated_pair, terminated},
    AsChar, Err, IResult,
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
    pub fn merge(&mut self, other: NameComponent<'a>) {
        self.components.extend(other.components);
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
fn initial(input: &str) -> IResult<&str, &str> {
    let (tail, initial) = terminated(inner_word, tag("."))(input)?;
    let (tail, _) = trim_left(tail)?;
    Ok((tail, initial))
}
fn trim_left(input: &str) -> IResult<&str, ()> {
    let (tail, _whitespace) = take_while(|c| is_space(c as u8))(input)?;
    Ok((tail, ()))
}

fn inner_word(input: &str) -> IResult<&str, &str> {
    let (tail, word) = take_while1(AsChar::is_alpha)(input)?;
    let (tail, _) = trim_left(tail)?;
    Ok((tail, word))
}
fn outer_word(input: &str) -> IResult<&str, &str> {
    let (tail, word) = terminated(inner_word, one_of(".,_"))(input)?;
    let (tail, _) = trim_left(tail)?;

    Ok((tail, word))
}

fn last_first(input: &str) -> IResult<&str, FullName, nom::error::Error<&str>> {
    let (tail, (last, first)) = separated_pair(
        hyphenated_name_component_trimmed,
        tag(","),
        hyphenated_name_component_trimmed,
    )(input)?;

    Ok((tail, FullName { first, last }))
}
fn first_last(input: &str) -> IResult<&str, FullName, nom::error::Error<&str>> {
    let (tail, (first, last)) = separated_pair(
        many1(hyphenated_name_component_trimmed),
        trim_left,
        hyphenated_name_component_trimmed,
    )(input)?;

    Ok((
        tail,
        FullName {
            first: first.fold(),
            last,
        },
    ))
}

fn hyphenated_name_component_trimmed<'a, 'b: 'a>(
    input: &'b str,
) -> IResult<&'b str, NameComponent<'a>> {
    let (tail, _whitespace) = trim_left(input)?;
    let (tail, trimmed_components) = separated_list0(tag("-"), inner_word)(tail)?;
    let components = NameComponent::from(trimmed_components);
    Ok((tail, components))
}

fn hyphenated_name_component<'a, 'b: 'a>(input: &'b str) -> IResult<&'b str, NameComponent<'a>> {
    let (tail, components) = separated_list1(tag("-"), inner_word)(input)?;
    let (tail, _) = trim_left(tail)?;
    Ok((tail, components.into()))
}

#[cfg(test)]
mod test {

    use super::*;
    use anyhow::Result;
    #[test]
    fn test_name_single_initial() -> Result<()> {
        let (tail, head) = initial("J. Doe")?;
        assert_eq!(head, "J");
        assert_eq!(tail, "Doe");
        Ok(())
    }

    #[test]
    fn test_name_component_hypenated() -> Result<()> {
        let test_name = "Hans-Jan-Jaap Pietersen";
        let (tail, name_parts) = hyphenated_name_component(test_name)?;

        assert_eq!(tail, "Pietersen");
        assert_eq!(name_parts.components, vec!["Hans", "Jan", "Jaap"]);

        Ok(())
    }
    #[test]
    fn test_name_component_non_hypenated() -> Result<()> {
        let test_name = "Jaap Pietersen";
        let (tail, name_parts) = hyphenated_name_component(test_name)?;

        assert_eq!(tail, "Pietersen");
        assert_eq!(name_parts.components, vec!["Jaap"]);

        Ok(())
    }
    #[test]
    fn test_last_first_single() -> Result<()> {
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
    fn test_first_last_single() -> Result<()> {
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

    //     }
    // Brinch Hansen, Per
    // Charles Louis Xavier Joseph de la Vallee Poussin -> First(Charles Louis Xavier Joseph) von(de la) Last(Vallee Poussin)
    //  "{Barnes and Noble, Inc.}" "{Barnes and} {Noble, Inc.}" "{Barnes} {and} {Noble,} {Inc.}"
    // Ford, Jr., Henry
    //% The King of Pop: Michael Joseph Jackson
    // author = ""
    // author = "Jackson, Michael Joseph"
    // author = "Jackson, Michael J"
    // author = "Jackson, M J"

    // % An example with a suffix
    // author = "Stoner, Jr, Winifred Sackville"

    // % An exmaple with a particle
    // author = "Ludwig van Beethoven"
    // author = "van Beethoven, Ludwig"
    // author = "van Beethoven, L"

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
