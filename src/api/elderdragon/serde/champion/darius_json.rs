
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/elder_dragon_quicktype.py
*/














use serde::{Serialize, Deserialize};
extern crate serde_json;
use self::serde_json::Error;

pub fn serialize(json: &str) -> Result<DariusJson,Error>{
    serde_json::from_str(json)
}

use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct DariusJson {
    #[serde(rename = "type")]
    pub darius_json_type: GroupEnum,
    pub format: Format,
    pub version: Version,
    pub data: Data,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "Darius")]
    pub darius: Darius,
}

#[derive(Serialize, Deserialize)]
pub struct Darius {
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
    pub cooldown: Vec<f64>,
    #[serde(rename = "cooldownBurn")]
    pub cooldown_burn: CooldownBurn,
    pub cost: Vec<i64>,
    #[serde(rename = "costBurn")]
    pub cost_burn: String,
    pub datavalues: Datavalues,
    pub effect: Vec<Option<Vec<f64>>>,
    #[serde(rename = "effectBurn")]
    pub effect_burn: Vec<Option<String>>,
    pub vars: Vec<Var>,
    #[serde(rename = "costType")]
    pub cost_type: CostType,
    pub maxammo: String,
    pub range: Vec<i64>,
    #[serde(rename = "rangeBurn")]
    pub range_burn: String,
    pub image: Image,
    pub resource: Resource,
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
pub enum GroupEnum {
    #[serde(rename = "champion")]
    Champion,
    #[serde(rename = "passive")]
    Passive,
    #[serde(rename = "spell")]
    Spell,
}

#[derive(Serialize, Deserialize)]
pub enum ChampionEnum {
    Darius,
    MissFortune,
}

#[derive(Serialize, Deserialize)]
pub enum Full {
    #[serde(rename = "DariusAxeGrabCone.png")]
    DariusAxeGrabConePng,
    #[serde(rename = "DariusCleave.png")]
    DariusCleavePng,
    #[serde(rename = "DariusExecute.png")]
    DariusExecutePng,
    #[serde(rename = "Darius_Icon_Hemorrhage.png")]
    DariusIconHemorrhagePng,
    #[serde(rename = "DariusNoxianTacticsONH.png")]
    DariusNoxianTacticsOnhPng,
    #[serde(rename = "Darius.png")]
    DariusPng,
}

#[derive(Serialize, Deserialize)]
pub enum Sprite {
    #[serde(rename = "champion0.png")]
    Champion0Png,
    #[serde(rename = "passive0.png")]
    Passive0Png,
    #[serde(rename = "spell2.png")]
    Spell2Png,
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
    #[serde(rename = "defensivejungle")]
    Defensivejungle,
    #[serde(rename = "early")]
    Early,
    #[serde(rename = "earlyjungle")]
    Earlyjungle,
    #[serde(rename = "essential")]
    Essential,
    #[serde(rename = "essentialjungle")]
    Essentialjungle,
    KingPoroSnax,
    #[serde(rename = "npe1")]
    Npe1,
    #[serde(rename = "npe2")]
    Npe2,
    #[serde(rename = "npe3")]
    Npe3,
    #[serde(rename = "npe4")]
    Npe4,
    #[serde(rename = "offensive")]
    Offensive,
    #[serde(rename = "siegeDefense")]
    SiegeDefense,
    #[serde(rename = "siegeOffense")]
    SiegeOffense,
    #[serde(rename = "situational")]
    Situational,
    #[serde(rename = "situationaljungle")]
    Situationaljungle,
    #[serde(rename = "starting")]
    Starting,
    #[serde(rename = "startingjungle")]
    Startingjungle,
}

#[derive(Serialize, Deserialize)]
pub enum IfSummonerSpell {
    #[serde(rename = "")]
    Empty,
    SummonerPoroThrow,
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
    #[serde(rename = "TUTORIAL_MODULE_2")]
    TutorialModule2,
    #[serde(rename = "TUTORIAL_MODULE_3")]
    TutorialModule3,
}

#[derive(Serialize, Deserialize)]
pub enum RecommendedType {
    #[serde(rename = "riot")]
    Riot,
}

#[derive(Serialize, Deserialize)]
pub enum Title {
    Beginner,
    #[serde(rename = "DariusARAM")]
    DariusAram,
    #[serde(rename = "DariusCS")]
    DariusCs,
    #[serde(rename = "DariusFIRSTBLOOD")]
    DariusFirstblood,
    #[serde(rename = "DariusKINGPORO")]
    DariusKingporo,
    #[serde(rename = "DariusNPE")]
    DariusNpe,
    #[serde(rename = "DariusSIEGE")]
    DariusSiege,
    #[serde(rename = "DariusSL")]
    DariusSl,
    #[serde(rename = "DariusSR")]
    DariusSr,
    #[serde(rename = "DariusTT")]
    DariusTt,
}

#[derive(Serialize, Deserialize)]
pub enum CooldownBurn {
    #[serde(rename = "120/100/80")]
    The12010080,
    #[serde(rename = "24/21/18/15/12")]
    The2421181512,
    #[serde(rename = "7/6.5/6/5.5/5")]
    The7656555,
    #[serde(rename = "9/8/7/6/5")]
    The98765,
}

#[derive(Serialize, Deserialize)]
pub enum CostType {
    #[serde(rename = " {{ abilityresourcename }}")]
    Abilityresourcename,
    #[serde(rename = ": {{ cost }}")]
    Cost,
    #[serde(rename = "{{ abilityresourcename }}")]
    CostTypeAbilityresourcename,
    #[serde(rename = " {{ cost }}")]
    CostTypeCost,
    #[serde(rename = " de {{ abilityresourcename }}")]
    DeAbilityresourcename,
    #[serde(rename = " pkt. ({{ abilityresourcename }})")]
    PktAbilityresourcename,
}

#[derive(Serialize, Deserialize)]
pub enum SpellId {
    DariusAxeGrabCone,
    DariusCleave,
    DariusExecute,
    #[serde(rename = "DariusNoxianTacticsONH")]
    DariusNoxianTacticsOnh,
}

#[derive(Serialize, Deserialize)]
pub enum Resource {
    #[serde(rename = "{{ abilityresourcename }}: {{ cost }}")]
    AbilityresourcenameCost,
    #[serde(rename = "{{ cost }} {{ abilityresourcename }}")]
    CostAbilityresourcename,
    #[serde(rename = "{{ cost }} de {{ abilityresourcename }}")]
    CostDeAbilityresourcename,
    #[serde(rename = "{{ cost }} pkt. ({{ abilityresourcename }})")]
    CostPktAbilityresourcename,
    #[serde(rename = "{{ abilityresourcename }} {{ cost }}")]
    ResourceAbilityresourcenameCost,
    #[serde(rename = "{{ cost }}{{ abilityresourcename }}")]
    ResourceCostAbilityresourcename,
}

#[derive(Serialize, Deserialize)]
pub enum Key {
    #[serde(rename = "f1")]
    F1,
}

#[derive(Serialize, Deserialize)]
pub enum Link {
    #[serde(rename = "bonusattackdamage")]
    Bonusattackdamage,
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
