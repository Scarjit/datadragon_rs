
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/elder_dragon_quicktype.py
*/














use serde::{Serialize, Deserialize};
extern crate serde_json;
use self::serde_json::Error;

pub fn serialize(json: &str) -> Result<SennaJson,Error>{
    serde_json::from_str(json)
}

use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct SennaJson {
    #[serde(rename = "type")]
    senna_json_type: GroupEnum,
    format: Format,
    version: Version,
    data: Data,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "Senna")]
    senna: Senna,
}

#[derive(Serialize, Deserialize)]
pub struct Senna {
    id: ChampionEnum,
    key: String,
    name: String,
    title: String,
    image: Image,
    skins: Vec<Skin>,
    lore: String,
    blurb: String,
    allytips: Vec<Option<serde_json::Value>>,
    enemytips: Vec<Option<serde_json::Value>>,
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
    map: Map,
    mode: Mode,
    #[serde(rename = "type")]
    recommended_type: RecommendedType,
    #[serde(rename = "customTag")]
    custom_tag: String,
    sortrank: i64,
    #[serde(rename = "extensionPage")]
    extension_page: bool,
    #[serde(rename = "useObviousCheckmark")]
    use_obvious_checkmark: bool,
    #[serde(rename = "customPanel")]
    custom_panel: Option<serde_json::Value>,
    blocks: Vec<Block>,
}

#[derive(Serialize, Deserialize)]
pub struct Block {
    #[serde(rename = "type")]
    block_type: BlockType,
    #[serde(rename = "recMath")]
    rec_math: bool,
    #[serde(rename = "recSteps")]
    rec_steps: bool,
    #[serde(rename = "minSummonerLevel")]
    min_summoner_level: i64,
    #[serde(rename = "maxSummonerLevel")]
    max_summoner_level: i64,
    #[serde(rename = "showIfSummonerSpell")]
    show_if_summoner_spell: String,
    #[serde(rename = "hideIfSummonerSpell")]
    hide_if_summoner_spell: String,
    #[serde(rename = "appendAfterSection")]
    append_after_section: String,
    #[serde(rename = "visibleWithAllOf")]
    visible_with_all_of: Vec<String>,
    #[serde(rename = "hiddenWithAnyOf")]
    hidden_with_any_of: Vec<String>,
    items: Vec<Item>,
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    id: String,
    count: i64,
    #[serde(rename = "hideCount")]
    hide_count: bool,
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
    cooldown_burn: String,
    cost: Vec<i64>,
    #[serde(rename = "costBurn")]
    cost_burn: String,
    datavalues: Datavalues,
    effect: Vec<Option<Vec<i64>>>,
    #[serde(rename = "effectBurn")]
    effect_burn: Vec<Option<String>>,
    vars: Vec<Option<serde_json::Value>>,
    #[serde(rename = "costType")]
    cost_type: CostType,
    maxammo: String,
    range: Vec<i64>,
    #[serde(rename = "rangeBurn")]
    range_burn: String,
    image: Image,
    resource: Resource,
}

#[derive(Serialize, Deserialize)]
pub struct Datavalues {
}

#[derive(Serialize, Deserialize)]
pub struct Leveltip {
    label: Vec<String>,
    effect: Vec<Effect>,
}

#[derive(Serialize, Deserialize)]
pub enum ChampionEnum {
    Senna,
}

#[derive(Serialize, Deserialize)]
pub enum Full {
    #[serde(rename = "SennaE.png")]
    SennaEPng,
    #[serde(rename = "Senna_Passive.Senna.png")]
    SennaPassiveSennaPng,
    #[serde(rename = "Senna.png")]
    SennaPng,
    #[serde(rename = "SennaQ.png")]
    SennaQPng,
    #[serde(rename = "SennaR.png")]
    SennaRPng,
    #[serde(rename = "SennaW.png")]
    SennaWPng,
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
    #[serde(rename = "essential")]
    Essential,
    #[serde(rename = "offensive")]
    Offensive,
    #[serde(rename = "starting")]
    Starting,
    #[serde(rename = "support")]
    Support,
}

#[derive(Serialize, Deserialize)]
pub enum Map {
    #[serde(rename = "HA")]
    Ha,
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
    #[serde(rename = "INTRO")]
    Intro,
    #[serde(rename = "KINGPORO")]
    Kingporo,
}

#[derive(Serialize, Deserialize)]
pub enum RecommendedType {
    #[serde(rename = "riot")]
    Riot,
    #[serde(rename = "riot-support")]
    RiotSupport,
}

#[derive(Serialize, Deserialize)]
pub enum Title {
    Beginner,
    #[serde(rename = "SennaARAM")]
    SennaAram,
    #[serde(rename = "SennaKINGPORO")]
    SennaKingporo,
    #[serde(rename = "SennaSR")]
    SennaSr,
    #[serde(rename = "SennaSRBottom")]
    SennaSrBottom,
    #[serde(rename = "SennaTT")]
    SennaTt,
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
    SennaE,
    SennaQ,
    SennaR,
    SennaW,
}

#[derive(Serialize, Deserialize)]
pub enum Effect {
    #[serde(rename = "{{ basedamage }} -> {{ basedamageNL }}")]
    BasedamageBasedamageNl,
    #[serde(rename = "{{ baseheal }} -> {{ basehealNL }}")]
    BasehealBasehealNl,
    #[serde(rename = "{{ buffduration }} -> {{ buffdurationNL }}")]
    BuffdurationBuffdurationNl,
    #[serde(rename = "{{ cooldown }} -> {{ cooldownNL }}")]
    CooldownCooldownNl,
    #[serde(rename = "{{ cost }} -> {{ costNL }}")]
    CostCostNl,
    #[serde(rename = "{{ damage }} -> {{ damageNL }}")]
    DamageDamageNl,
    #[serde(rename = "{{ basedamage }}->{{ basedamageNL }}")]
    EffectBasedamageBasedamageNl,
    #[serde(rename = "{{ baseheal }}->{{ basehealNL }}")]
    EffectBasehealBasehealNl,
    #[serde(rename = "{{ buffduration }}->{{ buffdurationNL }}")]
    EffectBuffdurationBuffdurationNl,
    #[serde(rename = "{{ cooldown }}->{{ cooldownNL }}")]
    EffectCooldownCooldownNl,
    #[serde(rename = "{{ cost }}->{{ costNL }}")]
    EffectCostCostNl,
    #[serde(rename = "{{ damage }}->{{ damageNL }}")]
    EffectDamageDamageNl,
    #[serde(rename = "{{ rootduration }}->{{ rootdurationNL }}")]
    EffectRootdurationRootdurationNl,
    #[serde(rename = "{{ shield }}->{{ shieldNL }}")]
    EffectShieldShieldNl,
    #[serde(rename = "{{ rootduration }} -> {{ rootdurationNL }}")]
    RootdurationRootdurationNl,
    #[serde(rename = "{{ shield }} -> {{ shieldNL }}")]
    ShieldShieldNl,
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
    Marksman,
    Support,
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
