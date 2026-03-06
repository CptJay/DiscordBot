// SUMMONER-V4 endpoints stub

pub enum SummonerEndpoint {
    ByName { name: String },
    ByPuuid { puuid: String },
    ByAccountId { account_id: String },
    BySummonerId { summoner_id: String },
    ByRiotId { game_name: String, tag_line: String },
}

impl super::RiotEndpoint for SummonerEndpoint {
    fn path(&self) -> String {
        match self {
            SummonerEndpoint::ByName { name } => format!("/lol/summoner/v4/summoners/by-name/{}", name),
            SummonerEndpoint::ByPuuid { puuid } => format!("/lol/summoner/v4/summoners/by-puuid/{}", puuid),
            SummonerEndpoint::ByAccountId { account_id } => format!("/lol/summoner/v4/summoners/by-account/{}", account_id),
            SummonerEndpoint::BySummonerId { summoner_id } => format!("/lol/summoner/v4/summoners/{}", summoner_id),
            SummonerEndpoint::ByRiotId { game_name, tag_line } => format!("/riot/account/v1/accounts/by-riot-id/{}/{}", game_name, tag_line),
        }
    }
}
