use anyhow::Result;
use std::env;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::process::{Command, ExitStatus};
use std::str;

fn run_cmb() -> Command {
    let mut command = Command::new("cargo");
    command
        .arg("run")
        .arg("-q")
        .arg("--bin")
        .arg("cmb")
        .arg("--");
    command
}
#[test]
fn no_known_keys_errors() -> Result<()> {
    let output = run_cmb()
        .args(["-b", "cite.bib", "asdf"])
        .output()
        .expect("could not run binary");
    let expected_output = "";
    let expected_warning = "Error: none of the keys [\"asdf\"] found in bib file(s) \"cite.bib\"\n";

    assert!(!&output.status.success(), "{:?}", output);
    assert_eq!(str::from_utf8(&output.stdout), Ok(expected_output));
    assert_eq!(str::from_utf8(&output.stderr), Ok(expected_warning));
    Ok(())
}
#[test]
fn inplace_file() -> Result<()> {
    let initial_contets =
        "there once was a citation: \\cite{book}. adsflkjwoiejflkdslslsldlkfki nrgiwf";
    let expected_contets = "there once was a citation: L. Susskind and G. Hrabovsky, Classical mechanics: the theoretical minimum. New York, NY: Penguin Random House, 2014.. adsflkjwoiejflkdslslsldlkfki nrgiwf";
    let path = {
        let tmp_dir = env::temp_dir();
        let path = tmp_dir.join("test_file.txt");
        let mut write_file = File::create(&path)?;
        write_file.write_all(initial_contets.as_bytes())?;
        path
    };
    run_cmb()
        .args(["-b", "cite.bib", "-i", path.to_str().unwrap()])
        .output()
        .expect("could not run binary");
    let contents = read_to_string(path)?;

    assert_eq!(expected_contets, contents);
    Ok(())
}
#[test]
fn run_full_file_ieee() {
    let output = run_cmb()
        .args(["-b", "cite.bib", "--style", "ieee"])
        .output()
        .expect("could not run binary");
    let expected = "L. Breiman, \"Random forests,\" Machine learning, vol. 45, no. 1, pp. 5-32, 2001. doi: https://doi.org/10.1023/a:1010933404324.
J. Liao, X. Cao, L. Zhao, et al., \"The importance of neutral and niche processes for bacterial community assembly differs between habitat generalists and specialists,\" FEMS Microbiology Ecology, vol. 92, no. 11, Aug. 2016, issn: 0168-6496. doi: https://doi.org/10.1093/femsec/fiw174. [Online]. Available: https://doi.org/10.1093/femsec/fiw174.
P. J. Cohen, \"The independence of the continuum hypothesis,\" Proceedings of the National Academy of Sciences, vol. 50, no. 6, pp. 1143-1148, 1963.
L. Susskind and G. Hrabovsky, Classical mechanics: the theoretical minimum. New York, NY: Penguin Random House, 2014.
M. Swetla, Canoe tours in Sweden, Distributed at the Stockholm Tourist Office, Jul. 2015.
L. A. Urry, M. L. Cain, S. A. Wasserman, P. V. Minorsky, and J. B. Reece, \"Photosynthesis,\" in Campbell biology. New York, NY: Pearson, 2016, pp. 187-221.
H. M. Shapiro, \"Flow cytometry: The glass is half full,\" in Flow cytometry protocols, T. S. Hawley and R. G. Hawley, Eds., New York, NY: Springer, 2018, pp. 1-10.
P. Holleis, M. Wagner, and J. Koolwaaij, \"Studying mobile context-aware social services in the wild,\" in Proc. of the 6th Nordic Conf. on Human-Computer Interaction, ser. NordiCHI, New York, NY: ACM, 2010, pp. 207-216.
R Core Team, R: A language and environment for statistical computing, R Foundation for Statistical Computing, Vienna, Austria, 2018.
J. Tang, \"Spin structure of the nucleon in the asymptotic limit,\" M.S. thesis, Massachusetts Institute of Technology, Cambridge, MA, Sep. 1996.
NASA, Pluto: The 'other' red planet, https://www.nasa.gov/nh/pluto-the-other-red-planet, Accessed: 2018-12-06, 2015.
R. C. Rempel, \"Relaxation effects for coupled nuclear spins,\" Ph.D. dissertation, Stanford University, Stanford, CA, Jun. 1956.
S. Stepney and S. Verlan, Eds., Proceedings of the 17th international conference on computation and natural computation, fontainebleau, france, vol. 10867, Lecture Notes in Computer Science, Cham, Switzerland: Springer, 2018
V. Bennett, K. Bowman, and S. Wright, \"Wasatch Solar Project final report,\" Salt Lake City Corporation, Salt Lake City, UT, Tech. Rep. DOE-SLC-6903-1, Sep. 2018.
M. Suresh, \"Evolution: A revised theory,\" unpublished.
J. Smith and J. Doe, \"The Effects of Climate Change,\" in Proceedings of the Annual Conference on Climate Change, B. Johnson, Ed., Climate Change Association, Los Angeles, CA: Springer, Jun. 2022, pp. 55-62.\n";

    assert_eq!(str::from_utf8(&output.stdout), Ok(expected));
}

#[test]
fn run_unknown_key_and_book_ieee() {
    let output = run_cmb()
        .args(["-b", "cite.bib", "--style", "ieee", "asdf", "book"])
        .output()
        .expect("could not run binary");
    let expected_output = "L. Susskind and G. Hrabovsky, Classical mechanics: the theoretical minimum. New York, NY: Penguin Random House, 2014.\n";
    let expected_warning = "No entry for key asdf was found, skipping...\n";

    assert!(&output.status.success());
    assert_eq!(str::from_utf8(&output.stdout), Ok(expected_output));
    assert_eq!(str::from_utf8(&output.stderr), Ok(expected_warning));
}

#[test]
fn run_tb_ieee_html() {
    let output = run_cmb()
        .args([
            "-b",
            "cite.bib",
            "--style",
            "ieee",
            "--format",
            "html",
            "10.1093/femsec/fiw174",
        ])
        .output()
        .expect("could not run binary");
    let expected_output = "J. Liao, X. Cao, L. Zhao, et al., \"The importance of neutral and niche processes for bacterial community assembly differs between habitat generalists and specialists,\" <i>FEMS Microbiology Ecology,</i> vol. 92, no. 11, Aug. 2016, issn: 0168-6496. doi: https://doi.org/10.1093/femsec/fiw174. [Online]. Available: <a href=\"https://doi.org/10.1093/femsec/fiw174\">https://doi.org/10.1093/femsec/fiw174</a>.\n";

    assert!(&output.status.success());
    assert_eq!(str::from_utf8(&output.stdout), Ok(expected_output));
}

#[test]
fn run_tb_ieee_md() {
    let output = run_cmb()
        .args([
            "-b",
            "cite.bib",
            "--style",
            "ieee",
            "--format",
            "markdown",
            "10.1093/femsec/fiw174",
        ])
        .output()
        .expect("could not run binary");
    let expected_output = "J. Liao, X. Cao, L. Zhao, et al., \"The importance of neutral and niche processes for bacterial community assembly differs between habitat generalists and specialists,\" *FEMS Microbiology Ecology,* vol. 92, no. 11, Aug. 2016, issn: 0168-6496. doi: https://doi.org/10.1093/femsec/fiw174. [Online]. Available: [https://doi.org/10.1093/femsec/fiw174](https://doi.org/10.1093/femsec/fiw174).\n";

    assert!(&output.status.success());
    assert_eq!(str::from_utf8(&output.stdout), Ok(expected_output));
}

#[test]
fn run_rf_apa_html() {
    let output = run_cmb()
        .args([
            "-b",
            "cite.bib",
            "--style",
            "apa",
            "--format",
            "html",
            "10.1093/femsec/fiw174",
        ])
        .output()
        .expect("could not run binary");
    let expected_output = "Liao, J., Cao, X., Zhao, L., Wang, J., Gao, Z., Wang, M. C., & Huang, Y. (2016). The importance of neutral and niche processes for bacterial community assembly differs between habitat generalists and specialists. <i>FEMS Microbiology Ecology, 92</i> (11), <a href=\"https://doi.org/10.1093/femsec/fiw174\">https://doi.org/10.1093/femsec/fiw174</a>\n";

    assert!(&output.status.success());
    assert_eq!(str::from_utf8(&output.stdout), Ok(expected_output));
}
#[test]
fn run_book_apa() {
    let output = run_cmb()
        .args(["-b", "cite.bib", "--style", "apa", "book"])
        .output()
        .expect("could not run binary");
    let expected_output = "Susskind, L., & Hrabovsky, G. (2014). Classical mechanics: the theoretical minimum. Penguin Random House.\n";
    let expected_warning = "";

    assert!(&output.status.success());
    assert_eq!(str::from_utf8(&output.stdout), Ok(expected_output));
    assert_eq!(str::from_utf8(&output.stderr), Ok(expected_warning));
}
#[test]
fn run_no_warning_on_quiet() {
    let output = run_cmb()
        .args(["-b", "cite.bib", "asdf", "-q"])
        .output()
        .expect("could not run binary");
    let expected_output = "";
    let expected_warning = "";

    assert!(&output.status.success(), "{:?}", output);
    assert_eq!(str::from_utf8(&output.stdout), Ok(expected_output));
    assert_eq!(str::from_utf8(&output.stderr), Ok(expected_warning));
}
#[test]
fn exists_on_fail_fast() {
    let output = run_cmb()
        .args(["-b", "cite.bib", "asdf", "book", "--fail-fast"])
        .output()
        .expect("error running binary");
    assert!(&output.status.success(), "{:?}", &output);
}
