use std::fmt::Debug;

use anyhow::Error;
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

#[derive(Default, Clone, PartialEq, Eq)]
pub struct FullName<'a> {
    first: Vec<&'a str>,
    last: Vec<&'a str>,
    von: Vec<&'a str>,
    title: Vec<&'a str>,
}

#[derive(Default, Clone, PartialEq, Eq)]
pub struct OwnedFullName {
    pub first: Vec<String>,
    pub last: Vec<String>,
    pub von: Vec<String>,
    pub title: Vec<String>,
}
impl Debug for OwnedFullName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "First({}) Von({}) Last({}) Title({})",
            &self.first.join(" "),
            &self.von.join(" "),
            &self.last.join(" "),
            &self.title.join(" "),
        )
    }
}

impl<'a> From<FullName<'a>> for OwnedFullName {
    fn from(value: FullName) -> Self {
        Self {
            first: value.first.into_iter().map(|v| v.to_string()).collect(),
            last: value.last.into_iter().map(|v| v.to_string()).collect(),
            von: value.von.into_iter().map(|v| v.to_string()).collect(),
            title: value.title.into_iter().map(|v| v.to_string()).collect(),
        }
    }
}

impl<'a> Debug for FullName<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "First({}) Von({}) Last({}) Title({})",
            &self.first.join(" "),
            &self.von.join(" "),
            &self.last.join(" "),
            &self.title.join(" "),
        )
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
    let (tail, word) = alt((
        brace_quoted_literal,
        quote_quoted_literal,
        hyphenated_word,
        inner_word,
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
fn quote_quoted_literal(input: &str) -> IResult<&str, &str> {
    delimited(tag("\""), take_until("\""), tag("\""))(input)
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
            last,
            first,
            title,
            von,
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
            first,
            last,
            von,
            title,
        },
    ))
}

pub fn name(input: &str) -> IResult<&str, FullName, nom::error::Error<&str>> {
    alt((last_first, first_last))(input)
}

fn and_seperated_words(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(
        terminated(tag_no_case("and"), space0),
        terminated(space0, space0),
    )(input)
}

pub fn and_seperated_names(input: &str) -> IResult<&str, Vec<FullName>, nom::error::Error<&str>> {
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
    use nom::character::is_alphabetic;

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
            first: vec!["Isaac"],
            title: vec![],
            von: vec![],
            last: vec!["Newton"]
        }
    );
    parse_test!(
        test_last_last_first,
        last_first,
        "Brinch Hansen, Per",
        FullName {
            first: vec!["Per"],
            title: vec![],
            von: vec![],
            last: vec!["Brinch", "Hansen"]
        }
    );

    parse_test!(
        test_last_first_first,
        last_first,
        "Jackson, Michael Joseph",
        FullName {
            first: vec!["Michael", "Joseph"],
            last: vec!["Jackson"],
            von: vec![],
            title: vec![],
        }
    );
    parse_test!(
        test_last_first_init,
        last_first,
        "Jackson, Michael J",
        FullName {
            first: vec!["Michael", "J"],
            last: vec!["Jackson"],
            von: vec![],
            title: vec![],
        }
    );
    parse_test!(
        test_last_init_init,
        last_first,
        "Jackson, M J",
        FullName {
            first: vec!["M", "J"],
            last: vec!["Jackson"],
            von: vec![],
            title: vec![],
        }
    );
    parse_test!(
        test_first_last,
        first_last,
        "Isaac Newton",
        FullName {
            first: vec!["Isaac"],
            last: vec!["Newton"],
            von: vec![],
            title: vec![],
        }
    );
    parse_test!(
        test_first_first_last,
        first_last,
        "Michael Joseph Jackson",
        FullName {
            first: vec!["Michael", "Joseph"],
            last: vec!["Jackson"],
            von: vec![],
            title: vec![],
        }
    );
    parse_test!(
        test_von_last_initial,
        last_first,
        "van      Beethoven ,    L",
        FullName {
            first: vec!["L"],
            last: vec!["Beethoven"],
            von: vec!["van"],
            title: vec![],
        }
    );
    parse_test!(
        test_von_last_first,
        last_first,
        "van Beethoven, Ludwig",
        FullName {
            first: vec!["Ludwig"],
            last: vec!["Beethoven"],
            von: vec!["van"],
            title: vec![],
        }
    );
    parse_test!(
        test_first_von_last,
        first_last,
        "Ludwig van Beethoven",
        FullName {
            first: vec!["Ludwig"],
            last: vec!["Beethoven"],
            von: vec!["van"],
            title: vec![],
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
            first: vec!["Donald", "E"],
            title: vec![],
            von: vec![],
            last: vec!["Knuth"]
        }
    );
    parse_test!(
        test_quoted_first_last,
        first_last,
        "Ronald {Van der Jawel}",
        FullName {
            first: vec!["Ronald"],
            title: vec![],
            von: vec![],
            last: vec!["Van der Jawel"]
        }
    );

    parse_test!(
        test_and_component_last_first,
        last_first,
        "Fisher, James",
        FullName {
            first: vec!["James"],
            title: vec![],
            von: vec![],
            last: vec!["Fisher"]
        }
    );
    parse_test!(
        test_and_component_first_last,
        first_last,
        "John Clark",
        FullName {
            first: vec!["John"],
            title: vec![],
            von: vec![],
            last: vec!["Clark"]
        }
    );
    parse_test!(
        test_last_first_and_first_last,
        and_seperated_names,
        "Fisher, James AND John Clark",
        vec![
            FullName {
                first: vec!["James"],
                title: vec![],
                von: vec![],
                last: vec!["Fisher"]
            },
            FullName {
                first: vec!["John"],
                title: vec![],
                von: vec![],
                last: vec!["Clark"]
            }
        ]
    );

    parse_test!(
        test_multiple_first_last_and_seperated,
        and_seperated_names,
        "Frank Mittelbach and Michel Gossens and Johannes Braams and David Carlisle and Chris Rowley",
        vec![
            FullName {
                first: vec!["Frank"],
            title: vec![],
            von: vec![],
                last: vec!["Mittelbach"]
            },
            FullName {
                first: vec!["Michel"],
            title: vec![],
            von: vec![],
                last: vec!["Gossens"]
            },
            FullName {
                first: vec!["Johannes"],
            title: vec![],
            von: vec![],
                last: vec!["Braams"]
            },
            FullName {
                first: vec!["David"],
            title: vec![],
            von: vec![],
                last: vec!["Carlisle"]
            },
            FullName {
                first: vec!["Chris"],
            von: vec![],
            title: vec![],
                last: vec!["Rowley"]
            }
        ]
    );
    parse_test!(
        test_and_seperated_quoted,
        and_seperated_names,
        "Geert {Van der Plas} and John Doe and {Barnes and Noble}",
        vec![
            FullName {
                first: vec!["Geert"],
                title: vec![],
                von: vec![],
                last: vec!["Van der Plas"]
            },
            FullName {
                first: vec!["John"],
                title: vec![],
                von: vec![],
                last: vec!["Doe"]
            },
            FullName {
                first: vec![],
                title: vec![],
                von: vec![],
                last: vec!["Barnes and Noble"]
            },
        ]
    );

    parse_test!(
        test_many_name_components,
        first_last,
        "Charles Louis Xavier Joseph de la Vallee Poussin III",
        FullName {
            first: vec!["Charles", "Louis", "Xavier", "Joseph"],
            title: vec!["III"],
            von: vec!["de", "la"],
            last: vec!["Vallee", "Poussin"]
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
                    first: vec!["Albert"],
                    last: vec!["Einstein"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Dr. Emmet Brown",
                FullName {
                    first: vec!["Emmet"],
                    last: vec!["Brown"],
                    von: vec![],
                    title: vec!["Dr"],
                },
            ),
            (
                "Leonardo da Vinci",
                FullName {
                    first: vec!["Leonardo"],
                    last: vec!["Vinci"],
                    von: vec!["da"],
                    title: vec![],
                },
            ),
            (
                "Conan Doyle, Sir Arthur",
                FullName {
                    first: vec!["Arthur"],
                    last: vec!["Conan", "Doyle"],
                    von: vec![],
                    title: vec!["Sir"],
                },
            ),
            (
                "Madame Marie Curie",
                FullName {
                    first: vec!["Marie"],
                    last: vec!["Curie"],
                    von: vec![],
                    title: vec!["Madame"],
                },
            ),
            (
                "Jean-Jacques Rousseau",
                FullName {
                    first: vec!["Jean-Jacques"],
                    last: vec!["Rousseau"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Friedrich Nietzsche",
                FullName {
                    first: vec!["Friedrich"],
                    last: vec!["Nietzsche"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Ada Lovelace",
                FullName {
                    first: vec!["Ada"],
                    last: vec!["Lovelace"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Vincent van Gogh",
                FullName {
                    first: vec!["Vincent"],
                    last: vec!["Gogh"],
                    von: vec!["van"],
                    title: vec![],
                },
            ),
            (
                "Amelia Earhart",
                FullName {
                    first: vec!["Amelia"],
                    last: vec!["Earhart"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Hermann Hesse",
                FullName {
                    first: vec!["Hermann"],
                    last: vec!["Hesse"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Alexandre Dumas",
                FullName {
                    first: vec!["Alexandre"],
                    last: vec!["Dumas"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Lise Meitner",
                FullName {
                    first: vec!["Lise"],
                    last: vec!["Meitner"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Karl Marx",
                FullName {
                    first: vec!["Karl"],
                    last: vec!["Marx"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Che Guevara",
                FullName {
                    first: vec!["Che"],
                    last: vec!["Guevara"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Sigmund Freud",
                FullName {
                    first: vec!["Sigmund"],
                    last: vec!["Freud"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Dr. Seuss",
                FullName {
                    first: vec![],
                    last: vec!["Seuss"],
                    von: vec![],
                    title: vec!["Dr"],
                },
            ),
            (
                "Virginia Woolf",
                FullName {
                    first: vec!["Virginia"],
                    last: vec!["Woolf"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Vasco da Gama",
                FullName {
                    first: vec!["Vasco"],
                    last: vec!["Gama"],
                    von: vec!["da"],
                    title: vec![],
                },
            ),
            (
                "Catherine de Medici",
                FullName {
                    first: vec!["Catherine"],
                    last: vec!["Medici"],
                    von: vec!["de"],
                    title: vec![],
                },
            ),
            (
                "Francisco de Goya",
                FullName {
                    first: vec!["Francisco"],
                    last: vec!["Goya"],
                    von: vec!["de"],
                    title: vec![],
                },
            ),
            (
                "William Shakespeare",
                FullName {
                    first: vec!["William"],
                    last: vec!["Shakespeare"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Niccolo Machiavelli",
                FullName {
                    first: vec!["Niccolo"],
                    last: vec!["Machiavelli"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Dante Alighieri",
                FullName {
                    first: vec!["Dante"],
                    last: vec!["Alighieri"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Gregor Mendel",
                FullName {
                    first: vec!["Gregor"],
                    last: vec!["Mendel"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Emily Dickinson",
                FullName {
                    first: vec!["Emily"],
                    last: vec!["Dickinson"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Jules Verne",
                FullName {
                    first: vec!["Jules"],
                    last: vec!["Verne"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Edgar Allan Poe",
                FullName {
                    first: vec!["Edgar", "Allan"],
                    last: vec!["Poe"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Simón Bolívar",
                FullName {
                    first: vec!["Simón"],
                    last: vec!["Bolívar"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Søren Kierkegaard",
                FullName {
                    first: vec!["Søren"],
                    last: vec!["Kierkegaard"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Fyodor Dostoevsky",
                FullName {
                    first: vec!["Fyodor"],
                    last: vec!["Dostoevsky"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Mikhail Lomonosov",
                FullName {
                    first: vec!["Mikhail"],
                    last: vec!["Lomonosov"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Suu Kyii, Aung San",
                FullName {
                    first: vec!["Aung", "San"],
                    last: vec!["Suu", "Kyii"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Nguyễn Du",
                FullName {
                    first: vec!["Nguyễn"],
                    last: vec!["Du"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Sun Yat-sen",
                FullName {
                    first: vec!["Sun"],
                    last: vec!["Yat-sen"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Hugo Chávez",
                FullName {
                    first: vec!["Hugo"],
                    last: vec!["Chávez"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Frida Kahlo",
                FullName {
                    first: vec!["Frida"],
                    last: vec!["Kahlo"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Salvador Allende",
                FullName {
                    first: vec!["Salvador"],
                    last: vec!["Allende"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Garcia Márquez, Gabriel",
                FullName {
                    first: vec!["Gabriel"],
                    last: vec!["Garcia", "Márquez"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Antoni Gaudí",
                FullName {
                    first: vec!["Antoni"],
                    last: vec!["Gaudí"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Johan Sebastian Bach",
                FullName {
                    first: vec!["Johan", "Sebastian"],
                    last: vec!["Bach"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Blaise Pascal",
                FullName {
                    first: vec!["Blaise"],
                    last: vec!["Pascal"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "René Descartes",
                FullName {
                    first: vec!["René"],
                    last: vec!["Descartes"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Mahatma Gandhi",
                FullName {
                    first: vec!["Mahatma"],
                    last: vec!["Gandhi"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Niels Bohr",
                FullName {
                    first: vec!["Niels"],
                    last: vec!["Bohr"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Léon Blum",
                FullName {
                    first: vec!["Léon"],
                    last: vec!["Blum"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Jacques Chirac",
                FullName {
                    first: vec!["Jacques"],
                    last: vec!["Chirac"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Václav Havel",
                FullName {
                    first: vec!["Václav"],
                    last: vec!["Havel"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Jorge Luis Borges",
                FullName {
                    first: vec!["Jorge", "Luis"],
                    last: vec!["Borges"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Paulo Coelho",
                FullName {
                    first: vec!["Paulo"],
                    last: vec!["Coelho"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "José Saramago",
                FullName {
                    first: vec!["José"],
                    last: vec!["Saramago"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Arundhati Roy",
                FullName {
                    first: vec!["Arundhati"],
                    last: vec!["Roy"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Haruki Murakami",
                FullName {
                    first: vec!["Haruki"],
                    last: vec!["Murakami"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Kenzaburō Ōe",
                FullName {
                    first: vec!["Kenzaburō"],
                    last: vec!["Ōe"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Naguib, Mahfouz",
                FullName {
                    last: vec!["Naguib"],
                    first: vec!["Mahfouz"],
                    von: vec![],
                    title: vec![],
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
                    first: vec!["Лев"],
                    last: vec!["Толстой"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Αριστοτέλης", // Greek
                FullName {
                    first: vec![],
                    last: vec!["Αριστοτέλης"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "孔子", // Chinese
                FullName {
                    first: vec![],
                    last: vec!["孔子"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "אברהם לינקולן", // Hebrew
                FullName {
                    first: vec!["אברהם"],
                    last: vec!["לינקולן"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "علي , محمد", // Arabic
                FullName {
                    first: vec!["محمد"],
                    last: vec!["علي"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "三島, 由紀夫", // Japanese
                FullName {
                    first: vec!["由紀夫"],
                    last: vec!["三島"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "정은 김", // Korean
                FullName {
                    first: vec!["정은"],
                    last: vec!["김"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Тарас Шевченко", // Ukrainian (Cyrillic)
                FullName {
                    first: vec!["Тарас"],
                    last: vec!["Шевченко"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "ابن سينا", // Persian (Arabic script)
                FullName {
                    first: vec!["ابن"],
                    last: vec!["سينا"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "書豪 林", // Chinese
                FullName {
                    first: vec!["書豪"],
                    last: vec!["林"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "정환 김", // Korean
                FullName {
                    first: vec!["정환"],
                    last: vec!["김"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "山田 太郎", // Japanese
                FullName {
                    last: vec!["太郎"],
                    first: vec!["山田"],
                    von: vec![],
                    title: vec![],
                },
            ),
            (
                "Владимир Путин", // Russian (Cyrillic)
                FullName {
                    first: vec!["Владимир"],
                    last: vec!["Путин"],
                    von: vec![],
                    title: vec![],
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
