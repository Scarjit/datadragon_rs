
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/elder_dragon_quicktype.py
*/














use serde::{Serialize, Deserialize};
extern crate serde_json;
use self::serde_json::Error;

pub fn serialize(json: &str) -> Result<ProfileiconJson,Error>{
    serde_json::from_str(json)
}

use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct ProfileiconJson {
    #[serde(rename = "type")]
    pub profileicon_json_type: Type,
    pub version: Version,
    pub data: HashMap<String, Datum>,
}

#[derive(Serialize, Deserialize)]
pub struct Datum {
    pub id: i64,
    pub image: Image,
}

#[derive(Serialize, Deserialize)]
pub struct Image {
    pub full: String,
    pub sprite: Sprite,
    pub group: Type,
    pub x: i64,
    pub y: i64,
    pub w: i64,
    pub h: i64,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "profileicon")]
    Profileicon,
}

#[derive(Serialize, Deserialize)]
pub enum Sprite {
    #[serde(rename = "profileicon0.png")]
    Profileicon0Png,
}

#[derive(Serialize, Deserialize)]
pub enum Version {
    #[serde(rename = "9.23.1")]
    The9231,
}
