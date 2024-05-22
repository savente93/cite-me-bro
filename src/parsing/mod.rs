#![allow(dead_code)]
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::one_of,
    multi::many1,
    sequence::{separated_pair, terminated},
    AsChar, IResult,
};

// pub fn initial(input: &[u8]) -> IResult<&[u8], &[u8]> {
//     let (tail, init_alpha_seg) = take_while1(is_alphabetic)(input)?;
//     let (tail, period) = tag(".")(tail)?;
//     Ok([init_alpha_seg, period].concat(), tail)
// }
fn initial(input: &str) -> IResult<&str, char> {
    // Define a parser that matches a capital letter and a period
    let (tail, initial) = terminated(
        one_of("QWERTYUIOPASDFGHJKLZXCVBNM,"),
        alt((tag("."), tag(". "))),
    )(input)?;
    Ok((tail, initial))
}

fn word(input: &str) -> IResult<&str, &str> {
    let (tail, word) = take_while1(AsChar::is_alpha)(input)?;
    Ok((tail, word))
}

fn initials(input: &str) -> IResult<&str, Vec<char>> {
    let (tail, initials) = many1(initial)(input)?;
    Ok((tail, initials))
}

fn name(input: &str) -> IResult<&str, (Vec<char>, &str)> {
    let (tail, (initials, last_name)) = separated_pair(initials, tag(" "), word)(input)?;

    Ok((tail, (initials, last_name)))
}
// fn hypenated_word(input: &str) -> IResult<&str, &str> {}

#[cfg(test)]
mod test {

    use super::*;
    use anyhow::Result;
    #[test]
    fn test_name_single_initial() -> Result<()> {
        let (tail, head) = initial("J. Doe")?;
        assert_eq!(head, 'J');
        assert_eq!(tail, " Doe");
        Ok(())
    }
    #[test]
    fn test_name_multiple_initials_spaces() -> Result<()> {
        let (tail, initials) = initials("J. R. R. Tolkien")?;
        assert_eq!(tail, " Tolkien");
        assert_eq!(initials, vec!['J', 'R', 'R']);
        Ok(())
    }
    #[test]
    fn test_full_name_parse() -> Result<()> {
        let (tail, (initials, last_name)) = name("J. R. R. Tolkien")?;
        assert_eq!(initials, vec!['J', 'R', 'R']);
        assert_eq!(last_name, " Tolkien");
        assert_eq!(tail, "");
        Ok(())
    }
    #[test]
    fn test_name_multiple_initials_no_spaces() -> Result<()> {
        let (tail, initials) = initials("J.R.R. Tolkien")?;
        assert_eq!(tail, " Tolkien");
        assert_eq!(initials, vec!['J', 'R', 'R']);
        Ok(())
    }
}
