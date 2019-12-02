
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/elder_dragon_quicktype.py
*/














use serde::{Serialize, Deserialize};
extern crate serde_json;
use self::serde_json::Error;

pub fn serialize(json: &str) -> Result<Aatrox,Error>{
    serde_json::from_str(json)
}

use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Aatrox {
    #[serde(rename = "type")]
    pub aatrox_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: AatroxData,
}

#[derive(Serialize, Deserialize)]
pub struct AatroxData {
    #[serde(rename = "Aatrox")]
    pub aatrox: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct PuneHedgehog {
    pub id: String,
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
    pub recommended: Vec<PurpleRecommended>,
}

#[derive(Serialize, Deserialize)]
pub struct Image {
    pub full: String,
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
pub struct PurpleRecommended {
    pub champion: String,
    pub title: String,
    pub map: Map,
    pub mode: Mode,
    #[serde(rename = "type")]
    pub recommended_type: RecommendedType,
    #[serde(rename = "customTag")]
    pub custom_tag: String,
    pub sortrank: Option<i64>,
    #[serde(rename = "extensionPage")]
    pub extension_page: Option<bool>,
    #[serde(rename = "useObviousCheckmark")]
    pub use_obvious_checkmark: Option<bool>,
    #[serde(rename = "customPanel")]
    pub custom_panel: Option<serde_json::Value>,
    pub blocks: Vec<Block>,
    #[serde(rename = "requiredPerk")]
    pub required_perk: Option<String>,
    #[serde(rename = "extenOrnnPage")]
    pub exten_ornn_page: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct Block {
    #[serde(rename = "type")]
    pub block_type: BlockType,
    #[serde(rename = "recMath")]
    pub rec_math: Option<bool>,
    #[serde(rename = "recSteps")]
    pub rec_steps: Option<bool>,
    #[serde(rename = "minSummonerLevel")]
    pub min_summoner_level: Option<i64>,
    #[serde(rename = "maxSummonerLevel")]
    pub max_summoner_level: Option<i64>,
    #[serde(rename = "showIfSummonerSpell")]
    pub show_if_summoner_spell: Option<IfSummonerSpell>,
    #[serde(rename = "hideIfSummonerSpell")]
    pub hide_if_summoner_spell: Option<IfSummonerSpell>,
    #[serde(rename = "appendAfterSection")]
    pub append_after_section: Option<String>,
    #[serde(rename = "visibleWithAllOf")]
    pub visible_with_all_of: Option<Vec<Of>>,
    #[serde(rename = "hiddenWithAnyOf")]
    pub hidden_with_any_of: Option<Vec<Of>>,
    pub items: Vec<Item>,
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
    pub id: String,
    pub name: String,
    pub description: String,
    pub tooltip: String,
    pub leveltip: Option<Leveltip>,
    pub maxrank: i64,
    pub cooldown: Vec<f64>,
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
    pub cost_type: CostType,
    pub maxammo: String,
    pub range: Vec<i64>,
    #[serde(rename = "rangeBurn")]
    pub range_burn: String,
    pub image: Image,
    pub resource: Option<String>,
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
    pub coeff: Coeff,
    pub key: Key,
}

#[derive(Serialize, Deserialize)]
pub struct Ahri {
    #[serde(rename = "type")]
    pub ahri_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: AhriData,
}

#[derive(Serialize, Deserialize)]
pub struct AhriData {
    #[serde(rename = "Ahri")]
    pub ahri: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct HammerfestPonies {
    pub id: String,
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
    pub partype: Partype,
    pub info: Info,
    pub stats: HashMap<String, f64>,
    pub spells: Vec<Spell>,
    pub passive: Passive,
    pub recommended: Vec<FluffyRecommended>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyRecommended {
    pub champion: String,
    pub title: String,
    pub map: Map,
    pub mode: Mode,
    #[serde(rename = "type")]
    pub recommended_type: RecommendedType,
    #[serde(rename = "customTag")]
    pub custom_tag: Option<String>,
    pub sortrank: Option<i64>,
    #[serde(rename = "extensionPage")]
    pub extension_page: Option<bool>,
    #[serde(rename = "customPanel")]
    pub custom_panel: Option<serde_json::Value>,
    pub blocks: Vec<Block>,
    #[serde(rename = "requiredPerk")]
    pub required_perk: Option<String>,
    #[serde(rename = "useObviousCheckmark")]
    pub use_obvious_checkmark: Option<bool>,
    pub priority: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct Akali {
    #[serde(rename = "type")]
    pub akali_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: AkaliData,
}

#[derive(Serialize, Deserialize)]
pub struct AkaliData {
    #[serde(rename = "Akali")]
    pub akali: AkaliClass,
}

#[derive(Serialize, Deserialize)]
pub struct AkaliClass {
    pub id: String,
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
    pub partype: Partype,
    pub info: Info,
    pub stats: HashMap<String, f64>,
    pub spells: Vec<Spell>,
    pub passive: Passive,
    pub recommended: Vec<AkaliRecommended>,
}

#[derive(Serialize, Deserialize)]
pub struct AkaliRecommended {
    pub champion: String,
    pub title: String,
    pub map: Map,
    pub mode: Mode,
    #[serde(rename = "type")]
    pub recommended_type: RecommendedType,
    #[serde(rename = "customTag")]
    pub custom_tag: String,
    #[serde(rename = "requiredPerk")]
    pub required_perk: Option<String>,
    pub sortrank: Option<i64>,
    #[serde(rename = "extensionPage")]
    pub extension_page: Option<bool>,
    #[serde(rename = "customPanel")]
    pub custom_panel: Option<String>,
    pub blocks: Vec<Block>,
    #[serde(rename = "useObviousCheckmark")]
    pub use_obvious_checkmark: Option<bool>,
    #[serde(rename = "customPanelCurrencyType")]
    pub custom_panel_currency_type: Option<String>,
    #[serde(rename = "customPanelBuffCurrencyName")]
    pub custom_panel_buff_currency_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Alistar {
    #[serde(rename = "type")]
    pub alistar_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: AlistarData,
}

#[derive(Serialize, Deserialize)]
pub struct AlistarData {
    #[serde(rename = "Alistar")]
    pub alistar: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Amumu {
    #[serde(rename = "type")]
    pub amumu_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: AmumuData,
}

#[derive(Serialize, Deserialize)]
pub struct AmumuData {
    #[serde(rename = "Amumu")]
    pub amumu: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Anivia {
    #[serde(rename = "type")]
    pub anivia_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: AniviaData,
}

#[derive(Serialize, Deserialize)]
pub struct AniviaData {
    #[serde(rename = "Anivia")]
    pub anivia: PurpleAnivia,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleAnivia {
    pub id: PurpleId,
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
    pub partype: Partype,
    pub info: Info,
    pub stats: HashMap<String, f64>,
    pub spells: Vec<Spell>,
    pub passive: Passive,
    pub recommended: Vec<AniviaRecommended>,
}

#[derive(Serialize, Deserialize)]
pub struct AniviaRecommended {
    pub champion: PurpleId,
    pub title: String,
    pub map: Map,
    pub mode: Mode,
    #[serde(rename = "type")]
    pub recommended_type: RecommendedType,
    #[serde(rename = "customTag")]
    pub custom_tag: String,
    pub sortrank: Option<i64>,
    #[serde(rename = "extensionPage")]
    pub extension_page: bool,
    #[serde(rename = "customPanel")]
    pub custom_panel: Option<serde_json::Value>,
    pub blocks: Vec<Block>,
    #[serde(rename = "useObviousCheckmark")]
    pub use_obvious_checkmark: Option<bool>,
    #[serde(rename = "requiredPerk")]
    pub required_perk: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Annie {
    #[serde(rename = "type")]
    pub annie_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: AnnieData,
}

#[derive(Serialize, Deserialize)]
pub struct AnnieData {
    #[serde(rename = "Annie")]
    pub annie: PurpleAnivia,
}

#[derive(Serialize, Deserialize)]
pub struct Ashe {
    #[serde(rename = "type")]
    pub ashe_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: AsheData,
}

#[derive(Serialize, Deserialize)]
pub struct AsheData {
    #[serde(rename = "Ashe")]
    pub ashe: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct AurelionSol {
    #[serde(rename = "type")]
    pub aurelion_sol_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: AurelionSolData,
}

#[derive(Serialize, Deserialize)]
pub struct AurelionSolData {
    #[serde(rename = "AurelionSol")]
    pub aurelion_sol: PurpleAnivia,
}

#[derive(Serialize, Deserialize)]
pub struct Azir {
    #[serde(rename = "type")]
    pub azir_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: AzirData,
}

#[derive(Serialize, Deserialize)]
pub struct AzirData {
    #[serde(rename = "Azir")]
    pub azir: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Bard {
    #[serde(rename = "type")]
    pub bard_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: BardData,
}

#[derive(Serialize, Deserialize)]
pub struct BardData {
    #[serde(rename = "Bard")]
    pub bard: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Blitzcrank {
    #[serde(rename = "type")]
    pub blitzcrank_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: BlitzcrankData,
}

#[derive(Serialize, Deserialize)]
pub struct BlitzcrankData {
    #[serde(rename = "Blitzcrank")]
    pub blitzcrank: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Brand {
    #[serde(rename = "type")]
    pub brand_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: BrandData,
}

#[derive(Serialize, Deserialize)]
pub struct BrandData {
    #[serde(rename = "Brand")]
    pub brand: BrandClass,
}

#[derive(Serialize, Deserialize)]
pub struct BrandClass {
    pub id: BrandId,
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
    pub partype: Partype,
    pub info: Info,
    pub stats: HashMap<String, f64>,
    pub spells: Vec<Spell>,
    pub passive: Passive,
    pub recommended: Vec<BrandRecommended>,
}

#[derive(Serialize, Deserialize)]
pub struct BrandRecommended {
    pub champion: BrandId,
    pub title: String,
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
pub struct Braum {
    #[serde(rename = "type")]
    pub braum_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: BraumData,
}

#[derive(Serialize, Deserialize)]
pub struct BraumData {
    #[serde(rename = "Braum")]
    pub braum: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Caitlyn {
    #[serde(rename = "type")]
    pub caitlyn_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: CaitlynData,
}

#[derive(Serialize, Deserialize)]
pub struct CaitlynData {
    #[serde(rename = "Caitlyn")]
    pub caitlyn: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Camille {
    #[serde(rename = "type")]
    pub camille_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: CamilleData,
}

#[derive(Serialize, Deserialize)]
pub struct CamilleData {
    #[serde(rename = "Camille")]
    pub camille: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Cassiopeia {
    #[serde(rename = "type")]
    pub cassiopeia_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: CassiopeiaData,
}

#[derive(Serialize, Deserialize)]
pub struct CassiopeiaData {
    #[serde(rename = "Cassiopeia")]
    pub cassiopeia: PurpleAnivia,
}

#[derive(Serialize, Deserialize)]
pub struct Chogath {
    #[serde(rename = "type")]
    pub chogath_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: ChogathData,
}

#[derive(Serialize, Deserialize)]
pub struct ChogathData {
    #[serde(rename = "Chogath")]
    pub chogath: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Corki {
    #[serde(rename = "type")]
    pub corki_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: CorkiData,
}

#[derive(Serialize, Deserialize)]
pub struct CorkiData {
    #[serde(rename = "Corki")]
    pub corki: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Darius {
    #[serde(rename = "type")]
    pub darius_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: DariusData,
}

#[derive(Serialize, Deserialize)]
pub struct DariusData {
    #[serde(rename = "Darius")]
    pub darius: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Diana {
    #[serde(rename = "type")]
    pub diana_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: DianaData,
}

#[derive(Serialize, Deserialize)]
pub struct DianaData {
    #[serde(rename = "Diana")]
    pub diana: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Draven {
    #[serde(rename = "type")]
    pub draven_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: DravenData,
}

#[derive(Serialize, Deserialize)]
pub struct DravenData {
    #[serde(rename = "Draven")]
    pub draven: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct DrMundo {
    #[serde(rename = "type")]
    pub dr_mundo_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: DrMundoData,
}

#[derive(Serialize, Deserialize)]
pub struct DrMundoData {
    #[serde(rename = "DrMundo")]
    pub dr_mundo: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Ekko {
    #[serde(rename = "type")]
    pub ekko_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: EkkoData,
}

#[derive(Serialize, Deserialize)]
pub struct EkkoData {
    #[serde(rename = "Ekko")]
    pub ekko: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Elise {
    #[serde(rename = "type")]
    pub elise_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: EliseData,
}

#[derive(Serialize, Deserialize)]
pub struct EliseData {
    #[serde(rename = "Elise")]
    pub elise: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Evelynn {
    #[serde(rename = "type")]
    pub evelynn_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: EvelynnData,
}

#[derive(Serialize, Deserialize)]
pub struct EvelynnData {
    #[serde(rename = "Evelynn")]
    pub evelynn: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Ezreal {
    #[serde(rename = "type")]
    pub ezreal_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: EzrealData,
}

#[derive(Serialize, Deserialize)]
pub struct EzrealData {
    #[serde(rename = "Ezreal")]
    pub ezreal: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Fiddlesticks {
    #[serde(rename = "type")]
    pub fiddlesticks_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: FiddlesticksData,
}

#[derive(Serialize, Deserialize)]
pub struct FiddlesticksData {
    #[serde(rename = "Fiddlesticks")]
    pub fiddlesticks: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Fiora {
    #[serde(rename = "type")]
    pub fiora_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: FioraData,
}

#[derive(Serialize, Deserialize)]
pub struct FioraData {
    #[serde(rename = "Fiora")]
    pub fiora: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Fizz {
    #[serde(rename = "type")]
    pub fizz_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: FizzData,
}

#[derive(Serialize, Deserialize)]
pub struct FizzData {
    #[serde(rename = "Fizz")]
    pub fizz: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Galio {
    #[serde(rename = "type")]
    pub galio_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: GalioData,
}

#[derive(Serialize, Deserialize)]
pub struct GalioData {
    #[serde(rename = "Galio")]
    pub galio: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Gangplank {
    #[serde(rename = "type")]
    pub gangplank_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: GangplankData,
}

#[derive(Serialize, Deserialize)]
pub struct GangplankData {
    #[serde(rename = "Gangplank")]
    pub gangplank: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Garen {
    #[serde(rename = "type")]
    pub garen_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: GarenData,
}

#[derive(Serialize, Deserialize)]
pub struct GarenData {
    #[serde(rename = "Garen")]
    pub garen: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Gnar {
    #[serde(rename = "type")]
    pub gnar_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: GnarData,
}

#[derive(Serialize, Deserialize)]
pub struct GnarData {
    #[serde(rename = "Gnar")]
    pub gnar: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Gragas {
    #[serde(rename = "type")]
    pub gragas_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: GragasData,
}

#[derive(Serialize, Deserialize)]
pub struct GragasData {
    #[serde(rename = "Gragas")]
    pub gragas: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Graves {
    #[serde(rename = "type")]
    pub graves_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: GravesData,
}

#[derive(Serialize, Deserialize)]
pub struct GravesData {
    #[serde(rename = "Graves")]
    pub graves: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Hecarim {
    #[serde(rename = "type")]
    pub hecarim_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: HecarimData,
}

#[derive(Serialize, Deserialize)]
pub struct HecarimData {
    #[serde(rename = "Hecarim")]
    pub hecarim: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Heimerdinger {
    #[serde(rename = "type")]
    pub heimerdinger_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: HeimerdingerData,
}

#[derive(Serialize, Deserialize)]
pub struct HeimerdingerData {
    #[serde(rename = "Heimerdinger")]
    pub heimerdinger: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Illaoi {
    #[serde(rename = "type")]
    pub illaoi_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: IllaoiData,
}

#[derive(Serialize, Deserialize)]
pub struct IllaoiData {
    #[serde(rename = "Illaoi")]
    pub illaoi: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Irelia {
    #[serde(rename = "type")]
    pub irelia_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: IreliaData,
}

#[derive(Serialize, Deserialize)]
pub struct IreliaData {
    #[serde(rename = "Irelia")]
    pub irelia: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Ivern {
    #[serde(rename = "type")]
    pub ivern_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: IvernData,
}

#[derive(Serialize, Deserialize)]
pub struct IvernData {
    #[serde(rename = "Ivern")]
    pub ivern: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Janna {
    #[serde(rename = "type")]
    pub janna_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: JannaData,
}

#[derive(Serialize, Deserialize)]
pub struct JannaData {
    #[serde(rename = "Janna")]
    pub janna: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct JarvanIv {
    #[serde(rename = "type")]
    pub jarvan_iv_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: JarvanIvData,
}

#[derive(Serialize, Deserialize)]
pub struct JarvanIvData {
    #[serde(rename = "JarvanIV")]
    pub jarvan_iv: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Jax {
    #[serde(rename = "type")]
    pub jax_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: JaxData,
}

#[derive(Serialize, Deserialize)]
pub struct JaxData {
    #[serde(rename = "Jax")]
    pub jax: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Jayce {
    #[serde(rename = "type")]
    pub jayce_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: JayceData,
}

#[derive(Serialize, Deserialize)]
pub struct JayceData {
    #[serde(rename = "Jayce")]
    pub jayce: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Jhin {
    #[serde(rename = "type")]
    pub jhin_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: JhinData,
}

#[derive(Serialize, Deserialize)]
pub struct JhinData {
    #[serde(rename = "Jhin")]
    pub jhin: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Jinx {
    #[serde(rename = "type")]
    pub jinx_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: JinxData,
}

#[derive(Serialize, Deserialize)]
pub struct JinxData {
    #[serde(rename = "Jinx")]
    pub jinx: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Kaisa {
    #[serde(rename = "type")]
    pub kaisa_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: KaisaData,
}

#[derive(Serialize, Deserialize)]
pub struct KaisaData {
    #[serde(rename = "Kaisa")]
    pub kaisa: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Kalista {
    #[serde(rename = "type")]
    pub kalista_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: KalistaData,
}

#[derive(Serialize, Deserialize)]
pub struct KalistaData {
    #[serde(rename = "Kalista")]
    pub kalista: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Karma {
    #[serde(rename = "type")]
    pub karma_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: KarmaData,
}

#[derive(Serialize, Deserialize)]
pub struct KarmaData {
    #[serde(rename = "Karma")]
    pub karma: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Karthus {
    #[serde(rename = "type")]
    pub karthus_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: KarthusData,
}

#[derive(Serialize, Deserialize)]
pub struct KarthusData {
    #[serde(rename = "Karthus")]
    pub karthus: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Kassadin {
    #[serde(rename = "type")]
    pub kassadin_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: KassadinData,
}

#[derive(Serialize, Deserialize)]
pub struct KassadinData {
    #[serde(rename = "Kassadin")]
    pub kassadin: PurpleAnivia,
}

#[derive(Serialize, Deserialize)]
pub struct Katarina {
    #[serde(rename = "type")]
    pub katarina_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: KatarinaData,
}

#[derive(Serialize, Deserialize)]
pub struct KatarinaData {
    #[serde(rename = "Katarina")]
    pub katarina: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Kayle {
    #[serde(rename = "type")]
    pub kayle_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: KayleData,
}

#[derive(Serialize, Deserialize)]
pub struct KayleData {
    #[serde(rename = "Kayle")]
    pub kayle: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Kayn {
    #[serde(rename = "type")]
    pub kayn_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: KaynData,
}

#[derive(Serialize, Deserialize)]
pub struct KaynData {
    #[serde(rename = "Kayn")]
    pub kayn: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Kennen {
    #[serde(rename = "type")]
    pub kennen_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: KennenData,
}

#[derive(Serialize, Deserialize)]
pub struct KennenData {
    #[serde(rename = "Kennen")]
    pub kennen: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Khazix {
    #[serde(rename = "type")]
    pub khazix_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: KhazixData,
}

#[derive(Serialize, Deserialize)]
pub struct KhazixData {
    #[serde(rename = "Khazix")]
    pub khazix: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Kindred {
    #[serde(rename = "type")]
    pub kindred_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: KindredData,
}

#[derive(Serialize, Deserialize)]
pub struct KindredData {
    #[serde(rename = "Kindred")]
    pub kindred: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Kled {
    #[serde(rename = "type")]
    pub kled_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: KledData,
}

#[derive(Serialize, Deserialize)]
pub struct KledData {
    #[serde(rename = "Kled")]
    pub kled: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct KogMaw {
    #[serde(rename = "type")]
    pub kog_maw_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: KogMawData,
}

#[derive(Serialize, Deserialize)]
pub struct KogMawData {
    #[serde(rename = "KogMaw")]
    pub kog_maw: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Leblanc {
    #[serde(rename = "type")]
    pub leblanc_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: LeblancData,
}

#[derive(Serialize, Deserialize)]
pub struct LeblancData {
    #[serde(rename = "Leblanc")]
    pub leblanc: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct LeeSin {
    #[serde(rename = "type")]
    pub lee_sin_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: LeeSinData,
}

#[derive(Serialize, Deserialize)]
pub struct LeeSinData {
    #[serde(rename = "LeeSin")]
    pub lee_sin: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Leona {
    #[serde(rename = "type")]
    pub leona_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: LeonaData,
}

#[derive(Serialize, Deserialize)]
pub struct LeonaData {
    #[serde(rename = "Leona")]
    pub leona: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Lissandra {
    #[serde(rename = "type")]
    pub lissandra_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: LissandraData,
}

#[derive(Serialize, Deserialize)]
pub struct LissandraData {
    #[serde(rename = "Lissandra")]
    pub lissandra: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Lucian {
    #[serde(rename = "type")]
    pub lucian_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: LucianData,
}

#[derive(Serialize, Deserialize)]
pub struct LucianData {
    #[serde(rename = "Lucian")]
    pub lucian: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Lulu {
    #[serde(rename = "type")]
    pub lulu_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: LuluData,
}

#[derive(Serialize, Deserialize)]
pub struct LuluData {
    #[serde(rename = "Lulu")]
    pub lulu: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Lux {
    #[serde(rename = "type")]
    pub lux_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: LuxData,
}

#[derive(Serialize, Deserialize)]
pub struct LuxData {
    #[serde(rename = "Lux")]
    pub lux: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Malphite {
    #[serde(rename = "type")]
    pub malphite_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: MalphiteData,
}

#[derive(Serialize, Deserialize)]
pub struct MalphiteData {
    #[serde(rename = "Malphite")]
    pub malphite: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Malzahar {
    #[serde(rename = "type")]
    pub malzahar_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: MalzaharData,
}

#[derive(Serialize, Deserialize)]
pub struct MalzaharData {
    #[serde(rename = "Malzahar")]
    pub malzahar: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Maokai {
    #[serde(rename = "type")]
    pub maokai_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: MaokaiData,
}

#[derive(Serialize, Deserialize)]
pub struct MaokaiData {
    #[serde(rename = "Maokai")]
    pub maokai: PurpleAnivia,
}

#[derive(Serialize, Deserialize)]
pub struct MasterYi {
    #[serde(rename = "type")]
    pub master_yi_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: MasterYiData,
}

#[derive(Serialize, Deserialize)]
pub struct MasterYiData {
    #[serde(rename = "MasterYi")]
    pub master_yi: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct MissFortune {
    #[serde(rename = "type")]
    pub miss_fortune_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: MissFortuneData,
}

#[derive(Serialize, Deserialize)]
pub struct MissFortuneData {
    #[serde(rename = "MissFortune")]
    pub miss_fortune: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct MonkeyKing {
    #[serde(rename = "type")]
    pub monkey_king_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: MonkeyKingData,
}

#[derive(Serialize, Deserialize)]
pub struct MonkeyKingData {
    #[serde(rename = "MonkeyKing")]
    pub monkey_king: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Mordekaiser {
    #[serde(rename = "type")]
    pub mordekaiser_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: MordekaiserData,
}

#[derive(Serialize, Deserialize)]
pub struct MordekaiserData {
    #[serde(rename = "Mordekaiser")]
    pub mordekaiser: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Morgana {
    #[serde(rename = "type")]
    pub morgana_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: MorganaData,
}

#[derive(Serialize, Deserialize)]
pub struct MorganaData {
    #[serde(rename = "Morgana")]
    pub morgana: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Nami {
    #[serde(rename = "type")]
    pub nami_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: NamiData,
}

#[derive(Serialize, Deserialize)]
pub struct NamiData {
    #[serde(rename = "Nami")]
    pub nami: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Nasus {
    #[serde(rename = "type")]
    pub nasus_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: NasusData,
}

#[derive(Serialize, Deserialize)]
pub struct NasusData {
    #[serde(rename = "Nasus")]
    pub nasus: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Nautilus {
    #[serde(rename = "type")]
    pub nautilus_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: NautilusData,
}

#[derive(Serialize, Deserialize)]
pub struct NautilusData {
    #[serde(rename = "Nautilus")]
    pub nautilus: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Neeko {
    #[serde(rename = "type")]
    pub neeko_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: NeekoData,
}

#[derive(Serialize, Deserialize)]
pub struct NeekoData {
    #[serde(rename = "Neeko")]
    pub neeko: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Nidalee {
    #[serde(rename = "type")]
    pub nidalee_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: NidaleeData,
}

#[derive(Serialize, Deserialize)]
pub struct NidaleeData {
    #[serde(rename = "Nidalee")]
    pub nidalee: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Nocturne {
    #[serde(rename = "type")]
    pub nocturne_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: NocturneData,
}

#[derive(Serialize, Deserialize)]
pub struct NocturneData {
    #[serde(rename = "Nocturne")]
    pub nocturne: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Nunu {
    #[serde(rename = "type")]
    pub nunu_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: NunuData,
}

#[derive(Serialize, Deserialize)]
pub struct NunuData {
    #[serde(rename = "Nunu")]
    pub nunu: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Olaf {
    #[serde(rename = "type")]
    pub olaf_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: OlafData,
}

#[derive(Serialize, Deserialize)]
pub struct OlafData {
    #[serde(rename = "Olaf")]
    pub olaf: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Orianna {
    #[serde(rename = "type")]
    pub orianna_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: OriannaData,
}

#[derive(Serialize, Deserialize)]
pub struct OriannaData {
    #[serde(rename = "Orianna")]
    pub orianna: PurpleAnivia,
}

#[derive(Serialize, Deserialize)]
pub struct Ornn {
    #[serde(rename = "type")]
    pub ornn_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: OrnnData,
}

#[derive(Serialize, Deserialize)]
pub struct OrnnData {
    #[serde(rename = "Ornn")]
    pub ornn: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Pantheon {
    #[serde(rename = "type")]
    pub pantheon_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: PantheonData,
}

#[derive(Serialize, Deserialize)]
pub struct PantheonData {
    #[serde(rename = "Pantheon")]
    pub pantheon: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Poppy {
    #[serde(rename = "type")]
    pub poppy_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: PoppyData,
}

#[derive(Serialize, Deserialize)]
pub struct PoppyData {
    #[serde(rename = "Poppy")]
    pub poppy: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Pyke {
    #[serde(rename = "type")]
    pub pyke_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: PykeData,
}

#[derive(Serialize, Deserialize)]
pub struct PykeData {
    #[serde(rename = "Pyke")]
    pub pyke: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Qiyana {
    #[serde(rename = "type")]
    pub qiyana_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: QiyanaData,
}

#[derive(Serialize, Deserialize)]
pub struct QiyanaData {
    #[serde(rename = "Qiyana")]
    pub qiyana: QiyanaClass,
}

#[derive(Serialize, Deserialize)]
pub struct QiyanaClass {
    pub id: FluffyId,
    pub key: String,
    pub name: String,
    pub title: String,
    pub image: Image,
    pub skins: Vec<Skin>,
    pub lore: String,
    pub blurb: String,
    pub allytips: Vec<Option<serde_json::Value>>,
    pub enemytips: Vec<Option<serde_json::Value>>,
    pub tags: Vec<Tag>,
    pub partype: Partype,
    pub info: Info,
    pub stats: HashMap<String, f64>,
    pub spells: Vec<Spell>,
    pub passive: Passive,
    pub recommended: Vec<QiyanaRecommended>,
}

#[derive(Serialize, Deserialize)]
pub struct QiyanaRecommended {
    pub champion: FluffyId,
    pub title: String,
    pub map: String,
    pub mode: Mode,
    #[serde(rename = "type")]
    pub recommended_type: RecommendedType,
    #[serde(rename = "customTag")]
    pub custom_tag: String,
    pub sortrank: i64,
    #[serde(rename = "extensionPage")]
    pub extension_page: bool,
    #[serde(rename = "useObviousCheckmark")]
    pub use_obvious_checkmark: Option<bool>,
    #[serde(rename = "customPanel")]
    pub custom_panel: Option<serde_json::Value>,
    pub blocks: Vec<Block>,
}

#[derive(Serialize, Deserialize)]
pub struct Quinn {
    #[serde(rename = "type")]
    pub quinn_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: QuinnData,
}

#[derive(Serialize, Deserialize)]
pub struct QuinnData {
    #[serde(rename = "Quinn")]
    pub quinn: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Rakan {
    #[serde(rename = "type")]
    pub rakan_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: RakanData,
}

#[derive(Serialize, Deserialize)]
pub struct RakanData {
    #[serde(rename = "Rakan")]
    pub rakan: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Rammus {
    #[serde(rename = "type")]
    pub rammus_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: RammusData,
}

#[derive(Serialize, Deserialize)]
pub struct RammusData {
    #[serde(rename = "Rammus")]
    pub rammus: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct RekSai {
    #[serde(rename = "type")]
    pub rek_sai_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: RekSaiData,
}

#[derive(Serialize, Deserialize)]
pub struct RekSaiData {
    #[serde(rename = "RekSai")]
    pub rek_sai: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Renekton {
    #[serde(rename = "type")]
    pub renekton_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: RenektonData,
}

#[derive(Serialize, Deserialize)]
pub struct RenektonData {
    #[serde(rename = "Renekton")]
    pub renekton: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Rengar {
    #[serde(rename = "type")]
    pub rengar_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: RengarData,
}

#[derive(Serialize, Deserialize)]
pub struct RengarData {
    #[serde(rename = "Rengar")]
    pub rengar: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Riven {
    #[serde(rename = "type")]
    pub riven_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: RivenData,
}

#[derive(Serialize, Deserialize)]
pub struct RivenData {
    #[serde(rename = "Riven")]
    pub riven: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Rumble {
    #[serde(rename = "type")]
    pub rumble_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: RumbleData,
}

#[derive(Serialize, Deserialize)]
pub struct RumbleData {
    #[serde(rename = "Rumble")]
    pub rumble: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Ryze {
    #[serde(rename = "type")]
    pub ryze_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: RyzeData,
}

#[derive(Serialize, Deserialize)]
pub struct RyzeData {
    #[serde(rename = "Ryze")]
    pub ryze: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Sejuani {
    #[serde(rename = "type")]
    pub sejuani_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: SejuaniData,
}

#[derive(Serialize, Deserialize)]
pub struct SejuaniData {
    #[serde(rename = "Sejuani")]
    pub sejuani: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Senna {
    #[serde(rename = "type")]
    pub senna_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: SennaData,
}

#[derive(Serialize, Deserialize)]
pub struct SennaData {
    #[serde(rename = "Senna")]
    pub senna: QiyanaClass,
}

#[derive(Serialize, Deserialize)]
pub struct Shaco {
    #[serde(rename = "type")]
    pub shaco_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: ShacoData,
}

#[derive(Serialize, Deserialize)]
pub struct ShacoData {
    #[serde(rename = "Shaco")]
    pub shaco: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Shen {
    #[serde(rename = "type")]
    pub shen_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: ShenData,
}

#[derive(Serialize, Deserialize)]
pub struct ShenData {
    #[serde(rename = "Shen")]
    pub shen: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Shyvana {
    #[serde(rename = "type")]
    pub shyvana_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: ShyvanaData,
}

#[derive(Serialize, Deserialize)]
pub struct ShyvanaData {
    #[serde(rename = "Shyvana")]
    pub shyvana: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Singed {
    #[serde(rename = "type")]
    pub singed_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: SingedData,
}

#[derive(Serialize, Deserialize)]
pub struct SingedData {
    #[serde(rename = "Singed")]
    pub singed: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Sion {
    #[serde(rename = "type")]
    pub sion_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: SionData,
}

#[derive(Serialize, Deserialize)]
pub struct SionData {
    #[serde(rename = "Sion")]
    pub sion: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Sivir {
    #[serde(rename = "type")]
    pub sivir_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: SivirData,
}

#[derive(Serialize, Deserialize)]
pub struct SivirData {
    #[serde(rename = "Sivir")]
    pub sivir: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Skarner {
    #[serde(rename = "type")]
    pub skarner_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: SkarnerData,
}

#[derive(Serialize, Deserialize)]
pub struct SkarnerData {
    #[serde(rename = "Skarner")]
    pub skarner: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Sona {
    #[serde(rename = "type")]
    pub sona_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: SonaData,
}

#[derive(Serialize, Deserialize)]
pub struct SonaData {
    #[serde(rename = "Sona")]
    pub sona: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Soraka {
    #[serde(rename = "type")]
    pub soraka_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: SorakaData,
}

#[derive(Serialize, Deserialize)]
pub struct SorakaData {
    #[serde(rename = "Soraka")]
    pub soraka: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Swain {
    #[serde(rename = "type")]
    pub swain_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: SwainData,
}

#[derive(Serialize, Deserialize)]
pub struct SwainData {
    #[serde(rename = "Swain")]
    pub swain: BrandClass,
}

#[derive(Serialize, Deserialize)]
pub struct Sylas {
    #[serde(rename = "type")]
    pub sylas_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: SylasData,
}

#[derive(Serialize, Deserialize)]
pub struct SylasData {
    #[serde(rename = "Sylas")]
    pub sylas: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Syndra {
    #[serde(rename = "type")]
    pub syndra_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: SyndraData,
}

#[derive(Serialize, Deserialize)]
pub struct SyndraData {
    #[serde(rename = "Syndra")]
    pub syndra: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct TahmKench {
    #[serde(rename = "type")]
    pub tahm_kench_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: TahmKenchData,
}

#[derive(Serialize, Deserialize)]
pub struct TahmKenchData {
    #[serde(rename = "TahmKench")]
    pub tahm_kench: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Taliyah {
    #[serde(rename = "type")]
    pub taliyah_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: TaliyahData,
}

#[derive(Serialize, Deserialize)]
pub struct TaliyahData {
    #[serde(rename = "Taliyah")]
    pub taliyah: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Talon {
    #[serde(rename = "type")]
    pub talon_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: TalonData,
}

#[derive(Serialize, Deserialize)]
pub struct TalonData {
    #[serde(rename = "Talon")]
    pub talon: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Taric {
    #[serde(rename = "type")]
    pub taric_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: TaricData,
}

#[derive(Serialize, Deserialize)]
pub struct TaricData {
    #[serde(rename = "Taric")]
    pub taric: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Teemo {
    #[serde(rename = "type")]
    pub teemo_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: TeemoData,
}

#[derive(Serialize, Deserialize)]
pub struct TeemoData {
    #[serde(rename = "Teemo")]
    pub teemo: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Thresh {
    #[serde(rename = "type")]
    pub thresh_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: ThreshData,
}

#[derive(Serialize, Deserialize)]
pub struct ThreshData {
    #[serde(rename = "Thresh")]
    pub thresh: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Tristana {
    #[serde(rename = "type")]
    pub tristana_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: TristanaData,
}

#[derive(Serialize, Deserialize)]
pub struct TristanaData {
    #[serde(rename = "Tristana")]
    pub tristana: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Trundle {
    #[serde(rename = "type")]
    pub trundle_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: TrundleData,
}

#[derive(Serialize, Deserialize)]
pub struct TrundleData {
    #[serde(rename = "Trundle")]
    pub trundle: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Tryndamere {
    #[serde(rename = "type")]
    pub tryndamere_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: TryndamereData,
}

#[derive(Serialize, Deserialize)]
pub struct TryndamereData {
    #[serde(rename = "Tryndamere")]
    pub tryndamere: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct TwistedFate {
    #[serde(rename = "type")]
    pub twisted_fate_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: TwistedFateData,
}

#[derive(Serialize, Deserialize)]
pub struct TwistedFateData {
    #[serde(rename = "TwistedFate")]
    pub twisted_fate: BrandClass,
}

#[derive(Serialize, Deserialize)]
pub struct Twitch {
    #[serde(rename = "type")]
    pub twitch_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: TwitchData,
}

#[derive(Serialize, Deserialize)]
pub struct TwitchData {
    #[serde(rename = "Twitch")]
    pub twitch: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Udyr {
    #[serde(rename = "type")]
    pub udyr_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: UdyrData,
}

#[derive(Serialize, Deserialize)]
pub struct UdyrData {
    #[serde(rename = "Udyr")]
    pub udyr: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Urgot {
    #[serde(rename = "type")]
    pub urgot_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: UrgotData,
}

#[derive(Serialize, Deserialize)]
pub struct UrgotData {
    #[serde(rename = "Urgot")]
    pub urgot: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Varus {
    #[serde(rename = "type")]
    pub varus_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: VarusData,
}

#[derive(Serialize, Deserialize)]
pub struct VarusData {
    #[serde(rename = "Varus")]
    pub varus: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Vayne {
    #[serde(rename = "type")]
    pub vayne_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: VayneData,
}

#[derive(Serialize, Deserialize)]
pub struct VayneData {
    #[serde(rename = "Vayne")]
    pub vayne: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Veigar {
    #[serde(rename = "type")]
    pub veigar_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: VeigarData,
}

#[derive(Serialize, Deserialize)]
pub struct VeigarData {
    #[serde(rename = "Veigar")]
    pub veigar: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Velkoz {
    #[serde(rename = "type")]
    pub velkoz_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: VelkozData,
}

#[derive(Serialize, Deserialize)]
pub struct VelkozData {
    #[serde(rename = "Velkoz")]
    pub velkoz: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Vi {
    #[serde(rename = "type")]
    pub vi_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: ViData,
}

#[derive(Serialize, Deserialize)]
pub struct ViData {
    #[serde(rename = "Vi")]
    pub vi: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Viktor {
    #[serde(rename = "type")]
    pub viktor_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: ViktorData,
}

#[derive(Serialize, Deserialize)]
pub struct ViktorData {
    #[serde(rename = "Viktor")]
    pub viktor: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct Vladimir {
    #[serde(rename = "type")]
    pub vladimir_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: VladimirData,
}

#[derive(Serialize, Deserialize)]
pub struct VladimirData {
    #[serde(rename = "Vladimir")]
    pub vladimir: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Volibear {
    #[serde(rename = "type")]
    pub volibear_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: VolibearData,
}

#[derive(Serialize, Deserialize)]
pub struct VolibearData {
    #[serde(rename = "Volibear")]
    pub volibear: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Warwick {
    #[serde(rename = "type")]
    pub warwick_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: WarwickData,
}

#[derive(Serialize, Deserialize)]
pub struct WarwickData {
    #[serde(rename = "Warwick")]
    pub warwick: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Xayah {
    #[serde(rename = "type")]
    pub xayah_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: XayahData,
}

#[derive(Serialize, Deserialize)]
pub struct XayahData {
    #[serde(rename = "Xayah")]
    pub xayah: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Xerath {
    #[serde(rename = "type")]
    pub xerath_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: XerathData,
}

#[derive(Serialize, Deserialize)]
pub struct XerathData {
    #[serde(rename = "Xerath")]
    pub xerath: PurpleAnivia,
}

#[derive(Serialize, Deserialize)]
pub struct XinZhao {
    #[serde(rename = "type")]
    pub xin_zhao_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: XinZhaoData,
}

#[derive(Serialize, Deserialize)]
pub struct XinZhaoData {
    #[serde(rename = "XinZhao")]
    pub xin_zhao: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Yasuo {
    #[serde(rename = "type")]
    pub yasuo_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: YasuoData,
}

#[derive(Serialize, Deserialize)]
pub struct YasuoData {
    #[serde(rename = "Yasuo")]
    pub yasuo: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Yorick {
    #[serde(rename = "type")]
    pub yorick_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: YorickData,
}

#[derive(Serialize, Deserialize)]
pub struct YorickData {
    #[serde(rename = "Yorick")]
    pub yorick: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Yuumi {
    #[serde(rename = "type")]
    pub yuumi_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: YuumiData,
}

#[derive(Serialize, Deserialize)]
pub struct YuumiData {
    #[serde(rename = "Yuumi")]
    pub yuumi: QiyanaClass,
}

#[derive(Serialize, Deserialize)]
pub struct Zac {
    #[serde(rename = "type")]
    pub zac_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: ZacData,
}

#[derive(Serialize, Deserialize)]
pub struct ZacData {
    #[serde(rename = "Zac")]
    pub zac: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Zed {
    #[serde(rename = "type")]
    pub zed_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: ZedData,
}

#[derive(Serialize, Deserialize)]
pub struct ZedData {
    #[serde(rename = "Zed")]
    pub zed: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Ziggs {
    #[serde(rename = "type")]
    pub ziggs_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: ZiggsData,
}

#[derive(Serialize, Deserialize)]
pub struct ZiggsData {
    #[serde(rename = "Ziggs")]
    pub ziggs: PuneHedgehog,
}

#[derive(Serialize, Deserialize)]
pub struct Zilean {
    #[serde(rename = "type")]
    pub zilean_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: ZileanData,
}

#[derive(Serialize, Deserialize)]
pub struct ZileanData {
    #[serde(rename = "Zilean")]
    pub zilean: PurpleAnivia,
}

#[derive(Serialize, Deserialize)]
pub struct Zoe {
    #[serde(rename = "type")]
    pub zoe_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: ZoeData,
}

#[derive(Serialize, Deserialize)]
pub struct ZoeData {
    #[serde(rename = "Zoe")]
    pub zoe: ZoeClass,
}

#[derive(Serialize, Deserialize)]
pub struct ZoeClass {
    pub id: FluffyId,
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
    pub partype: Partype,
    pub info: Info,
    pub stats: HashMap<String, f64>,
    pub spells: Vec<Spell>,
    pub passive: Passive,
    pub recommended: Vec<QiyanaRecommended>,
}

#[derive(Serialize, Deserialize)]
pub struct Zyra {
    #[serde(rename = "type")]
    pub zyra_type: GroupEnum,
    pub format: String,
    pub version: String,
    pub data: ZyraData,
}

#[derive(Serialize, Deserialize)]
pub struct ZyraData {
    #[serde(rename = "Zyra")]
    pub zyra: HammerfestPonies,
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
