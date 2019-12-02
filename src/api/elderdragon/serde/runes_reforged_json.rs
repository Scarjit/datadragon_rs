
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/elder_dragon_quicktype.py
*/














use serde::{Serialize, Deserialize};
extern crate serde_json;
use self::serde_json::Error;

pub fn serialize(json: &str) -> Result<RunesReforgedJsonElement,Error>{
    serde_json::from_str(json)
}


pub type RunesReforgedJson = Vec<RunesReforgedJsonElement>;

#[derive(Serialize, Deserialize)]
pub struct RunesReforgedJsonElement {
    pub id: i64,
    pub key: Key,
    pub icon: Icon,
    pub name: String,
    pub slots: Vec<Slot>,
}

#[derive(Serialize, Deserialize)]
pub struct Slot {
    pub runes: Vec<Rune>,
}

#[derive(Serialize, Deserialize)]
pub struct Rune {
    pub id: i64,
    pub key: String,
    pub icon: String,
    pub name: String,
    #[serde(rename = "shortDesc")]
    pub short_desc: String,
    #[serde(rename = "longDesc")]
    pub long_desc: String,
}

#[derive(Serialize, Deserialize)]
pub enum Icon {
    #[serde(rename = "perk-images/Styles/7200_Domination.png")]
    PerkImagesStyles7200_DominationPng,
    #[serde(rename = "perk-images/Styles/7201_Precision.png")]
    PerkImagesStyles7201_PrecisionPng,
    #[serde(rename = "perk-images/Styles/7202_Sorcery.png")]
    PerkImagesStyles7202_SorceryPng,
    #[serde(rename = "perk-images/Styles/7203_Whimsy.png")]
    PerkImagesStyles7203_WhimsyPng,
    #[serde(rename = "perk-images/Styles/7204_Resolve.png")]
    PerkImagesStyles7204_ResolvePng,
}

#[derive(Serialize, Deserialize)]
pub enum Key {
    Domination,
    Inspiration,
    Precision,
    Resolve,
    Sorcery,
}
