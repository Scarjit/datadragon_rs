
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/elder_dragon_quicktype.py
*/














use serde::{Serialize, Deserialize};
extern crate serde_json;
use self::serde_json::Error;

pub fn serialize(json: &str) -> Result<EzrealJson,Error>{
    serde_json::from_str(json)
}

use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct EzrealJson {
    #[serde(rename = "type")]
    pub ezreal_json_type: GroupEnum,
    pub format: Format,
    pub version: Version,
    pub data: Data,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "Ezreal")]
    pub ezreal: Ezreal,
}

#[derive(Serialize, Deserialize)]
pub struct Ezreal {
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
    pub cooldown_burn: String,
    pub cost: Vec<i64>,
    #[serde(rename = "costBurn")]
    pub cost_burn: String,
    pub datavalues: Datavalues,
    pub effect: Vec<Option<Vec<i64>>>,
    #[serde(rename = "effectBurn")]
    pub effect_burn: Vec<Option<String>>,
    pub vars: Vec<Option<serde_json::Value>>,
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
pub enum ChampionEnum {
    Ezreal,
}

#[derive(Serialize, Deserialize)]
pub enum Full {
    #[serde(rename = "EzrealE.png")]
    EzrealEPng,
    #[serde(rename = "Ezreal.png")]
    EzrealPng,
    #[serde(rename = "EzrealQ.png")]
    EzrealQPng,
    #[serde(rename = "EzrealR.png")]
    EzrealRPng,
    #[serde(rename = "Ezreal_RisingSpellForce.png")]
    EzrealRisingSpellForcePng,
    #[serde(rename = "EzrealW.png")]
    EzrealWPng,
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
    #[serde(rename = "champion0.png")]
    Champion0Png,
    #[serde(rename = "passive0.png")]
    Passive0Png,
    #[serde(rename = "spell3.png")]
    Spell3Png,
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
    #[serde(rename = "offmeta")]
    Offmeta,
    #[serde(rename = "recommended")]
    Recommended,
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
    SummonerSmite,
}

#[derive(Serialize, Deserialize)]
pub enum Map {
    CityPark,
    CrystalScar,
    #[serde(rename = "HA")]
    Ha,
    ProjectSlums,
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
    #[serde(rename = "PROJECT")]
    Project,
    #[serde(rename = "SIEGE")]
    Siege,
    #[serde(rename = "STARGUARDIAN")]
    Starguardian,
}

#[derive(Serialize, Deserialize)]
pub enum RecommendedType {
    #[serde(rename = "riot")]
    Riot,
}

#[derive(Serialize, Deserialize)]
pub enum Title {
    Beginner,
    #[serde(rename = "EzrealARAM")]
    EzrealAram,
    #[serde(rename = "EzrealCS")]
    EzrealCs,
    #[serde(rename = "EzrealFIRSTBLOOD")]
    EzrealFirstblood,
    #[serde(rename = "EzrealKINGPORO")]
    EzrealKingporo,
    #[serde(rename = "EzrealPRJ17")]
    EzrealPrj17,
    #[serde(rename = "EzrealSG")]
    EzrealSg,
    #[serde(rename = "EzrealSIEGE")]
    EzrealSiege,
    #[serde(rename = "EzrealSL")]
    EzrealSl,
    #[serde(rename = "EzrealSR")]
    EzrealSr,
    #[serde(rename = "EzrealTT")]
    EzrealTt,
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
    EzrealE,
    EzrealQ,
    EzrealR,
    EzrealW,
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
pub enum Tag {
    Mage,
    Marksman,
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
