
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/elder_dragon_quicktype.py
*/














use serde::{Serialize, Deserialize};
extern crate serde_json;
use self::serde_json::Error;

pub fn serialize(json: &str) -> Result<BrandJson,Error>{
    serde_json::from_str(json)
}

use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct BrandJson {
    #[serde(rename = "type")]
    pub brand_json_type: GroupEnum,
    pub format: Format,
    pub version: Version,
    pub data: Data,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "Brand")]
    pub brand: Brand,
}

#[derive(Serialize, Deserialize)]
pub struct Brand {
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
    #[serde(rename = "type")]
    pub recommended_type: RecommendedType,
    pub map: Map,
    pub mode: Mode,
    pub priority: Option<bool>,
    pub blocks: Vec<Block>,
    #[serde(rename = "customTag")]
    pub custom_tag: Option<String>,
    pub sortrank: Option<i64>,
    #[serde(rename = "extensionPage")]
    pub extension_page: Option<bool>,
    #[serde(rename = "customPanel")]
    pub custom_panel: Option<serde_json::Value>,
    #[serde(rename = "useObviousCheckmark")]
    pub use_obvious_checkmark: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct Block {
    #[serde(rename = "type")]
    pub block_type: BlockType,
    #[serde(rename = "maxSummonerLevel")]
    pub max_summoner_level: Option<i64>,
    pub items: Vec<Item>,
    #[serde(rename = "minSummonerLevel")]
    pub min_summoner_level: Option<i64>,
    #[serde(rename = "recMath")]
    pub rec_math: Option<bool>,
    #[serde(rename = "recSteps")]
    pub rec_steps: Option<bool>,
    #[serde(rename = "showIfSummonerSpell")]
    pub show_if_summoner_spell: Option<IfSummonerSpell>,
    #[serde(rename = "hideIfSummonerSpell")]
    pub hide_if_summoner_spell: Option<IfSummonerSpell>,
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
    pub hide_count: Option<bool>,
    pub hidecount: Option<bool>,
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
    pub effect: Vec<Effect>,
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
    Brand,
}

#[derive(Serialize, Deserialize)]
pub enum Full {
    #[serde(rename = "BrandE.png")]
    BrandEPng,
    #[serde(rename = "BrandP.png")]
    BrandPPng,
    #[serde(rename = "Brand.png")]
    BrandPng,
    #[serde(rename = "BrandQ.png")]
    BrandQPng,
    #[serde(rename = "BrandR.png")]
    BrandRPng,
    #[serde(rename = "BrandW.png")]
    BrandWPng,
}

#[derive(Serialize, Deserialize)]
pub enum Sprite {
    #[serde(rename = "champion0.png")]
    Champion0Png,
    #[serde(rename = "passive0.png")]
    Passive0Png,
    #[serde(rename = "spell1.png")]
    Spell1Png,
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
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "starting")]
    Starting,
    #[serde(rename = "startingjungle")]
    Startingjungle,
}

#[derive(Serialize, Deserialize)]
pub enum IfSummonerSpell {
    #[serde(rename = "")]
    Empty,
    ItemSmiteAoE,
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
    #[serde(rename = "riot-mid")]
    RiotMid,
    #[serde(rename = "riot-support")]
    RiotSupport,
}

#[derive(Serialize, Deserialize)]
pub enum Title {
    Beginner,
    #[serde(rename = "BrandARAM")]
    BrandAram,
    #[serde(rename = "BrandCS")]
    BrandCs,
    #[serde(rename = "BrandFIRSTBLOOD")]
    BrandFirstblood,
    #[serde(rename = "BrandKINGPORO")]
    BrandKingporo,
    #[serde(rename = "BrandSIEGE")]
    BrandSiege,
    #[serde(rename = "BrandSL")]
    BrandSl,
    #[serde(rename = "BrandSR")]
    BrandSr,
    #[serde(rename = "BrandSRSupport")]
    BrandSrSupport,
    #[serde(rename = "BrandTT")]
    BrandTt,
}

#[derive(Serialize, Deserialize)]
pub enum CooldownBurn {
    #[serde(rename = "105/90/75")]
    The1059075,
    #[serde(rename = "10/9.5/9/8.5/8")]
    The10959858,
    #[serde(rename = "10/9/8/7/6")]
    The109876,
    #[serde(rename = "8/7.5/7/6.5/6")]
    The8757656,
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
    BrandE,
    BrandQ,
    BrandR,
    BrandW,
}

#[derive(Serialize, Deserialize)]
pub enum Effect {
    #[serde(rename = "{{ basedamage }} -> {{ basedamageNL }}")]
    BasedamageBasedamageNl,
    #[serde(rename = "{{ cooldown }} -> {{ cooldownNL }}")]
    CooldownCooldownNl,
    #[serde(rename = "{{ cost }} -> {{ costNL }}")]
    CostCostNl,
    #[serde(rename = "{{ e1 }} -> {{ e1NL }}")]
    E1E1Nl,
    #[serde(rename = "{{ basedamage }}->{{ basedamageNL }}")]
    EffectBasedamageBasedamageNl,
    #[serde(rename = "{{ cooldown }}->{{ cooldownNL }}")]
    EffectCooldownCooldownNl,
    #[serde(rename = "{{ cost }}->{{ costNL }}")]
    EffectCostCostNl,
    #[serde(rename = "{{ e1 }}->{{ e1NL }}")]
    EffectE1E1Nl,
    #[serde(rename = "{{ slowamount }}% -> {{ slowamountNL }}%")]
    EffectSlowamountSlowamountNl,
    #[serde(rename = "{{ slowamount }}%->{{ slowamountNL }}%")]
    FluffySlowamountSlowamountNl,
    #[serde(rename = "%{{ slowamount }} -> %{{ slowamountNL }}")]
    PurpleSlowamountSlowamountNl,
    #[serde(rename = "{{ slowamount }}&nbsp;% -> {{ slowamountNL }}&nbsp;%")]
    SlowamountNbspSlowamountNlNbsp,
    #[serde(rename = "{{ slowamount }} % -> {{ slowamountNL }} %")]
    SlowamountSlowamountNl,
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
    Mage,
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
