// SUMMONER-V4 endpoints stub

#[derive(Debug)]
pub enum SummonerEndpoint {
    ByName { name: String },
    ByPuuid { puuid: String },
    ByAccountId { account_id: String },
    BySummonerId { summoner_id: String },
    ByRiotId { game_name: String, tag_line: String },
}

impl super::RiotEndpoint for SummonerEndpoint {
    fn path(&self) -> String {
        // Helper for percent-encoding
        fn encode(s: &str) -> String {
            urlencoding::encode(s).into_owned()
        }
        match self {
            SummonerEndpoint::ByName { name } => {
                format!("/lol/summoner/v4/summoners/by-name/{}", encode(name))
            }
            SummonerEndpoint::ByPuuid { puuid } => {
                format!("/lol/summoner/v4/summoners/by-puuid/{}", encode(puuid))
            }
            SummonerEndpoint::ByAccountId { account_id } => format!(
                "/lol/summoner/v4/summoners/by-account/{}",
                encode(account_id)
            ),
            SummonerEndpoint::BySummonerId { summoner_id } => {
                format!("/lol/summoner/v4/summoners/{}", encode(summoner_id))
            }
            SummonerEndpoint::ByRiotId {
                game_name,
                tag_line,
            } => format!(
                "/riot/account/v1/accounts/by-riot-id/{}/{}",
                encode(game_name),
                encode(tag_line)
            ),
        }
    }
}
