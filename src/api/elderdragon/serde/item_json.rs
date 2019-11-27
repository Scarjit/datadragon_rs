
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
    item_json_type: GroupEnum,
    version: Version,
    basic: Basic,
    data: HashMap<String, Datum>,
    groups: Vec<Group>,
    tree: Vec<Tree>,
}

#[derive(Serialize, Deserialize)]
pub struct Basic {
    name: String,
    rune: Rune,
    gold: Gold,
    group: String,
    description: String,
    colloq: Colloq,
    plaintext: String,
    consumed: bool,
    stacks: i64,
    depth: i64,
    #[serde(rename = "consumeOnFull")]
    consume_on_full: bool,
    from: Vec<Option<serde_json::Value>>,
    into: Vec<Option<serde_json::Value>>,
    #[serde(rename = "specialRecipe")]
    special_recipe: i64,
    #[serde(rename = "inStore")]
    in_store: bool,
    #[serde(rename = "hideFromAll")]
    hide_from_all: bool,
    #[serde(rename = "requiredChampion")]
    required_champion: String,
    #[serde(rename = "requiredAlly")]
    required_ally: String,
    stats: HashMap<String, i64>,
    tags: Vec<Option<serde_json::Value>>,
    maps: HashMap<String, bool>,
}

#[derive(Serialize, Deserialize)]
pub struct Gold {
    base: i64,
    total: i64,
    sell: i64,
    purchasable: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Rune {
    isrune: bool,
    tier: i64,
    #[serde(rename = "type")]
    rune_type: RuneType,
}

#[derive(Serialize, Deserialize)]
pub struct Datum {
    name: String,
    description: String,
    colloq: String,
    plaintext: String,
    into: Option<Vec<String>>,
    image: Image,
    gold: Gold,
    tags: Vec<Tag>,
    maps: HashMap<String, bool>,
    stats: HashMap<String, f64>,
    #[serde(rename = "inStore")]
    in_store: Option<bool>,
    from: Option<Vec<String>>,
    effect: Option<Effect>,
    depth: Option<i64>,
    stacks: Option<i64>,
    consumed: Option<bool>,
    #[serde(rename = "hideFromAll")]
    hide_from_all: Option<bool>,
    #[serde(rename = "consumeOnFull")]
    consume_on_full: Option<bool>,
    #[serde(rename = "specialRecipe")]
    special_recipe: Option<i64>,
    #[serde(rename = "requiredChampion")]
    required_champion: Option<RequiredChampion>,
    #[serde(rename = "requiredAlly")]
    required_ally: Option<RequiredAlly>,
}

#[derive(Serialize, Deserialize)]
pub struct Effect {
    #[serde(rename = "Effect1Amount")]
    effect1_amount: String,
    #[serde(rename = "Effect2Amount")]
    effect2_amount: Option<String>,
    #[serde(rename = "Effect3Amount")]
    effect3_amount: Option<String>,
    #[serde(rename = "Effect4Amount")]
    effect4_amount: Option<String>,
    #[serde(rename = "Effect5Amount")]
    effect5_amount: Option<String>,
    #[serde(rename = "Effect6Amount")]
    effect6_amount: Option<String>,
    #[serde(rename = "Effect7Amount")]
    effect7_amount: Option<String>,
    #[serde(rename = "Effect8Amount")]
    effect8_amount: Option<String>,
    #[serde(rename = "Effect9Amount")]
    effect9_amount: Option<String>,
    #[serde(rename = "Effect10Amount")]
    effect10_amount: Option<String>,
    #[serde(rename = "Effect11Amount")]
    effect11_amount: Option<String>,
    #[serde(rename = "Effect12Amount")]
    effect12_amount: Option<String>,
    #[serde(rename = "Effect13Amount")]
    effect13_amount: Option<String>,
    #[serde(rename = "Effect14Amount")]
    effect14_amount: Option<String>,
    #[serde(rename = "Effect15Amount")]
    effect15_amount: Option<String>,
    #[serde(rename = "Effect16Amount")]
    effect16_amount: Option<String>,
    #[serde(rename = "Effect17Amount")]
    effect17_amount: Option<String>,
    #[serde(rename = "Effect18Amount")]
    effect18_amount: Option<String>,
    #[serde(rename = "Effect19Amount")]
    effect19_amount: Option<String>,
    #[serde(rename = "Effect20Amount")]
    effect20_amount: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Image {
    full: String,
    sprite: Sprite,
    group: GroupEnum,
    x: i64,
    y: i64,
    w: i64,
    h: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Group {
    id: String,
    #[serde(rename = "MaxGroupOwnable")]
    max_group_ownable: String,
}

#[derive(Serialize, Deserialize)]
pub struct Tree {
    header: Header,
    tags: Vec<String>,
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
