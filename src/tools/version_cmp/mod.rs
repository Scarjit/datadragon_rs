use std::cmp::Ordering;
use semver::Version;

pub fn compare(a: &str, b: &str) -> Ordering{
    let av = Version::parse(&format!("{}.0", a)).expect("Couldn't parse version");
    let bv = Version::parse(&format!("{}.0", b)).expect("Couldn't parse version");

    av.cmp(&bv)
}