
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/ddragon_generator.py
*/
use crate::tools::http::cached_http_byte_request;
pub mod champion;

pub fn champion() -> Result<String, ()>{
    cached_http_json_request("https://ddragon.leagueoflegends.com/cdn/d9_23_1/data/ko_KR/champion.json".as_string())    
}

pub fn champion_full() -> Result<String, ()>{
    cached_http_json_request("https://ddragon.leagueoflegends.com/cdn/d9_23_1/data/ko_KR/championFull.json".as_string())    
}

pub fn item() -> Result<String, ()>{
    cached_http_json_request("https://ddragon.leagueoflegends.com/cdn/d9_23_1/data/ko_KR/item.json".as_string())    
}

pub fn language() -> Result<String, ()>{
    cached_http_json_request("https://ddragon.leagueoflegends.com/cdn/d9_23_1/data/ko_KR/language.json".as_string())    
}

pub fn map() -> Result<String, ()>{
    cached_http_json_request("https://ddragon.leagueoflegends.com/cdn/d9_23_1/data/ko_KR/map.json".as_string())    
}

pub fn mission-assets() -> Result<String, ()>{
    cached_http_json_request("https://ddragon.leagueoflegends.com/cdn/d9_23_1/data/ko_KR/mission-assets.json".as_string())    
}

pub fn profileicon() -> Result<String, ()>{
    cached_http_json_request("https://ddragon.leagueoflegends.com/cdn/d9_23_1/data/ko_KR/profileicon.json".as_string())    
}

pub fn runes_reforged() -> Result<String, ()>{
    cached_http_json_request("https://ddragon.leagueoflegends.com/cdn/d9_23_1/data/ko_KR/runesReforged.json".as_string())    
}

pub fn sticker() -> Result<String, ()>{
    cached_http_json_request("https://ddragon.leagueoflegends.com/cdn/d9_23_1/data/ko_KR/sticker.json".as_string())    
}

pub fn summoner() -> Result<String, ()>{
    cached_http_json_request("https://ddragon.leagueoflegends.com/cdn/d9_23_1/data/ko_KR/summoner.json".as_string())    
}
