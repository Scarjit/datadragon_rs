
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/cdragon_generator.py
*/
use std::io::Read;
use std::io;
pub fn get_generic(patch: &str) -> Result<Vec<u8>, reqwest::Error>{
    let resp = reqwest::get(&format!("https://cdn.communitydragon.org/{}/honor/generic", patch))?;
    let bytes_x = resp.bytes();
    let bytes = bytes_x.enumerate().collect::<Vec<(usize,Result<u8, io::Error>)>>();
    let mut bvec: Vec<u8> = vec![];
    for byte in bytes {
        bvec.push(byte.1.expect("Couldn't convert Bytes to Vec<u8>"))
    }
    Ok(bvec)
}

pub fn get(patch: &str, honor_id: &str) -> Result<Vec<u8>, reqwest::Error>{
    let resp = reqwest::get(&format!("https://cdn.communitydragon.org/{}/honor/{}", patch, honor_id))?;
    let bytes_x = resp.bytes();
    let bytes = bytes_x.enumerate().collect::<Vec<(usize,Result<u8, io::Error>)>>();
    let mut bvec: Vec<u8> = vec![];
    for byte in bytes {
        bvec.push(byte.1.expect("Couldn't convert Bytes to Vec<u8>"))
    }
    Ok(bvec)
}

pub fn get_locked(patch: &str, honor_id: &str) -> Result<Vec<u8>, reqwest::Error>{
    let resp = reqwest::get(&format!("https://cdn.communitydragon.org/{}/honor/{}/locked", patch, honor_id))?;
    let bytes_x = resp.bytes();
    let bytes = bytes_x.enumerate().collect::<Vec<(usize,Result<u8, io::Error>)>>();
    let mut bvec: Vec<u8> = vec![];
    for byte in bytes {
        bvec.push(byte.1.expect("Couldn't convert Bytes to Vec<u8>"))
    }
    Ok(bvec)
}

pub fn get_level(patch: &str, honor_id: &str, level: &str) -> Result<Vec<u8>, reqwest::Error>{
    let resp = reqwest::get(&format!("https://cdn.communitydragon.org/{}/honor/{}/level/{}", patch, honor_id, level))?;
    let bytes_x = resp.bytes();
    let bytes = bytes_x.enumerate().collect::<Vec<(usize,Result<u8, io::Error>)>>();
    let mut bvec: Vec<u8> = vec![];
    for byte in bytes {
        bvec.push(byte.1.expect("Couldn't convert Bytes to Vec<u8>"))
    }
    Ok(bvec)
}

pub fn get_emblem_generic(patch: &str) -> Result<Vec<u8>, reqwest::Error>{
    let resp = reqwest::get(&format!("https://cdn.communitydragon.org/{}/honor/emblem/generic", patch))?;
    let bytes_x = resp.bytes();
    let bytes = bytes_x.enumerate().collect::<Vec<(usize,Result<u8, io::Error>)>>();
    let mut bvec: Vec<u8> = vec![];
    for byte in bytes {
        bvec.push(byte.1.expect("Couldn't convert Bytes to Vec<u8>"))
    }
    Ok(bvec)
}

pub fn get_emblem(patch: &str, honor_id: &str) -> Result<Vec<u8>, reqwest::Error>{
    let resp = reqwest::get(&format!("https://cdn.communitydragon.org/{}/honor/emblem/{}", patch, honor_id))?;
    let bytes_x = resp.bytes();
    let bytes = bytes_x.enumerate().collect::<Vec<(usize,Result<u8, io::Error>)>>();
    let mut bvec: Vec<u8> = vec![];
    for byte in bytes {
        bvec.push(byte.1.expect("Couldn't convert Bytes to Vec<u8>"))
    }
    Ok(bvec)
}

pub fn get_emblem_locked(patch: &str, honor_id: &str) -> Result<Vec<u8>, reqwest::Error>{
    let resp = reqwest::get(&format!("https://cdn.communitydragon.org/{}/honor/emblem/{}/locked", patch, honor_id))?;
    let bytes_x = resp.bytes();
    let bytes = bytes_x.enumerate().collect::<Vec<(usize,Result<u8, io::Error>)>>();
    let mut bvec: Vec<u8> = vec![];
    for byte in bytes {
        bvec.push(byte.1.expect("Couldn't convert Bytes to Vec<u8>"))
    }
    Ok(bvec)
}

pub fn get_emblem_level(patch: &str, honor_id: &str, level: &str) -> Result<Vec<u8>, reqwest::Error>{
    let resp = reqwest::get(&format!("https://cdn.communitydragon.org/{}/honor/emblem/{}/level/{}", patch, honor_id, level))?;
    let bytes_x = resp.bytes();
    let bytes = bytes_x.enumerate().collect::<Vec<(usize,Result<u8, io::Error>)>>();
    let mut bvec: Vec<u8> = vec![];
    for byte in bytes {
        bvec.push(byte.1.expect("Couldn't convert Bytes to Vec<u8>"))
    }
    Ok(bvec)
}

