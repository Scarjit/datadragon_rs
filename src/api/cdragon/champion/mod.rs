
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/cdragon_generator.py
*/
use std::io::Read;
use std::io;
pub fn get_generic_square(patch: &str) -> Result<Vec<u8>, reqwest::Error>{
    let mut resp = reqwest::get(&format!("https://cdn.communitydragon.org/{}/champion/generic/square", patch))?;
    let bytes_x = resp.bytes();
    let bytes = bytes_x.enumerate().collect::<Vec<(usize,Result<u8, io::Error>)>>();
    let mut bvec: Vec<u8> = vec![];
    for byte in bytes {
        bvec.push(byte.1.unwrap())
    }
    Ok(bvec)
}

pub fn get_square(patch: &str, championKey: &str) -> Result<Vec<u8>, reqwest::Error>{
    let mut resp = reqwest::get(&format!("https://cdn.communitydragon.org/{}/champion/{}/square", patch, championKey))?;
    let bytes_x = resp.bytes();
    let bytes = bytes_x.enumerate().collect::<Vec<(usize,Result<u8, io::Error>)>>();
    let mut bvec: Vec<u8> = vec![];
    for byte in bytes {
        bvec.push(byte.1.unwrap())
    }
    Ok(bvec)
}

pub fn get_data(patch: &str, championKey: &str) -> Result<Vec<u8>, reqwest::Error>{
    let mut resp = reqwest::get(&format!("https://cdn.communitydragon.org/{}/champion/{}/data", patch, championKey))?;
    let bytes_x = resp.bytes();
    let bytes = bytes_x.enumerate().collect::<Vec<(usize,Result<u8, io::Error>)>>();
    let mut bvec: Vec<u8> = vec![];
    for byte in bytes {
        bvec.push(byte.1.unwrap())
    }
    Ok(bvec)
}

pub fn get_splash_art(patch: &str, championKey: &str) -> Result<Vec<u8>, reqwest::Error>{
    let mut resp = reqwest::get(&format!("https://cdn.communitydragon.org/{}/champion/{}/splash-art", patch, championKey))?;
    let bytes_x = resp.bytes();
    let bytes = bytes_x.enumerate().collect::<Vec<(usize,Result<u8, io::Error>)>>();
    let mut bvec: Vec<u8> = vec![];
    for byte in bytes {
        bvec.push(byte.1.unwrap())
    }
    Ok(bvec)
}

pub fn get_splash_art_skin(patch: &str, championKey: &str, skinId: &str) -> Result<Vec<u8>, reqwest::Error>{
    let mut resp = reqwest::get(&format!("https://cdn.communitydragon.org/{}/champion/{}/splash-art/skin/{}", patch, championKey, skinId))?;
    let bytes_x = resp.bytes();
    let bytes = bytes_x.enumerate().collect::<Vec<(usize,Result<u8, io::Error>)>>();
    let mut bvec: Vec<u8> = vec![];
    for byte in bytes {
        bvec.push(byte.1.unwrap())
    }
    Ok(bvec)
}

pub fn get_splash_art_centered(patch: &str, championKey: &str) -> Result<Vec<u8>, reqwest::Error>{
    let mut resp = reqwest::get(&format!("https://cdn.communitydragon.org/{}/champion/{}/splash-art/centered", patch, championKey))?;
    let bytes_x = resp.bytes();
    let bytes = bytes_x.enumerate().collect::<Vec<(usize,Result<u8, io::Error>)>>();
    let mut bvec: Vec<u8> = vec![];
    for byte in bytes {
        bvec.push(byte.1.unwrap())
    }
    Ok(bvec)
}

pub fn get_splash_art_centered_skin(patch: &str, championKey: &str, skinId: &str) -> Result<Vec<u8>, reqwest::Error>{
    let mut resp = reqwest::get(&format!("https://cdn.communitydragon.org/{}/champion/{}/splash-art/centered/skin/{}", patch, championKey, skinId))?;
    let bytes_x = resp.bytes();
    let bytes = bytes_x.enumerate().collect::<Vec<(usize,Result<u8, io::Error>)>>();
    let mut bvec: Vec<u8> = vec![];
    for byte in bytes {
        bvec.push(byte.1.unwrap())
    }
    Ok(bvec)
}

pub fn get_champ_select_sounds_ban(patch: &str, championKey: &str) -> Result<Vec<u8>, reqwest::Error>{
    let mut resp = reqwest::get(&format!("https://cdn.communitydragon.org/{}/champion/{}/champ-select/sounds/ban", patch, championKey))?;
    let bytes_x = resp.bytes();
    let bytes = bytes_x.enumerate().collect::<Vec<(usize,Result<u8, io::Error>)>>();
    let mut bvec: Vec<u8> = vec![];
    for byte in bytes {
        bvec.push(byte.1.unwrap())
    }
    Ok(bvec)
}

pub fn get_champ_select_sounds_choose(patch: &str, championKey: &str) -> Result<Vec<u8>, reqwest::Error>{
    let mut resp = reqwest::get(&format!("https://cdn.communitydragon.org/{}/champion/{}/champ-select/sounds/choose", patch, championKey))?;
    let bytes_x = resp.bytes();
    let bytes = bytes_x.enumerate().collect::<Vec<(usize,Result<u8, io::Error>)>>();
    let mut bvec: Vec<u8> = vec![];
    for byte in bytes {
        bvec.push(byte.1.unwrap())
    }
    Ok(bvec)
}

pub fn get_champ_select_sounds_sfx(patch: &str, championKey: &str) -> Result<Vec<u8>, reqwest::Error>{
    let mut resp = reqwest::get(&format!("https://cdn.communitydragon.org/{}/champion/{}/champ-select/sounds/sfx", patch, championKey))?;
    let bytes_x = resp.bytes();
    let bytes = bytes_x.enumerate().collect::<Vec<(usize,Result<u8, io::Error>)>>();
    let mut bvec: Vec<u8> = vec![];
    for byte in bytes {
        bvec.push(byte.1.unwrap())
    }
    Ok(bvec)
}

pub fn get_tile(patch: &str, championKey: &str) -> Result<Vec<u8>, reqwest::Error>{
    let mut resp = reqwest::get(&format!("https://cdn.communitydragon.org/{}/champion/{}/tile", patch, championKey))?;
    let bytes_x = resp.bytes();
    let bytes = bytes_x.enumerate().collect::<Vec<(usize,Result<u8, io::Error>)>>();
    let mut bvec: Vec<u8> = vec![];
    for byte in bytes {
        bvec.push(byte.1.unwrap())
    }
    Ok(bvec)
}

pub fn get_tile_skin(patch: &str, championKey: &str, skinId: &str) -> Result<Vec<u8>, reqwest::Error>{
    let mut resp = reqwest::get(&format!("https://cdn.communitydragon.org/{}/champion/{}/tile/skin/{}", patch, championKey, skinId))?;
    let bytes_x = resp.bytes();
    let bytes = bytes_x.enumerate().collect::<Vec<(usize,Result<u8, io::Error>)>>();
    let mut bvec: Vec<u8> = vec![];
    for byte in bytes {
        bvec.push(byte.1.unwrap())
    }
    Ok(bvec)
}

pub fn get_portrait(patch: &str, championKey: &str) -> Result<Vec<u8>, reqwest::Error>{
    let mut resp = reqwest::get(&format!("https://cdn.communitydragon.org/{}/champion/{}/portrait", patch, championKey))?;
    let bytes_x = resp.bytes();
    let bytes = bytes_x.enumerate().collect::<Vec<(usize,Result<u8, io::Error>)>>();
    let mut bvec: Vec<u8> = vec![];
    for byte in bytes {
        bvec.push(byte.1.unwrap())
    }
    Ok(bvec)
}

pub fn get_portrait_skin(patch: &str, championKey: &str, skinId: &str) -> Result<Vec<u8>, reqwest::Error>{
    let mut resp = reqwest::get(&format!("https://cdn.communitydragon.org/{}/champion/{}/portrait/skin/{}", patch, championKey, skinId))?;
    let bytes_x = resp.bytes();
    let bytes = bytes_x.enumerate().collect::<Vec<(usize,Result<u8, io::Error>)>>();
    let mut bvec: Vec<u8> = vec![];
    for byte in bytes {
        bvec.push(byte.1.unwrap())
    }
    Ok(bvec)
}

pub fn get_ability_icon_passive(patch: &str, championKey: &str) -> Result<Vec<u8>, reqwest::Error>{
    let mut resp = reqwest::get(&format!("https://cdn.communitydragon.org/{}/champion/{}/ability-icon/passive", patch, championKey))?;
    let bytes_x = resp.bytes();
    let bytes = bytes_x.enumerate().collect::<Vec<(usize,Result<u8, io::Error>)>>();
    let mut bvec: Vec<u8> = vec![];
    for byte in bytes {
        bvec.push(byte.1.unwrap())
    }
    Ok(bvec)
}

pub fn get_ability_icon_p(patch: &str, championKey: &str) -> Result<Vec<u8>, reqwest::Error>{
    let mut resp = reqwest::get(&format!("https://cdn.communitydragon.org/{}/champion/{}/ability-icon/p", patch, championKey))?;
    let bytes_x = resp.bytes();
    let bytes = bytes_x.enumerate().collect::<Vec<(usize,Result<u8, io::Error>)>>();
    let mut bvec: Vec<u8> = vec![];
    for byte in bytes {
        bvec.push(byte.1.unwrap())
    }
    Ok(bvec)
}

pub fn get_ability_icon_q(patch: &str, championKey: &str) -> Result<Vec<u8>, reqwest::Error>{
    let mut resp = reqwest::get(&format!("https://cdn.communitydragon.org/{}/champion/{}/ability-icon/q", patch, championKey))?;
    let bytes_x = resp.bytes();
    let bytes = bytes_x.enumerate().collect::<Vec<(usize,Result<u8, io::Error>)>>();
    let mut bvec: Vec<u8> = vec![];
    for byte in bytes {
        bvec.push(byte.1.unwrap())
    }
    Ok(bvec)
}

pub fn get_ability_icon_w(patch: &str, championKey: &str) -> Result<Vec<u8>, reqwest::Error>{
    let mut resp = reqwest::get(&format!("https://cdn.communitydragon.org/{}/champion/{}/ability-icon/w", patch, championKey))?;
    let bytes_x = resp.bytes();
    let bytes = bytes_x.enumerate().collect::<Vec<(usize,Result<u8, io::Error>)>>();
    let mut bvec: Vec<u8> = vec![];
    for byte in bytes {
        bvec.push(byte.1.unwrap())
    }
    Ok(bvec)
}

pub fn get_ability_icon_e(patch: &str, championKey: &str) -> Result<Vec<u8>, reqwest::Error>{
    let mut resp = reqwest::get(&format!("https://cdn.communitydragon.org/{}/champion/{}/ability-icon/e", patch, championKey))?;
    let bytes_x = resp.bytes();
    let bytes = bytes_x.enumerate().collect::<Vec<(usize,Result<u8, io::Error>)>>();
    let mut bvec: Vec<u8> = vec![];
    for byte in bytes {
        bvec.push(byte.1.unwrap())
    }
    Ok(bvec)
}

pub fn get_ability_icon_r(patch: &str, championKey: &str) -> Result<Vec<u8>, reqwest::Error>{
    let mut resp = reqwest::get(&format!("https://cdn.communitydragon.org/{}/champion/{}/ability-icon/r", patch, championKey))?;
    let bytes_x = resp.bytes();
    let bytes = bytes_x.enumerate().collect::<Vec<(usize,Result<u8, io::Error>)>>();
    let mut bvec: Vec<u8> = vec![];
    for byte in bytes {
        bvec.push(byte.1.unwrap())
    }
    Ok(bvec)
}

