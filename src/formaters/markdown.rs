use super::Formatter;

#[derive(Default)]
pub struct MarkdownFormatter;

impl Formatter for MarkdownFormatter {
    fn italics(&self, input: &mut String) {
        input.insert(0, '*');
        input.push('*');
    }

    fn bold(&self, input: &mut String) {
        input.insert_str(0, "**");
        input.push_str("**");
    }

    fn hyperlink(&self, input: &mut String) {
        let dup = input.clone();
        input.insert(0, '[');
        input.push_str("](");
        input.push_str(&dup);
        input.push(')');
    }

    fn verbatim(&self, input: &mut String) {
        input.insert(0, '`');
        input.push('`');
    }

    fn escape(&self, input: &mut String) {
        let escapeable_characters = ['\\', '`', '{', '}', '[', ']', '(', ')', '#', '|'];
        let escaped = input.chars().fold(String::new(), |mut acc, c| {
            if escapeable_characters.contains(&c) {
                acc.push('\\');
            };
            acc.push(c);
            acc
        });
        let _ = input.drain(..);
        input.push_str(&escaped);
    }
}

#[cfg(test)]
mod test {
    use crate::formaters::markdown::MarkdownFormatter;
    use crate::formaters::Formatter;

    #[test]
    fn bold() {
        let mut s = String::from("asdf  hey! this is _some_ weird__ input::!");

        MarkdownFormatter.bold(&mut s);
        assert_eq!(
            s,
            String::from("**asdf  hey! this is _some_ weird__ input::!**")
        );
    }

    #[test]
    fn italics() {
        let mut s = String::from("asdf  hey! this is _some_ weird__ input::!");

        MarkdownFormatter.italics(&mut s);
        assert_eq!(
            s,
            String::from("*asdf  hey! this is _some_ weird__ input::!*")
        );
    }

    #[test]
    fn hyperlink() {
        let mut s = String::from("https://example.com");

        MarkdownFormatter.hyperlink(&mut s);
        assert_eq!(
            s,
            String::from("[https://example.com](https://example.com)")
        );
    }
    #[test]
    fn escaped() {
        let mut s = String::from("asdf  []() hey! this is _some_ weird__ input::! <>=--<--");

        MarkdownFormatter.escape(&mut s);
        assert_eq!(
            s,
            String::from("asdf  \\[\\]\\(\\) hey! this is _some_ weird__ input::! <>=--<--")
        );
    }
    #[test]
    fn verbatim() {
        let mut s = String::from("asdf  hey! this is _some_ weird__ input::!");

        MarkdownFormatter.verbatim(&mut s);
        assert_eq!(
            s,
            String::from("`asdf  hey! this is _some_ weird__ input::!`")
        );
    }
}
