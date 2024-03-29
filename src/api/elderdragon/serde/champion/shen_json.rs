
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/elder_dragon_quicktype.py
*/














use serde::{Serialize, Deserialize};
extern crate serde_json;
use self::serde_json::Error;

pub fn serialize(json: &str) -> Result<ShenJson,Error>{
    serde_json::from_str(json)
}

use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct ShenJson {
    #[serde(rename = "type")]
    shen_json_type: GroupEnum,
    format: Format,
    version: Version,
    data: Data,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "Shen")]
    shen: Shen,
}

#[derive(Serialize, Deserialize)]
pub struct Shen {
    id: ChampionEnum,
    key: String,
    name: String,
    title: String,
    image: Image,
    skins: Vec<Skin>,
    lore: String,
    blurb: String,
    allytips: Vec<String>,
    enemytips: Vec<String>,
    tags: Vec<Tag>,
    partype: String,
    info: Info,
    stats: HashMap<String, f64>,
    spells: Vec<Spell>,
    passive: Passive,
    recommended: Vec<Recommended>,
}

#[derive(Serialize, Deserialize)]
pub struct Image {
    full: Full,
    sprite: Sprite,
    group: GroupEnum,
    x: i64,
    y: i64,
    w: i64,
    h: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Info {
    attack: i64,
    defense: i64,
    magic: i64,
    difficulty: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Passive {
    name: String,
    description: String,
    image: Image,
}

#[derive(Serialize, Deserialize)]
pub struct Recommended {
    champion: ChampionEnum,
    title: Title,
    #[serde(rename = "type")]
    recommended_type: RecommendedType,
    map: Map,
    mode: Mode,
    priority: Option<bool>,
    blocks: Vec<Block>,
    #[serde(rename = "customTag")]
    custom_tag: Option<String>,
    sortrank: Option<i64>,
    #[serde(rename = "extensionPage")]
    extension_page: Option<bool>,
    #[serde(rename = "useObviousCheckmark")]
    use_obvious_checkmark: Option<bool>,
    #[serde(rename = "customPanel")]
    custom_panel: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub struct Block {
    #[serde(rename = "type")]
    block_type: BlockType,
    #[serde(rename = "maxSummonerLevel")]
    max_summoner_level: Option<i64>,
    items: Vec<Item>,
    #[serde(rename = "minSummonerLevel")]
    min_summoner_level: Option<i64>,
    #[serde(rename = "recMath")]
    rec_math: Option<bool>,
    #[serde(rename = "recSteps")]
    rec_steps: Option<bool>,
    #[serde(rename = "showIfSummonerSpell")]
    show_if_summoner_spell: Option<IfSummonerSpell>,
    #[serde(rename = "hideIfSummonerSpell")]
    hide_if_summoner_spell: Option<IfSummonerSpell>,
    #[serde(rename = "appendAfterSection")]
    append_after_section: Option<String>,
    #[serde(rename = "visibleWithAllOf")]
    visible_with_all_of: Option<Vec<String>>,
    #[serde(rename = "hiddenWithAnyOf")]
    hidden_with_any_of: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    id: String,
    count: i64,
    #[serde(rename = "hideCount")]
    hide_count: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct Skin {
    id: String,
    num: i64,
    name: String,
    chromas: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Spell {
    id: SpellId,
    name: String,
    description: String,
    tooltip: String,
    leveltip: Leveltip,
    maxrank: i64,
    cooldown: Vec<f64>,
    #[serde(rename = "cooldownBurn")]
    cooldown_burn: CooldownBurn,
    cost: Vec<i64>,
    #[serde(rename = "costBurn")]
    cost_burn: String,
    datavalues: Datavalues,
    effect: Vec<Option<Vec<f64>>>,
    #[serde(rename = "effectBurn")]
    effect_burn: Vec<Option<String>>,
    vars: Vec<Option<serde_json::Value>>,
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
pub struct Leveltip {
    label: Vec<String>,
    effect: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub enum ChampionEnum {
    Shen,
}

#[derive(Serialize, Deserialize)]
pub enum Full {
    #[serde(rename = "ShenE.png")]
    ShenEPng,
    #[serde(rename = "Shen_Passive.png")]
    ShenPassivePng,
    #[serde(rename = "Shen.png")]
    ShenPng,
    #[serde(rename = "ShenQ.png")]
    ShenQPng,
    #[serde(rename = "ShenR.png")]
    ShenRPng,
    #[serde(rename = "ShenW.png")]
    ShenWPng,
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
    #[serde(rename = "beginner_Advanced")]
    BeginnerAdvanced,
    #[serde(rename = "beginner_LegendaryItem")]
    BeginnerLegendaryItem,
    #[serde(rename = "beginner_MoreLegendaryItems")]
    BeginnerMoreLegendaryItems,
    #[serde(rename = "beginner_MovementSpeed")]
    BeginnerMovementSpeed,
    #[serde(rename = "beginner_Starter")]
    BeginnerStarter,
    #[serde(rename = "consumables")]
    Consumables,
    #[serde(rename = "defensive")]
    Defensive,
    #[serde(rename = "early")]
    Early,
    #[serde(rename = "earlyjungle")]
    Earlyjungle,
    #[serde(rename = "")]
    Empty,
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
    #[serde(rename = "ShenARAM")]
    ShenAram,
    #[serde(rename = "ShenCS")]
    ShenCs,
    #[serde(rename = "ShenFIRSTBLOOD")]
    ShenFirstblood,
    #[serde(rename = "ShenKINGPORO")]
    ShenKingporo,
    #[serde(rename = "ShenSIEGE")]
    ShenSiege,
    #[serde(rename = "ShenSL")]
    ShenSl,
    #[serde(rename = "ShenSR")]
    ShenSr,
    #[serde(rename = "ShenTT")]
    ShenTt,
}

#[derive(Serialize, Deserialize)]
pub enum CooldownBurn {
    #[serde(rename = "18/16/14/12/10")]
    The1816141210,
    #[serde(rename = "18/16.5/15/13.5/11")]
    The181651513511,
    #[serde(rename = "200/180/160")]
    The200180160,
    #[serde(rename = "8/7.25/6.5/5.75/5")]
    The8725655755,
}

#[derive(Serialize, Deserialize)]
pub enum SpellId {
    ShenE,
    ShenQ,
    ShenR,
    ShenW,
}

#[derive(Serialize, Deserialize)]
pub enum Tag {
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
