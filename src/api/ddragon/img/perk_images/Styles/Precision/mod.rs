
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/ddragon_generator.py
*/

pub mod Conqueror;
pub mod CoupDeGrace;
pub mod CutDown;
pub mod FleetFootwork;
pub mod LegendAlacrity;
pub mod LegendBloodline;
pub mod LegendTenacity;
pub mod LethalTempo;
pub mod PresenceOfMind;
pub mod PressTheAttack;

use crate::tools::http::cached_http_byte_request;

pub fn overheal() -> Result<Vec<u8>, ()>{
    cached_http_byte_request("https://ddragon.leagueoflegends.com/cdn/img/perk_images/Styles/Precision/Overheal.png".to_string())    
}

pub fn triumph() -> Result<Vec<u8>, ()>{
    cached_http_byte_request("https://ddragon.leagueoflegends.com/cdn/img/perk_images/Styles/Precision/Triumph.png".to_string())    
}
