use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ClashPlayer {
    pub summoner_id: String,
    pub team_id: Option<String>,
    pub team_role: Option<String>,
    pub position: Option<String>,
    pub role: Option<String>,
}

// The Riot API returns a Vec<ClashPlayer> for the endpoint
