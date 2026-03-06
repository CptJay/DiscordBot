// LOL-CHALLENGES-V1 endpoints stub

pub enum LolChallengesEndpoint {
    PlayerData { puuid: String },
}

impl super::RiotEndpoint for LolChallengesEndpoint {
    fn path(&self) -> String {
        match self {
            LolChallengesEndpoint::PlayerData { puuid } => format!("/lol/challenges/v1/player-data/{}", puuid),
        }
    }
}
