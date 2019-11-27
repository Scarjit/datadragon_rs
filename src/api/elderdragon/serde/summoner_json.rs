
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
    summoner_json_type: Type,
    version: Version,
    data: HashMap<String, Datum>,
}

#[derive(Serialize, Deserialize)]
pub struct Datum {
    id: Id,
    name: String,
    description: String,
    tooltip: String,
    maxrank: i64,
    cooldown: Vec<f64>,
    #[serde(rename = "cooldownBurn")]
    cooldown_burn: String,
    cost: Vec<i64>,
    #[serde(rename = "costBurn")]
    cost_burn: String,
    datavalues: Datavalues,
    effect: Vec<Option<Vec<f64>>>,
    #[serde(rename = "effectBurn")]
    effect_burn: Vec<Option<String>>,
    vars: Vec<Var>,
    key: String,
    #[serde(rename = "summonerLevel")]
    summoner_level: i64,
    modes: Vec<Mode>,
    #[serde(rename = "costType")]
    cost_type: String,
    maxammo: String,
    range: Vec<i64>,
    #[serde(rename = "rangeBurn")]
    range_burn: String,
    image: Image,
    resource: String,
}

#[derive(Serialize, Deserialize)]
pub struct Datavalues {
}

#[derive(Serialize, Deserialize)]
pub struct Image {
    full: Full,
    sprite: Sprite,
    group: Group,
    x: i64,
    y: i64,
    w: i64,
    h: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Var {
    link: Link,
    coeff: Coeff,
    key: Key,
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
pub enum Full {
    #[serde(rename = "SummonerBarrier.png")]
    SummonerBarrierPng,
    #[serde(rename = "SummonerBoost.png")]
    SummonerBoostPng,
    #[serde(rename = "SummonerDot.png")]
    SummonerDotPng,
    #[serde(rename = "SummonerExhaust.png")]
    SummonerExhaustPng,
    #[serde(rename = "SummonerFlash.png")]
    SummonerFlashPng,
    #[serde(rename = "SummonerHaste.png")]
    SummonerHastePng,
    #[serde(rename = "SummonerHeal.png")]
    SummonerHealPng,
    #[serde(rename = "SummonerMana.png")]
    SummonerManaPng,
    #[serde(rename = "SummonerOdysseyFlash.png")]
    SummonerOdysseyFlashPng,
    #[serde(rename = "SummonerOdysseyRevive.png")]
    SummonerOdysseyRevivePng,
    #[serde(rename = "SummonerPoroRecall.png")]
    SummonerPoroRecallPng,
    #[serde(rename = "SummonerPoroThrow.png")]
    SummonerPoroThrowPng,
    #[serde(rename = "SummonerSmite.png")]
    SummonerSmitePng,
    #[serde(rename = "SummonerSnowURFSnowball_Mark.png")]
    SummonerSnowUrfSnowballMarkPng,
    #[serde(rename = "SummonerSnowball.png")]
    SummonerSnowballPng,
    #[serde(rename = "SummonerTeleport.png")]
    SummonerTeleportPng,
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
