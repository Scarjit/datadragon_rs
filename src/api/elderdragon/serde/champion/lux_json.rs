
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/elder_dragon_quicktype.py
*/
// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// 
use serde::{Serialize, Deserialize};
extern crate serde_json;

//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }


use serde::{Serialize, Deserialize};
extern crate serde_json;

use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct LuxJson {
    #[serde(rename = "type")]
    lux_json_type: GroupEnum,
    format: Format,
    version: Version,
    data: Data,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "Lux")]
    lux: Lux,
}

#[derive(Serialize, Deserialize)]
pub struct Lux {
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
    #[serde(rename = "customPanel")]
    custom_panel: Option<serde_json::Value>,
    #[serde(rename = "requiredPerk")]
    required_perk: Option<String>,
    #[serde(rename = "useObviousCheckmark")]
    use_obvious_checkmark: Option<bool>,
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
    show_if_summoner_spell: Option<ShowIfSummonerSpell>,
    #[serde(rename = "hideIfSummonerSpell")]
    hide_if_summoner_spell: Option<HideIfSummonerSpell>,
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
    vars: Vec<Var>,
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
pub struct Var {
    link: Link,
    coeff: f64,
    key: Key,
}

#[derive(Serialize, Deserialize)]
pub enum ChampionEnum {
    Lux,
    MissFortune,
}

#[derive(Serialize, Deserialize)]
pub enum Full {
    #[serde(rename = "LuxIlluminatingFraulein.png")]
    LuxIlluminatingFrauleinPng,
    #[serde(rename = "LuxLightBinding.png")]
    LuxLightBindingPng,
    #[serde(rename = "LuxLightStrikeKugel.png")]
    LuxLightStrikeKugelPng,
    #[serde(rename = "LuxMaliceCannon.png")]
    LuxMaliceCannonPng,
    #[serde(rename = "Lux.png")]
    LuxPng,
    #[serde(rename = "LuxPrismaticWave.png")]
    LuxPrismaticWavePng,
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
    #[serde(rename = "champion2.png")]
    Champion2Png,
    #[serde(rename = "passive2.png")]
    Passive2Png,
    #[serde(rename = "spell7.png")]
    Spell7Png,
}

#[derive(Serialize, Deserialize)]
pub enum BlockType {
    #[serde(rename = "agressive")]
    Agressive,
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
    #[serde(rename = "protective")]
    Protective,
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
    #[serde(rename = "support")]
    Support,
}

#[derive(Serialize, Deserialize)]
pub enum HideIfSummonerSpell {
    #[serde(rename = "")]
    Empty,
    ItemTeleportCancel,
    SummonerSmite,
}

#[derive(Serialize, Deserialize)]
pub enum ShowIfSummonerSpell {
    #[serde(rename = "")]
    Empty,
    ItemSmiteAoE,
    #[serde(rename = "S5_SummonerSmiteQuick")]
    S5SummonerSmiteQuick,
    SummonerSmite,
}

#[derive(Serialize, Deserialize)]
pub enum Map {
    CityPark,
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
    #[serde(rename = "STARGUARDIAN")]
    Starguardian,
    #[serde(rename = "TUTORIAL_MODULE_2")]
    TutorialModule2,
    #[serde(rename = "TUTORIAL_MODULE_3")]
    TutorialModule3,
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
    #[serde(rename = "LuxARAM")]
    LuxAram,
    #[serde(rename = "LuxCS")]
    LuxCs,
    #[serde(rename = "LuxFIRSTBLOOD")]
    LuxFirstblood,
    #[serde(rename = "LuxKINGPORO")]
    LuxKingporo,
    #[serde(rename = "LuxNPE")]
    LuxNpe,
    #[serde(rename = "LuxSG")]
    LuxSg,
    #[serde(rename = "LuxSIEGE")]
    LuxSiege,
    #[serde(rename = "LuxSL")]
    LuxSl,
    #[serde(rename = "LuxSR")]
    LuxSr,
    #[serde(rename = "LuxSupportSR")]
    LuxSupportSr,
    #[serde(rename = "LuxTT")]
    LuxTt,
}

#[derive(Serialize, Deserialize)]
pub enum CooldownBurn {
    #[serde(rename = "10/9.5/9/8.5/8")]
    The10959858,
    #[serde(rename = "13/12/11/10/9")]
    The131211109,
    #[serde(rename = "14/13/12/11/10")]
    The1413121110,
    #[serde(rename = "80/60/40")]
    The806040,
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
    LuxLightBinding,
    LuxLightStrikeKugel,
    LuxMaliceCannon,
    LuxPrismaticWave,
}

#[derive(Serialize, Deserialize)]
pub enum Effect {
    #[serde(rename = "{{ cooldown }} -> {{ cooldownNL }}")]
    CooldownCooldownNl,
    #[serde(rename = "{{ cost }} -> {{ costNL }}")]
    CostCostNl,
    #[serde(rename = "{{ damage }} -> {{ damageNL }}")]
    DamageDamageNl,
    #[serde(rename = "{{ e1 }} -> {{ e1NL }}")]
    E1E1Nl,
    #[serde(rename = "{{ e2 }} -> {{ e2NL }}")]
    E2E2Nl,
    #[serde(rename = "{{ cooldown }}->{{ cooldownNL }}")]
    EffectCooldownCooldownNl,
    #[serde(rename = "{{ cost }}->{{ costNL }}")]
    EffectCostCostNl,
    #[serde(rename = "{{ damage }}->{{ damageNL }}")]
    EffectDamageDamageNl,
    #[serde(rename = "{{ e1 }}->{{ e1NL }}")]
    EffectE1E1Nl,
    #[serde(rename = "{{ e2 }}->{{ e2NL }}")]
    EffectE2E2Nl,
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
