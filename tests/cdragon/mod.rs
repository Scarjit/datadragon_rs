use datadragon_rs::api::cdragon::versions::is_latest;
use datadragon_rs::api::cdragon::champion::get_generic_square;
use datadragon_rs::api::cdragon::versions;

#[test]
pub fn cd_version(){
    let latest = versions::latest();
    assert!(is_latest(latest));
}

#[test]
pub fn test_generic_square(){
    get_generic_square("latest").expect("Couldn't get generic square from cdragon");
}