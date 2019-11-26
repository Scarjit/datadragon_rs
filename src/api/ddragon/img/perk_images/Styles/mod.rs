
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

pub fn f7200__domination() -> Result<Vec<u8>, ()>{
    cached_http_byte_request("https://ddragon.leagueoflegends.com/cdn/img/perk_images/Styles/7200_Domination.png".to_string())    
}

pub fn f7201__precision() -> Result<Vec<u8>, ()>{
    cached_http_byte_request("https://ddragon.leagueoflegends.com/cdn/img/perk_images/Styles/7201_Precision.png".to_string())    
}

pub fn f7202__sorcery() -> Result<Vec<u8>, ()>{
    cached_http_byte_request("https://ddragon.leagueoflegends.com/cdn/img/perk_images/Styles/7202_Sorcery.png".to_string())    
}

pub fn f7203__whimsy() -> Result<Vec<u8>, ()>{
    cached_http_byte_request("https://ddragon.leagueoflegends.com/cdn/img/perk_images/Styles/7203_Whimsy.png".to_string())    
}

pub fn f7204__resolve() -> Result<Vec<u8>, ()>{
    cached_http_byte_request("https://ddragon.leagueoflegends.com/cdn/img/perk_images/Styles/7204_Resolve.png".to_string())    
}

pub fn runes_icon() -> Result<Vec<u8>, ()>{
    cached_http_byte_request("https://ddragon.leagueoflegends.com/cdn/img/perk_images/Styles/RunesIcon.png".to_string())    
}
