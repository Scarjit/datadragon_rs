
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/ddragon_generator.py
*/


use crate::tools::http::cached_http_byte_request;

pub fn get() -> Result<Vec<u8>, ()>{
    cached_http_byte_request("https://ddragon.leagueoflegends.com/cdn/img/perk_images/styles/resolve/demolish/Demolish.png".to_string())    
}
