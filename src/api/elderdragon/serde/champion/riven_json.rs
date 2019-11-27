
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/elder_dragon_quicktype.py
*/














use serde::{Serialize, Deserialize};
extern crate serde_json;

use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct RivenJson {
    #[serde(rename = "type")]
    riven_json_type: GroupEnum,
    format: Format,
    version: Version,
    data: Data,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "Riven")]
    riven: Riven,
}

#[derive(Serialize, Deserialize)]
pub struct Riven {
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
    map: Map,
    mode: Mode,
    #[serde(rename = "type")]
    recommended_type: RecommendedType,
    #[serde(rename = "customTag")]
    custom_tag: String,
    #[serde(rename = "extensionPage")]
    extension_page: bool,
    #[serde(rename = "customPanel")]
    custom_panel: Option<serde_json::Value>,
    blocks: Vec<Block>,
    sortrank: Option<i64>,
    #[serde(rename = "useObviousCheckmark")]
    use_obvious_checkmark: Option<bool>,
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
    show_if_summoner_spell: ShowIfSummonerSpell,
    #[serde(rename = "hideIfSummonerSpell")]
    hide_if_summoner_spell: HideIfSummonerSpell,
    items: Vec<Item>,
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
    cooldown: Vec<i64>,
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
    effect: Vec<Effect>,
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
    DoubleArray(Vec<f64>),
    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum ChampionEnum {
    Riven,
}

#[derive(Serialize, Deserialize)]
pub enum Full {
    #[serde(rename = "RivenFeint.png")]
    RivenFeintPng,
    #[serde(rename = "RivenFengShuiEngine.png")]
    RivenFengShuiEnginePng,
    #[serde(rename = "RivenMartyr.png")]
    RivenMartyrPng,
    #[serde(rename = "Riven.png")]
    RivenPng,
    #[serde(rename = "RivenRunicBlades.png")]
    RivenRunicBladesPng,
    #[serde(rename = "RivenTriCleave.png")]
    RivenTriCleavePng,
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
    #[serde(rename = "spell9.png")]
    Spell9Png,
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
    #[serde(rename = "starting")]
    Starting,
    #[serde(rename = "startingjungle")]
    Startingjungle,
}

#[derive(Serialize, Deserialize)]
pub enum HideIfSummonerSpell {
    #[serde(rename = "")]
    Empty,
    SummonerPoroRecall,
    SummonerSmite,
}

#[derive(Serialize, Deserialize)]
pub enum ShowIfSummonerSpell {
    #[serde(rename = "")]
    Empty,
    SummonerSmite,
    SummonerTeleport,
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
    #[serde(rename = "RivenARAM")]
    RivenAram,
    #[serde(rename = "RivenCS")]
    RivenCs,
    #[serde(rename = "RivenFIRSTBLOOD")]
    RivenFirstblood,
    #[serde(rename = "RivenKINGPORO")]
    RivenKingporo,
    #[serde(rename = "RivenSIEGE")]
    RivenSiege,
    #[serde(rename = "RivenSL")]
    RivenSl,
    #[serde(rename = "RivenSR")]
    RivenSr,
    #[serde(rename = "RivenTT")]
    RivenTt,
}

#[derive(Serialize, Deserialize)]
pub enum SpellId {
    RivenFeint,
    RivenFengShuiEngine,
    RivenMartyr,
    RivenTriCleave,
}

#[derive(Serialize, Deserialize)]
pub enum Effect {
    #[serde(rename = "{{ cooldown }} -> {{ cooldownNL }}")]
    CooldownCooldownNl,
    #[serde(rename = "{{ e1 }} -> {{ e1NL }}")]
    E1E1Nl,
    #[serde(rename = "{{ e2 }} -> {{ e2NL }}")]
    E2E2Nl,
    #[serde(rename = "{{ e5 }} % -> {{ e5NL }} %")]
    E5E5Nl,
    #[serde(rename = "{{ e5 }}&nbsp;% -> {{ e5NL }}&nbsp;%")]
    E5NbspE5NlNbsp,
    #[serde(rename = "{{ cooldown }}->{{ cooldownNL }}")]
    EffectCooldownCooldownNl,
    #[serde(rename = "{{ e1 }}->{{ e1NL }}")]
    EffectE1E1Nl,
    #[serde(rename = "{{ e2 }}->{{ e2NL }}")]
    EffectE2E2Nl,
    #[serde(rename = "{{ e5 }}% -> {{ e5NL }}%")]
    EffectE5E5Nl,
    #[serde(rename = "{{ maxbase }}->{{ maxbaseNL }}")]
    EffectMaxbaseMaxbaseNl,
    #[serde(rename = "{{ e5 }}%->{{ e5NL }}%")]
    FluffyE5E5Nl,
    #[serde(rename = "{{ maxbase }} -> {{ maxbaseNL }}")]
    MaxbaseMaxbaseNl,
    #[serde(rename = "%{{ e5 }} -> %{{ e5NL }}")]
    PurpleE5E5Nl,
}

#[derive(Serialize, Deserialize)]
pub enum Key {
    #[serde(rename = "f1")]
    F1,
}

#[derive(Serialize, Deserialize)]
pub enum Link {
    #[serde(rename = "attackdamage")]
    Attackdamage,
    #[serde(rename = "bonusattackdamage")]
    Bonusattackdamage,
}

#[derive(Serialize, Deserialize)]
pub enum Tag {
    Assassin,
    Fighter,
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
