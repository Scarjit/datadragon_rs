
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/ddragon_generator.py
*/
use crate::tools::http::cached_http_byte_request;

pub fn guardian() -> Result<Vec<u8>, ()>{
    cached_http_byte_request("https://ddragon.leagueoflegends.com/cdn/img/perk_images/Styles/Resolve/Guardian/Guardian.png".as_string())    
}
