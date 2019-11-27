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
