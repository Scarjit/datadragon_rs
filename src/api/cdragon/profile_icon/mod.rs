
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/cdragon_generator.py
*/
use std::io::Read;
use std::io;
pub fn get(patch: &str, profile_icon_id: &str) -> Result<Vec<u8>, reqwest::Error>{
    let resp = reqwest::get(&format!("https://cdn.communitydragon.org/{}/profile-icon/{}", patch, profile_icon_id))?;
    let bytes_x = resp.bytes();
    let bytes = bytes_x.enumerate().collect::<Vec<(usize,Result<u8, io::Error>)>>();
    let mut bvec: Vec<u8> = vec![];
    for byte in bytes {
        bvec.push(byte.1.expect("Couldn't convert Bytes to Vec<u8>"))
    }
    Ok(bvec)
}

