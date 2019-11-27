
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
