use std::process::Command;
use std::str;

fn run_cmb() -> Command {
    let mut command = Command::new("cargo");
    command.arg("run").arg("--");
    command
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
L. A. Urry, M. L. Cain, S. A. Wasserman, P. V. Minorsky, and J. B. Reece, \"Photosynthesis,\" in Campbell Biology. New York, NY: Pearson, 2016, pp. 187-221.
H. M. Shapiro, \"Flow cytometry: The glass is half full,\" in Flow Cytometry Protocols, T. S. Hawley and R. G. Hawley, Eds., New York, NY: Springer, 2018, pp. 1-10.
P. Holleis, M. Wagner, and J. Koolwaaij, \"Studying mobile context-aware social services in the wild,\" in Proc. of the 6th Nordic Conf. on Human-Computer Interaction, ser. NordiCHI, New York, NY: ACM, 2010, pp. 207-216.
R Core Team, R: A language and environment for statistical computing, R Foundation for Statistical Computing, Vienna, Austria, 2018.
J. Tang, \"Spin structure of the nucleon in the asymptotic limit,\" M.S. thesis, Massachusetts Institute of Technology, Cambridge, MA, Sep. 1996.
NASA, Pluto: The 'other' red planet, https://www.nasa.gov/nh/pluto-the-other-red-planet, Accessed: 2018-12-06, 2015.
R. C. Rempel, \"Relaxation effects for coupled nuclear spins,\" Ph.D. dissertation, Stanford University, Stanford, CA, Jun. 1956.
S. Stepney and S. Verlan, Eds., Proceedings of the 17th International Conference on Computation and Natural Computation, Fontainebleau, France, vol. 10867, Lecture Notes in Computer Science, Cham, Switzerland: Springer, 2018
V. Bennett, K. Bowman, and S. Wright, \"Wasatch Solar Project final report,\" Salt Lake City Corporation, Salt Lake City, UT, Tech. Rep. DOE-SLC-6903-1, Sep. 2018.
M. Suresh, \"Evolution: A revised theory,\" unpublished.\n";

    dbg!(&output);
    assert_eq!(str::from_utf8(&output.stdout), Ok(expected));
}
