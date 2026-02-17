/// Riot API regions
#[derive(Debug, Clone)]
pub enum Region {
    NA1,
    EUW1,
    EUN1,
    KR,
    BR1,
    JP1,
    RU,
    OC1,
    TR1,
    LA1,
    LA2,
    EUROPE,
}

impl Region {
    pub fn base_url(&self) -> &str {
        match self {
            Region::NA1 => "https://na1.api.riotgames.com",
            Region::EUW1 => "https://euw1.api.riotgames.com",
            Region::EUN1 => "https://eun1.api.riotgames.com",
            Region::KR => "https://kr.api.riotgames.com",
            Region::BR1 => "https://br1.api.riotgames.com",
            Region::JP1 => "https://jp1.api.riotgames.com",
            Region::RU => "https://ru.api.riotgames.com",
            Region::OC1 => "https://oc1.api.riotgames.com",
            Region::TR1 => "https://tr1.api.riotgames.com",
            Region::LA1 => "https://la1.api.riotgames.com",
            Region::LA2 => "https://la2.api.riotgames.com",
            Region::EUROPE => "https://europe.api.riotgames.com",
        }
    }
}
