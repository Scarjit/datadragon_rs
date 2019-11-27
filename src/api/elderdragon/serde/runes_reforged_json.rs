
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/elder_dragon_quicktype.py
*/














use serde::{Serialize, Deserialize};
extern crate serde_json;


pub type RunesReforgedJson = Vec<RunesReforgedJsonElement>;

#[derive(Serialize, Deserialize)]
pub struct RunesReforgedJsonElement {
    id: i64,
    key: Key,
    icon: Icon,
    name: String,
    slots: Vec<Slot>,
}

#[derive(Serialize, Deserialize)]
pub struct Slot {
    runes: Vec<Rune>,
}

#[derive(Serialize, Deserialize)]
pub struct Rune {
    id: i64,
    key: String,
    icon: String,
    name: String,
    #[serde(rename = "shortDesc")]
    short_desc: String,
    #[serde(rename = "longDesc")]
    long_desc: String,
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
