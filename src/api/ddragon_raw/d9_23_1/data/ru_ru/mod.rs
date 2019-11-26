
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/ddragon_generator.py
*/

pub mod champion;

use crate::tools::http::cached_http_json_request;

pub fn champion() -> Result<String, ()>{
    cached_http_json_request("https://ddragon.leagueoflegends.com/cdn/9.23.1/data/ru_RU/champion.json".to_string())    
}

pub fn champion_full() -> Result<String, ()>{
    cached_http_json_request("https://ddragon.leagueoflegends.com/cdn/9.23.1/data/ru_RU/championFull.json".to_string())    
}

pub fn item() -> Result<String, ()>{
    cached_http_json_request("https://ddragon.leagueoflegends.com/cdn/9.23.1/data/ru_RU/item.json".to_string())    
}

pub fn language() -> Result<String, ()>{
    cached_http_json_request("https://ddragon.leagueoflegends.com/cdn/9.23.1/data/ru_RU/language.json".to_string())    
}

pub fn map() -> Result<String, ()>{
    cached_http_json_request("https://ddragon.leagueoflegends.com/cdn/9.23.1/data/ru_RU/map.json".to_string())    
}

pub fn mission_assets() -> Result<String, ()>{
    cached_http_json_request("https://ddragon.leagueoflegends.com/cdn/9.23.1/data/ru_RU/mission-assets.json".to_string())    
}

pub fn profileicon() -> Result<String, ()>{
    cached_http_json_request("https://ddragon.leagueoflegends.com/cdn/9.23.1/data/ru_RU/profileicon.json".to_string())    
}

pub fn runes_reforged() -> Result<String, ()>{
    cached_http_json_request("https://ddragon.leagueoflegends.com/cdn/9.23.1/data/ru_RU/runesReforged.json".to_string())    
}

pub fn sticker() -> Result<String, ()>{
    cached_http_json_request("https://ddragon.leagueoflegends.com/cdn/9.23.1/data/ru_RU/sticker.json".to_string())    
}

pub fn summoner() -> Result<String, ()>{
    cached_http_json_request("https://ddragon.leagueoflegends.com/cdn/9.23.1/data/ru_RU/summoner.json".to_string())    
}
