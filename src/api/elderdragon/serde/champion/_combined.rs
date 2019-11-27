
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
pub struct Aatrox {
    #[serde(rename = "type")]
    aatrox_type: GroupEnum,
    format: String,
    version: String,
    data: AatroxData,
}

#[derive(Serialize, Deserialize)]
pub struct AatroxData {
    #[serde(rename = "Aatrox")]
    aatrox: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct PuneHedgehog {
    id: String,
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
    recommended: Vec<PurpleRecommended>,
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
pub struct PurpleRecommended {
    champion: String,
    title: String,
    map: Map,
    mode: Mode,
    #[serde(rename = "type")]
    recommended_type: RecommendedType,
    #[serde(rename = "customTag")]
    custom_tag: String,
    sortrank: Option<i64>,
    #[serde(rename = "extensionPage")]
    extension_page: Option<bool>,
    #[serde(rename = "useObviousCheckmark")]
    use_obvious_checkmark: Option<bool>,
    #[serde(rename = "customPanel")]
    custom_panel: Option<serde_json::Value>,
    blocks: Vec<Block>,
    #[serde(rename = "requiredPerk")]
    required_perk: Option<String>,
    #[serde(rename = "extenOrnnPage")]
    exten_ornn_page: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct Block {
    #[serde(rename = "type")]
    block_type: BlockType,
    #[serde(rename = "recMath")]
    rec_math: Option<bool>,
    #[serde(rename = "recSteps")]
    rec_steps: Option<bool>,
    #[serde(rename = "minSummonerLevel")]
    min_summoner_level: Option<i64>,
    #[serde(rename = "maxSummonerLevel")]
    max_summoner_level: Option<i64>,
    #[serde(rename = "showIfSummonerSpell")]
    show_if_summoner_spell: Option<IfSummonerSpell>,
    #[serde(rename = "hideIfSummonerSpell")]
    hide_if_summoner_spell: Option<IfSummonerSpell>,
    #[serde(rename = "appendAfterSection")]
    append_after_section: Option<String>,
    #[serde(rename = "visibleWithAllOf")]
    visible_with_all_of: Option<Vec<Of>>,
    #[serde(rename = "hiddenWithAnyOf")]
    hidden_with_any_of: Option<Vec<Of>>,
    items: Vec<Item>,
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    id: String,
    count: i64,
    #[serde(rename = "hideCount")]
    hide_count: Option<bool>,
    hidecount: Option<bool>,
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
    id: String,
    name: String,
    description: String,
    tooltip: String,
    leveltip: Option<Leveltip>,
    maxrank: i64,
    cooldown: Vec<f64>,
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
    cost_type: CostType,
    maxammo: String,
    range: Vec<i64>,
    #[serde(rename = "rangeBurn")]
    range_burn: String,
    image: Image,
    resource: Option<String>,
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
pub struct Var {
    link: Link,
    coeff: Coeff,
    key: Key,
}

#[derive(Serialize, Deserialize)]
pub struct Ahri {
    #[serde(rename = "type")]
    ahri_type: GroupEnum,
    format: String,
    version: String,
    data: AhriData,
}

#[derive(Serialize, Deserialize)]
pub struct AhriData {
    #[serde(rename = "Ahri")]
    ahri: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct HammerfestPonies {
    id: String,
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
    partype: Partype,
    info: Info,
    stats: HashMap<String, f64>,
    spells: Vec<Spell>,
    passive: Passive,
    recommended: Vec<FluffyRecommended>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyRecommended {
    champion: String,
    title: String,
    map: Map,
    mode: Mode,
    #[serde(rename = "type")]
    recommended_type: RecommendedType,
    #[serde(rename = "customTag")]
    custom_tag: Option<String>,
    sortrank: Option<i64>,
    #[serde(rename = "extensionPage")]
    extension_page: Option<bool>,
    #[serde(rename = "customPanel")]
    custom_panel: Option<serde_json::Value>,
    blocks: Vec<Block>,
    #[serde(rename = "requiredPerk")]
    required_perk: Option<String>,
    #[serde(rename = "useObviousCheckmark")]
    use_obvious_checkmark: Option<bool>,
    priority: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct Akali {
    #[serde(rename = "type")]
    akali_type: GroupEnum,
    format: String,
    version: String,
    data: AkaliData,
}

#[derive(Serialize, Deserialize)]
pub struct AkaliData {
    #[serde(rename = "Akali")]
    akali: AkaliClass,
}

#[derive(Serialize, Deserialize)]
pub struct AkaliClass {
    id: String,
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
    partype: Partype,
    info: Info,
    stats: HashMap<String, f64>,
    spells: Vec<Spell>,
    passive: Passive,
    recommended: Vec<AkaliRecommended>,
}

#[derive(Serialize, Deserialize)]
pub struct AkaliRecommended {
    champion: String,
    title: String,
    map: Map,
    mode: Mode,
    #[serde(rename = "type")]
    recommended_type: RecommendedType,
    #[serde(rename = "customTag")]
    custom_tag: String,
    #[serde(rename = "requiredPerk")]
    required_perk: Option<String>,
    sortrank: Option<i64>,
    #[serde(rename = "extensionPage")]
    extension_page: Option<bool>,
    #[serde(rename = "customPanel")]
    custom_panel: Option<String>,
    blocks: Vec<Block>,
    #[serde(rename = "useObviousCheckmark")]
    use_obvious_checkmark: Option<bool>,
    #[serde(rename = "customPanelCurrencyType")]
    custom_panel_currency_type: Option<String>,
    #[serde(rename = "customPanelBuffCurrencyName")]
    custom_panel_buff_currency_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Alistar {
    #[serde(rename = "type")]
    alistar_type: GroupEnum,
    format: String,
    version: String,
    data: AlistarData,
}

#[derive(Serialize, Deserialize)]
pub struct AlistarData {
    #[serde(rename = "Alistar")]
    alistar: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Amumu {
    #[serde(rename = "type")]
    amumu_type: GroupEnum,
    format: String,
    version: String,
    data: AmumuData,
}

#[derive(Serialize, Deserialize)]
pub struct AmumuData {
    #[serde(rename = "Amumu")]
    amumu: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Anivia {
    #[serde(rename = "type")]
    anivia_type: GroupEnum,
    format: String,
    version: String,
    data: AniviaData,
}

#[derive(Serialize, Deserialize)]
pub struct AniviaData {
    #[serde(rename = "Anivia")]
    anivia: PurpleAnivia,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleAnivia {
    id: PurpleId,
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
    partype: Partype,
    info: Info,
    stats: HashMap<String, f64>,
    spells: Vec<Spell>,
    passive: Passive,
    recommended: Vec<AniviaRecommended>,
}

#[derive(Serialize, Deserialize)]
pub struct AniviaRecommended {
    champion: PurpleId,
    title: String,
    map: Map,
    mode: Mode,
    #[serde(rename = "type")]
    recommended_type: RecommendedType,
    #[serde(rename = "customTag")]
    custom_tag: String,
    sortrank: Option<i64>,
    #[serde(rename = "extensionPage")]
    extension_page: bool,
    #[serde(rename = "customPanel")]
    custom_panel: Option<serde_json::Value>,
    blocks: Vec<Block>,
    #[serde(rename = "useObviousCheckmark")]
    use_obvious_checkmark: Option<bool>,
    #[serde(rename = "requiredPerk")]
    required_perk: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Annie {
    #[serde(rename = "type")]
    annie_type: GroupEnum,
    format: String,
    version: String,
    data: AnnieData,
}

#[derive(Serialize, Deserialize)]
pub struct AnnieData {
    #[serde(rename = "Annie")]
    annie: PurpleAnivia,
}

#[derive(Serialize, Deserialize)]
pub struct Ashe {
    #[serde(rename = "type")]
    ashe_type: GroupEnum,
    format: String,
    version: String,
    data: AsheData,
}

#[derive(Serialize, Deserialize)]
pub struct AsheData {
    #[serde(rename = "Ashe")]
    ashe: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct AurelionSol {
    #[serde(rename = "type")]
    aurelion_sol_type: GroupEnum,
    format: String,
    version: String,
    data: AurelionSolData,
}

#[derive(Serialize, Deserialize)]
pub struct AurelionSolData {
    #[serde(rename = "AurelionSol")]
    aurelion_sol: PurpleAnivia,
}

#[derive(Serialize, Deserialize)]
pub struct Azir {
    #[serde(rename = "type")]
    azir_type: GroupEnum,
    format: String,
    version: String,
    data: AzirData,
}

#[derive(Serialize, Deserialize)]
pub struct AzirData {
    #[serde(rename = "Azir")]
    azir: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Bard {
    #[serde(rename = "type")]
    bard_type: GroupEnum,
    format: String,
    version: String,
    data: BardData,
}

#[derive(Serialize, Deserialize)]
pub struct BardData {
    #[serde(rename = "Bard")]
    bard: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Blitzcrank {
    #[serde(rename = "type")]
    blitzcrank_type: GroupEnum,
    format: String,
    version: String,
    data: BlitzcrankData,
}

#[derive(Serialize, Deserialize)]
pub struct BlitzcrankData {
    #[serde(rename = "Blitzcrank")]
    blitzcrank: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Brand {
    #[serde(rename = "type")]
    brand_type: GroupEnum,
    format: String,
    version: String,
    data: BrandData,
}

#[derive(Serialize, Deserialize)]
pub struct BrandData {
    #[serde(rename = "Brand")]
    brand: BrandClass,
}

#[derive(Serialize, Deserialize)]
pub struct BrandClass {
    id: BrandId,
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
    partype: Partype,
    info: Info,
    stats: HashMap<String, f64>,
    spells: Vec<Spell>,
    passive: Passive,
    recommended: Vec<BrandRecommended>,
}

#[derive(Serialize, Deserialize)]
pub struct BrandRecommended {
    champion: BrandId,
    title: String,
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
    #[serde(rename = "useObviousCheckmark")]
    use_obvious_checkmark: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct Braum {
    #[serde(rename = "type")]
    braum_type: GroupEnum,
    format: String,
    version: String,
    data: BraumData,
}

#[derive(Serialize, Deserialize)]
pub struct BraumData {
    #[serde(rename = "Braum")]
    braum: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Caitlyn {
    #[serde(rename = "type")]
    caitlyn_type: GroupEnum,
    format: String,
    version: String,
    data: CaitlynData,
}

#[derive(Serialize, Deserialize)]
pub struct CaitlynData {
    #[serde(rename = "Caitlyn")]
    caitlyn: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Camille {
    #[serde(rename = "type")]
    camille_type: GroupEnum,
    format: String,
    version: String,
    data: CamilleData,
}

#[derive(Serialize, Deserialize)]
pub struct CamilleData {
    #[serde(rename = "Camille")]
    camille: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Cassiopeia {
    #[serde(rename = "type")]
    cassiopeia_type: GroupEnum,
    format: String,
    version: String,
    data: CassiopeiaData,
}

#[derive(Serialize, Deserialize)]
pub struct CassiopeiaData {
    #[serde(rename = "Cassiopeia")]
    cassiopeia: PurpleAnivia,
}

#[derive(Serialize, Deserialize)]
pub struct Chogath {
    #[serde(rename = "type")]
    chogath_type: GroupEnum,
    format: String,
    version: String,
    data: ChogathData,
}

#[derive(Serialize, Deserialize)]
pub struct ChogathData {
    #[serde(rename = "Chogath")]
    chogath: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Corki {
    #[serde(rename = "type")]
    corki_type: GroupEnum,
    format: String,
    version: String,
    data: CorkiData,
}

#[derive(Serialize, Deserialize)]
pub struct CorkiData {
    #[serde(rename = "Corki")]
    corki: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Darius {
    #[serde(rename = "type")]
    darius_type: GroupEnum,
    format: String,
    version: String,
    data: DariusData,
}

#[derive(Serialize, Deserialize)]
pub struct DariusData {
    #[serde(rename = "Darius")]
    darius: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Diana {
    #[serde(rename = "type")]
    diana_type: GroupEnum,
    format: String,
    version: String,
    data: DianaData,
}

#[derive(Serialize, Deserialize)]
pub struct DianaData {
    #[serde(rename = "Diana")]
    diana: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Draven {
    #[serde(rename = "type")]
    draven_type: GroupEnum,
    format: String,
    version: String,
    data: DravenData,
}

#[derive(Serialize, Deserialize)]
pub struct DravenData {
    #[serde(rename = "Draven")]
    draven: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct DrMundo {
    #[serde(rename = "type")]
    dr_mundo_type: GroupEnum,
    format: String,
    version: String,
    data: DrMundoData,
}

#[derive(Serialize, Deserialize)]
pub struct DrMundoData {
    #[serde(rename = "DrMundo")]
    dr_mundo: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Ekko {
    #[serde(rename = "type")]
    ekko_type: GroupEnum,
    format: String,
    version: String,
    data: EkkoData,
}

#[derive(Serialize, Deserialize)]
pub struct EkkoData {
    #[serde(rename = "Ekko")]
    ekko: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Elise {
    #[serde(rename = "type")]
    elise_type: GroupEnum,
    format: String,
    version: String,
    data: EliseData,
}

#[derive(Serialize, Deserialize)]
pub struct EliseData {
    #[serde(rename = "Elise")]
    elise: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Evelynn {
    #[serde(rename = "type")]
    evelynn_type: GroupEnum,
    format: String,
    version: String,
    data: EvelynnData,
}

#[derive(Serialize, Deserialize)]
pub struct EvelynnData {
    #[serde(rename = "Evelynn")]
    evelynn: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Ezreal {
    #[serde(rename = "type")]
    ezreal_type: GroupEnum,
    format: String,
    version: String,
    data: EzrealData,
}

#[derive(Serialize, Deserialize)]
pub struct EzrealData {
    #[serde(rename = "Ezreal")]
    ezreal: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Fiddlesticks {
    #[serde(rename = "type")]
    fiddlesticks_type: GroupEnum,
    format: String,
    version: String,
    data: FiddlesticksData,
}

#[derive(Serialize, Deserialize)]
pub struct FiddlesticksData {
    #[serde(rename = "Fiddlesticks")]
    fiddlesticks: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Fiora {
    #[serde(rename = "type")]
    fiora_type: GroupEnum,
    format: String,
    version: String,
    data: FioraData,
}

#[derive(Serialize, Deserialize)]
pub struct FioraData {
    #[serde(rename = "Fiora")]
    fiora: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Fizz {
    #[serde(rename = "type")]
    fizz_type: GroupEnum,
    format: String,
    version: String,
    data: FizzData,
}

#[derive(Serialize, Deserialize)]
pub struct FizzData {
    #[serde(rename = "Fizz")]
    fizz: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Galio {
    #[serde(rename = "type")]
    galio_type: GroupEnum,
    format: String,
    version: String,
    data: GalioData,
}

#[derive(Serialize, Deserialize)]
pub struct GalioData {
    #[serde(rename = "Galio")]
    galio: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Gangplank {
    #[serde(rename = "type")]
    gangplank_type: GroupEnum,
    format: String,
    version: String,
    data: GangplankData,
}

#[derive(Serialize, Deserialize)]
pub struct GangplankData {
    #[serde(rename = "Gangplank")]
    gangplank: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Garen {
    #[serde(rename = "type")]
    garen_type: GroupEnum,
    format: String,
    version: String,
    data: GarenData,
}

#[derive(Serialize, Deserialize)]
pub struct GarenData {
    #[serde(rename = "Garen")]
    garen: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Gnar {
    #[serde(rename = "type")]
    gnar_type: GroupEnum,
    format: String,
    version: String,
    data: GnarData,
}

#[derive(Serialize, Deserialize)]
pub struct GnarData {
    #[serde(rename = "Gnar")]
    gnar: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Gragas {
    #[serde(rename = "type")]
    gragas_type: GroupEnum,
    format: String,
    version: String,
    data: GragasData,
}

#[derive(Serialize, Deserialize)]
pub struct GragasData {
    #[serde(rename = "Gragas")]
    gragas: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Graves {
    #[serde(rename = "type")]
    graves_type: GroupEnum,
    format: String,
    version: String,
    data: GravesData,
}

#[derive(Serialize, Deserialize)]
pub struct GravesData {
    #[serde(rename = "Graves")]
    graves: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Hecarim {
    #[serde(rename = "type")]
    hecarim_type: GroupEnum,
    format: String,
    version: String,
    data: HecarimData,
}

#[derive(Serialize, Deserialize)]
pub struct HecarimData {
    #[serde(rename = "Hecarim")]
    hecarim: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Heimerdinger {
    #[serde(rename = "type")]
    heimerdinger_type: GroupEnum,
    format: String,
    version: String,
    data: HeimerdingerData,
}

#[derive(Serialize, Deserialize)]
pub struct HeimerdingerData {
    #[serde(rename = "Heimerdinger")]
    heimerdinger: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Illaoi {
    #[serde(rename = "type")]
    illaoi_type: GroupEnum,
    format: String,
    version: String,
    data: IllaoiData,
}

#[derive(Serialize, Deserialize)]
pub struct IllaoiData {
    #[serde(rename = "Illaoi")]
    illaoi: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Irelia {
    #[serde(rename = "type")]
    irelia_type: GroupEnum,
    format: String,
    version: String,
    data: IreliaData,
}

#[derive(Serialize, Deserialize)]
pub struct IreliaData {
    #[serde(rename = "Irelia")]
    irelia: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Ivern {
    #[serde(rename = "type")]
    ivern_type: GroupEnum,
    format: String,
    version: String,
    data: IvernData,
}

#[derive(Serialize, Deserialize)]
pub struct IvernData {
    #[serde(rename = "Ivern")]
    ivern: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Janna {
    #[serde(rename = "type")]
    janna_type: GroupEnum,
    format: String,
    version: String,
    data: JannaData,
}

#[derive(Serialize, Deserialize)]
pub struct JannaData {
    #[serde(rename = "Janna")]
    janna: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct JarvanIv {
    #[serde(rename = "type")]
    jarvan_iv_type: GroupEnum,
    format: String,
    version: String,
    data: JarvanIvData,
}

#[derive(Serialize, Deserialize)]
pub struct JarvanIvData {
    #[serde(rename = "JarvanIV")]
    jarvan_iv: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Jax {
    #[serde(rename = "type")]
    jax_type: GroupEnum,
    format: String,
    version: String,
    data: JaxData,
}

#[derive(Serialize, Deserialize)]
pub struct JaxData {
    #[serde(rename = "Jax")]
    jax: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Jayce {
    #[serde(rename = "type")]
    jayce_type: GroupEnum,
    format: String,
    version: String,
    data: JayceData,
}

#[derive(Serialize, Deserialize)]
pub struct JayceData {
    #[serde(rename = "Jayce")]
    jayce: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Jhin {
    #[serde(rename = "type")]
    jhin_type: GroupEnum,
    format: String,
    version: String,
    data: JhinData,
}

#[derive(Serialize, Deserialize)]
pub struct JhinData {
    #[serde(rename = "Jhin")]
    jhin: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Jinx {
    #[serde(rename = "type")]
    jinx_type: GroupEnum,
    format: String,
    version: String,
    data: JinxData,
}

#[derive(Serialize, Deserialize)]
pub struct JinxData {
    #[serde(rename = "Jinx")]
    jinx: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Kaisa {
    #[serde(rename = "type")]
    kaisa_type: GroupEnum,
    format: String,
    version: String,
    data: KaisaData,
}

#[derive(Serialize, Deserialize)]
pub struct KaisaData {
    #[serde(rename = "Kaisa")]
    kaisa: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Kalista {
    #[serde(rename = "type")]
    kalista_type: GroupEnum,
    format: String,
    version: String,
    data: KalistaData,
}

#[derive(Serialize, Deserialize)]
pub struct KalistaData {
    #[serde(rename = "Kalista")]
    kalista: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Karma {
    #[serde(rename = "type")]
    karma_type: GroupEnum,
    format: String,
    version: String,
    data: KarmaData,
}

#[derive(Serialize, Deserialize)]
pub struct KarmaData {
    #[serde(rename = "Karma")]
    karma: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Karthus {
    #[serde(rename = "type")]
    karthus_type: GroupEnum,
    format: String,
    version: String,
    data: KarthusData,
}

#[derive(Serialize, Deserialize)]
pub struct KarthusData {
    #[serde(rename = "Karthus")]
    karthus: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Kassadin {
    #[serde(rename = "type")]
    kassadin_type: GroupEnum,
    format: String,
    version: String,
    data: KassadinData,
}

#[derive(Serialize, Deserialize)]
pub struct KassadinData {
    #[serde(rename = "Kassadin")]
    kassadin: PurpleAnivia,
}

#[derive(Serialize, Deserialize)]
pub struct Katarina {
    #[serde(rename = "type")]
    katarina_type: GroupEnum,
    format: String,
    version: String,
    data: KatarinaData,
}

#[derive(Serialize, Deserialize)]
pub struct KatarinaData {
    #[serde(rename = "Katarina")]
    katarina: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Kayle {
    #[serde(rename = "type")]
    kayle_type: GroupEnum,
    format: String,
    version: String,
    data: KayleData,
}

#[derive(Serialize, Deserialize)]
pub struct KayleData {
    #[serde(rename = "Kayle")]
    kayle: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Kayn {
    #[serde(rename = "type")]
    kayn_type: GroupEnum,
    format: String,
    version: String,
    data: KaynData,
}

#[derive(Serialize, Deserialize)]
pub struct KaynData {
    #[serde(rename = "Kayn")]
    kayn: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Kennen {
    #[serde(rename = "type")]
    kennen_type: GroupEnum,
    format: String,
    version: String,
    data: KennenData,
}

#[derive(Serialize, Deserialize)]
pub struct KennenData {
    #[serde(rename = "Kennen")]
    kennen: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Khazix {
    #[serde(rename = "type")]
    khazix_type: GroupEnum,
    format: String,
    version: String,
    data: KhazixData,
}

#[derive(Serialize, Deserialize)]
pub struct KhazixData {
    #[serde(rename = "Khazix")]
    khazix: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Kindred {
    #[serde(rename = "type")]
    kindred_type: GroupEnum,
    format: String,
    version: String,
    data: KindredData,
}

#[derive(Serialize, Deserialize)]
pub struct KindredData {
    #[serde(rename = "Kindred")]
    kindred: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Kled {
    #[serde(rename = "type")]
    kled_type: GroupEnum,
    format: String,
    version: String,
    data: KledData,
}

#[derive(Serialize, Deserialize)]
pub struct KledData {
    #[serde(rename = "Kled")]
    kled: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct KogMaw {
    #[serde(rename = "type")]
    kog_maw_type: GroupEnum,
    format: String,
    version: String,
    data: KogMawData,
}

#[derive(Serialize, Deserialize)]
pub struct KogMawData {
    #[serde(rename = "KogMaw")]
    kog_maw: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Leblanc {
    #[serde(rename = "type")]
    leblanc_type: GroupEnum,
    format: String,
    version: String,
    data: LeblancData,
}

#[derive(Serialize, Deserialize)]
pub struct LeblancData {
    #[serde(rename = "Leblanc")]
    leblanc: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct LeeSin {
    #[serde(rename = "type")]
    lee_sin_type: GroupEnum,
    format: String,
    version: String,
    data: LeeSinData,
}

#[derive(Serialize, Deserialize)]
pub struct LeeSinData {
    #[serde(rename = "LeeSin")]
    lee_sin: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Leona {
    #[serde(rename = "type")]
    leona_type: GroupEnum,
    format: String,
    version: String,
    data: LeonaData,
}

#[derive(Serialize, Deserialize)]
pub struct LeonaData {
    #[serde(rename = "Leona")]
    leona: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Lissandra {
    #[serde(rename = "type")]
    lissandra_type: GroupEnum,
    format: String,
    version: String,
    data: LissandraData,
}

#[derive(Serialize, Deserialize)]
pub struct LissandraData {
    #[serde(rename = "Lissandra")]
    lissandra: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Lucian {
    #[serde(rename = "type")]
    lucian_type: GroupEnum,
    format: String,
    version: String,
    data: LucianData,
}

#[derive(Serialize, Deserialize)]
pub struct LucianData {
    #[serde(rename = "Lucian")]
    lucian: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Lulu {
    #[serde(rename = "type")]
    lulu_type: GroupEnum,
    format: String,
    version: String,
    data: LuluData,
}

#[derive(Serialize, Deserialize)]
pub struct LuluData {
    #[serde(rename = "Lulu")]
    lulu: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Lux {
    #[serde(rename = "type")]
    lux_type: GroupEnum,
    format: String,
    version: String,
    data: LuxData,
}

#[derive(Serialize, Deserialize)]
pub struct LuxData {
    #[serde(rename = "Lux")]
    lux: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Malphite {
    #[serde(rename = "type")]
    malphite_type: GroupEnum,
    format: String,
    version: String,
    data: MalphiteData,
}

#[derive(Serialize, Deserialize)]
pub struct MalphiteData {
    #[serde(rename = "Malphite")]
    malphite: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Malzahar {
    #[serde(rename = "type")]
    malzahar_type: GroupEnum,
    format: String,
    version: String,
    data: MalzaharData,
}

#[derive(Serialize, Deserialize)]
pub struct MalzaharData {
    #[serde(rename = "Malzahar")]
    malzahar: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Maokai {
    #[serde(rename = "type")]
    maokai_type: GroupEnum,
    format: String,
    version: String,
    data: MaokaiData,
}

#[derive(Serialize, Deserialize)]
pub struct MaokaiData {
    #[serde(rename = "Maokai")]
    maokai: PurpleAnivia,
}

#[derive(Serialize, Deserialize)]
pub struct MasterYi {
    #[serde(rename = "type")]
    master_yi_type: GroupEnum,
    format: String,
    version: String,
    data: MasterYiData,
}

#[derive(Serialize, Deserialize)]
pub struct MasterYiData {
    #[serde(rename = "MasterYi")]
    master_yi: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct MissFortune {
    #[serde(rename = "type")]
    miss_fortune_type: GroupEnum,
    format: String,
    version: String,
    data: MissFortuneData,
}

#[derive(Serialize, Deserialize)]
pub struct MissFortuneData {
    #[serde(rename = "MissFortune")]
    miss_fortune: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct MonkeyKing {
    #[serde(rename = "type")]
    monkey_king_type: GroupEnum,
    format: String,
    version: String,
    data: MonkeyKingData,
}

#[derive(Serialize, Deserialize)]
pub struct MonkeyKingData {
    #[serde(rename = "MonkeyKing")]
    monkey_king: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Mordekaiser {
    #[serde(rename = "type")]
    mordekaiser_type: GroupEnum,
    format: String,
    version: String,
    data: MordekaiserData,
}

#[derive(Serialize, Deserialize)]
pub struct MordekaiserData {
    #[serde(rename = "Mordekaiser")]
    mordekaiser: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Morgana {
    #[serde(rename = "type")]
    morgana_type: GroupEnum,
    format: String,
    version: String,
    data: MorganaData,
}

#[derive(Serialize, Deserialize)]
pub struct MorganaData {
    #[serde(rename = "Morgana")]
    morgana: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Nami {
    #[serde(rename = "type")]
    nami_type: GroupEnum,
    format: String,
    version: String,
    data: NamiData,
}

#[derive(Serialize, Deserialize)]
pub struct NamiData {
    #[serde(rename = "Nami")]
    nami: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Nasus {
    #[serde(rename = "type")]
    nasus_type: GroupEnum,
    format: String,
    version: String,
    data: NasusData,
}

#[derive(Serialize, Deserialize)]
pub struct NasusData {
    #[serde(rename = "Nasus")]
    nasus: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Nautilus {
    #[serde(rename = "type")]
    nautilus_type: GroupEnum,
    format: String,
    version: String,
    data: NautilusData,
}

#[derive(Serialize, Deserialize)]
pub struct NautilusData {
    #[serde(rename = "Nautilus")]
    nautilus: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Neeko {
    #[serde(rename = "type")]
    neeko_type: GroupEnum,
    format: String,
    version: String,
    data: NeekoData,
}

#[derive(Serialize, Deserialize)]
pub struct NeekoData {
    #[serde(rename = "Neeko")]
    neeko: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Nidalee {
    #[serde(rename = "type")]
    nidalee_type: GroupEnum,
    format: String,
    version: String,
    data: NidaleeData,
}

#[derive(Serialize, Deserialize)]
pub struct NidaleeData {
    #[serde(rename = "Nidalee")]
    nidalee: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Nocturne {
    #[serde(rename = "type")]
    nocturne_type: GroupEnum,
    format: String,
    version: String,
    data: NocturneData,
}

#[derive(Serialize, Deserialize)]
pub struct NocturneData {
    #[serde(rename = "Nocturne")]
    nocturne: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Nunu {
    #[serde(rename = "type")]
    nunu_type: GroupEnum,
    format: String,
    version: String,
    data: NunuData,
}

#[derive(Serialize, Deserialize)]
pub struct NunuData {
    #[serde(rename = "Nunu")]
    nunu: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Olaf {
    #[serde(rename = "type")]
    olaf_type: GroupEnum,
    format: String,
    version: String,
    data: OlafData,
}

#[derive(Serialize, Deserialize)]
pub struct OlafData {
    #[serde(rename = "Olaf")]
    olaf: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Orianna {
    #[serde(rename = "type")]
    orianna_type: GroupEnum,
    format: String,
    version: String,
    data: OriannaData,
}

#[derive(Serialize, Deserialize)]
pub struct OriannaData {
    #[serde(rename = "Orianna")]
    orianna: PurpleAnivia,
}

#[derive(Serialize, Deserialize)]
pub struct Ornn {
    #[serde(rename = "type")]
    ornn_type: GroupEnum,
    format: String,
    version: String,
    data: OrnnData,
}

#[derive(Serialize, Deserialize)]
pub struct OrnnData {
    #[serde(rename = "Ornn")]
    ornn: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Pantheon {
    #[serde(rename = "type")]
    pantheon_type: GroupEnum,
    format: String,
    version: String,
    data: PantheonData,
}

#[derive(Serialize, Deserialize)]
pub struct PantheonData {
    #[serde(rename = "Pantheon")]
    pantheon: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Poppy {
    #[serde(rename = "type")]
    poppy_type: GroupEnum,
    format: String,
    version: String,
    data: PoppyData,
}

#[derive(Serialize, Deserialize)]
pub struct PoppyData {
    #[serde(rename = "Poppy")]
    poppy: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Pyke {
    #[serde(rename = "type")]
    pyke_type: GroupEnum,
    format: String,
    version: String,
    data: PykeData,
}

#[derive(Serialize, Deserialize)]
pub struct PykeData {
    #[serde(rename = "Pyke")]
    pyke: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Qiyana {
    #[serde(rename = "type")]
    qiyana_type: GroupEnum,
    format: String,
    version: String,
    data: QiyanaData,
}

#[derive(Serialize, Deserialize)]
pub struct QiyanaData {
    #[serde(rename = "Qiyana")]
    qiyana: QiyanaClass,
}

#[derive(Serialize, Deserialize)]
pub struct QiyanaClass {
    id: FluffyId,
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
    partype: Partype,
    info: Info,
    stats: HashMap<String, f64>,
    spells: Vec<Spell>,
    passive: Passive,
    recommended: Vec<QiyanaRecommended>,
}

#[derive(Serialize, Deserialize)]
pub struct QiyanaRecommended {
    champion: FluffyId,
    title: String,
    map: String,
    mode: Mode,
    #[serde(rename = "type")]
    recommended_type: RecommendedType,
    #[serde(rename = "customTag")]
    custom_tag: String,
    sortrank: i64,
    #[serde(rename = "extensionPage")]
    extension_page: bool,
    #[serde(rename = "useObviousCheckmark")]
    use_obvious_checkmark: Option<bool>,
    #[serde(rename = "customPanel")]
    custom_panel: Option<serde_json::Value>,
    blocks: Vec<Block>,
}

#[derive(Serialize, Deserialize)]
pub struct Quinn {
    #[serde(rename = "type")]
    quinn_type: GroupEnum,
    format: String,
    version: String,
    data: QuinnData,
}

#[derive(Serialize, Deserialize)]
pub struct QuinnData {
    #[serde(rename = "Quinn")]
    quinn: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Rakan {
    #[serde(rename = "type")]
    rakan_type: GroupEnum,
    format: String,
    version: String,
    data: RakanData,
}

#[derive(Serialize, Deserialize)]
pub struct RakanData {
    #[serde(rename = "Rakan")]
    rakan: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Rammus {
    #[serde(rename = "type")]
    rammus_type: GroupEnum,
    format: String,
    version: String,
    data: RammusData,
}

#[derive(Serialize, Deserialize)]
pub struct RammusData {
    #[serde(rename = "Rammus")]
    rammus: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct RekSai {
    #[serde(rename = "type")]
    rek_sai_type: GroupEnum,
    format: String,
    version: String,
    data: RekSaiData,
}

#[derive(Serialize, Deserialize)]
pub struct RekSaiData {
    #[serde(rename = "RekSai")]
    rek_sai: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Renekton {
    #[serde(rename = "type")]
    renekton_type: GroupEnum,
    format: String,
    version: String,
    data: RenektonData,
}

#[derive(Serialize, Deserialize)]
pub struct RenektonData {
    #[serde(rename = "Renekton")]
    renekton: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Rengar {
    #[serde(rename = "type")]
    rengar_type: GroupEnum,
    format: String,
    version: String,
    data: RengarData,
}

#[derive(Serialize, Deserialize)]
pub struct RengarData {
    #[serde(rename = "Rengar")]
    rengar: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Riven {
    #[serde(rename = "type")]
    riven_type: GroupEnum,
    format: String,
    version: String,
    data: RivenData,
}

#[derive(Serialize, Deserialize)]
pub struct RivenData {
    #[serde(rename = "Riven")]
    riven: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Rumble {
    #[serde(rename = "type")]
    rumble_type: GroupEnum,
    format: String,
    version: String,
    data: RumbleData,
}

#[derive(Serialize, Deserialize)]
pub struct RumbleData {
    #[serde(rename = "Rumble")]
    rumble: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Ryze {
    #[serde(rename = "type")]
    ryze_type: GroupEnum,
    format: String,
    version: String,
    data: RyzeData,
}

#[derive(Serialize, Deserialize)]
pub struct RyzeData {
    #[serde(rename = "Ryze")]
    ryze: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Sejuani {
    #[serde(rename = "type")]
    sejuani_type: GroupEnum,
    format: String,
    version: String,
    data: SejuaniData,
}

#[derive(Serialize, Deserialize)]
pub struct SejuaniData {
    #[serde(rename = "Sejuani")]
    sejuani: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Senna {
    #[serde(rename = "type")]
    senna_type: GroupEnum,
    format: String,
    version: String,
    data: SennaData,
}

#[derive(Serialize, Deserialize)]
pub struct SennaData {
    #[serde(rename = "Senna")]
    senna: QiyanaClass,
}

#[derive(Serialize, Deserialize)]
pub struct Shaco {
    #[serde(rename = "type")]
    shaco_type: GroupEnum,
    format: String,
    version: String,
    data: ShacoData,
}

#[derive(Serialize, Deserialize)]
pub struct ShacoData {
    #[serde(rename = "Shaco")]
    shaco: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Shen {
    #[serde(rename = "type")]
    shen_type: GroupEnum,
    format: String,
    version: String,
    data: ShenData,
}

#[derive(Serialize, Deserialize)]
pub struct ShenData {
    #[serde(rename = "Shen")]
    shen: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Shyvana {
    #[serde(rename = "type")]
    shyvana_type: GroupEnum,
    format: String,
    version: String,
    data: ShyvanaData,
}

#[derive(Serialize, Deserialize)]
pub struct ShyvanaData {
    #[serde(rename = "Shyvana")]
    shyvana: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Singed {
    #[serde(rename = "type")]
    singed_type: GroupEnum,
    format: String,
    version: String,
    data: SingedData,
}

#[derive(Serialize, Deserialize)]
pub struct SingedData {
    #[serde(rename = "Singed")]
    singed: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Sion {
    #[serde(rename = "type")]
    sion_type: GroupEnum,
    format: String,
    version: String,
    data: SionData,
}

#[derive(Serialize, Deserialize)]
pub struct SionData {
    #[serde(rename = "Sion")]
    sion: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Sivir {
    #[serde(rename = "type")]
    sivir_type: GroupEnum,
    format: String,
    version: String,
    data: SivirData,
}

#[derive(Serialize, Deserialize)]
pub struct SivirData {
    #[serde(rename = "Sivir")]
    sivir: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Skarner {
    #[serde(rename = "type")]
    skarner_type: GroupEnum,
    format: String,
    version: String,
    data: SkarnerData,
}

#[derive(Serialize, Deserialize)]
pub struct SkarnerData {
    #[serde(rename = "Skarner")]
    skarner: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Sona {
    #[serde(rename = "type")]
    sona_type: GroupEnum,
    format: String,
    version: String,
    data: SonaData,
}

#[derive(Serialize, Deserialize)]
pub struct SonaData {
    #[serde(rename = "Sona")]
    sona: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Soraka {
    #[serde(rename = "type")]
    soraka_type: GroupEnum,
    format: String,
    version: String,
    data: SorakaData,
}

#[derive(Serialize, Deserialize)]
pub struct SorakaData {
    #[serde(rename = "Soraka")]
    soraka: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Swain {
    #[serde(rename = "type")]
    swain_type: GroupEnum,
    format: String,
    version: String,
    data: SwainData,
}

#[derive(Serialize, Deserialize)]
pub struct SwainData {
    #[serde(rename = "Swain")]
    swain: BrandClass,
}

#[derive(Serialize, Deserialize)]
pub struct Sylas {
    #[serde(rename = "type")]
    sylas_type: GroupEnum,
    format: String,
    version: String,
    data: SylasData,
}

#[derive(Serialize, Deserialize)]
pub struct SylasData {
    #[serde(rename = "Sylas")]
    sylas: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Syndra {
    #[serde(rename = "type")]
    syndra_type: GroupEnum,
    format: String,
    version: String,
    data: SyndraData,
}

#[derive(Serialize, Deserialize)]
pub struct SyndraData {
    #[serde(rename = "Syndra")]
    syndra: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct TahmKench {
    #[serde(rename = "type")]
    tahm_kench_type: GroupEnum,
    format: String,
    version: String,
    data: TahmKenchData,
}

#[derive(Serialize, Deserialize)]
pub struct TahmKenchData {
    #[serde(rename = "TahmKench")]
    tahm_kench: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Taliyah {
    #[serde(rename = "type")]
    taliyah_type: GroupEnum,
    format: String,
    version: String,
    data: TaliyahData,
}

#[derive(Serialize, Deserialize)]
pub struct TaliyahData {
    #[serde(rename = "Taliyah")]
    taliyah: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Talon {
    #[serde(rename = "type")]
    talon_type: GroupEnum,
    format: String,
    version: String,
    data: TalonData,
}

#[derive(Serialize, Deserialize)]
pub struct TalonData {
    #[serde(rename = "Talon")]
    talon: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Taric {
    #[serde(rename = "type")]
    taric_type: GroupEnum,
    format: String,
    version: String,
    data: TaricData,
}

#[derive(Serialize, Deserialize)]
pub struct TaricData {
    #[serde(rename = "Taric")]
    taric: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Teemo {
    #[serde(rename = "type")]
    teemo_type: GroupEnum,
    format: String,
    version: String,
    data: TeemoData,
}

#[derive(Serialize, Deserialize)]
pub struct TeemoData {
    #[serde(rename = "Teemo")]
    teemo: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Thresh {
    #[serde(rename = "type")]
    thresh_type: GroupEnum,
    format: String,
    version: String,
    data: ThreshData,
}

#[derive(Serialize, Deserialize)]
pub struct ThreshData {
    #[serde(rename = "Thresh")]
    thresh: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Tristana {
    #[serde(rename = "type")]
    tristana_type: GroupEnum,
    format: String,
    version: String,
    data: TristanaData,
}

#[derive(Serialize, Deserialize)]
pub struct TristanaData {
    #[serde(rename = "Tristana")]
    tristana: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Trundle {
    #[serde(rename = "type")]
    trundle_type: GroupEnum,
    format: String,
    version: String,
    data: TrundleData,
}

#[derive(Serialize, Deserialize)]
pub struct TrundleData {
    #[serde(rename = "Trundle")]
    trundle: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Tryndamere {
    #[serde(rename = "type")]
    tryndamere_type: GroupEnum,
    format: String,
    version: String,
    data: TryndamereData,
}

#[derive(Serialize, Deserialize)]
pub struct TryndamereData {
    #[serde(rename = "Tryndamere")]
    tryndamere: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct TwistedFate {
    #[serde(rename = "type")]
    twisted_fate_type: GroupEnum,
    format: String,
    version: String,
    data: TwistedFateData,
}

#[derive(Serialize, Deserialize)]
pub struct TwistedFateData {
    #[serde(rename = "TwistedFate")]
    twisted_fate: BrandClass,
}

#[derive(Serialize, Deserialize)]
pub struct Twitch {
    #[serde(rename = "type")]
    twitch_type: GroupEnum,
    format: String,
    version: String,
    data: TwitchData,
}

#[derive(Serialize, Deserialize)]
pub struct TwitchData {
    #[serde(rename = "Twitch")]
    twitch: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Udyr {
    #[serde(rename = "type")]
    udyr_type: GroupEnum,
    format: String,
    version: String,
    data: UdyrData,
}

#[derive(Serialize, Deserialize)]
pub struct UdyrData {
    #[serde(rename = "Udyr")]
    udyr: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Urgot {
    #[serde(rename = "type")]
    urgot_type: GroupEnum,
    format: String,
    version: String,
    data: UrgotData,
}

#[derive(Serialize, Deserialize)]
pub struct UrgotData {
    #[serde(rename = "Urgot")]
    urgot: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Varus {
    #[serde(rename = "type")]
    varus_type: GroupEnum,
    format: String,
    version: String,
    data: VarusData,
}

#[derive(Serialize, Deserialize)]
pub struct VarusData {
    #[serde(rename = "Varus")]
    varus: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Vayne {
    #[serde(rename = "type")]
    vayne_type: GroupEnum,
    format: String,
    version: String,
    data: VayneData,
}

#[derive(Serialize, Deserialize)]
pub struct VayneData {
    #[serde(rename = "Vayne")]
    vayne: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Veigar {
    #[serde(rename = "type")]
    veigar_type: GroupEnum,
    format: String,
    version: String,
    data: VeigarData,
}

#[derive(Serialize, Deserialize)]
pub struct VeigarData {
    #[serde(rename = "Veigar")]
    veigar: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Velkoz {
    #[serde(rename = "type")]
    velkoz_type: GroupEnum,
    format: String,
    version: String,
    data: VelkozData,
}

#[derive(Serialize, Deserialize)]
pub struct VelkozData {
    #[serde(rename = "Velkoz")]
    velkoz: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Vi {
    #[serde(rename = "type")]
    vi_type: GroupEnum,
    format: String,
    version: String,
    data: ViData,
}

#[derive(Serialize, Deserialize)]
pub struct ViData {
    #[serde(rename = "Vi")]
    vi: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Viktor {
    #[serde(rename = "type")]
    viktor_type: GroupEnum,
    format: String,
    version: String,
    data: ViktorData,
}

#[derive(Serialize, Deserialize)]
pub struct ViktorData {
    #[serde(rename = "Viktor")]
    viktor: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Vladimir {
    #[serde(rename = "type")]
    vladimir_type: GroupEnum,
    format: String,
    version: String,
    data: VladimirData,
}

#[derive(Serialize, Deserialize)]
pub struct VladimirData {
    #[serde(rename = "Vladimir")]
    vladimir: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Volibear {
    #[serde(rename = "type")]
    volibear_type: GroupEnum,
    format: String,
    version: String,
    data: VolibearData,
}

#[derive(Serialize, Deserialize)]
pub struct VolibearData {
    #[serde(rename = "Volibear")]
    volibear: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Warwick {
    #[serde(rename = "type")]
    warwick_type: GroupEnum,
    format: String,
    version: String,
    data: WarwickData,
}

#[derive(Serialize, Deserialize)]
pub struct WarwickData {
    #[serde(rename = "Warwick")]
    warwick: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Xayah {
    #[serde(rename = "type")]
    xayah_type: GroupEnum,
    format: String,
    version: String,
    data: XayahData,
}

#[derive(Serialize, Deserialize)]
pub struct XayahData {
    #[serde(rename = "Xayah")]
    xayah: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Xerath {
    #[serde(rename = "type")]
    xerath_type: GroupEnum,
    format: String,
    version: String,
    data: XerathData,
}

#[derive(Serialize, Deserialize)]
pub struct XerathData {
    #[serde(rename = "Xerath")]
    xerath: PurpleAnivia,
}

#[derive(Serialize, Deserialize)]
pub struct XinZhao {
    #[serde(rename = "type")]
    xin_zhao_type: GroupEnum,
    format: String,
    version: String,
    data: XinZhaoData,
}

#[derive(Serialize, Deserialize)]
pub struct XinZhaoData {
    #[serde(rename = "XinZhao")]
    xin_zhao: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Yasuo {
    #[serde(rename = "type")]
    yasuo_type: GroupEnum,
    format: String,
    version: String,
    data: YasuoData,
}

#[derive(Serialize, Deserialize)]
pub struct YasuoData {
    #[serde(rename = "Yasuo")]
    yasuo: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Yorick {
    #[serde(rename = "type")]
    yorick_type: GroupEnum,
    format: String,
    version: String,
    data: YorickData,
}

#[derive(Serialize, Deserialize)]
pub struct YorickData {
    #[serde(rename = "Yorick")]
    yorick: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Yuumi {
    #[serde(rename = "type")]
    yuumi_type: GroupEnum,
    format: String,
    version: String,
    data: YuumiData,
}

#[derive(Serialize, Deserialize)]
pub struct YuumiData {
    #[serde(rename = "Yuumi")]
    yuumi: QiyanaClass,
}

#[derive(Serialize, Deserialize)]
pub struct Zac {
    #[serde(rename = "type")]
    zac_type: GroupEnum,
    format: String,
    version: String,
    data: ZacData,
}

#[derive(Serialize, Deserialize)]
pub struct ZacData {
    #[serde(rename = "Zac")]
    zac: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Zed {
    #[serde(rename = "type")]
    zed_type: GroupEnum,
    format: String,
    version: String,
    data: ZedData,
}

#[derive(Serialize, Deserialize)]
pub struct ZedData {
    #[serde(rename = "Zed")]
    zed: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Ziggs {
    #[serde(rename = "type")]
    ziggs_type: GroupEnum,
    format: String,
    version: String,
    data: ZiggsData,
}

#[derive(Serialize, Deserialize)]
pub struct ZiggsData {
    #[serde(rename = "Ziggs")]
    ziggs: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Zilean {
    #[serde(rename = "type")]
    zilean_type: GroupEnum,
    format: String,
    version: String,
    data: ZileanData,
}

#[derive(Serialize, Deserialize)]
pub struct ZileanData {
    #[serde(rename = "Zilean")]
    zilean: PurpleAnivia,
}

#[derive(Serialize, Deserialize)]
pub struct Zoe {
    #[serde(rename = "type")]
    zoe_type: GroupEnum,
    format: String,
    version: String,
    data: ZoeData,
}

#[derive(Serialize, Deserialize)]
pub struct ZoeData {
    #[serde(rename = "Zoe")]
    zoe: ZoeClass,
}

#[derive(Serialize, Deserialize)]
pub struct ZoeClass {
    id: FluffyId,
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
    partype: Partype,
    info: Info,
    stats: HashMap<String, f64>,
    spells: Vec<Spell>,
    passive: Passive,
    recommended: Vec<QiyanaRecommended>,
}

#[derive(Serialize, Deserialize)]
pub struct Zyra {
    #[serde(rename = "type")]
    zyra_type: GroupEnum,
    format: String,
    version: String,
    data: ZyraData,
}

#[derive(Serialize, Deserialize)]
pub struct ZyraData {
    #[serde(rename = "Zyra")]
    zyra: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Coeff {
    Double(f64),
    DoubleArray(Vec<f64>),
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
    #[serde(rename = "champion1.png")]
    Champion1Png,
    #[serde(rename = "champion2.png")]
    Champion2Png,
    #[serde(rename = "champion3.png")]
    Champion3Png,
    #[serde(rename = "champion4.png")]
    Champion4Png,
    #[serde(rename = "passive0.png")]
    Passive0Png,
    #[serde(rename = "passive1.png")]
    Passive1Png,
    #[serde(rename = "passive2.png")]
    Passive2Png,
    #[serde(rename = "passive3.png")]
    Passive3Png,
    #[serde(rename = "passive4.png")]
    Passive4Png,
    #[serde(rename = "spell0.png")]
    Spell0Png,
    #[serde(rename = "spell10.png")]
    Spell10Png,
    #[serde(rename = "spell11.png")]
    Spell11Png,
    #[serde(rename = "spell12.png")]
    Spell12Png,
    #[serde(rename = "spell13.png")]
    Spell13Png,
    #[serde(rename = "spell14.png")]
    Spell14Png,
    #[serde(rename = "spell1.png")]
    Spell1Png,
    #[serde(rename = "spell2.png")]
    Spell2Png,
    #[serde(rename = "spell3.png")]
    Spell3Png,
    #[serde(rename = "spell4.png")]
    Spell4Png,
    #[serde(rename = "spell5.png")]
    Spell5Png,
    #[serde(rename = "spell6.png")]
    Spell6Png,
    #[serde(rename = "spell7.png")]
    Spell7Png,
    #[serde(rename = "spell8.png")]
    Spell8Png,
    #[serde(rename = "spell9.png")]
    Spell9Png,
}

#[derive(Serialize, Deserialize)]
pub enum BlockType {
    #[serde(rename = "ability_scaling")]
    AbilityScaling,
    #[serde(rename = "aggressive")]
    Aggressive,
    #[serde(rename = "agressive")]
    Agressive,
    #[serde(rename = "beginner_advanced")]
    BeginnerAdvanced,
    #[serde(rename = "beginner_legendary")]
    BeginnerLegendary,
    #[serde(rename = "beginner_LegendaryItem")]
    BeginnerLegendaryItem,
    #[serde(rename = "beginner_legendaryitem")]
    BeginnerLegendaryitem,
    #[serde(rename = "beginner_MoreLegendaryItems")]
    BeginnerMoreLegendaryItems,
    #[serde(rename = "beginner_morelegendaryitems")]
    BeginnerMorelegendaryitems,
    #[serde(rename = "beginner_MovementSpeed")]
    BeginnerMovementSpeed,
    #[serde(rename = "beginner_movementspeed")]
    BeginnerMovementspeed,
    #[serde(rename = "beginner_movespeed")]
    BeginnerMovespeed,
    #[serde(rename = "beginner_starter")]
    BeginnerStarter,
    #[serde(rename = "champspecific")]
    Champspecific,
    #[serde(rename = "consumable")]
    Consumable,
    #[serde(rename = "consumables")]
    Consumables,
    #[serde(rename = "consumablesjungle")]
    Consumablesjungle,
    #[serde(rename = "defensive")]
    Defensive,
    #[serde(rename = "defensivejungle")]
    Defensivejungle,
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
    #[serde(rename = "kingporosnax")]
    Kingporosnax,
    #[serde(rename = "npe1")]
    Npe1,
    #[serde(rename = "npe2")]
    Npe2,
    #[serde(rename = "npe3")]
    Npe3,
    #[serde(rename = "npe4")]
    Npe4,
    #[serde(rename = "odyjinx1")]
    Odyjinx1,
    #[serde(rename = "odyjinx2")]
    Odyjinx2,
    #[serde(rename = "odyjinx3")]
    Odyjinx3,
    #[serde(rename = "odymalphite1")]
    Odymalphite1,
    #[serde(rename = "odymalphite2")]
    Odymalphite2,
    #[serde(rename = "odymalphite3")]
    Odymalphite3,
    #[serde(rename = "odysona1")]
    Odysona1,
    #[serde(rename = "odysona2")]
    Odysona2,
    #[serde(rename = "odysona3")]
    Odysona3,
    #[serde(rename = "odyyasuo1")]
    Odyyasuo1,
    #[serde(rename = "odyyasuo2")]
    Odyyasuo2,
    #[serde(rename = "odyyasuo3")]
    Odyyasuo3,
    #[serde(rename = "odyziggs1")]
    Odyziggs1,
    #[serde(rename = "odyziggs2")]
    Odyziggs2,
    #[serde(rename = "odyziggs3")]
    Odyziggs3,
    #[serde(rename = "offensive")]
    Offensive,
    #[serde(rename = "offmeta")]
    Offmeta,
    #[serde(rename = "ornnupgrades")]
    Ornnupgrades,
    #[serde(rename = "protective")]
    Protective,
    #[serde(rename = "recommended")]
    Recommended,
    #[serde(rename = "selective")]
    Selective,
    #[serde(rename = "siegeDefense")]
    SiegeDefense,
    #[serde(rename = "siegeOffense")]
    SiegeOffense,
    #[serde(rename = "siegedefense")]
    Siegedefense,
    #[serde(rename = "siegeoffense")]
    Siegeoffense,
    #[serde(rename = "situational")]
    Situational,
    #[serde(rename = "situationaljungle")]
    Situationaljungle,
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "standardjungle")]
    Standardjungle,
    #[serde(rename = "starting")]
    Starting,
    #[serde(rename = "startingjungle")]
    Startingjungle,
    #[serde(rename = "support")]
    Support,
    #[serde(rename = "1)buystarteritems")]
    The1Buystarteritems,
    #[serde(rename = "beginner_Advanced")]
    TypeBeginnerAdvanced,
    #[serde(rename = "beginner_Starter")]
    TypeBeginnerStarter,
}

#[derive(Serialize, Deserialize)]
pub enum Of {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "kaynass")]
    Kaynass,
    #[serde(rename = "kaynslay")]
    Kaynslay,
}

#[derive(Serialize, Deserialize)]
pub enum IfSummonerSpell {
    #[serde(rename = "")]
    Empty,
    ItemSmiteAoE,
    ItemTeleportCancel,
    OdinTrinketRevive,
    #[serde(rename = "S5_SummonerSmiteDuel")]
    S5SummonerSmiteDuel,
    #[serde(rename = "S5_SummonerSmitePlayerGanker")]
    S5SummonerSmitePlayerGanker,
    #[serde(rename = "S5_SummonerSmiteQuick")]
    S5SummonerSmiteQuick,
    SummonerOdinPromote,
    SummonerPoroRecall,
    SummonerPoroThrow,
    SummonerReturn,
    SummonerSiegeChampSelect2,
    SummonerSmite,
    #[serde(rename = "SummonerSnowURFSnowball_Mark")]
    SummonerSnowUrfSnowballMark,
    SummonerSnowball,
    SummonerTeleport,
    TeleportCancel,
}

#[derive(Serialize, Deserialize)]
pub enum Map {
    #[serde(rename = "any")]
    Any,
    CityPark,
    CrystalScar,
    #[serde(rename = "HA")]
    Ha,
    Odyssey,
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
    #[serde(rename = "any")]
    Any,
    #[serde(rename = "ARAM")]
    Aram,
    #[serde(rename = "ASCENSION")]
    Ascension,
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
    #[serde(rename = "ODYSSEY")]
    Odyssey,
    #[serde(rename = "PROJECT")]
    Project,
    #[serde(rename = "SIEGE")]
    Siege,
    #[serde(rename = "STARGUARDIAN")]
    Starguardian,
    #[serde(rename = "TUTORIAL")]
    Tutorial,
    #[serde(rename = "TUTORIAL_MODULE_2")]
    TutorialModule2,
    #[serde(rename = "TUTORIAL_MODULE_3")]
    TutorialModule3,
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
pub enum CostType {
    #[serde(rename = " {{ abilityresourcename }}")]
    Abilityresourcename,
    #[serde(rename = " 魔力、所有充能")]
    Ambitious,
    #[serde(rename = "  個炮臺儲存數 & {{ cost }} 魔力")]
    Cost,
    #[serde(rename = " 能量")]
    CostType,
    #[serde(rename = "1 顆種子")]
    CostType1,
    #[serde(rename = "% 最大生命、{{ cost }} 魔力")]
    CostTypeCost,
    #[serde(rename = "被動")]
    Cunning,
    #[serde(rename = "無消耗")]
    Empty,
    #[serde(rename = " 生命")]
    Fluffy,
    #[serde(rename = "% 最大生命")]
    Frisky,
    #[serde(rename = " 點憤怒值 / 每秒")]
    Hilarious,
    #[serde(rename = " 熱能")]
    Indecent,
    #[serde(rename = "消耗  生命")]
    Indigo,
    #[serde(rename = "% 目前生命")]
    Magenta,
    #[serde(rename = "&nbsp;")]
    Nbsp,
    #[serde(rename = " 魔力")]
    Purple,
    #[serde(rename = "")]
    Sticky,
    #[serde(rename = "% 當前生命")]
    Tentacled,
    #[serde(rename = "產生 1 點殘虐值")]
    The1,
}

#[derive(Serialize, Deserialize)]
pub enum Key {
    #[serde(rename = "a1")]
    A1,
    #[serde(rename = "a2")]
    A2,
    #[serde(rename = "f1")]
    F1,
    #[serde(rename = "f2")]
    F2,
    #[serde(rename = "f3")]
    F3,
    #[serde(rename = "f4")]
    F4,
}

#[derive(Serialize, Deserialize)]
pub enum Link {
    #[serde(rename = "armor")]
    Armor,
    #[serde(rename = "attackdamage")]
    Attackdamage,
    #[serde(rename = "bonusarmor")]
    Bonusarmor,
    #[serde(rename = "bonusattackdamage")]
    Bonusattackdamage,
    #[serde(rename = "bonushealth")]
    Bonushealth,
    #[serde(rename = "bonusspellblock")]
    Bonusspellblock,
    #[serde(rename = "health")]
    Health,
    #[serde(rename = "@special.BraumWArmor")]
    SpecialBraumWArmor,
    #[serde(rename = "@special.BraumWMR")]
    SpecialBraumWmr,
    #[serde(rename = "@special.jaxrarmor")]
    SpecialJaxrarmor,
    #[serde(rename = "@special.jaxrmr")]
    SpecialJaxrmr,
    #[serde(rename = "@special.nautilusq")]
    SpecialNautilusq,
    #[serde(rename = "@special.viw")]
    SpecialViw,
    #[serde(rename = "spelldamage")]
    Spelldamage,
    #[serde(rename = "@stacks")]
    Stacks,
    #[serde(rename = "@text")]
    Text,
}

#[derive(Serialize, Deserialize)]
pub enum Tag {
    Assassin,
    Fighter,
    Mage,
    Marksman,
    Support,
    Tank,
}

#[derive(Serialize, Deserialize)]
pub enum Partype {
    #[serde(rename = "魔力")]
    Empty,
    #[serde(rename = "能量")]
    Partype,
    #[serde(rename = "怒氣")]
    Purple,
}

#[derive(Serialize, Deserialize)]
pub enum PurpleId {
    Anivia,
    Annie,
    AurelionSol,
    Cassiopeia,
    Kassadin,
    Maokai,
    Orianna,
    Xerath,
    Zilean,
}

#[derive(Serialize, Deserialize)]
pub enum BrandId {
    Brand,
    Swain,
    TwistedFate,
}

#[derive(Serialize, Deserialize)]
pub enum FluffyId {
    Qiyana,
    Senna,
    Yuumi,
    Zoe,
}
