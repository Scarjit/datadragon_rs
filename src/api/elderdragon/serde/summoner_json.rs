
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/elder_dragon_quicktype.py
*/














use serde::{Serialize, Deserialize};
extern crate serde_json;
use self::serde_json::Error;

pub fn serialize(json: &str) -> Result<SummonerJson,Error>{
    serde_json::from_str(json)
}

use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct SummonerJson {
    #[serde(rename = "type")]
    pub summoner_json_type: Type,
    pub version: Version,
    pub data: HashMap<String, Datum>,
}

#[derive(Serialize, Deserialize)]
pub struct Datum {
    pub id: Id,
    pub name: String,
    pub description: String,
    pub tooltip: String,
    pub maxrank: i64,
    pub cooldown: Vec<f64>,
    #[serde(rename = "cooldownBurn")]
    pub cooldown_burn: String,
    pub cost: Vec<i64>,
    #[serde(rename = "costBurn")]
    pub cost_burn: String,
    pub datavalues: Datavalues,
    pub effect: Vec<Option<Vec<f64>>>,
    #[serde(rename = "effectBurn")]
    pub effect_burn: Vec<Option<String>>,
    pub vars: Vec<Var>,
    pub key: String,
    #[serde(rename = "summonerLevel")]
    pub summoner_level: i64,
    pub modes: Vec<Mode>,
    #[serde(rename = "costType")]
    pub cost_type: String,
    pub maxammo: String,
    pub range: Vec<i64>,
    #[serde(rename = "rangeBurn")]
    pub range_burn: String,
    pub image: Image,
    pub resource: String,
}

#[derive(Serialize, Deserialize)]
pub struct Datavalues {
}

#[derive(Serialize, Deserialize)]
pub struct Image {
    pub full: String,
    pub sprite: Sprite,
    pub group: Group,
    pub x: i64,
    pub y: i64,
    pub w: i64,
    pub h: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Var {
    pub link: Link,
    pub coeff: Coeff,
    pub key: Key,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Coeff {
    Integer(i64),
    IntegerArray(Vec<i64>),
}

#[derive(Serialize, Deserialize)]
pub enum Id {
    SummonerBarrier,
    SummonerBoost,
    SummonerDot,
    SummonerExhaust,
    SummonerFlash,
    SummonerHaste,
    SummonerHeal,
    SummonerMana,
    SummonerOdysseyFlash,
    SummonerOdysseyRevive,
    SummonerPoroRecall,
    SummonerPoroThrow,
    SummonerSmite,
    #[serde(rename = "SummonerSnowURFSnowball_Mark")]
    SummonerSnowUrfSnowballMark,
    SummonerSnowball,
    SummonerTeleport,
}


#[derive(Serialize, Deserialize)]
pub enum Group {
    #[serde(rename = "spell")]
    Spell,
}

#[derive(Serialize, Deserialize)]
pub enum Sprite {
    #[serde(rename = "spell0.png")]
    Spell0Png,
}

#[derive(Serialize, Deserialize)]
pub enum Mode {
    #[serde(rename = "ARAM")]
    Aram,
    #[serde(rename = "ARSR")]
    Arsr,
    #[serde(rename = "ASCENSION")]
    Ascension,
    #[serde(rename = "ASSASSINATE")]
    Assassinate,
    #[serde(rename = "CLASSIC")]
    Classic,
    #[serde(rename = "DOOMBOTSTEEMO")]
    Doombotsteemo,
    #[serde(rename = "FIRSTBLOOD")]
    Firstblood,
    #[serde(rename = "GAMEMODEX")]
    Gamemodex,
    #[serde(rename = "KINGPORO")]
    Kingporo,
    #[serde(rename = "ODIN")]
    Odin,
    #[serde(rename = "ODYSSEY")]
    Odyssey,
    #[serde(rename = "ONEFORALL")]
    Oneforall,
    #[serde(rename = "PRACTICETOOL")]
    Practicetool,
    #[serde(rename = "PROJECT")]
    Project,
    #[serde(rename = "SNOWURF")]
    Snowurf,
    #[serde(rename = "STARGUARDIAN")]
    Starguardian,
    #[serde(rename = "TUTORIAL")]
    Tutorial,
    #[serde(rename = "TUTORIAL_MODULE_1")]
    TutorialModule1,
    #[serde(rename = "TUTORIAL_MODULE_2")]
    TutorialModule2,
    #[serde(rename = "URF")]
    Urf,
}

#[derive(Serialize, Deserialize)]
pub enum Key {
    #[serde(rename = "f1")]
    F1,
    #[serde(rename = "f2")]
    F2,
}

#[derive(Serialize, Deserialize)]
pub enum Link {
    #[serde(rename = "@player.level")]
    PlayerLevel,
    #[serde(rename = "@text")]
    Text,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "summoner")]
    Summoner,
}

#[derive(Serialize, Deserialize)]
pub enum Version {
    #[serde(rename = "9.23.1")]
    The9231,
}
