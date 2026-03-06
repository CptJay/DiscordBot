// CHAMPION-MASTERY-V4 endpoints stub

pub enum ChampionMasteryEndpoint {
	BySummonerId { summoner_id: String },
}

impl super::RiotEndpoint for ChampionMasteryEndpoint {
	fn path(&self) -> String {
		match self {
			ChampionMasteryEndpoint::BySummonerId { summoner_id } => format!("/lol/champion-mastery/v4/champion-masteries/by-summoner/{}", summoner_id),
		}
	}
}










