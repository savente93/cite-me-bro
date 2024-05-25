use std::fmt::Debug;

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
    combinator::{eof, map, recognize, verify},
    multi::{many1, many_till, separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
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

#[derive(Default, Clone, PartialEq, Eq)]
pub struct FullName<'a> {
    first: NameComponent<'a>,
    last: NameComponent<'a>,
    von: NameComponent<'a>,
    title: NameComponent<'a>,
}

impl<'a> Debug for FullName<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "First({}) Von({}) Last({}) Title({})",
            &self.first.components.join(" "),
            &self.von.components.join(" "),
            &self.last.components.join(" "),
            &self.title.components.join(" "),
        )
    }
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
    let (tail, word) = alt((
        brace_quoted_literal,
        verify(take_while1(AsChar::is_alpha), |w: &str| {
            w.to_lowercase() != "and"
        }),
    ))(input)?;
    Ok((tail, word))
}

fn initial(input: &str) -> IResult<&str, &str> {
    let (tail, word) = terminated(word, tag("."))(input)?;
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
    let (tail, words) = separated_list1(space1, alt((initial, word)))(input)?;
    Ok((tail, words))
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
            title: vec![].into(),
            von: vec![].into(),
        },
    ))
}

fn last_title_first(input: &str) -> IResult<&str, FullName, nom::error::Error<&str>> {
    let (tail, mut components) =
        separated_list1(delimited(space0, tag(","), space0), space_seperated_words)(input)?;
    let last = components.remove(0);
    let title = components.remove(0);
    let first = components.remove(0);
    Ok((
        tail,
        FullName {
            first: first.into(),
            last: last.into(),
            von: vec![].into(),
            title: title.into(),
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
            von: vec![].into(),
            title: vec![].into(),
        },
    ))
}
fn first_von_last(input: &str) -> IResult<&str, FullName, nom::error::Error<&str>> {
    let (tail, (first, von, last)) = tuple((
        space_seperated_words,
        many1(terminated(von, space1)),
        space_seperated_words,
    ))(input)?;
    Ok((
        tail,
        FullName {
            first: first.into(),
            last: last.into(),
            von: von.into(),
            title: vec![].into(),
        },
    ))
}
fn von_last_first(input: &str) -> IResult<&str, FullName, nom::error::Error<&str>> {
    let (tail, ((von, last), first)) = separated_pair(
        tuple((many1(terminated(von, space0)), space_seperated_words)),
        delimited(space0, tag(","), space0),
        space_seperated_words,
    )(input)?;
    Ok((
        tail,
        FullName {
            first: first.into(),
            last: last.into(),
            von: von.into(),
            title: vec![].into(),
        },
    ))
}

fn and_seperated_words(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(
        terminated(tag_no_case("and"), space0),
        terminated(space0, space0),
    )(input)
}

fn and_seperated_names(input: &str) -> IResult<&str, Vec<FullName>, nom::error::Error<&str>> {
    let (tail, names) = separated_list1(
        delimited(space0, tag_no_case("and"), space0),
        alt((last_first, first_last)),
    )(input)?;

    Ok((tail, names))
}

#[cfg(test)]
mod test {

    use super::*;
    use anyhow::Result;

    macro_rules! parse_assert {
        ($func:ident, $test:expr, $expected:expr) => {
            let (tail, ans) = $func($test)?;
            assert_eq!(tail, "");
            assert_eq!(ans, $expected);
        };
    }

    macro_rules! parse_test {
        ($name:ident, $func:ident, $test:expr, $expected:expr) => {
            #[test]
            fn $name() -> Result<()> {
                parse_assert!($func, $test, $expected);
                Ok(())
            }
        };
    }

    parse_test!(
        test_last_first,
        last_first,
        "Newton, Isaac",
        FullName {
            first: "Isaac".into(),
            title: vec![].into(),
            von: vec![].into(),
            last: "Newton".into()
        }
    );
    parse_test!(
        test_last_last_first,
        last_first,
        "Brinch Hansen, Per",
        FullName {
            first: vec!["Per"].into(),
            title: vec![].into(),
            von: vec![].into(),
            last: vec!["Brinch", "Hansen"].into()
        }
    );

    parse_test!(
        test_last_first_first,
        last_first,
        "Jackson, Michael Joseph",
        FullName {
            first: vec!["Michael", "Joseph"].into(),
            last: "Jackson".into(),
            von: vec![].into(),
            title: vec![].into(),
        }
    );
    parse_test!(
        test_last_first_init,
        last_first,
        "Jackson, Michael J",
        FullName {
            first: vec!["Michael", "J"].into(),
            last: "Jackson".into(),
            von: vec![].into(),
            title: vec![].into(),
        }
    );
    parse_test!(
        test_last_init_init,
        last_first,
        "Jackson, M J",
        FullName {
            first: vec!["M", "J"].into(),
            last: "Jackson".into(),
            von: vec![].into(),
            title: vec![].into(),
        }
    );
    parse_test!(
        test_first_last,
        first_last,
        "Isaac Newton",
        FullName {
            first: "Isaac".into(),
            last: "Newton".into(),
            von: vec![].into(),
            title: vec![].into(),
        }
    );
    parse_test!(
        test_first_first_last,
        first_last,
        "Michael Joseph Jackson",
        FullName {
            first: vec!["Michael", "Joseph"].into(),
            last: "Jackson".into(),
            von: vec![].into(),
            title: vec![].into(),
        }
    );
    parse_test!(
        test_von_last_initial,
        von_last_first,
        "van      Beethoven ,    L",
        FullName {
            first: vec!["L"].into(),
            last: vec!["Beethoven"].into(),
            von: vec!["van"].into(),
            title: vec![].into(),
        }
    );
    parse_test!(
        test_von_last_first,
        von_last_first,
        "van Beethoven, Ludwig",
        FullName {
            first: vec!["Ludwig"].into(),
            last: vec!["Beethoven"].into(),
            von: vec!["van"].into(),
            title: vec![].into(),
        }
    );
    parse_test!(
        test_first_von_last,
        first_von_last,
        "Ludwig van Beethoven",
        FullName {
            first: vec!["Ludwig"].into(),
            last: "Beethoven".into(),
            von: vec!["van"].into(),
            title: vec![].into(),
        }
    );
    #[test]
    fn test_quoted_literal() -> Result<()> {
        // author = "{Barnes and Noble, Inc.}"
        for (test, answer) in vec![
            ("{Barnes and Noble, Inc.}", "Barnes and Noble, Inc."),
            ("{FCC H2020 Project}", "FCC H2020 Project"),
            ("{World Health Organisation}", "World Health Organisation"),
            ("{Barnes and}", "Barnes and"),
            ("{Noble, Inc.}", "Noble, Inc."),
            ("{Barnes}", "Barnes"),
            ("{and}", "and"),
            ("{Noble,}", "Noble,"),
            ("{Inc.}", "Inc."),
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

    parse_test!(
        test_first_init_last,
        first_last,
        "Donald E. Knuth",
        FullName {
            first: vec!["Donald", "E"].into(),
            title: vec![].into(),
            von: vec![].into(),
            last: "Knuth".into()
        }
    );
    parse_test!(
        test_quoted_first_last,
        first_last,
        "Ronald {Van der Jawel}",
        FullName {
            first: vec!["Ronald"].into(),
            title: vec![].into(),
            von: vec![].into(),
            last: "Van der Jawel".into()
        }
    );

    parse_test!(
        test_and_component_last_first,
        last_first,
        "Fisher, James",
        FullName {
            first: "James".into(),
            title: vec![].into(),
            von: vec![].into(),
            last: "Fisher".into()
        }
    );
    parse_test!(
        test_and_component_first_last,
        first_last,
        "John Clark",
        FullName {
            first: "John".into(),
            title: vec![].into(),
            von: vec![].into(),
            last: "Clark".into()
        }
    );
    parse_test!(
        test_last_first_and_first_last,
        and_seperated_names,
        "Fisher, James AND John Clark",
        vec![
            FullName {
                first: "James".into(),
                title: vec![].into(),
                von: vec![].into(),
                last: "Fisher".into()
            },
            FullName {
                first: "John".into(),
                title: vec![].into(),
                von: vec![].into(),
                last: "Clark".into()
            }
        ]
    );

    parse_test!(
        test_multiple_first_last_and_seperated,
        and_seperated_names,
        "Frank Mittelbach and Michel Gossens and Johannes Braams and David Carlisle and Chris Rowley",
        vec![
            FullName {
                first: "Frank".into(),
            title: vec![].into(),
            von: vec![].into(),
                last: "Mittelbach".into()
            },
            FullName {
                first: "Michel".into(),
            title: vec![].into(),
            von: vec![].into(),
                last: "Gossens".into()
            },
            FullName {
                first: "Johannes".into(),
            title: vec![].into(),
            von: vec![].into(),
                last: "Braams".into()
            },
            FullName {
                first: "David".into(),
            title: vec![].into(),
            von: vec![].into(),
                last: "Carlisle".into()
            },
            FullName {
                first: "Chris".into(),
            von: vec![].into(),
            title: vec![].into(),
                last: "Rowley".into()
            }
        ]
    );
    parse_test!(
        test_and_seperated_quoted,
        and_seperated_names,
        "Geert {Van der Plas} and John Doe and {Barnes and Noble}",
        vec![
            FullName {
                first: "Geert".into(),
                title: vec![].into(),
                von: vec![].into(),
                last: "Van der Plas".into()
            },
            FullName {
                first: "John".into(),
                title: vec![].into(),
                von: vec![].into(),
                last: "Doe".into()
            },
            FullName {
                first: vec![].into(),
                title: vec![].into(),
                von: vec![].into(),
                last: "Barnes and Noble".into()
            },
        ]
    );

    parse_test!(
        test_last_title_dotted_first,
        last_title_first,
        "Ford , Jr. , Henry",
        FullName {
            first: vec!["Henry"].into(),
            von: vec![].into(),
            title: "Jr".into(),
            last: "Ford".into()
        }
    );
    parse_test!(
        test_last_title_first,
        last_title_first,
        "King, Jr, Martin Luther",
        FullName {
            first: vec!["Martin", "Luther"].into(),
            title: "Jr".into(),
            von: vec![].into(),
            last: "King".into()
        }
    );
    parse_test!(
        test_insanity,
        first_von_last,
        "Charles Louis Xavier Joseph de la Vallee Poussin",
        FullName {
            first: vec!["Charles", "Louis", "Xavier", "Joseph"].into(),
            title: vec![].into(),
            von: vec!["de", "la"].into(),
            last: vec!["Vallee", "Poussin"].into()
        }
    );
}
