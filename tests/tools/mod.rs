use std::cmp::Ordering;
use datadragon_rs::tools::version_cmp::compare;

#[test]
pub fn test_compare(){

    assert_eq!(compare("9.20", "9.9"), Ordering::Greater);
    assert_eq!(compare("9.20", "9.10"), Ordering::Greater);
    assert_eq!(compare("9.20", "7.9"), Ordering::Greater);
    assert_eq!(compare("1.20", "9.9"), Ordering::Less);
    assert_eq!(compare("9.20", "9.20"), Ordering::Equal);

}