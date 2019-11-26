use lib::api::cdragon::versions;
use lib::tools::version_cmp::compare;
use lib::api::cdragon::versions::is_latest;
use lib::api::cdragon::champion::get_champion_generic_square;
use datadragon_rs::api::cdragon::versions;
use datadragon_rs::api::cdragon::champion::get_champion_generic_square;
use datadragon_rs::api::cdragon::versions::is_latest;

#[test]
pub fn cd_version(){
    let versions = versions::get();
    let latest = versions::latest();
    assert!(is_latest(latest));

}

#[test]
pub fn test_generic_square(){
    get_champion_generic_square(&"latest");
}