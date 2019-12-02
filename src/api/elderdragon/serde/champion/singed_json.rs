
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/elder_dragon_quicktype.py
*/














use serde::{Serialize, Deserialize};
extern crate serde_json;
use self::serde_json::Error;

pub fn serialize(json: &str) -> Result<SingedJson,Error>{
    serde_json::from_str(json)
}

use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct SingedJson {
    #[serde(rename = "type")]
    pub singed_json_type: GroupEnum,
    pub format: Format,
    pub version: Version,
    pub data: Data,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "Singed")]
    pub singed: Singed,
}

#[derive(Serialize, Deserialize)]
pub struct Singed {
    pub id: ChampionEnum,
    pub key: String,
    pub name: String,
    pub title: String,
    pub image: Image,
    pub skins: Vec<Skin>,
    pub lore: String,
    pub blurb: String,
    pub allytips: Vec<String>,
    pub enemytips: Vec<String>,
    pub tags: Vec<Tag>,
    pub partype: String,
    pub info: Info,
    pub stats: HashMap<String, f64>,
    pub spells: Vec<Spell>,
    pub passive: Passive,
    pub recommended: Vec<Recommended>,
}

#[derive(Serialize, Deserialize)]
pub struct Image {
    pub full: Full,
    pub sprite: Sprite,
    pub group: GroupEnum,
    pub x: i64,
    pub y: i64,
    pub w: i64,
    pub h: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Info {
    pub attack: i64,
    pub defense: i64,
    pub magic: i64,
    pub difficulty: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Passive {
    pub name: String,
    pub description: String,
    pub image: Image,
}

#[derive(Serialize, Deserialize)]
pub struct Recommended {
    pub champion: ChampionEnum,
    pub title: Title,
    pub map: Map,
    pub mode: Mode,
    #[serde(rename = "type")]
    pub recommended_type: RecommendedType,
    #[serde(rename = "customTag")]
    pub custom_tag: String,
    #[serde(rename = "extensionPage")]
    pub extension_page: bool,
    #[serde(rename = "customPanel")]
    pub custom_panel: Option<serde_json::Value>,
    pub blocks: Vec<Block>,
    pub sortrank: Option<i64>,
    #[serde(rename = "useObviousCheckmark")]
    pub use_obvious_checkmark: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct Block {
    #[serde(rename = "type")]
    pub block_type: BlockType,
    #[serde(rename = "recMath")]
    pub rec_math: bool,
    #[serde(rename = "recSteps")]
    pub rec_steps: bool,
    #[serde(rename = "minSummonerLevel")]
    pub min_summoner_level: i64,
    #[serde(rename = "maxSummonerLevel")]
    pub max_summoner_level: i64,
    #[serde(rename = "showIfSummonerSpell")]
    pub show_if_summoner_spell: IfSummonerSpell,
    #[serde(rename = "hideIfSummonerSpell")]
    pub hide_if_summoner_spell: IfSummonerSpell,
    pub items: Vec<Item>,
    #[serde(rename = "appendAfterSection")]
    pub append_after_section: Option<String>,
    #[serde(rename = "visibleWithAllOf")]
    pub visible_with_all_of: Option<Vec<String>>,
    #[serde(rename = "hiddenWithAnyOf")]
    pub hidden_with_any_of: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub count: i64,
    #[serde(rename = "hideCount")]
    pub hide_count: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Skin {
    pub id: String,
    pub num: i64,
    pub name: String,
    pub chromas: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Spell {
    pub id: SpellId,
    pub name: String,
    pub description: String,
    pub tooltip: String,
    pub leveltip: Leveltip,
    pub maxrank: i64,
    pub cooldown: Vec<i64>,
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
pub struct Leveltip {
    pub label: Vec<String>,
    pub effect: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Var {
    pub link: Link,
    pub coeff: f64,
    pub key: Key,
}

#[derive(Serialize, Deserialize)]
pub enum ChampionEnum {
    Singed,
}

#[derive(Serialize, Deserialize)]
pub enum Full {
    #[serde(rename = "Fling.png")]
    FlingPng,
    #[serde(rename = "InsanityPotion.png")]
    InsanityPotionPng,
    #[serde(rename = "MegaAdhesive.png")]
    MegaAdhesivePng,
    #[serde(rename = "PoisonTrail.png")]
    PoisonTrailPng,
    #[serde(rename = "Singed_Passive.png")]
    SingedPassivePng,
    #[serde(rename = "Singed.png")]
    SingedPng,
}

#[derive(Serialize, Deserialize)]
pub enum GroupEnum {
    #[serde(rename = "champion")]
    Champion,
    #[serde(rename = "passive")]
    Passive,
    #[serde(rename = "spell")]
    Spell,
}

#[derive(Serialize, Deserialize)]
pub enum Sprite {
    #[serde(rename = "champion3.png")]
    Champion3Png,
    #[serde(rename = "passive3.png")]
    Passive3Png,
    #[serde(rename = "spell10.png")]
    Spell10Png,
}

#[derive(Serialize, Deserialize)]
pub enum BlockType {
    #[serde(rename = "beginner_advanced")]
    BeginnerAdvanced,
    #[serde(rename = "beginner_legendaryitem")]
    BeginnerLegendaryitem,
    #[serde(rename = "beginner_morelegendaryitems")]
    BeginnerMorelegendaryitems,
    #[serde(rename = "beginner_movementspeed")]
    BeginnerMovementspeed,
    #[serde(rename = "beginner_starter")]
    BeginnerStarter,
    #[serde(rename = "consumables")]
    Consumables,
    #[serde(rename = "defensive")]
    Defensive,
    #[serde(rename = "early")]
    Early,
    #[serde(rename = "earlyjungle")]
    Earlyjungle,
    #[serde(rename = "essential")]
    Essential,
    #[serde(rename = "essentialjungle")]
    Essentialjungle,
    KingPoroSnax,
    #[serde(rename = "offensive")]
    Offensive,
    #[serde(rename = "siegeDefense")]
    SiegeDefense,
    #[serde(rename = "siegeOffense")]
    SiegeOffense,
    #[serde(rename = "situational")]
    Situational,
    #[serde(rename = "starting")]
    Starting,
    #[serde(rename = "startingjungle")]
    Startingjungle,
}

#[derive(Serialize, Deserialize)]
pub enum IfSummonerSpell {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "S5_SummonerSmitePlayerGanker")]
    S5SummonerSmitePlayerGanker,
    SummonerSmite,
}

#[derive(Serialize, Deserialize)]
pub enum Map {
    CrystalScar,
    #[serde(rename = "HA")]
    Ha,
    #[serde(rename = "SL")]
    Sl,
    #[serde(rename = "SR")]
    Sr,
    #[serde(rename = "TT")]
    Tt,
}

#[derive(Serialize, Deserialize)]
pub enum Mode {
    #[serde(rename = "ARAM")]
    Aram,
    #[serde(rename = "CLASSIC")]
    Classic,
    #[serde(rename = "FIRSTBLOOD")]
    Firstblood,
    #[serde(rename = "GAMEMODEX")]
    Gamemodex,
    #[serde(rename = "INTRO")]
    Intro,
    #[serde(rename = "KINGPORO")]
    Kingporo,
    #[serde(rename = "ODIN")]
    Odin,
    #[serde(rename = "SIEGE")]
    Siege,
}

#[derive(Serialize, Deserialize)]
pub enum RecommendedType {
    #[serde(rename = "riot")]
    Riot,
}

#[derive(Serialize, Deserialize)]
pub enum Title {
    Beginner,
    #[serde(rename = "SingedARAM")]
    SingedAram,
    #[serde(rename = "SingedCS")]
    SingedCs,
    #[serde(rename = "SingedFIRSTBLOOD")]
    SingedFirstblood,
    #[serde(rename = "SingedKINGPORO")]
    SingedKingporo,
    #[serde(rename = "SingedSIEGE")]
    SingedSiege,
    #[serde(rename = "SingedSL")]
    SingedSl,
    #[serde(rename = "SingedSR")]
    SingedSr,
    #[serde(rename = "SingedTT")]
    SingedTt,
}

#[derive(Serialize, Deserialize)]
pub enum SpellId {
    Fling,
    InsanityPotion,
    MegaAdhesive,
    PoisonTrail,
}

#[derive(Serialize, Deserialize)]
pub enum Key {
    #[serde(rename = "a1")]
    A1,
}

#[derive(Serialize, Deserialize)]
pub enum Link {
    #[serde(rename = "spelldamage")]
    Spelldamage,
}

#[derive(Serialize, Deserialize)]
pub enum Tag {
    Fighter,
    Tank,
}

#[derive(Serialize, Deserialize)]
pub enum Format {
    #[serde(rename = "standAloneComplex")]
    StandAloneComplex,
}

#[derive(Serialize, Deserialize)]
pub enum Version {
    #[serde(rename = "9.23.1")]
    The9231,
}
