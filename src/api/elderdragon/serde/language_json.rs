
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/elder_dragon_quicktype.py
*/














use serde::{Serialize, Deserialize};
extern crate serde_json;
use self::serde_json::Error;

pub fn serialize(json: &str) -> Result<LanguageJson,Error>{
    serde_json::from_str(json)
}

use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct LanguageJson {
    #[serde(rename = "type")]
    language_json_type: Type,
    version: Version,
    data: HashMap<String, String>,
    tree: Tree,
}

#[derive(Serialize, Deserialize)]
pub struct Tree {
    #[serde(rename = "searchKeyIgnore")]
    search_key_ignore: SearchKeyIgnore,
    #[serde(rename = "searchKeyRemap")]
    search_key_remap: Vec<SearchKeyRemap>,
}

#[derive(Serialize, Deserialize)]
pub struct SearchKeyRemap {
    k: String,
    v: String,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "language")]
    Language,
}

#[derive(Serialize, Deserialize)]
pub enum SearchKeyIgnore {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "ㅇㅁㄴㄴㅂㄷㄱㅃㄸㅉㄲㅍㅌㅊㅋㅅㅆㄹㅎ")]
    SearchKeyIgnore,
}

#[derive(Serialize, Deserialize)]
pub enum Version {
    #[serde(rename = "9.23.1")]
    The9231,
}
