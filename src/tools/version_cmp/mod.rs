use std::cmp::Ordering;
use semver::Version;

pub fn compare(a: &str, b: &str) -> Ordering{
    let av = Version::parse(&format!("{}.0", a)).unwrap();
    let bv = Version::parse(&format!("{}.0", b)).unwrap();

    av.cmp(&bv)
}