// TOURNAMENT-V5 endpoints stub

#[derive(Debug)]
pub enum TournamentEndpoint {
    Codes,
}

impl super::RiotEndpoint for TournamentEndpoint {
    fn path(&self) -> String {
        match self {
            TournamentEndpoint::Codes => "/lol/tournament/v5/codes".to_string(),
        }
    }
}
