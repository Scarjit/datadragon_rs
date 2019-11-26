
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/cdragon_generator.py
*/
use std::io::Read;
use std::io;
pub fn get(patch: &str, profileIconId: &str) -> Result<Vec<u8>, reqwest::Error>{
    let mut resp = reqwest::get(&format!("https://cdn.communitydragon.org/{}/profile-icon/{}", patch, profileIconId))?;
    let bytes_x = resp.bytes();
    let bytes = bytes_x.enumerate().collect::<Vec<(usize,Result<u8, io::Error>)>>();
    let mut bvec: Vec<u8> = vec![];
    for byte in bytes {
        bvec.push(byte.1.unwrap())
    }
    Ok(bvec)
}

