use std::path::PathBuf;

use crate::styles::ReferenceStyle;
use crate::Format;

use super::bibligraphy::Bibliography;
use mdbook::book::Book;
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use mdbook::BookItem;
use toml::Value;

/// A preprocessor to expand citations within the book
#[derive(Default)]
pub struct CitationPreprocessor;

impl Preprocessor for CitationPreprocessor {
    fn name(&self) -> &str {
        "citations"
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        Format::try_from(renderer).is_ok()
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        if let Some(cite_cfg) = ctx.config.get_preprocessor(self.name()) {
            if let Some(bib_file_val) = cite_cfg.get("bibfile") {
                let bib_file_paths = match bib_file_val {
                    Value::String(s) => Ok(vec![PathBuf::from(s)]),
                    Value::Array(a) => Ok(a
                        .iter()
                        .filter_map(|v| v.as_str())
                        .map(PathBuf::from)
                        .collect()),
                    _ => Err(Error::msg("config of bibfile did not have correct type")),
                }?;
                // mdbook preprocessorrs seem to always opperate on markdown
                let format = Format::Markdown;

                let style_str = cite_cfg
                    .get("style")
                    .and_then(|k| k.as_str())
                    .unwrap_or("ieee");
                let style = ReferenceStyle::try_from(style_str)?;

                let bibliography = Bibliography::from_files(bib_file_paths)?;
                book.for_each_mut(|item| expandify_item(&bibliography, style, format, item));
                Ok(book)
            } else {
                Err(Error::msg("config entry did not contain 'bibfile' key"))
            }
        } else {
            Err(Error::msg("no config entry found"))
        }
    }
}

// TODO pick a better name
fn expandify_item(bib: &Bibliography, style: ReferenceStyle, fmt: Format, bi: &mut BookItem) {
    match bi {
        mdbook::BookItem::PartTitle(t) => {
            let new = bib.expand_citations_in_string(t, style, fmt);
            t.clear();
            t.push_str(&new);
        }
        mdbook::BookItem::Chapter(c) => {
            let new = bib.expand_citations_in_string(&mut c.content, style, fmt);
            c.content = new;
            c.sub_items
                .iter_mut()
                .for_each(|si| expandify_item(bib, style, fmt, si));
            let new = bib.expand_citations_in_string(&mut c.name, style, fmt);
            c.name = new;
        }
        mdbook::BookItem::Separator => (),
    }
}
