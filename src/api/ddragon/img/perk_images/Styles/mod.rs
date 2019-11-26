
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/ddragon_generator.py
*/

pub mod Domination;
pub mod Inspiration;
pub mod Precision;
pub mod Resolve;
pub mod Sorcery;

use crate::tools::http::cached_http_byte_request;

pub fn 7200__domination() -> Result<Vec<u8>, ()>{
    cached_http_byte_request("https://ddragon.leagueoflegends.com/cdn/img/perk_images/Styles/7200_Domination.png".as_string())    
}

pub fn 7201__precision() -> Result<Vec<u8>, ()>{
    cached_http_byte_request("https://ddragon.leagueoflegends.com/cdn/img/perk_images/Styles/7201_Precision.png".as_string())    
}

pub fn 7202__sorcery() -> Result<Vec<u8>, ()>{
    cached_http_byte_request("https://ddragon.leagueoflegends.com/cdn/img/perk_images/Styles/7202_Sorcery.png".as_string())    
}

pub fn 7203__whimsy() -> Result<Vec<u8>, ()>{
    cached_http_byte_request("https://ddragon.leagueoflegends.com/cdn/img/perk_images/Styles/7203_Whimsy.png".as_string())    
}

pub fn 7204__resolve() -> Result<Vec<u8>, ()>{
    cached_http_byte_request("https://ddragon.leagueoflegends.com/cdn/img/perk_images/Styles/7204_Resolve.png".as_string())    
}

pub fn runes_icon() -> Result<Vec<u8>, ()>{
    cached_http_byte_request("https://ddragon.leagueoflegends.com/cdn/img/perk_images/Styles/RunesIcon.png".as_string())    
}
