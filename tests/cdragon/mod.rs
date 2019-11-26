use datadragon_rs::api::cdragon::versions::is_latest;
use datadragon_rs::api::cdragon::champion::get_generic_square;
use datadragon_rs::api::cdragon::versions;
use std::time::Instant;

#[test]
pub fn cd_version(){
    let latest = versions::latest();
    assert!(is_latest(latest));
}

#[test]
pub fn test_generic_square(){
    get_generic_square("latest").expect("Couldn't get generic square from cdragon");
}

#[test]
pub fn test_cache(){
    let now_1 = Instant::now();
    get_generic_square("latest").expect("Couldn't get generic square from cdragon");
    let elapsed_1 = now_1.elapsed().as_nanos();

    get_generic_square("latest").expect("Couldn't get generic square from cdragon");
    let elapsed_2 = now_1.elapsed().as_nanos();

    get_generic_square("latest").expect("Couldn't get generic square from cdragon");
    let elapsed_3 = now_1.elapsed().as_nanos();

    get_generic_square("latest").expect("Couldn't get generic square from cdragon");
    let elapsed_4 = now_1.elapsed().as_nanos();

    assert!(elapsed_2-elapsed_1 < 10000);
    assert!(elapsed_3-elapsed_2 < 10000);
    assert!(elapsed_4-elapsed_3 < 10000);
}