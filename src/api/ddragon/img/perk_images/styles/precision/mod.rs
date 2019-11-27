
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/ddragon_generator.py
*/

pub mod conqueror;
pub mod coup_de_grace;
pub mod cut_down;
pub mod fleet_footwork;
pub mod legend_alacrity;
pub mod legend_bloodline;
pub mod legend_tenacity;
pub mod lethal_tempo;
pub mod presence_of_mind;
pub mod press_the_attack;

use crate::tools::http::cached_http_byte_request;

pub fn overheal() -> Result<Vec<u8>, ()>{
    cached_http_byte_request("https://ddragon.leagueoflegends.com/cdn/../src/api/ddragon_raw/img/perk_images/styles/precision/Overheal.png".to_string())    
}

pub fn triumph() -> Result<Vec<u8>, ()>{
    cached_http_byte_request("https://ddragon.leagueoflegends.com/cdn/../src/api/ddragon_raw/img/perk_images/styles/precision/Triumph.png".to_string())    
}
