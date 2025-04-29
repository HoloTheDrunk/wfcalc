use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Upgrade {
    pub available_challenges: Vec<AvailableChallenge>,
    pub base_drain: u32,
    pub codex_secret: bool,
    pub compat_name: String,
    pub description: Description,
    pub exclude_from_codex: bool,
    pub export_avionics: Option<Box<Upgrade>>,

    pub name: String,
    pub polarity: Polarity,
    pub rarity: Rarity,
    pub fusion_limit: u32,
    pub r#type: Type,
    pub level_stats: Vec<LevelStat>,
}

#[derive(Debug, Deserialize)]
pub enum Polarity {
    #[serde(rename(deserialize = "AP_ATTACK"))]
    Madurai,
    #[serde(rename(deserialize = "AP_DEFENSE"))]
    Vazarin,
    #[serde(rename(deserialize = "AP_POWER"))]
    Naramon,
    #[serde(rename(deserialize = "AP_PRECEPT"))]
    Precept,
    #[serde(rename(deserialize = "AP_TACTIC"))]
    Tactic,
    #[serde(rename(deserialize = "AP_UMBRA"))]
    Umbra,
    #[serde(rename(deserialize = "AP_UNIVERSAL"), alias = "AP_ANY")]
    Universal,
    #[serde(rename(deserialize = "AP_WARD"))]
    Aura,
    #[serde(other)]
    Error,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Legendary,
    #[serde(other)]
    Error,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Description {
    String(String),
    AoString(Vec<String>),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE")]
pub enum Type {
    ArchGun,
    ArchMelee,
    Archwing,
    Aura,
    #[serde(rename(deserialize = "HELMINTH CHARGER"))]
    HelminthCharger,
    Kavat,
    Kubrow,
    Melee,
    Parazon,
    Primary,
    Secondary,
    Sentinel,
    Stance,
    Warframe,
    #[serde(rename(deserialize = "---"))]
    Unspecified,
    #[serde(other)]
    Error,
}

#[derive(Debug, Deserialize)]
pub struct LevelStat {
    pub stats: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct AvailableChallenge {
    pub full_name: String,
    pub description: String,
    pub complications: Vec<Complication>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Complication {
    pub full_name: String,
    pub description: String,
}
