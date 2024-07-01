use super::Formatter;

#[derive(Default)]
pub struct PlainTextFormatter;

impl Formatter for PlainTextFormatter {
    fn italics(&self, _input: &mut String) {}

    fn bold(&self, _input: &mut String) {}

    fn hyperlink(&self, _input: &mut String) {}

    fn verbatim(&self, _input: &mut String) {}

    fn escape(&self, _input: &mut String) {}
}

#[cfg(test)]
mod test {
    use crate::formaters::plain::PlainTextFormatter;
    use crate::formaters::Formatter;

    #[test]
    fn bold() {
        let mut s = String::from("asdf  hey! this is _some_ weird__ input::!");

        PlainTextFormatter.bold(&mut s);
        assert_eq!(
            s,
            String::from("asdf  hey! this is _some_ weird__ input::!")
        );
    }

    #[test]
    fn italics() {
        let mut s = String::from("asdf  hey! this is _some_ weird__ input::!");

        PlainTextFormatter.italics(&mut s);
        assert_eq!(
            s,
            String::from("asdf  hey! this is _some_ weird__ input::!")
        );
    }

    #[test]
    fn hyperlink() {
        let mut s = String::from("asdf  hey! this is _some_ weird__ input::!");

        PlainTextFormatter.hyperlink(&mut s);
        assert_eq!(
            s,
            String::from("asdf  hey! this is _some_ weird__ input::!")
        );
    }
}
