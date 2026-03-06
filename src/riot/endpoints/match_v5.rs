// MATCH-V5 endpoints stub

pub enum MatchV5Endpoint {
    ByPuuid { puuid: String },
}

impl super::RiotEndpoint for MatchV5Endpoint {
    fn path(&self) -> String {
        match self {
            MatchV5Endpoint::ByPuuid { puuid } => format!("/lol/match/v5/matches/by-puuid/{}/ids", puuid),
        }
    }
}
