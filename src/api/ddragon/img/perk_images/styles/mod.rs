
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/ddragon_generator.py
*/

pub mod domination;
pub mod inspiration;
pub mod precision;
pub mod resolve;
pub mod sorcery;

use crate::tools::http::cached_http_byte_request;

pub fn f7200_domination() -> Result<Vec<u8>, ()>{
    cached_http_byte_request("https://ddragon.leagueoflegends.com/cdn/img/perk_images/styles/7200_Domination.png".to_string())    
}

pub fn f7201_precision() -> Result<Vec<u8>, ()>{
    cached_http_byte_request("https://ddragon.leagueoflegends.com/cdn/img/perk_images/styles/7201_Precision.png".to_string())    
}

pub fn f7202_sorcery() -> Result<Vec<u8>, ()>{
    cached_http_byte_request("https://ddragon.leagueoflegends.com/cdn/img/perk_images/styles/7202_Sorcery.png".to_string())    
}

pub fn f7203_whimsy() -> Result<Vec<u8>, ()>{
    cached_http_byte_request("https://ddragon.leagueoflegends.com/cdn/img/perk_images/styles/7203_Whimsy.png".to_string())    
}

pub fn f7204_resolve() -> Result<Vec<u8>, ()>{
    cached_http_byte_request("https://ddragon.leagueoflegends.com/cdn/img/perk_images/styles/7204_Resolve.png".to_string())    
}

pub fn runes_icon() -> Result<Vec<u8>, ()>{
    cached_http_byte_request("https://ddragon.leagueoflegends.com/cdn/img/perk_images/styles/RunesIcon.png".to_string())    
}
