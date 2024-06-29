use anyhow::Result;
use std::env;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::process::Command;
use std::str;

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

    assert!(!&output.status.success(), "{:?}", output);
    Ok(())
}
#[test]
fn supports_html() -> Result<()> {
    let output = run_bin()
        .args(["supports", "html"])
        .output()
        .expect("could not run binary");

    assert!(!&output.status.success(), "{:?}", output);
    Ok(())
}
