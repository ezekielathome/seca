use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::value::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct LicenceSlot {
    key: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Licence {
    title: String,
    slots: HashMap<String, LicenceSlot>,
    shader: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Character {
    id: i32,
    name: String,
    released: bool,
    protons: i32,
    tokens: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeaponSkin {
    tool: i32,
    skin: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterSkin {
    character: i32,
    skin: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Career {
    #[serde(rename = "Protons")]
    protons: Option<i32>,
    #[serde(rename = "Level")]
    level: Option<i32>,
    #[serde(rename = "Titles")]
    titles: Option<i32>,
    #[serde(rename = "Trial")]
    trial: CareerTrail,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CareerTrail {
    #[serde(rename = "Bronze")]
    bronze: Option<i32>,
    #[serde(rename = "Silver")]
    silver: Option<i32>,
    #[serde(rename = "Gold")]
    gold: Option<i32>,
    #[serde(rename = "Platinum")]
    platinum: Option<i32>,
}

/// Stats is a partial struct, i didn't want to complete it as it's alot of work
/// and the struct itself is very inconsistent.
/// i'd rather wait for the api to be reworked / stabilised...
#[derive(Debug, Serialize, Deserialize)]
pub struct Stats {
    #[serde(rename = "portalID")]
    portal_id: i32,
    #[serde(rename = "doubleXPEnd")]
    double_xp_end: u64,
    kills: i64,
    deaths: i64,
    headshots: i64,
    assists: i64,
    #[serde(rename = "selectedCharacterSkin")]
    selected_character_skin: i64,
    #[serde(rename = "aegisPlays")]
    aegis_plays: i32,
    #[serde(rename = "helixPlays")]
    helix_plays: i32,
    #[serde(rename = "secondsPlayed")]
    seconds_played: i32,
    xp: i32,
    level: i32,
    protons: i64,
    #[serde(rename = "bronzeTrials")]
    bronze_trials: i32,
    #[serde(rename = "silverTrials")]
    silver_trials: i32,
    #[serde(rename = "goldTrials")]
    gold_trials: i32,
    supporter: bool,
    titlesbeta: Vec<String>,
    licence: Licence,
    stats: Value,
    crosshairs: Vec<String>,
    #[serde(rename = "weaponSkins")]
    weapon_skins: Vec<WeaponSkin>,
    characters: Vec<Character>,
    #[serde(rename = "characterSkins")]
    character_skins: Vec<CharacterSkin>,
    #[serde(rename = "twitchAccess")]
    twitch_access: bool,
    career: Career,
}
