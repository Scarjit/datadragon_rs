
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/ddragon_generator.py
*/


use crate::tools::http::cached_http_byte_request;

pub fn summon_aery() -> Result<Vec<u8>, ()>{
    cached_http_byte_request("https://ddragon.leagueoflegends.com/cdn/../src/api/ddragon_raw/img/perk_images/styles/sorcery/summon_aery/SummonAery.png".to_string())    
}
