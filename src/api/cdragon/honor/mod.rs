
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/cdragon_generator.py
*/
use std::io::Read;
use std::io;
pub fn get_generic(patch: &str) -> Result<Vec<u8>, reqwest::Error>{
    let mut resp = reqwest::get(&format!("https://cdn.communitydragon.org/{}/honor/generic", patch))?;
    let bytes_x = resp.bytes();
    let bytes = bytes_x.enumerate().collect::<Vec<(usize,Result<u8, io::Error>)>>();
    let mut bvec: Vec<u8> = vec![];
    for byte in bytes {
        bvec.push(byte.1.unwrap())
    }
    Ok(bvec)
}

pub fn get(patch: &str, honorId: &str) -> Result<Vec<u8>, reqwest::Error>{
    let mut resp = reqwest::get(&format!("https://cdn.communitydragon.org/{}/honor/{}", patch, honorId))?;
    let bytes_x = resp.bytes();
    let bytes = bytes_x.enumerate().collect::<Vec<(usize,Result<u8, io::Error>)>>();
    let mut bvec: Vec<u8> = vec![];
    for byte in bytes {
        bvec.push(byte.1.unwrap())
    }
    Ok(bvec)
}

pub fn get_locked(patch: &str, honorId: &str) -> Result<Vec<u8>, reqwest::Error>{
    let mut resp = reqwest::get(&format!("https://cdn.communitydragon.org/{}/honor/{}/locked", patch, honorId))?;
    let bytes_x = resp.bytes();
    let bytes = bytes_x.enumerate().collect::<Vec<(usize,Result<u8, io::Error>)>>();
    let mut bvec: Vec<u8> = vec![];
    for byte in bytes {
        bvec.push(byte.1.unwrap())
    }
    Ok(bvec)
}

pub fn get_level(patch: &str, honorId: &str, level: &str) -> Result<Vec<u8>, reqwest::Error>{
    let mut resp = reqwest::get(&format!("https://cdn.communitydragon.org/{}/honor/{}/level/{}", patch, honorId, level))?;
    let bytes_x = resp.bytes();
    let bytes = bytes_x.enumerate().collect::<Vec<(usize,Result<u8, io::Error>)>>();
    let mut bvec: Vec<u8> = vec![];
    for byte in bytes {
        bvec.push(byte.1.unwrap())
    }
    Ok(bvec)
}

pub fn get_emblem_generic(patch: &str) -> Result<Vec<u8>, reqwest::Error>{
    let mut resp = reqwest::get(&format!("https://cdn.communitydragon.org/{}/honor/emblem/generic", patch))?;
    let bytes_x = resp.bytes();
    let bytes = bytes_x.enumerate().collect::<Vec<(usize,Result<u8, io::Error>)>>();
    let mut bvec: Vec<u8> = vec![];
    for byte in bytes {
        bvec.push(byte.1.unwrap())
    }
    Ok(bvec)
}

pub fn get_emblem(patch: &str, honorId: &str) -> Result<Vec<u8>, reqwest::Error>{
    let mut resp = reqwest::get(&format!("https://cdn.communitydragon.org/{}/honor/emblem/{}", patch, honorId))?;
    let bytes_x = resp.bytes();
    let bytes = bytes_x.enumerate().collect::<Vec<(usize,Result<u8, io::Error>)>>();
    let mut bvec: Vec<u8> = vec![];
    for byte in bytes {
        bvec.push(byte.1.unwrap())
    }
    Ok(bvec)
}

pub fn get_emblem_locked(patch: &str, honorId: &str) -> Result<Vec<u8>, reqwest::Error>{
    let mut resp = reqwest::get(&format!("https://cdn.communitydragon.org/{}/honor/emblem/{}/locked", patch, honorId))?;
    let bytes_x = resp.bytes();
    let bytes = bytes_x.enumerate().collect::<Vec<(usize,Result<u8, io::Error>)>>();
    let mut bvec: Vec<u8> = vec![];
    for byte in bytes {
        bvec.push(byte.1.unwrap())
    }
    Ok(bvec)
}

pub fn get_emblem_level(patch: &str, honorId: &str, level: &str) -> Result<Vec<u8>, reqwest::Error>{
    let mut resp = reqwest::get(&format!("https://cdn.communitydragon.org/{}/honor/emblem/{}/level/{}", patch, honorId, level))?;
    let bytes_x = resp.bytes();
    let bytes = bytes_x.enumerate().collect::<Vec<(usize,Result<u8, io::Error>)>>();
    let mut bvec: Vec<u8> = vec![];
    for byte in bytes {
        bvec.push(byte.1.unwrap())
    }
    Ok(bvec)
}

