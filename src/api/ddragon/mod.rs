
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/ddragon_generator.py
*/
use crate::tools::http::cached_http_byte_request;
pub mod d9_23_1;
pub mod img;

pub fn languages() -> Result<String, ()>{
    cached_http_json_request("https://ddragon.leagueoflegends.com/cdn//languages.json".as_string())    
}
