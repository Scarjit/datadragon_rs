
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/ddragon_generator.py
*/


use crate::tools::http::cached_http_byte_request;

pub fn hail_of_blades() -> Result<Vec<u8>, ()>{
    cached_http_byte_request("https://ddragon.leagueoflegends.com/cdn/../src/api/ddragon_raw/img/perk_images/styles/domination/hail_of_blades/HailOfBlades.png".to_string())    
}
