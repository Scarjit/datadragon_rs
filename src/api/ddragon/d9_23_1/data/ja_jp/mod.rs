
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/ddragon_generator.py
*/

pub mod champion;

use crate::tools::http::cached_http_json_request;

pub fn champion() -> Result<String, ()>{
    cached_http_json_request("https://ddragon.leagueoflegends.com/cdn/d9_23_1/data/ja_jp/champion.json".to_string())    
}

pub fn champion_full() -> Result<String, ()>{
    cached_http_json_request("https://ddragon.leagueoflegends.com/cdn/d9_23_1/data/ja_jp/championFull.json".to_string())    
}

pub fn item() -> Result<String, ()>{
    cached_http_json_request("https://ddragon.leagueoflegends.com/cdn/d9_23_1/data/ja_jp/item.json".to_string())    
}

pub fn language() -> Result<String, ()>{
    cached_http_json_request("https://ddragon.leagueoflegends.com/cdn/d9_23_1/data/ja_jp/language.json".to_string())    
}

pub fn map() -> Result<String, ()>{
    cached_http_json_request("https://ddragon.leagueoflegends.com/cdn/d9_23_1/data/ja_jp/map.json".to_string())    
}

pub fn mission_assets() -> Result<String, ()>{
    cached_http_json_request("https://ddragon.leagueoflegends.com/cdn/d9_23_1/data/ja_jp/mission-assets.json".to_string())    
}

pub fn profileicon() -> Result<String, ()>{
    cached_http_json_request("https://ddragon.leagueoflegends.com/cdn/d9_23_1/data/ja_jp/profileicon.json".to_string())    
}

pub fn runes_reforged() -> Result<String, ()>{
    cached_http_json_request("https://ddragon.leagueoflegends.com/cdn/d9_23_1/data/ja_jp/runesReforged.json".to_string())    
}

pub fn sticker() -> Result<String, ()>{
    cached_http_json_request("https://ddragon.leagueoflegends.com/cdn/d9_23_1/data/ja_jp/sticker.json".to_string())    
}

pub fn summoner() -> Result<String, ()>{
    cached_http_json_request("https://ddragon.leagueoflegends.com/cdn/d9_23_1/data/ja_jp/summoner.json".to_string())    
}