
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/elder_dragon_quicktype.py
*/














use serde::{Serialize, Deserialize};
extern crate serde_json;

use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct ChampionFullJson {
    #[serde(rename = "type")]
    champion_full_json_type: GroupEnum,
    format: Format,
    version: Version,
    data: Data,
    keys: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "Aatrox")]
    aatrox: PuneHedgehog,
    #[serde(rename = "Ahri")]
    ahri: HammerfestPonies,
    #[serde(rename = "Akali")]
    akali: Akali,
    #[serde(rename = "Alistar")]
    alistar: HammerfestPonies,
    #[serde(rename = "Amumu")]
    amumu: HammerfestPonies,
    #[serde(rename = "Anivia")]
    anivia: Anivia,
    #[serde(rename = "Annie")]
    annie: Anivia,
    #[serde(rename = "Ashe")]
    ashe: HammerfestPonies,
    #[serde(rename = "AurelionSol")]
    aurelion_sol: Anivia,
    #[serde(rename = "Azir")]
    azir: PuneHedgehog,
    #[serde(rename = "Bard")]
    bard: PuneHedgehog,
    #[serde(rename = "Blitzcrank")]
    blitzcrank: PuneHedgehog,
    #[serde(rename = "Brand")]
    brand: Brand,
    #[serde(rename = "Braum")]
    braum: PuneHedgehog,
    #[serde(rename = "Caitlyn")]
    caitlyn: HammerfestPonies,
    #[serde(rename = "Camille")]
    camille: PuneHedgehog,
    #[serde(rename = "Cassiopeia")]
    cassiopeia: Anivia,
    #[serde(rename = "Chogath")]
    chogath: HammerfestPonies,
    #[serde(rename = "Corki")]
    corki: PuneHedgehog,
    #[serde(rename = "Darius")]
    darius: PuneHedgehog,
    #[serde(rename = "Diana")]
    diana: HammerfestPonies,
    #[serde(rename = "Draven")]
    draven: HammerfestPonies,
    #[serde(rename = "DrMundo")]
    dr_mundo: PuneHedgehog,
    #[serde(rename = "Ekko")]
    ekko: PuneHedgehog,
    #[serde(rename = "Elise")]
    elise: PuneHedgehog,
    #[serde(rename = "Evelynn")]
    evelynn: PuneHedgehog,
    #[serde(rename = "Ezreal")]
    ezreal: PuneHedgehog,
    #[serde(rename = "Fiddlesticks")]
    fiddlesticks: PuneHedgehog,
    #[serde(rename = "Fiora")]
    fiora: PuneHedgehog,
    #[serde(rename = "Fizz")]
    fizz: HammerfestPonies,
    #[serde(rename = "Galio")]
    galio: PuneHedgehog,
    #[serde(rename = "Gangplank")]
    gangplank: PuneHedgehog,
    #[serde(rename = "Garen")]
    garen: PuneHedgehog,
    #[serde(rename = "Gnar")]
    gnar: PuneHedgehog,
    #[serde(rename = "Gragas")]
    gragas: HammerfestPonies,
    #[serde(rename = "Graves")]
    graves: HammerfestPonies,
    #[serde(rename = "Hecarim")]
    hecarim: HammerfestPonies,
    #[serde(rename = "Heimerdinger")]
    heimerdinger: PuneHedgehog,
    #[serde(rename = "Illaoi")]
    illaoi: PuneHedgehog,
    #[serde(rename = "Irelia")]
    irelia: PuneHedgehog,
    #[serde(rename = "Ivern")]
    ivern: PuneHedgehog,
    #[serde(rename = "Janna")]
    janna: HammerfestPonies,
    #[serde(rename = "JarvanIV")]
    jarvan_iv: HammerfestPonies,
    #[serde(rename = "Jax")]
    jax: HammerfestPonies,
    #[serde(rename = "Jayce")]
    jayce: PuneHedgehog,
    #[serde(rename = "Jhin")]
    jhin: PuneHedgehog,
    #[serde(rename = "Jinx")]
    jinx: PuneHedgehog,
    #[serde(rename = "Kaisa")]
    kaisa: PuneHedgehog,
    #[serde(rename = "Kalista")]
    kalista: PuneHedgehog,
    #[serde(rename = "Karma")]
    karma: PuneHedgehog,
    #[serde(rename = "Karthus")]
    karthus: HammerfestPonies,
    #[serde(rename = "Kassadin")]
    kassadin: Anivia,
    #[serde(rename = "Katarina")]
    katarina: PuneHedgehog,
    #[serde(rename = "Kayle")]
    kayle: PuneHedgehog,
    #[serde(rename = "Kayn")]
    kayn: PuneHedgehog,
    #[serde(rename = "Kennen")]
    kennen: PuneHedgehog,
    #[serde(rename = "Khazix")]
    khazix: PuneHedgehog,
    #[serde(rename = "Kindred")]
    kindred: PuneHedgehog,
    #[serde(rename = "Kled")]
    kled: PuneHedgehog,
    #[serde(rename = "KogMaw")]
    kog_maw: PuneHedgehog,
    #[serde(rename = "Leblanc")]
    leblanc: PuneHedgehog,
    #[serde(rename = "LeeSin")]
    lee_sin: PuneHedgehog,
    #[serde(rename = "Leona")]
    leona: PuneHedgehog,
    #[serde(rename = "Lissandra")]
    lissandra: PuneHedgehog,
    #[serde(rename = "Lucian")]
    lucian: PuneHedgehog,
    #[serde(rename = "Lulu")]
    lulu: HammerfestPonies,
    #[serde(rename = "Lux")]
    lux: HammerfestPonies,
    #[serde(rename = "Malphite")]
    malphite: PuneHedgehog,
    #[serde(rename = "Malzahar")]
    malzahar: PuneHedgehog,
    #[serde(rename = "Maokai")]
    maokai: Anivia,
    #[serde(rename = "MasterYi")]
    master_yi: PuneHedgehog,
    #[serde(rename = "MissFortune")]
    miss_fortune: PuneHedgehog,
    #[serde(rename = "MonkeyKing")]
    monkey_king: PuneHedgehog,
    #[serde(rename = "Mordekaiser")]
    mordekaiser: PuneHedgehog,
    #[serde(rename = "Morgana")]
    morgana: PuneHedgehog,
    #[serde(rename = "Nami")]
    nami: HammerfestPonies,
    #[serde(rename = "Nasus")]
    nasus: PuneHedgehog,
    #[serde(rename = "Nautilus")]
    nautilus: PuneHedgehog,
    #[serde(rename = "Neeko")]
    neeko: PuneHedgehog,
    #[serde(rename = "Nidalee")]
    nidalee: PuneHedgehog,
    #[serde(rename = "Nocturne")]
    nocturne: PuneHedgehog,
    #[serde(rename = "Nunu")]
    nunu: HammerfestPonies,
    #[serde(rename = "Olaf")]
    olaf: PuneHedgehog,
    #[serde(rename = "Orianna")]
    orianna: Anivia,
    #[serde(rename = "Ornn")]
    ornn: PuneHedgehog,
    #[serde(rename = "Pantheon")]
    pantheon: HammerfestPonies,
    #[serde(rename = "Poppy")]
    poppy: PuneHedgehog,
    #[serde(rename = "Pyke")]
    pyke: PuneHedgehog,
    #[serde(rename = "Qiyana")]
    qiyana: Qiyana,
    #[serde(rename = "Quinn")]
    quinn: PuneHedgehog,
    #[serde(rename = "Rakan")]
    rakan: PuneHedgehog,
    #[serde(rename = "Rammus")]
    rammus: PuneHedgehog,
    #[serde(rename = "RekSai")]
    rek_sai: PuneHedgehog,
    #[serde(rename = "Renekton")]
    renekton: PuneHedgehog,
    #[serde(rename = "Rengar")]
    rengar: PuneHedgehog,
    #[serde(rename = "Riven")]
    riven: PuneHedgehog,
    #[serde(rename = "Rumble")]
    rumble: PuneHedgehog,
    #[serde(rename = "Ryze")]
    ryze: PuneHedgehog,
    #[serde(rename = "Sejuani")]
    sejuani: PuneHedgehog,
    #[serde(rename = "Senna")]
    senna: Qiyana,
    #[serde(rename = "Shaco")]
    shaco: PuneHedgehog,
    #[serde(rename = "Shen")]
    shen: HammerfestPonies,
    #[serde(rename = "Shyvana")]
    shyvana: PuneHedgehog,
    #[serde(rename = "Singed")]
    singed: PuneHedgehog,
    #[serde(rename = "Sion")]
    sion: PuneHedgehog,
    #[serde(rename = "Sivir")]
    sivir: PuneHedgehog,
    #[serde(rename = "Skarner")]
    skarner: PuneHedgehog,
    #[serde(rename = "Sona")]
    sona: HammerfestPonies,
    #[serde(rename = "Soraka")]
    soraka: PuneHedgehog,
    #[serde(rename = "Swain")]
    swain: Brand,
    #[serde(rename = "Sylas")]
    sylas: HammerfestPonies,
    #[serde(rename = "Syndra")]
    syndra: HammerfestPonies,
    #[serde(rename = "TahmKench")]
    tahm_kench: HammerfestPonies,
    #[serde(rename = "Taliyah")]
    taliyah: PuneHedgehog,
    #[serde(rename = "Talon")]
    talon: PuneHedgehog,
    #[serde(rename = "Taric")]
    taric: PuneHedgehog,
    #[serde(rename = "Teemo")]
    teemo: PuneHedgehog,
    #[serde(rename = "Thresh")]
    thresh: PuneHedgehog,
    #[serde(rename = "Tristana")]
    tristana: PuneHedgehog,
    #[serde(rename = "Trundle")]
    trundle: PuneHedgehog,
    #[serde(rename = "Tryndamere")]
    tryndamere: HammerfestPonies,
    #[serde(rename = "TwistedFate")]
    twisted_fate: Brand,
    #[serde(rename = "Twitch")]
    twitch: PuneHedgehog,
    #[serde(rename = "Udyr")]
    udyr: PuneHedgehog,
    #[serde(rename = "Urgot")]
    urgot: PuneHedgehog,
    #[serde(rename = "Varus")]
    varus: HammerfestPonies,
    #[serde(rename = "Vayne")]
    vayne: HammerfestPonies,
    #[serde(rename = "Veigar")]
    veigar: PuneHedgehog,
    #[serde(rename = "Velkoz")]
    velkoz: PuneHedgehog,
    #[serde(rename = "Vi")]
    vi: HammerfestPonies,
    #[serde(rename = "Viktor")]
    viktor: HammerfestPonies,
    #[serde(rename = "Vladimir")]
    vladimir: PuneHedgehog,
    #[serde(rename = "Volibear")]
    volibear: PuneHedgehog,
    #[serde(rename = "Warwick")]
    warwick: PuneHedgehog,
    #[serde(rename = "Xayah")]
    xayah: PuneHedgehog,
    #[serde(rename = "Xerath")]
    xerath: Anivia,
    #[serde(rename = "XinZhao")]
    xin_zhao: PuneHedgehog,
    #[serde(rename = "Yasuo")]
    yasuo: PuneHedgehog,
    #[serde(rename = "Yorick")]
    yorick: PuneHedgehog,
    #[serde(rename = "Yuumi")]
    yuumi: Qiyana,
    #[serde(rename = "Zac")]
    zac: PuneHedgehog,
    #[serde(rename = "Zed")]
    zed: PuneHedgehog,
    #[serde(rename = "Ziggs")]
    ziggs: PuneHedgehog,
    #[serde(rename = "Zilean")]
    zilean: Anivia,
    #[serde(rename = "Zoe")]
    zoe: Zoe,
    #[serde(rename = "Zyra")]
    zyra: HammerfestPonies,
}

#[derive(Serialize, Deserialize)]
pub struct PuneHedgehog {
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
    cost_type: String,
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
pub struct HammerfestPonies {
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
    partype: String,
    info: Info,
    stats: HashMap<String, f64>,
    spells: Vec<Spell>,
    passive: Passive,
    recommended: Vec<FluffyRecommended>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyRecommended {
    champion: FluffyId,
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
    id: AkaliId,
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
    recommended: Vec<AkaliRecommended>,
}

#[derive(Serialize, Deserialize)]
pub struct AkaliRecommended {
    champion: AkaliId,
    title: PurpleTitle,
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
pub struct Anivia {
    id: TentacledId,
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
    champion: TentacledId,
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
pub struct Brand {
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
    title: FluffyTitle,
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
pub struct Qiyana {
    id: StickyId,
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
    champion: StickyId,
    title: TentacledTitle,
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
pub struct Zoe {
    id: StickyId,
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
