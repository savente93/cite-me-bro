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

    fn escape(&self, _input: &mut String) {
        todo!()
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
}
