pub mod account;
pub mod base;
pub mod champion;
pub mod champion_mastery;
pub mod clash;
pub mod league;
pub mod league_exp;
pub mod lol_challenges;
pub mod lol_status;
pub mod match_v5;
pub mod spectator;
pub mod summoner;
pub mod tournament;

/// Trait for all Riot API endpoints
pub trait RiotEndpoint {
    fn path(&self) -> String;
    fn method(&self) -> &'static str {
        "GET"
    }
}

#[derive(Debug)]
/// Enum to wrap all endpoint groups
pub enum Endpoint {
    Account(account::AccountEndpoint),
    ChampionMastery(champion_mastery::ChampionMasteryEndpoint),
    Champion(champion::ChampionEndpoint),
    Clash(clash::ClashEndpoint),
    LeagueExp(league_exp::LeagueExpEndpoint),
    League(league::LeagueEndpoint),
    LolChallenges(lol_challenges::LolChallengesEndpoint),
    LolStatus(lol_status::LolStatusEndpoint),
    MatchV5(match_v5::MatchV5Endpoint),
    Spectator(spectator::SpectatorEndpoint),
    Summoner(summoner::SummonerEndpoint),
    Tournament(tournament::TournamentEndpoint),
}

impl RiotEndpoint for Endpoint {
    fn path(&self) -> String {
        match self {
            Endpoint::Account(e) => e.path(),
            Endpoint::ChampionMastery(e) => e.path(),
            Endpoint::Champion(e) => e.path(),
            Endpoint::Clash(e) => e.path(),
            Endpoint::LeagueExp(e) => e.path(),
            Endpoint::League(e) => e.path(),
            Endpoint::LolChallenges(e) => e.path(),
            Endpoint::LolStatus(e) => e.path(),
            Endpoint::MatchV5(e) => e.path(),
            Endpoint::Spectator(e) => e.path(),
            Endpoint::Summoner(e) => e.path(),
            Endpoint::Tournament(e) => e.path(),
        }
    }
    fn method(&self) -> &'static str {
        match self {
            Endpoint::Account(e) => e.method(),
            Endpoint::ChampionMastery(e) => e.method(),
            Endpoint::Champion(e) => e.method(),
            Endpoint::Clash(e) => e.method(),
            Endpoint::LeagueExp(e) => e.method(),
            Endpoint::League(e) => e.method(),
            Endpoint::LolChallenges(e) => e.method(),
            Endpoint::LolStatus(e) => e.method(),
            Endpoint::MatchV5(e) => e.method(),
            Endpoint::Spectator(e) => e.method(),
            Endpoint::Summoner(e) => e.method(),
            Endpoint::Tournament(e) => e.method(),
        }
    }
}
