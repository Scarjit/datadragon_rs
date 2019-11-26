
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/ddragon_generator.py
*/


use crate::tools::http::cached_http_byte_request;

pub fn biscuit_delivery() -> Result<Vec<u8>, ()>{
    cached_http_byte_request("https://ddragon.leagueoflegends.com/cdn/../src/api/ddragon_raw/img/perk_images/styles/inspiration/biscuit_delivery/BiscuitDelivery.png".to_string())    
}
