
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/elder_dragon_quicktype.py
*/














use serde::{Serialize, Deserialize};
extern crate serde_json;
use self::serde_json::Error;

pub fn serialize(json: &str) -> Result<ItemJson,Error>{
    serde_json::from_str(json)
}

use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct ItemJson {
    #[serde(rename = "type")]
    pub item_json_type: GroupEnum,
    pub version: Version,
    pub basic: Basic,
    pub data: HashMap<String, Datum>,
    pub groups: Vec<Group>,
    pub tree: Vec<Tree>,
}

#[derive(Serialize, Deserialize)]
pub struct Basic {
    pub name: String,
    pub rune: Rune,
    pub gold: Gold,
    pub group: String,
    pub description: String,
    pub colloq: Colloq,
    pub plaintext: String,
    pub consumed: bool,
    pub stacks: i64,
    pub depth: i64,
    #[serde(rename = "consumeOnFull")]
    pub consume_on_full: bool,
    pub from: Vec<Option<serde_json::Value>>,
    pub into: Vec<Option<serde_json::Value>>,
    #[serde(rename = "specialRecipe")]
    pub special_recipe: i64,
    #[serde(rename = "inStore")]
    pub in_store: bool,
    #[serde(rename = "hideFromAll")]
    pub hide_from_all: bool,
    #[serde(rename = "requiredChampion")]
    pub required_champion: String,
    #[serde(rename = "requiredAlly")]
    pub required_ally: String,
    pub stats: HashMap<String, i64>,
    pub tags: Vec<Option<serde_json::Value>>,
    pub maps: HashMap<String, bool>,
}

#[derive(Serialize, Deserialize)]
pub struct Gold {
    pub base: i64,
    pub total: i64,
    pub sell: i64,
    pub purchasable: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Rune {
    pub isrune: bool,
    pub tier: i64,
    #[serde(rename = "type")]
    pub rune_type: RuneType,
}

#[derive(Serialize, Deserialize)]
pub struct Datum {
    pub name: String,
    pub description: String,
    pub colloq: String,
    pub plaintext: String,
    pub into: Option<Vec<String>>,
    pub image: Image,
    pub gold: Gold,
    pub tags: Vec<Tag>,
    pub maps: HashMap<String, bool>,
    pub stats: HashMap<String, f64>,
    #[serde(rename = "inStore")]
    pub in_store: Option<bool>,
    pub from: Option<Vec<String>>,
    pub effect: Option<Effect>,
    pub depth: Option<i64>,
    pub stacks: Option<i64>,
    pub consumed: Option<bool>,
    #[serde(rename = "hideFromAll")]
    pub hide_from_all: Option<bool>,
    #[serde(rename = "consumeOnFull")]
    pub consume_on_full: Option<bool>,
    #[serde(rename = "specialRecipe")]
    pub special_recipe: Option<i64>,
    #[serde(rename = "requiredChampion")]
    pub required_champion: Option<RequiredChampion>,
    #[serde(rename = "requiredAlly")]
    pub required_ally: Option<RequiredAlly>,
}

#[derive(Serialize, Deserialize)]
pub struct Effect {
    #[serde(rename = "Effect1Amount")]
    pub effect1_amount: String,
    #[serde(rename = "Effect2Amount")]
    pub effect2_amount: Option<String>,
    #[serde(rename = "Effect3Amount")]
    pub effect3_amount: Option<String>,
    #[serde(rename = "Effect4Amount")]
    pub effect4_amount: Option<String>,
    #[serde(rename = "Effect5Amount")]
    pub effect5_amount: Option<String>,
    #[serde(rename = "Effect6Amount")]
    pub effect6_amount: Option<String>,
    #[serde(rename = "Effect7Amount")]
    pub effect7_amount: Option<String>,
    #[serde(rename = "Effect8Amount")]
    pub effect8_amount: Option<String>,
    #[serde(rename = "Effect9Amount")]
    pub effect9_amount: Option<String>,
    #[serde(rename = "Effect10Amount")]
    pub effect10_amount: Option<String>,
    #[serde(rename = "Effect11Amount")]
    pub effect11_amount: Option<String>,
    #[serde(rename = "Effect12Amount")]
    pub effect12_amount: Option<String>,
    #[serde(rename = "Effect13Amount")]
    pub effect13_amount: Option<String>,
    #[serde(rename = "Effect14Amount")]
    pub effect14_amount: Option<String>,
    #[serde(rename = "Effect15Amount")]
    pub effect15_amount: Option<String>,
    #[serde(rename = "Effect16Amount")]
    pub effect16_amount: Option<String>,
    #[serde(rename = "Effect17Amount")]
    pub effect17_amount: Option<String>,
    #[serde(rename = "Effect18Amount")]
    pub effect18_amount: Option<String>,
    #[serde(rename = "Effect19Amount")]
    pub effect19_amount: Option<String>,
    #[serde(rename = "Effect20Amount")]
    pub effect20_amount: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Image {
    pub full: String,
    pub sprite: Sprite,
    pub group: GroupEnum,
    pub x: i64,
    pub y: i64,
    pub w: i64,
    pub h: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Group {
    pub id: String,
    #[serde(rename = "MaxGroupOwnable")]
    pub max_group_ownable: String,
}

#[derive(Serialize, Deserialize)]
pub struct Tree {
    pub header: Header,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub enum Colloq {
    #[serde(rename = ";")]
    Empty,
}

#[derive(Serialize, Deserialize)]
pub enum RuneType {
    #[serde(rename = "red")]
    Red,
}

#[derive(Serialize, Deserialize)]
pub enum GroupEnum {
    #[serde(rename = "item")]
    Item,
}

#[derive(Serialize, Deserialize)]
pub enum Sprite {
    #[serde(rename = "item0.png")]
    Item0Png,
    #[serde(rename = "item1.png")]
    Item1Png,
    #[serde(rename = "item2.png")]
    Item2Png,
}

#[derive(Serialize, Deserialize)]
pub enum RequiredAlly {
    Ornn,
}

#[derive(Serialize, Deserialize)]
pub enum RequiredChampion {
    Gangplank,
    Kalista,
    Rengar,
    Sylas,
    Viktor,
}

#[derive(Serialize, Deserialize)]
pub enum Tag {
    Active,
    Armor,
    ArmorPenetration,
    AttackSpeed,
    Aura,
    Bilgewater,
    Boots,
    Consumable,
    CooldownReduction,
    CriticalStrike,
    Damage,
    GoldPer,
    Health,
    HealthRegen,
    Jungle,
    Lane,
    LifeSteal,
    MagicPenetration,
    Mana,
    ManaRegen,
    NonbootsMovement,
    OnHit,
    Slow,
    SpellBlock,
    SpellDamage,
    SpellVamp,
    Stealth,
    Tenacity,
    Trinket,
    Vision,
}

#[derive(Serialize, Deserialize)]
pub enum Header {
    #[serde(rename = "ATTACK")]
    Attack,
    #[serde(rename = "DEFENSE")]
    Defense,
    #[serde(rename = "MAGIC")]
    Magic,
    #[serde(rename = "MOVEMENT")]
    Movement,
    #[serde(rename = "START")]
    Start,
    #[serde(rename = "TOOLS")]
    Tools,
    #[serde(rename = "UNCATEGORIZED")]
    Uncategorized,
}

#[derive(Serialize, Deserialize)]
pub enum Version {
    #[serde(rename = "9.23.1")]
    The9231,
}
