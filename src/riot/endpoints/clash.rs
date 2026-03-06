// CLASH-V1 endpoints stub

pub enum ClashEndpoint {
    PlayersBySummoner { summoner_id: String },
}

impl super::RiotEndpoint for ClashEndpoint {
    fn path(&self) -> String {
        match self {
            ClashEndpoint::PlayersBySummoner { summoner_id } => format!("/lol/clash/v1/players/by-summoner/{}", summoner_id),
        }
    }
}
