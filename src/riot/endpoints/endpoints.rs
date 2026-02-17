/// API endpoints with associated data
#[derive(Debug, Clone)]
pub enum Endpoint {
    // pUuid
    SommonerByRiotId { game_name: String, tag_line: String },

    // Summoner endpoints
    SummonerByName { name: String },
    SummonerByPuuid { puuid: String },
    SummonerById { id: String },
}

impl Endpoint {
    /// Constructs the full URL path for this endpoint
    pub fn path(&self) -> String {
        match self {
            Endpoint::SommonerByRiotId {
                game_name,
                tag_line,
            } => {
                format!(
                    "/riot/account/v1/accounts/by-riot-id/{}/{}",
                    urlencoding::encode(game_name),
                    urlencoding::encode(tag_line)
                )
            }
            Endpoint::SummonerByName { name } => {
                format!(
                    "/lol/summoner/v4/summoners/by-name/{}",
                    urlencoding::encode(name)
                )
            }
            Endpoint::SummonerByPuuid { puuid } => {
                format!("/lol/summoner/v4/summoners/by-puuid/{}", puuid)
            }
            Endpoint::SummonerById { id } => {
                format!("/lol/summoner/v4/summoners/{}", id)
            }
        }
    }
}
