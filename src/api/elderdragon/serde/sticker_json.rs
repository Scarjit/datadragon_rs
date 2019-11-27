
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/elder_dragon_quicktype.py
*/














use serde::{Serialize, Deserialize};
extern crate serde_json;
use self::serde_json::Error;

pub fn serialize(json: &str) -> Result<StickerJson,Error>{
    serde_json::from_str(json)
}


#[derive(Serialize, Deserialize)]
pub struct StickerJson {
    #[serde(rename = "type")]
    sticker_json_type: Type,
    version: Version,
    data: Data,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "sticker")]
    Sticker,
}

#[derive(Serialize, Deserialize)]
pub enum Version {
    #[serde(rename = "9.23.1")]
    The9231,
}
