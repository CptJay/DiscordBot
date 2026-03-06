// LEAGUE-EXP-V4 endpoints stub

#[derive(Debug)]
pub enum LeagueExpEndpoint {
    Entries {
        queue: String,
        tier: String,
        division: String,
    },
}

impl super::RiotEndpoint for LeagueExpEndpoint {
    fn path(&self) -> String {
        match self {
            LeagueExpEndpoint::Entries {
                queue,
                tier,
                division,
            } => format!("/lol/league-exp/v4/entries/{}/{}/{}", queue, tier, division),
        }
    }
}
