
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/elder_dragon_quicktype.py
*/
// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// 
use serde::{Serialize, Deserialize};
extern crate serde_json;

//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }


use serde::{Serialize, Deserialize};
extern crate serde_json;

use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct ChampionJson {
    #[serde(rename = "type")]
    champion_json_type: Type,
    format: Format,
    version: Version,
    data: HashMap<String, Datum>,
}

#[derive(Serialize, Deserialize)]
pub struct Datum {
    version: Version,
    id: String,
    key: String,
    name: String,
    title: String,
    blurb: String,
    info: Info,
    image: Image,
    tags: Vec<Tag>,
    partype: String,
    stats: HashMap<String, f64>,
}

#[derive(Serialize, Deserialize)]
pub struct Image {
    full: String,
    sprite: Sprite,
    group: Type,
    x: i64,
    y: i64,
    w: i64,
    h: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Info {
    attack: i64,
    defense: i64,
    magic: i64,
    difficulty: i64,
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
