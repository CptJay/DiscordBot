// CLASH-V1 endpoints stub

#[derive(Debug)]
pub enum ClashEndpoint {
    PlayersBySummoner { summoner_id: String },
    PlayersByPuuid { puuid: String },
}

impl super::RiotEndpoint for ClashEndpoint {
    fn path(&self) -> String {
        match self {
            ClashEndpoint::PlayersBySummoner { summoner_id } => {
                format!("/lol/clash/v1/players/by-summoner/{}", summoner_id)
            }
            ClashEndpoint::PlayersByPuuid { puuid } => {
                format!("/lol/clash/v1/players/by-puuid/{}", puuid)
            }
        }
    }
}
