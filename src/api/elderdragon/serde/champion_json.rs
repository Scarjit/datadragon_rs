
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/elder_dragon_quicktype.py
*/














use serde::{Serialize, Deserialize};
extern crate serde_json;
use self::serde_json::Error;

pub fn serialize(json: &str) -> Result<ChampionJson,Error>{
    serde_json::from_str(json)
}

use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct ChampionJson {
    #[serde(rename = "type")]
    pub champion_json_type: Type,
    pub format: Format,
    pub version: Version,
    pub data: HashMap<String, Datum>,
}

#[derive(Serialize, Deserialize)]
pub struct Datum {
    pub version: Version,
    pub id: String,
    pub key: String,
    pub name: String,
    pub title: String,
    pub blurb: String,
    pub info: Info,
    pub image: Image,
    pub tags: Vec<Tag>,
    pub partype: String,
    pub stats: HashMap<String, f64>,
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
pub struct Info {
    pub attack: i64,
    pub defense: i64,
    pub magic: i64,
    pub difficulty: i64,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "champion")]
    Champion,
}

#[derive(Serialize, Deserialize)]
pub enum Sprite {
    #[serde(rename = "champion0.png")]
    Champion0Png,
    #[serde(rename = "champion1.png")]
    Champion1Png,
    #[serde(rename = "champion2.png")]
    Champion2Png,
    #[serde(rename = "champion3.png")]
    Champion3Png,
    #[serde(rename = "champion4.png")]
    Champion4Png,
}

#[derive(Serialize, Deserialize)]
pub enum Tag {
    Assassin,
    Fighter,
    Mage,
    Marksman,
    Support,
    Tank,
}

#[derive(Serialize, Deserialize)]
pub enum Version {
    #[serde(rename = "9.23.1")]
    The9231,
}

#[derive(Serialize, Deserialize)]
pub enum Format {
    #[serde(rename = "standAloneComplex")]
    StandAloneComplex,
}
