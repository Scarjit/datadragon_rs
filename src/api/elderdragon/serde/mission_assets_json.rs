// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

extern crate serde_json;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct MissionAssetsJson {
    #[serde(rename = "type")]
    mission_assets_json_type: Type,
    version: Version,
    data: HashMap<String, Datum>,
}

#[derive(Serialize, Deserialize)]
pub struct Datum {
    id: i64,
    image: Image,
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
pub enum Type {
    #[serde(rename = "mission")]
    Mission,
}

#[derive(Serialize, Deserialize)]
pub enum Sprite {
    #[serde(rename = "mission0.png")]
    Mission0Png,
}

#[derive(Serialize, Deserialize)]
pub enum Version {
    #[serde(rename = "9.23.1")]
    The9231,
}
