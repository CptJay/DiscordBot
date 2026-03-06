// LEAGUE-V4 endpoints stub

pub enum LeagueEndpoint {
    Challenger { queue: String },
}

impl super::RiotEndpoint for LeagueEndpoint {
    fn path(&self) -> String {
        match self {
            LeagueEndpoint::Challenger { queue } => format!("/lol/league/v4/challengerleagues/by-queue/{}", queue),
        }
    }
}
