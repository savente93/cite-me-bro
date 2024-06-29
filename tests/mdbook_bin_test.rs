use anyhow::Result;
use std::process::Command;

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
