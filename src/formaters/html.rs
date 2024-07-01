use super::Formatter;

#[derive(Default)]
pub struct HtmlFormatter;

impl Formatter for HtmlFormatter {
    fn italics(&self, input: &mut String) {
        input.insert_str(0, "<i>");
        input.push_str("</i>");
    }

    fn bold(&self, input: &mut String) {
        input.insert_str(0, "<b>");
        input.push_str("</b>");
    }

    fn hyperlink(&self, input: &mut String) {
        let dup = input.clone();
        input.insert_str(0, "<a href=\"");
        input.push_str("\">");
        input.push_str(&dup);
        input.push_str("</a>");
    }

    fn verbatim(&self, input: &mut String) {
        input.insert_str(0, "<pre>");
        input.push_str("</pre>");
    }

    fn escape(&self, _input: &mut String) {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::formaters::html::HtmlFormatter;
    use crate::formaters::Formatter;

    #[test]
    fn bold() {
        let mut s = String::from("asdf  hey! this is _some_ weird__ input::!");

        HtmlFormatter.bold(&mut s);
        assert_eq!(
            s,
            String::from("<b>asdf  hey! this is _some_ weird__ input::!</b>")
        );
    }

    #[test]
    fn italics() {
        let mut s = String::from("asdf  hey! this is _some_ weird__ input::!");

        HtmlFormatter.italics(&mut s);
        assert_eq!(
            s,
            String::from("<i>asdf  hey! this is _some_ weird__ input::!</i>")
        );
    }

    #[test]
    fn hyperlink() {
        let mut s = String::from("https://example.com");

        HtmlFormatter.hyperlink(&mut s);
        assert_eq!(
            s,
            String::from("<a href=\"https://example.com\">https://example.com</a>")
        );
    }
}
