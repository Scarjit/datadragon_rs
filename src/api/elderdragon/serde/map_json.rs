
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/elder_dragon_quicktype.py
*/














use serde::{Serialize, Deserialize};
extern crate serde_json;
use self::serde_json::Error;

pub fn serialize(json: &str) -> Result<MapJson,Error>{
    serde_json::from_str(json)
}

use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct MapJson {
    #[serde(rename = "type")]
    pub map_json_type: Type,
    pub version: Version,
    pub data: HashMap<String, Datum>,
}

#[derive(Serialize, Deserialize)]
pub struct Datum {
    #[serde(rename = "MapName")]
    pub map_name: String,
    #[serde(rename = "MapId")]
    pub map_id: String,
    pub image: Image,
}

#[derive(Serialize, Deserialize)]
pub struct Image {
    pub full: Full,
    pub sprite: Sprite,
    pub group: Type,
    pub x: i64,
    pub y: i64,
    pub w: i64,
    pub h: i64,
}

#[derive(Serialize, Deserialize)]
pub enum Full {
    #[serde(rename = "map10.png")]
    Map10Png,
    #[serde(rename = "map11.png")]
    Map11Png,
    #[serde(rename = "map12.png")]
    Map12Png,
    #[serde(rename = "map22.png")]
    Map22Png,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "map")]
    Map,
}

#[derive(Serialize, Deserialize)]
pub enum Sprite {
    #[serde(rename = "map0.png")]
    Map0Png,
}

#[derive(Serialize, Deserialize)]
pub enum Version {
    #[serde(rename = "9.23.1")]
    The9231,
}
