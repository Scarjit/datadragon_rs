
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/ddragon_generator.py
*/


use crate::tools::http::cached_http_byte_request;

pub fn adaptive_force_icon() -> Result<Vec<u8>, ()>{
    cached_http_byte_request("https://ddragon.leagueoflegends.com/cdn/img/perk_images/stat_mods/StatModsAdaptiveForceIcon.png".to_string())    
}

pub fn armor_icon() -> Result<Vec<u8>, ()>{
    cached_http_byte_request("https://ddragon.leagueoflegends.com/cdn/img/perk_images/stat_mods/StatModsArmorIcon.png".to_string())    
}

pub fn attack_speed_icon() -> Result<Vec<u8>, ()>{
    cached_http_byte_request("https://ddragon.leagueoflegends.com/cdn/img/perk_images/stat_mods/StatModsAttackSpeedIcon.png".to_string())    
}

pub fn cdr_scaling_icon() -> Result<Vec<u8>, ()>{
    cached_http_byte_request("https://ddragon.leagueoflegends.com/cdn/img/perk_images/stat_mods/StatModsCDRScalingIcon.png".to_string())    
}

pub fn health_scaling_icon() -> Result<Vec<u8>, ()>{
    cached_http_byte_request("https://ddragon.leagueoflegends.com/cdn/img/perk_images/stat_mods/StatModsHealthScalingIcon.png".to_string())    
}

pub fn magic_res_icon() -> Result<Vec<u8>, ()>{
    cached_http_byte_request("https://ddragon.leagueoflegends.com/cdn/img/perk_images/stat_mods/StatModsMagicResIcon.png".to_string())    
}
