
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/ddragon_generator.py
*/


use crate::tools::http::cached_http_byte_request;

pub fn last_stand() -> Result<Vec<u8>, ()>{
    cached_http_byte_request("https://ddragon.leagueoflegends.com/cdn/../src/api/ddragon_raw/img/perk_images/styles/sorcery/last_stand/LastStand.png".to_string())    
}
