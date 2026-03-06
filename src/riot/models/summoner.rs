use serde::{Deserialize, Serialize};

// V1 Summoner Data (for summoner name) (uses EUROPE endpoint???)
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SummonerData {
    pub puuid: String,
    #[serde(default)]
    pub game_name: Option<String>,
    #[serde(default)]
    pub tag_line: Option<String>,
}

/// V4 Summoner Data (for puuid)
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SummonerDataV4 {
    pub profile_icon_id: i32,
    pub revision_date: i64,
    pub puuid: String,
    pub summoner_level: i64,
}
