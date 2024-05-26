use std::fmt::Debug;

use anyhow::Error;
use biblatex::Pagination;
// lint allows are just while developing, will be removed soon
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_until, take_while, take_while1},
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

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct NameComponent<'a> {
    components: Vec<&'a str>,
}

use lazy_static::*;
use std::collections::BTreeSet;

lazy_static! {
    static ref VON: BTreeSet<&'static str> = {
        let mut m = BTreeSet::new();
        m.insert("d'");
        m.insert("da");
        m.insert("das");
        m.insert("de la");
        m.insert("de las");
        m.insert("de los");
        m.insert("de");
        m.insert("del");
        m.insert("des");
        m.insert("do");
        m.insert("dos");
        m.insert("du");
        m.insert("van de");
        m.insert("van der");
        m.insert("van");
        m.insert("d'");
        m.insert("dei");
        m.insert("del");
        m.insert("della");
        m.insert("delle");
        m.insert("di");
        m.insert("la");
        m.insert("las");
        m.insert("los");
        m.insert("von der");
        m.insert("von");
        m.insert("zu");
        m.insert("zum");
        m.insert("zur");
        m.insert("af");
        m.insert("ze");
        m.insert("Z");
        m.insert("из");
        m.insert("iz");
        m
    };
    static ref COUNT: usize = VON.len();
    static ref TITLE: BTreeSet<&'static str> = {
        let mut m = BTreeSet::new();
        m.insert("sir");
        m.insert("Sir");
        m.insert("madam");
        m.insert("monsieur");
        m.insert("Madame");
        m.insert("madame");
        m.insert("Monsieur");
        m.insert("Ir");
        m.insert("dr");
        m.insert("Dr");
        m.insert("III");
        m
    };
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

fn hyphenated_word(input: &str) -> IResult<&str, &str> {
    recognize(separated_list1(tag("-"), inner_word))(input)
}

fn inner_word(input: &str) -> IResult<&str, &str> {
    verify(take_while1(|c: char| c.is_alphabetic()), |w: &str| {
        w.to_lowercase() != "and"
    })(input)
}
fn word(input: &str) -> IResult<&str, &str> {
    let (tail, word) = alt((brace_quoted_literal, hyphenated_word, inner_word))(input)?;
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
    let (tail, (last_with_von, first_with_von)) = separated_pair(
        space_seperated_words,
        delimited(multispace0, tag(","), multispace0),
        space_seperated_words,
    )(input)?;
    let (mut von, first): (Vec<&str>, Vec<&str>) =
        first_with_von.iter().partition(|&w| VON.contains(w));
    let (last_von, last): (Vec<&str>, Vec<&str>) =
        last_with_von.iter().partition(|&w| VON.contains(w));
    von.extend(last_von);
    let (mut title, first): (Vec<&str>, Vec<&str>) = first.iter().partition(|&w| TITLE.contains(w));
    let (title_last, last): (Vec<&str>, Vec<&str>) = last.iter().partition(|&w| TITLE.contains(w));
    title.extend(title_last);
    Ok((
        tail,
        FullName {
            last: last.into(),
            first: first.into(),
            title: title.into(),
            von: von.into(),
        },
    ))
}

fn first_last(input: &str) -> IResult<&str, FullName, nom::error::Error<&str>> {
    let (tail, mut words) = space_seperated_words(input)?;

    let first_von = words.iter().position(|w| VON.contains(w));
    let last_von = words.iter().rev().position(|w| VON.contains(w));
    let (first, von, last) = match first_von {
        Some(i) => {
            // not the prettiest code I've ever written I'll admit,
            // but hey, it works
            let num_words = words.len();
            let mut first = vec![];
            let mut von = vec![];
            let mut last = vec![];

            // if first will be found, so will last.
            let j = num_words - last_von.unwrap() - 1;

            for _ in 0..i {
                first.push(words.remove(0))
            }
            for _ in i..=j {
                von.push(words.remove(0))
            }
            for _ in j..num_words - 1 {
                last.push(words.remove(0))
            }
            (first, von, last)
        }
        None => {
            let last = vec![words.remove(&words.len() - 1)];
            let first = words;
            let von = vec![];
            (first, von, last)
        }
    };

    let (mut title, first): (Vec<&str>, Vec<&str>) = first.iter().partition(|&w| TITLE.contains(w));
    let (title_last, last): (Vec<&str>, Vec<&str>) = last.iter().partition(|&w| TITLE.contains(w));
    title.extend(title_last);

    Ok((
        tail,
        FullName {
            first: first.into(),
            last: last.into(),
            von: von.into(),
            title: title.into(),
        },
    ))
}

fn name(input: &str) -> IResult<&str, FullName, nom::error::Error<&str>> {
    alt((last_first, first_last))(input)
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
        last_first,
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
        last_first,
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
        first_last,
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
        test_many_name_components,
        first_last,
        "Charles Louis Xavier Joseph de la Vallee Poussin III",
        FullName {
            first: vec!["Charles", "Louis", "Xavier", "Joseph"].into(),
            title: vec!["III"].into(),
            von: vec!["de", "la"].into(),
            last: vec!["Vallee", "Poussin"].into()
        }
    );

    // this one is more about being able to parse different naming conventions
    // rather than any particular format
    #[test]
    fn stress_test() -> Result<()> {
        for (test, expected) in vec![
            (
                "Albert Einstein",
                FullName {
                    first: "Albert".into(),
                    last: "Einstein".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Dr. Emmet Brown",
                FullName {
                    first: "Emmet".into(),
                    last: "Brown".into(),
                    von: vec![].into(),
                    title: "Dr".into(),
                },
            ),
            (
                "Leonardo da Vinci",
                FullName {
                    first: "Leonardo".into(),
                    last: "Vinci".into(),
                    von: "da".into(),
                    title: vec![].into(),
                },
            ),
            (
                "Conan Doyle, Sir Arthur",
                FullName {
                    first: "Arthur".into(),
                    last: vec!["Conan", "Doyle"].into(),
                    von: vec![].into(),
                    title: "Sir".into(),
                },
            ),
            (
                "Madame Marie Curie",
                FullName {
                    first: "Marie".into(),
                    last: "Curie".into(),
                    von: vec![].into(),
                    title: "Madame".into(),
                },
            ),
            (
                "Jean-Jacques Rousseau",
                FullName {
                    first: "Jean-Jacques".into(),
                    last: "Rousseau".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Friedrich Nietzsche",
                FullName {
                    first: "Friedrich".into(),
                    last: "Nietzsche".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Ada Lovelace",
                FullName {
                    first: "Ada".into(),
                    last: "Lovelace".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Vincent van Gogh",
                FullName {
                    first: "Vincent".into(),
                    last: "Gogh".into(),
                    von: "van".into(),
                    title: vec![].into(),
                },
            ),
            (
                "Amelia Earhart",
                FullName {
                    first: "Amelia".into(),
                    last: "Earhart".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Hermann Hesse",
                FullName {
                    first: "Hermann".into(),
                    last: "Hesse".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Alexandre Dumas",
                FullName {
                    first: "Alexandre".into(),
                    last: "Dumas".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Lise Meitner",
                FullName {
                    first: "Lise".into(),
                    last: "Meitner".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Karl Marx",
                FullName {
                    first: "Karl".into(),
                    last: "Marx".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Che Guevara",
                FullName {
                    first: "Che".into(),
                    last: "Guevara".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Sigmund Freud",
                FullName {
                    first: "Sigmund".into(),
                    last: "Freud".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Dr. Seuss",
                FullName {
                    first: vec![].into(),
                    last: "Seuss".into(),
                    von: vec![].into(),
                    title: "Dr".into(),
                },
            ),
            (
                "Virginia Woolf",
                FullName {
                    first: "Virginia".into(),
                    last: "Woolf".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Vasco da Gama",
                FullName {
                    first: "Vasco".into(),
                    last: "Gama".into(),
                    von: "da".into(),
                    title: vec![].into(),
                },
            ),
            (
                "Catherine de Medici",
                FullName {
                    first: "Catherine".into(),
                    last: "Medici".into(),
                    von: "de".into(),
                    title: vec![].into(),
                },
            ),
            (
                "Francisco de Goya",
                FullName {
                    first: "Francisco".into(),
                    last: "Goya".into(),
                    von: "de".into(),
                    title: vec![].into(),
                },
            ),
            (
                "William Shakespeare",
                FullName {
                    first: "William".into(),
                    last: "Shakespeare".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Niccolo Machiavelli",
                FullName {
                    first: "Niccolo".into(),
                    last: "Machiavelli".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Dante Alighieri",
                FullName {
                    first: "Dante".into(),
                    last: "Alighieri".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Gregor Mendel",
                FullName {
                    first: "Gregor".into(),
                    last: "Mendel".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Emily Dickinson",
                FullName {
                    first: "Emily".into(),
                    last: "Dickinson".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Jules Verne",
                FullName {
                    first: "Jules".into(),
                    last: "Verne".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Edgar Allan Poe",
                FullName {
                    first: vec!["Edgar", "Allan"].into(),
                    last: "Poe".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Simón Bolívar",
                FullName {
                    first: "Simón".into(),
                    last: "Bolívar".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Søren Kierkegaard",
                FullName {
                    first: "Søren".into(),
                    last: "Kierkegaard".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Fyodor Dostoevsky",
                FullName {
                    first: "Fyodor".into(),
                    last: "Dostoevsky".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Mikhail Lomonosov",
                FullName {
                    first: "Mikhail".into(),
                    last: "Lomonosov".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Suu Kyii, Aung San",
                FullName {
                    first: vec!["Aung", "San"].into(),
                    last: vec!["Suu", "Kyii"].into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Nguyễn Du",
                FullName {
                    first: "Nguyễn".into(),
                    last: "Du".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Sun Yat-sen",
                FullName {
                    first: "Sun".into(),
                    last: "Yat-sen".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Hugo Chávez",
                FullName {
                    first: "Hugo".into(),
                    last: "Chávez".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Frida Kahlo",
                FullName {
                    first: "Frida".into(),
                    last: "Kahlo".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Salvador Allende",
                FullName {
                    first: "Salvador".into(),
                    last: "Allende".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Garcia Márquez, Gabriel",
                FullName {
                    first: "Gabriel".into(),
                    last: vec!["Garcia", "Márquez"].into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Antoni Gaudí",
                FullName {
                    first: "Antoni".into(),
                    last: "Gaudí".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Johan Sebastian Bach",
                FullName {
                    first: vec!["Johan", "Sebastian"].into(),
                    last: "Bach".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Blaise Pascal",
                FullName {
                    first: "Blaise".into(),
                    last: "Pascal".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "René Descartes",
                FullName {
                    first: "René".into(),
                    last: "Descartes".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Mahatma Gandhi",
                FullName {
                    first: "Mahatma".into(),
                    last: "Gandhi".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Niels Bohr",
                FullName {
                    first: "Niels".into(),
                    last: "Bohr".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Léon Blum",
                FullName {
                    first: "Léon".into(),
                    last: "Blum".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Jacques Chirac",
                FullName {
                    first: "Jacques".into(),
                    last: "Chirac".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Václav Havel",
                FullName {
                    first: "Václav".into(),
                    last: "Havel".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Jorge Luis Borges",
                FullName {
                    first: vec!["Jorge", "Luis"].into(),
                    last: "Borges".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Paulo Coelho",
                FullName {
                    first: "Paulo".into(),
                    last: "Coelho".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "José Saramago",
                FullName {
                    first: "José".into(),
                    last: "Saramago".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Arundhati Roy",
                FullName {
                    first: "Arundhati".into(),
                    last: "Roy".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Haruki Murakami",
                FullName {
                    first: "Haruki".into(),
                    last: "Murakami".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Kenzaburō Ōe",
                FullName {
                    first: "Kenzaburō".into(),
                    last: "Ōe".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Naguib, Mahfouz",
                FullName {
                    last: "Naguib".into(),
                    first: "Mahfouz".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
        ] {
            let (tail, name) = name(test)?;
            assert_eq!(tail, "", "{}", &test);
            assert_eq!(name, expected, "{}", &test);
        }
        Ok(())
    }

    #[test]
    fn stress_test_unicode() -> Result<()> {
        // courtesy of Chat GPT
        // my appologies if these are nog correct
        // I cannot check them myself but if they are
        // please open an issue!
        // not sure if unicode is even allowed by bibtex
        // but why not give it a try eh?
        for (test, expected) in vec![
            (
                "Лев Толстой", // Russian (Cyrillic)
                FullName {
                    first: "Лев".into(),
                    last: "Толстой".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Αριστοτέλης", // Greek
                FullName {
                    first: vec![].into(),
                    last: "Αριστοτέλης".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "孔子", // Chinese
                FullName {
                    first: vec![].into(),
                    last: vec!["孔子"].into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "אברהם לינקולן", // Hebrew
                FullName {
                    first: "אברהם".into(),
                    last: "לינקולן".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "علي , محمد", // Arabic
                FullName {
                    first: "محمد".into(),
                    last: "علي".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "三島, 由紀夫", // Japanese
                FullName {
                    first: "由紀夫".into(),
                    last: "三島".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "정은 김", // Korean
                FullName {
                    first: "정은".into(),
                    last: "김".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Тарас Шевченко", // Ukrainian (Cyrillic)
                FullName {
                    first: "Тарас".into(),
                    last: "Шевченко".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "ابن سينا", // Persian (Arabic script)
                FullName {
                    first: "ابن".into(),
                    last: "سينا".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "書豪 林", // Chinese
                FullName {
                    first: "書豪".into(),
                    last: "林".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "정환 김", // Korean
                FullName {
                    first: "정환".into(),
                    last: "김".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "山田 太郎", // Japanese
                FullName {
                    last: "太郎".into(),
                    first: "山田".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
            (
                "Владимир Путин", // Russian (Cyrillic)
                FullName {
                    first: "Владимир".into(),
                    last: "Путин".into(),
                    von: vec![].into(),
                    title: vec![].into(),
                },
            ),
        ] {
            let (tail, name) = name(test)?;
            assert_eq!(tail, "", "{}", test);
            assert_eq!(name, expected, "{}", test);
        }
        Ok(())
    }
}
