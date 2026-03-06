// ACCOUNT-V1 endpoints stub

#[derive(Debug)]
pub enum AccountEndpoint {
	ByPuuid { puuid: String },
	ByRiotId { game_name: String, tag_line: String },
}

impl super::RiotEndpoint for AccountEndpoint {
	fn path(&self) -> String {
		match self {
			AccountEndpoint::ByPuuid { puuid } => format!("/riot/account/v1/accounts/by-puuid/{}", puuid),
			AccountEndpoint::ByRiotId { game_name, tag_line } => format!("/riot/account/v1/accounts/by-riot-id/{}/{}", game_name, tag_line),
		}
	}
}
