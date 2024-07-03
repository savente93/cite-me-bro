use std::path::PathBuf;

use super::bibligraphy::Bibliography;
use mdbook::book::Book;
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use toml::Value;

/// A preprocessor to expand citations within the book
#[derive(Default)]
pub struct CitationPreprocessor;

impl Preprocessor for CitationPreprocessor {
    fn name(&self) -> &str {
        "citations"
    }

    fn run(&self, ctx: &PreprocessorContext, book: Book) -> Result<Book, Error> {
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
                let _bibligraphy = Bibliography::from_files(bib_file_paths);
                Ok(book)
            } else {
                Err(Error::msg("config entry did not contain 'bibfile' key"))
            }
        } else {
            Err(Error::msg("no config entry found"))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run_without_citations_is_noop() {
        let input_json = r##"[
                {
                    "root": "/path/to/book",
                    "config": {
                        "book": {
                            "authors": ["AUTHOR"],
                            "language": "en",
                            "multilingual": false,
                            "src": "src",
                            "title": "TITLE"
                        },
                        "preprocessor": {
                            "citations": {"bibfile":"cite.bib"}
                        }
                    },
                    "renderer": "html",
                    "mdbook_version": "0.4.21"
                },
                {
                    "sections": [
                        {
                            "Chapter": {
                                "name": "Chapter 1",
                                "content": "# Chapter 1\n",
                                "number": [1],
                                "sub_items": [],
                                "path": "chapter_1.md",
                                "source_path": "chapter_1.md",
                                "parent_names": []
                            }
                        }
                    ],
                    "__non_exhaustive": null
                }
            ]"##;
        let input_json = input_json.as_bytes();

        let (ctx, book) = mdbook::preprocess::CmdPreprocessor::parse_input(input_json).unwrap();
        let expected_book = book.clone();
        let result = CitationPreprocessor::default().run(&ctx, book);
        assert!(result.is_ok());

        // The nop-preprocessor should not have made any changes to the book content.
        let actual_book = result.unwrap();
        assert_eq!(actual_book, expected_book);
    }

    #[test]
    fn errors_on_nonexistant_config() {
        let input_json = r##"[
                {
                    "root": "/path/to/book",
                    "config": {
                        "book": {
                            "authors": ["AUTHOR"],
                            "language": "en",
                            "multilingual": false,
                            "src": "src",
                            "title": "TITLE"
                        },
                        "preprocessor": {
                            "citations": {"bibfile": "asdfasdfasdffasdf"}
                        }
                    },
                    "renderer": "html",
                    "mdbook_version": "0.4.21"
                },
                {
                    "sections": [
                        {
                            "Chapter": {
                                "name": "Chapter 1",
                                "content": "# Chapter 1\n",
                                "number": [1],
                                "sub_items": [],
                                "path": "chapter_1.md",
                                "source_path": "chapter_1.md",
                                "parent_names": []
                            }
                        }
                    ],
                    "__non_exhaustive": null
                }
            ]"##;
        let input_json = input_json.as_bytes();

        let (ctx, book) = mdbook::preprocess::CmdPreprocessor::parse_input(input_json).unwrap();
        let result = CitationPreprocessor::default().run(&ctx, book);
        // should error because we didn't specify a bib file
        assert!(result.is_err());
    }
    #[test]
    fn respects_style() {
        let input_json = r##"[
                {
                    "root": "/path/to/book",
                    "config": {
                        "book": {
                            "authors": ["AUTHOR"],
                            "language": "en",
                            "multilingual": false,
                            "src": "src",
                            "title": "TITLE"
                        },
                        "preprocessor": {
                            "citations": {"bibfile":"cite.bib"}
                        }
                    },
                    "renderer": "html",
                    "mdbook_version": "0.4.21"
                },
                {
                    "sections": [
                        {
                            "Chapter": {
                                "name": "\cite{book}",
                                "content": "\cite{article}",
                                "number": [1],
                                "sub_items": [],
                                "path": "chapter_1.md",
                                "source_path": "chapter_1.md",
                                "parent_names": []
                            }
                        }
                    ],
                    "__non_exhaustive": null
                }
            ]"##;
        let input_json = input_json.as_bytes();
        let expected_json = r##"[
                {
                    "root": "/path/to/book",
                    "config": {
                        "book": {
                            "authors": ["AUTHOR"],
                            "language": "en",
                            "multilingual": false,
                            "src": "src",
                            "title": "TITLE"
                        },
                        "preprocessor": {
                            "citations": {"bibfile":"cite.bib", "style":"apa"}
                        }
                    },
                    "renderer": "markdown",
                    "mdbook_version": "0.4.21"
                },
                {
                    "sections": [
                        {
                            "Chapter": {
                                "name": "Susskind, L., & Hrabovsky, G. (2014). <i>Classical mechanics: the theoretical minimum</i>. Penguin Random House.",
                                "content": "Cohen, P. J. (1963). The independence of the continuum hypothesis. <i>Proceedings of the National Academy of Sciences, 50</i> (6), 1143-1148.",
                                "number": [1],
                                "sub_items": [],
                                "path": "chapter_1.md",
                                "source_path": "chapter_1.md",
                                "parent_names": []
                            }
                        }
                    ],
                    "__non_exhaustive": null
                }
            ]"##;
        let expected_json = expected_json.as_bytes();

        let (ctx, book) = mdbook::preprocess::CmdPreprocessor::parse_input(input_json).unwrap();
        let (_ctx, expected_book) =
            mdbook::preprocess::CmdPreprocessor::parse_input(expected_json).unwrap();
        let result = CitationPreprocessor::default().run(&ctx, book);
        assert!(result.is_ok());

        let actual_book = result.unwrap();
        assert_eq!(actual_book, expected_book);
    }

    #[test]
    fn respects_format() {
        let input_json = r##"[
                {
                    "root": "/path/to/book",
                    "config": {
                        "book": {
                            "authors": ["AUTHOR"],
                            "language": "en",
                            "multilingual": false,
                            "src": "src",
                            "title": "TITLE"
                        },
                        "preprocessor": {
                            "citations": {"bibfile":"cite.bib"}
                        }
                    },
                    "renderer": "html",
                    "mdbook_version": "0.4.21"
                },
                {
                    "sections": [
                        {
                            "Chapter": {
                                "name": "\cite{book}",
                                "content": "\cite{article}",
                                "number": [1],
                                "sub_items": [],
                                "path": "chapter_1.md",
                                "source_path": "chapter_1.md",
                                "parent_names": []
                            }
                        }
                    ],
                    "__non_exhaustive": null
                }
            ]"##;
        let input_json = input_json.as_bytes();
        let expected_json = r##"[
                {
                    "root": "/path/to/book",
                    "config": {
                        "book": {
                            "authors": ["AUTHOR"],
                            "language": "en",
                            "multilingual": false,
                            "src": "src",
                            "title": "TITLE"
                        },
                        "preprocessor": {
                            "citations": {"bibfile":"cite.bib"}
                        }
                    },
                    "renderer": "markdown",
                    "mdbook_version": "0.4.21"
                },
                {
                    "sections": [
                        {
                            "Chapter": {
                                "name": "L. Susskind and G. Hrabovsky, *Classical mechanics: the theoretical minimum*. New York, NY: Penguin Random House, 2014.",
                                "content": "P. J. Cohen, \"The independence of the continuum hypothesis,\" *Proceedings of the National Academy of Sciences,* vol. 50, no. 6, pp. 1143-1148, 1963.",
                                "number": [1],
                                "sub_items": [],
                                "path": "chapter_1.md",
                                "source_path": "chapter_1.md",
                                "parent_names": []
                            }
                        }
                    ],
                    "__non_exhaustive": null
                }
            ]"##;
        let expected_json = expected_json.as_bytes();

        let (ctx, book) = mdbook::preprocess::CmdPreprocessor::parse_input(input_json).unwrap();
        let (_ctx, expected_book) =
            mdbook::preprocess::CmdPreprocessor::parse_input(expected_json).unwrap();
        let result = CitationPreprocessor::default().run(&ctx, book);
        assert!(result.is_ok());

        let actual_book = result.unwrap();
        assert_eq!(actual_book, expected_book);
    }

    #[test]
    fn citation_in_content_and_title() {
        let input_json = r##"[
                {
                    "root": "/path/to/book",
                    "config": {
                        "book": {
                            "authors": ["AUTHOR"],
                            "language": "en",
                            "multilingual": false,
                            "src": "src",
                            "title": "TITLE"
                        },
                        "preprocessor": {
                            "citations": {"bibfile":"cite.bib"}
                        }
                    },
                    "renderer": "html",
                    "mdbook_version": "0.4.21"
                },
                {
                    "sections": [
                        {
                            "Chapter": {
                                "name": "\cite{book}",
                                "content": "\cite{article}",
                                "number": [1],
                                "sub_items": [],
                                "path": "chapter_1.md",
                                "source_path": "chapter_1.md",
                                "parent_names": []
                            }
                        }
                    ],
                    "__non_exhaustive": null
                }
            ]"##;
        let input_json = input_json.as_bytes();
        let expected_json = r##"[
                {
                    "root": "/path/to/book",
                    "config": {
                        "book": {
                            "authors": ["AUTHOR"],
                            "language": "en",
                            "multilingual": false,
                            "src": "src",
                            "title": "TITLE"
                        },
                        "preprocessor": {
                            "citations": {"bibfile":"cite.bib"}
                        }
                    },
                    "renderer": "html",
                    "mdbook_version": "0.4.21"
                },
                {
                    "sections": [
                        {
                            "Chapter": {
                                "name": "L. Susskind and G. Hrabovsky, <i>Classical mechanics: the theoretical minimum</i>. New York, NY: Penguin Random House, 2014.",
                                "content": "P. J. Cohen, \"The independence of the continuum hypothesis,\" <i>Proceedings of the National Academy of Sciences,</i> vol. 50, no. 6, pp. 1143-1148, 1963.",
                                "number": [1],
                                "sub_items": [],
                                "path": "chapter_1.md",
                                "source_path": "chapter_1.md",
                                "parent_names": []
                            }
                        }
                    ],
                    "__non_exhaustive": null
                }
            ]"##;
        let expected_json = expected_json.as_bytes();

        let (ctx, book) = mdbook::preprocess::CmdPreprocessor::parse_input(input_json).unwrap();
        let (_ctx, expected_book) =
            mdbook::preprocess::CmdPreprocessor::parse_input(expected_json).unwrap();
        let result = CitationPreprocessor::default().run(&ctx, book);
        assert!(result.is_ok());

        let actual_book = result.unwrap();
        assert_eq!(actual_book, expected_book);
    }

    #[test]
    fn errors_on_no_config() {
        let input_json = r##"[
                {
                    "root": "/path/to/book",
                    "config": {
                        "book": {
                            "authors": ["AUTHOR"],
                            "language": "en",
                            "multilingual": false,
                            "src": "src",
                            "title": "TITLE"
                        },
                        "preprocessor": {
                            "citations": {}
                        }
                    },
                    "renderer": "html",
                    "mdbook_version": "0.4.21"
                },
                {
                    "sections": [
                        {
                            "Chapter": {
                                "name": "Chapter 1",
                                "content": "# Chapter 1\n",
                                "number": [1],
                                "sub_items": [],
                                "path": "chapter_1.md",
                                "source_path": "chapter_1.md",
                                "parent_names": []
                            }
                        }
                    ],
                    "__non_exhaustive": null
                }
            ]"##;
        let input_json = input_json.as_bytes();

        let (ctx, book) = mdbook::preprocess::CmdPreprocessor::parse_input(input_json).unwrap();
        let result = CitationPreprocessor::default().run(&ctx, book);
        // should error because we didn't specify a bib file
        assert!(result.is_err());
    }
}
