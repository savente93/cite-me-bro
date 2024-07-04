use anyhow::Result;
use std::{
    io::{Read, Write},
    process::{Command, ExitStatus, Stdio},
};

fn run_bin() -> Command {
    let mut command = Command::new("cargo");
    command
        .arg("run")
        .arg("-q")
        .arg("--bin")
        .arg("mdbook-citations")
        .arg("--");
    command
}

#[test]
fn supports_markdown() -> Result<()> {
    let output = run_bin()
        .args(["supports", "markdown"])
        .output()
        .expect("could not run binary");

    assert!(&output.status.success(), "{:?}", output);
    Ok(())
}
#[test]
fn doesnt_support_unknown_format() -> Result<()> {
    let output = run_bin()
        .args(["supports", "asdfasdf"])
        .output()
        .expect("could not run binary");

    assert!(!&output.status.success(), "{:?}", output);
    Ok(())
}
#[test]
fn supports_html() -> Result<()> {
    let output = run_bin()
        .args(["supports", "html"])
        .output()
        .expect("could not run binary");

    assert!(&output.status.success(), "{:?}", output);
    Ok(())
}

#[test]
fn errors_on_nonexistant_config() -> Result<()>{
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
                    "mdbook_version": "0.4.20"
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
            let mut child = run_bin()
            .stdin(Stdio::piped())
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to run bsinary");
        let mut stdin = child.stdin.take().unwrap();
        stdin.write_all(input_json.as_bytes())?;
        drop(stdin);
        let exit_code = child.wait().expect("DOH!+");
        assert!(!ExitStatus::success(&exit_code));
        Ok(())
}

#[test]
fn errors_on_no_config() -> Result<()> {
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
                    "mdbook_version": "0.4.20"
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

    let mut child = run_bin()
        .stdin(Stdio::piped())
        .spawn()
        .expect("failed to run bsinary");
    let mut stdin = child.stdin.take().unwrap();
    stdin.write_all(input_json.as_bytes())?;
    drop(stdin);
    let exit_code = child.wait().expect("DOH!+");
    assert!(!ExitStatus::success(&exit_code));
    Ok(())
}


#[test]
fn run_without_citations_is_noop() -> Result<()> {
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
                    "mdbook_version": "0.4.20"
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
    let expected_output_json = r##"{"sections":[{"Chapter":{"name":"Chapter 1","content":"# Chapter 1\n","number":[1],"sub_items":[],"path":"chapter_1.md","source_path":"chapter_1.md","parent_names":[]}}],"__non_exhaustive":null}"##;

    let mut child = run_bin()
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to run bsinary");
    let mut stdin = child.stdin.take().unwrap();
    stdin.write_all(input_json.as_bytes())?;
    drop(stdin);
    let exit_code = child.wait().expect("DOH!+");
    let mut output = String::new();
    let mut stdout = child.stdout.unwrap();
    stdout.read_to_string(&mut output)?;
    assert!(ExitStatus::success(&exit_code));
    assert_eq!(output, expected_output_json);
    Ok(())
}
#[test]
fn respects_style() -> Result<()> {
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
                            "citations": {"bibfile":"cite.bib", "style":"apa"}
                        }
                    },
                    "renderer": "html",
                    "mdbook_version": "0.4.20"
                },
                {
                    "sections": [
                        {
                            "Chapter": {
                                "name": "\\cite{book}",
                                "content": "\\cite{article}",
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
    let expected_output_json = r##"{"sections":[{"Chapter":{"name":"Susskind, L., & Hrabovsky, G. (2014). *Classical mechanics: the theoretical minimum*. Penguin Random House.","content":"Cohen, P. J. (1963). The independence of the continuum hypothesis. *Proceedings of the National Academy of Sciences, 50* (6), 1143-1148.","number":[1],"sub_items":[],"path":"chapter_1.md","source_path":"chapter_1.md","parent_names":[]}}],"__non_exhaustive":null}"##;

    let mut child = run_bin()
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to run bsinary");
    let mut stdin = child.stdin.take().unwrap();
    stdin.write_all(input_json.as_bytes())?;
    drop(stdin);
    let exit_code = child.wait().expect("DOH!+");
    let mut output = String::new();
    let mut stdout = child.stdout.unwrap();
    stdout.read_to_string(&mut output)?;
    assert!(ExitStatus::success(&exit_code), );
    assert_eq!(output, expected_output_json);
    Ok(())
}

#[test]
fn respects_format() -> Result<()> {
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
                    "renderer": "markdown",
                    "mdbook_version": "0.4.20"
                },
                {
                    "sections": [
                        {
                            "Chapter": {
                                "name": "\\cite{book}",
                                "content": "\\cite{article}",
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
    let expected_output_json = r##"{"sections":[{"Chapter":{"name":"L. Susskind and G. Hrabovsky, *Classical mechanics: the theoretical minimum*. New York, NY: Penguin Random House, 2014.","content":"P. J. Cohen, \"The independence of the continuum hypothesis,\" *Proceedings of the National Academy of Sciences,* vol. 50, no. 6, pp. 1143-1148, 1963.","number":[1],"sub_items":[],"path":"chapter_1.md","source_path":"chapter_1.md","parent_names":[]}}],"__non_exhaustive":null}"##;

    let mut child = run_bin()
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to run bsinary");
    let mut stdin = child.stdin.take().unwrap();
    stdin.write_all(input_json.as_bytes())?;
    drop(stdin);
    let exit_code = child.wait().expect("DOH!+");
    let mut output = String::new();
    let mut stdout = child.stdout.unwrap();
    stdout.read_to_string(&mut output)?;
    assert!(ExitStatus::success(&exit_code));
    assert_eq!(output, expected_output_json);
    Ok(())
}

#[test]
fn citation_in_content_and_title() -> Result<()> {
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
                    "mdbook_version": "0.4.20"
                },
                {
                    "sections": [
                        {
                            "Chapter": {
                                "name": "\\cite{book}",
                                "content": "\\cite{article}",
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
    let expected_output_json = r##"{"sections":[{"Chapter":{"name":"L. Susskind and G. Hrabovsky, *Classical mechanics: the theoretical minimum*. New York, NY: Penguin Random House, 2014.","content":"P. J. Cohen, \"The independence of the continuum hypothesis,\" *Proceedings of the National Academy of Sciences,* vol. 50, no. 6, pp. 1143-1148, 1963.","number":[1],"sub_items":[],"path":"chapter_1.md","source_path":"chapter_1.md","parent_names":[]}}],"__non_exhaustive":null}"##;

    let mut child = run_bin()
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to run bsinary");
    let mut stdin = child.stdin.take().unwrap();
    stdin.write_all(input_json.as_bytes())?;
    drop(stdin);
    let exit_code = child.wait().expect("DOH!+");
    let mut output = String::new();
    let mut stdout = child.stdout.unwrap();
    stdout.read_to_string(&mut output)?;
    assert!(ExitStatus::success(&exit_code));
    assert_eq!(output, expected_output_json);
    Ok(())
}

