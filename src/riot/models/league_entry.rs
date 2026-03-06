use serde::{Deserialize, Serialize};

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
