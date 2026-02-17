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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionMastery {
    pub puuid: String,
    pub champion_id: i64,
    pub champion_level: i32,
    pub champion_points: i32,
    pub last_play_time: i64,
    pub champion_points_since_last_level: i32,
    pub champion_points_until_next_level: i32,
    pub chest_granted: bool,
    pub tokens_earned: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LeagueEntry {
    pub league_id: String,
    pub summoner_id: String,
    pub summoner_name: String,
    pub queue_type: String,
    pub tier: String,
    pub rank: String,
    pub league_points: i32,
    pub wins: i32,
    pub losses: i32,
    pub hot_streak: bool,
    pub veteran: bool,
    pub fresh_blood: bool,
    pub inactive: bool,
}
