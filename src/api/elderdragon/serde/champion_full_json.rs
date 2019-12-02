
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/elder_dragon_quicktype.py
*/














use serde::{Serialize, Deserialize};
extern crate serde_json;
use self::serde_json::Error;

pub fn serialize(json: &str) -> Result<ChampionFullJson,Error>{
    serde_json::from_str(json)
}

use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct ChampionFullJson {
    #[serde(rename = "type")]
    pub champion_full_json_type: GroupEnum,
    pub format: Format,
    pub version: Version,
    pub data: Data,
    pub keys: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "Aatrox")]
    pub aatrox: PuneHedgehog,
    #[serde(rename = "Ahri")]
    pub ahri: HammerfestPonies,
    #[serde(rename = "Akali")]
    pub akali: Akali,
    #[serde(rename = "Alistar")]
    pub alistar: HammerfestPonies,
    #[serde(rename = "Amumu")]
    pub amumu: HammerfestPonies,
    #[serde(rename = "Anivia")]
    pub anivia: Anivia,
    #[serde(rename = "Annie")]
    pub annie: Anivia,
    #[serde(rename = "Ashe")]
    pub ashe: HammerfestPonies,
    #[serde(rename = "AurelionSol")]
    pub aurelion_sol: Anivia,
    #[serde(rename = "Azir")]
    pub azir: PuneHedgehog,
    #[serde(rename = "Bard")]
    pub bard: PuneHedgehog,
    #[serde(rename = "Blitzcrank")]
    pub blitzcrank: PuneHedgehog,
    #[serde(rename = "Brand")]
    pub brand: Brand,
    #[serde(rename = "Braum")]
    pub braum: PuneHedgehog,
    #[serde(rename = "Caitlyn")]
    pub caitlyn: HammerfestPonies,
    #[serde(rename = "Camille")]
    pub camille: PuneHedgehog,
    #[serde(rename = "Cassiopeia")]
    pub cassiopeia: Anivia,
    #[serde(rename = "Chogath")]
    pub chogath: HammerfestPonies,
    #[serde(rename = "Corki")]
    pub corki: PuneHedgehog,
    #[serde(rename = "Darius")]
    pub darius: PuneHedgehog,
    #[serde(rename = "Diana")]
    pub diana: HammerfestPonies,
    #[serde(rename = "Draven")]
    pub draven: HammerfestPonies,
    #[serde(rename = "DrMundo")]
    pub dr_mundo: PuneHedgehog,
    #[serde(rename = "Ekko")]
    pub ekko: PuneHedgehog,
    #[serde(rename = "Elise")]
    pub elise: PuneHedgehog,
    #[serde(rename = "Evelynn")]
    pub evelynn: PuneHedgehog,
    #[serde(rename = "Ezreal")]
    pub ezreal: PuneHedgehog,
    #[serde(rename = "Fiddlesticks")]
    pub fiddlesticks: PuneHedgehog,
    #[serde(rename = "Fiora")]
    pub fiora: PuneHedgehog,
    #[serde(rename = "Fizz")]
    pub fizz: HammerfestPonies,
    #[serde(rename = "Galio")]
    pub galio: PuneHedgehog,
    #[serde(rename = "Gangplank")]
    pub gangplank: PuneHedgehog,
    #[serde(rename = "Garen")]
    pub garen: PuneHedgehog,
    #[serde(rename = "Gnar")]
    pub gnar: PuneHedgehog,
    #[serde(rename = "Gragas")]
    pub gragas: HammerfestPonies,
    #[serde(rename = "Graves")]
    pub graves: HammerfestPonies,
    #[serde(rename = "Hecarim")]
    pub hecarim: HammerfestPonies,
    #[serde(rename = "Heimerdinger")]
    pub heimerdinger: PuneHedgehog,
    #[serde(rename = "Illaoi")]
    pub illaoi: PuneHedgehog,
    #[serde(rename = "Irelia")]
    pub irelia: PuneHedgehog,
    #[serde(rename = "Ivern")]
    pub ivern: PuneHedgehog,
    #[serde(rename = "Janna")]
    pub janna: HammerfestPonies,
    #[serde(rename = "JarvanIV")]
    pub jarvan_iv: HammerfestPonies,
    #[serde(rename = "Jax")]
    pub jax: HammerfestPonies,
    #[serde(rename = "Jayce")]
    pub jayce: PuneHedgehog,
    #[serde(rename = "Jhin")]
    pub jhin: PuneHedgehog,
    #[serde(rename = "Jinx")]
    pub jinx: PuneHedgehog,
    #[serde(rename = "Kaisa")]
    pub kaisa: PuneHedgehog,
    #[serde(rename = "Kalista")]
    pub kalista: PuneHedgehog,
    #[serde(rename = "Karma")]
    pub karma: PuneHedgehog,
    #[serde(rename = "Karthus")]
    pub karthus: HammerfestPonies,
    #[serde(rename = "Kassadin")]
    pub kassadin: Anivia,
    #[serde(rename = "Katarina")]
    pub katarina: PuneHedgehog,
    #[serde(rename = "Kayle")]
    pub kayle: PuneHedgehog,
    #[serde(rename = "Kayn")]
    pub kayn: PuneHedgehog,
    #[serde(rename = "Kennen")]
    pub kennen: PuneHedgehog,
    #[serde(rename = "Khazix")]
    pub khazix: PuneHedgehog,
    #[serde(rename = "Kindred")]
    pub kindred: PuneHedgehog,
    #[serde(rename = "Kled")]
    pub kled: PuneHedgehog,
    #[serde(rename = "KogMaw")]
    pub kog_maw: PuneHedgehog,
    #[serde(rename = "Leblanc")]
    pub leblanc: PuneHedgehog,
    #[serde(rename = "LeeSin")]
    pub lee_sin: PuneHedgehog,
    #[serde(rename = "Leona")]
    pub leona: PuneHedgehog,
    #[serde(rename = "Lissandra")]
    pub lissandra: PuneHedgehog,
    #[serde(rename = "Lucian")]
    pub lucian: PuneHedgehog,
    #[serde(rename = "Lulu")]
    pub lulu: HammerfestPonies,
    #[serde(rename = "Lux")]
    pub lux: HammerfestPonies,
    #[serde(rename = "Malphite")]
    pub malphite: PuneHedgehog,
    #[serde(rename = "Malzahar")]
    pub malzahar: PuneHedgehog,
    #[serde(rename = "Maokai")]
    pub maokai: Anivia,
    #[serde(rename = "MasterYi")]
    pub master_yi: PuneHedgehog,
    #[serde(rename = "MissFortune")]
    pub miss_fortune: PuneHedgehog,
    #[serde(rename = "MonkeyKing")]
    pub monkey_king: PuneHedgehog,
    #[serde(rename = "Mordekaiser")]
    pub mordekaiser: PuneHedgehog,
    #[serde(rename = "Morgana")]
    pub morgana: PuneHedgehog,
    #[serde(rename = "Nami")]
    pub nami: HammerfestPonies,
    #[serde(rename = "Nasus")]
    pub nasus: PuneHedgehog,
    #[serde(rename = "Nautilus")]
    pub nautilus: PuneHedgehog,
    #[serde(rename = "Neeko")]
    pub neeko: PuneHedgehog,
    #[serde(rename = "Nidalee")]
    pub nidalee: PuneHedgehog,
    #[serde(rename = "Nocturne")]
    pub nocturne: PuneHedgehog,
    #[serde(rename = "Nunu")]
    pub nunu: HammerfestPonies,
    #[serde(rename = "Olaf")]
    pub olaf: PuneHedgehog,
    #[serde(rename = "Orianna")]
    pub orianna: Anivia,
    #[serde(rename = "Ornn")]
    pub ornn: PuneHedgehog,
    #[serde(rename = "Pantheon")]
    pub pantheon: HammerfestPonies,
    #[serde(rename = "Poppy")]
    pub poppy: PuneHedgehog,
    #[serde(rename = "Pyke")]
    pub pyke: PuneHedgehog,
    #[serde(rename = "Qiyana")]
    pub qiyana: Qiyana,
    #[serde(rename = "Quinn")]
    pub quinn: PuneHedgehog,
    #[serde(rename = "Rakan")]
    pub rakan: PuneHedgehog,
    #[serde(rename = "Rammus")]
    pub rammus: PuneHedgehog,
    #[serde(rename = "RekSai")]
    pub rek_sai: PuneHedgehog,
    #[serde(rename = "Renekton")]
    pub renekton: PuneHedgehog,
    #[serde(rename = "Rengar")]
    pub rengar: PuneHedgehog,
    #[serde(rename = "Riven")]
    pub riven: PuneHedgehog,
    #[serde(rename = "Rumble")]
    pub rumble: PuneHedgehog,
    #[serde(rename = "Ryze")]
    pub ryze: PuneHedgehog,
    #[serde(rename = "Sejuani")]
    pub sejuani: PuneHedgehog,
    #[serde(rename = "Senna")]
    pub senna: Qiyana,
    #[serde(rename = "Shaco")]
    pub shaco: PuneHedgehog,
    #[serde(rename = "Shen")]
    pub shen: HammerfestPonies,
    #[serde(rename = "Shyvana")]
    pub shyvana: PuneHedgehog,
    #[serde(rename = "Singed")]
    pub singed: PuneHedgehog,
    #[serde(rename = "Sion")]
    pub sion: PuneHedgehog,
    #[serde(rename = "Sivir")]
    pub sivir: PuneHedgehog,
    #[serde(rename = "Skarner")]
    pub skarner: PuneHedgehog,
    #[serde(rename = "Sona")]
    pub sona: HammerfestPonies,
    #[serde(rename = "Soraka")]
    pub soraka: PuneHedgehog,
    #[serde(rename = "Swain")]
    pub swain: Brand,
    #[serde(rename = "Sylas")]
    pub sylas: HammerfestPonies,
    #[serde(rename = "Syndra")]
    pub syndra: HammerfestPonies,
    #[serde(rename = "TahmKench")]
    pub tahm_kench: HammerfestPonies,
    #[serde(rename = "Taliyah")]
    pub taliyah: PuneHedgehog,
    #[serde(rename = "Talon")]
    pub talon: PuneHedgehog,
    #[serde(rename = "Taric")]
    pub taric: PuneHedgehog,
    #[serde(rename = "Teemo")]
    pub teemo: PuneHedgehog,
    #[serde(rename = "Thresh")]
    pub thresh: PuneHedgehog,
    #[serde(rename = "Tristana")]
    pub tristana: PuneHedgehog,
    #[serde(rename = "Trundle")]
    pub trundle: PuneHedgehog,
    #[serde(rename = "Tryndamere")]
    pub tryndamere: HammerfestPonies,
    #[serde(rename = "TwistedFate")]
    pub twisted_fate: Brand,
    #[serde(rename = "Twitch")]
    pub twitch: PuneHedgehog,
    #[serde(rename = "Udyr")]
    pub udyr: PuneHedgehog,
    #[serde(rename = "Urgot")]
    pub urgot: PuneHedgehog,
    #[serde(rename = "Varus")]
    pub varus: HammerfestPonies,
    #[serde(rename = "Vayne")]
    pub vayne: HammerfestPonies,
    #[serde(rename = "Veigar")]
    pub veigar: PuneHedgehog,
    #[serde(rename = "Velkoz")]
    pub velkoz: PuneHedgehog,
    #[serde(rename = "Vi")]
    pub vi: HammerfestPonies,
    #[serde(rename = "Viktor")]
    pub viktor: HammerfestPonies,
    #[serde(rename = "Vladimir")]
    pub vladimir: PuneHedgehog,
    #[serde(rename = "Volibear")]
    pub volibear: PuneHedgehog,
    #[serde(rename = "Warwick")]
    pub warwick: PuneHedgehog,
    #[serde(rename = "Xayah")]
    pub xayah: PuneHedgehog,
    #[serde(rename = "Xerath")]
    pub xerath: Anivia,
    #[serde(rename = "XinZhao")]
    pub xin_zhao: PuneHedgehog,
    #[serde(rename = "Yasuo")]
    pub yasuo: PuneHedgehog,
    #[serde(rename = "Yorick")]
    pub yorick: PuneHedgehog,
    #[serde(rename = "Yuumi")]
    pub yuumi: Qiyana,
    #[serde(rename = "Zac")]
    pub zac: PuneHedgehog,
    #[serde(rename = "Zed")]
    pub zed: PuneHedgehog,
    #[serde(rename = "Ziggs")]
    pub ziggs: PuneHedgehog,
    #[serde(rename = "Zilean")]
    pub zilean: Anivia,
    #[serde(rename = "Zoe")]
    pub zoe: Zoe,
    #[serde(rename = "Zyra")]
    pub zyra: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct PuneHedgehog {
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
    pub cost_type: String,
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
pub struct HammerfestPonies {
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
    pub partype: String,
    pub info: Info,
    pub stats: HashMap<String, f64>,
    pub spells: Vec<Spell>,
    pub passive: Passive,
    pub recommended: Vec<FluffyRecommended>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyRecommended {
    pub champion: FluffyId,
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
    pub id: AkaliId,
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
    pub recommended: Vec<AkaliRecommended>,
}

#[derive(Serialize, Deserialize)]
pub struct AkaliRecommended {
    pub champion: AkaliId,
    pub title: PurpleTitle,
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
pub struct Anivia {
    pub id: TentacledId,
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
    pub champion: TentacledId,
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
pub struct Brand {
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
    pub title: FluffyTitle,
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
pub struct Qiyana {
    pub id: StickyId,
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
    pub champion: StickyId,
    pub title: TentacledTitle,
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
pub struct Zoe {
    pub id: StickyId,
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
pub enum PurpleId {
    Aatrox,
    Azir,
    Bard,
    Blitzcrank,
    Braum,
    Camille,
    Corki,
    Darius,
    DrMundo,
    Ekko,
    Elise,
    Evelynn,
    Ezreal,
    FiddleSticks,
    Fiddlesticks,
    Fiora,
    Galio,
    Gangplank,
    Garen,
    Gnar,
    Heimerdinger,
    Illaoi,
    Irelia,
    Ivern,
    Jayce,
    Jhin,
    Jinx,
    Kaisa,
    Kalista,
    Karma,
    Katarina,
    Kayle,
    Kayn,
    Kennen,
    Khazix,
    Kindred,
    Kled,
    KogMaw,
    LeBlanc,
    Leblanc,
    LeeSin,
    Leona,
    Lissandra,
    Lucian,
    Malphite,
    Malzahar,
    MasterYi,
    MissFortune,
    MonkeyKing,
    Mordekaiser,
    Morgana,
    Nasus,
    Nautilus,
    Neeko,
    Nidalee,
    Nocturne,
    Olaf,
    Ornn,
    Poppy,
    Pyke,
    Quinn,
    Rakan,
    Rammus,
    RekSai,
    Renekton,
    Rengar,
    Riven,
    Rumble,
    Ryze,
    Sejuani,
    Shaco,
    Shyvana,
    Singed,
    Sion,
    Sivir,
    Skarner,
    Soraka,
    Taliyah,
    Talon,
    Taric,
    Teemo,
    Thresh,
    Tristana,
    Trundle,
    Twitch,
    Udyr,
    Urgot,
    Veigar,
    Velkoz,
    Vladimir,
    Volibear,
    Warwick,
    Xayah,
    XinZhao,
    Yasuo,
    Yorick,
    Zac,
    Zed,
    Ziggs,
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
pub enum FluffyId {
    Ahri,
    Alistar,
    Amumu,
    Ashe,
    Caitlyn,
    Chogath,
    Diana,
    Draven,
    Fizz,
    Gragas,
    Graves,
    Hecarim,
    Janna,
    #[serde(rename = "JarvanIV")]
    JarvanIv,
    Jax,
    Karthus,
    Lulu,
    Lux,
    Maokai,
    MissFortune,
    Nami,
    Nunu,
    Pantheon,
    Shen,
    Sona,
    Sylas,
    Syndra,
    TahmKench,
    Tryndamere,
    Varus,
    Vayne,
    Vi,
    Viktor,
    Zyra,
}

#[derive(Serialize, Deserialize)]
pub enum AkaliId {
    Akali,
}

#[derive(Serialize, Deserialize)]
pub enum PurpleTitle {
    #[serde(rename = "AkaliARAM")]
    AkaliAram,
    #[serde(rename = "AkaliCS")]
    AkaliCs,
    #[serde(rename = "AkaliFIRSTBLOOD")]
    AkaliFirstblood,
    #[serde(rename = "AkaliKINGPORO")]
    AkaliKingporo,
    #[serde(rename = "AkaliSIEGE")]
    AkaliSiege,
    #[serde(rename = "AkaliSL")]
    AkaliSl,
    #[serde(rename = "AkaliSR")]
    AkaliSr,
    #[serde(rename = "AkaliTT")]
    AkaliTt,
    Beginner,
}

#[derive(Serialize, Deserialize)]
pub enum TentacledId {
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
pub enum Partype {
    #[serde(rename = "Μάνα")]
    Empty,
    #[serde(rename = "Мана")]
    Fluffy,
    #[serde(rename = "魔力")]
    Indigo,
    #[serde(rename = "Maná")]
    Man,
    Mana,
    #[serde(rename = "Năng Lượng")]
    NngLng,
    #[serde(rename = "マナ")]
    Partype,
    #[serde(rename = "Mană")]
    PartypeMan,
    #[serde(rename = "마나")]
    Purple,
    #[serde(rename = "法力")]
    Sticky,
    #[serde(rename = "มานา")]
    Tentacled,
}

#[derive(Serialize, Deserialize)]
pub enum BrandId {
    Brand,
    Swain,
    TwistedFate,
}

#[derive(Serialize, Deserialize)]
pub enum FluffyTitle {
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
    #[serde(rename = "SwainARAM")]
    SwainAram,
    #[serde(rename = "SwainCS")]
    SwainCs,
    #[serde(rename = "SwainFIRSTBLOOD")]
    SwainFirstblood,
    #[serde(rename = "SwainKINGPORO")]
    SwainKingporo,
    #[serde(rename = "SwainSIEGE")]
    SwainSiege,
    #[serde(rename = "SwainSL")]
    SwainSl,
    #[serde(rename = "SwainSR")]
    SwainSr,
    #[serde(rename = "SwainTT")]
    SwainTt,
    #[serde(rename = "TwistedFateARAM")]
    TwistedFateAram,
    #[serde(rename = "TwistedFateCS")]
    TwistedFateCs,
    #[serde(rename = "TwistedFateFIRSTBLOOD")]
    TwistedFateFirstblood,
    #[serde(rename = "TwistedFateKINGPORO")]
    TwistedFateKingporo,
    #[serde(rename = "TwistedFateSIEGE")]
    TwistedFateSiege,
    #[serde(rename = "TwistedFateSL")]
    TwistedFateSl,
    #[serde(rename = "TwistedFateSR")]
    TwistedFateSr,
    #[serde(rename = "TwistedFateTT")]
    TwistedFateTt,
}

#[derive(Serialize, Deserialize)]
pub enum StickyId {
    Qiyana,
    Senna,
    Yuumi,
    Zoe,
}

#[derive(Serialize, Deserialize)]
pub enum TentacledTitle {
    Beginner,
    #[serde(rename = "QiyanaARAM")]
    QiyanaAram,
    QiyanaBeginner,
    #[serde(rename = "QiyanaSR")]
    QiyanaSr,
    #[serde(rename = "QiyanaTT")]
    QiyanaTt,
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
    #[serde(rename = "YuumiARAM")]
    YuumiAram,
    #[serde(rename = "YuumiSR")]
    YuumiSr,
    #[serde(rename = "YuumiTT")]
    YuumiTt,
    #[serde(rename = "ZoeARAM")]
    ZoeAram,
    #[serde(rename = "ZoeASC")]
    ZoeAsc,
    #[serde(rename = "ZoeDM")]
    ZoeDm,
    #[serde(rename = "ZoeFIRSTBLOOD")]
    ZoeFirstblood,
    #[serde(rename = "ZoePG")]
    ZoePg,
    #[serde(rename = "ZoeSIEGE")]
    ZoeSiege,
    #[serde(rename = "ZoeSL")]
    ZoeSl,
    #[serde(rename = "ZoeSR")]
    ZoeSr,
    #[serde(rename = "ZoeTT")]
    ZoeTt,
}

#[derive(Serialize, Deserialize)]
pub enum Format {
    #[serde(rename = "full")]
    Full,
}

#[derive(Serialize, Deserialize)]
pub enum Version {
    #[serde(rename = "9.23.1")]
    The9231,
}
