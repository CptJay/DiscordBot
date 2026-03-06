// CHAMPION-V3 endpoints stub

pub enum ChampionEndpoint {
    Rotation,
}

impl super::RiotEndpoint for ChampionEndpoint {
    fn path(&self) -> String {
        match self {
            ChampionEndpoint::Rotation => "/lol/platform/v3/champion-rotations".to_string(),
        }
    }
}
