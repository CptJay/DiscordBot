use serde::{Deserialize, Serialize};

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
