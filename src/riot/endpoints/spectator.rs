// SPECTATOR-V5 endpoints stub

#[derive(Debug)]
pub enum SpectatorEndpoint {
    ActiveGames { summoner_id: String },
}

impl super::RiotEndpoint for SpectatorEndpoint {
    fn path(&self) -> String {
        match self {
            SpectatorEndpoint::ActiveGames { summoner_id } => {
                format!("/lol/spectator/v5/active-games/by-summoner/{}", summoner_id)
            }
        }
    }
}
