use crate::{
    discord::client::PREFIX,
    riot::endpoints::{Endpoint, RiotEndpoint, base::Region, summoner::SummonerEndpoint},
    riot::models::summoner::{SummonerData, SummonerDataV4},
};
use reqwest::Client;
use std::error::Error;

#[derive(Debug)]
pub struct RiotApiClient {
    client: Client,
    api_token: String,
    region: String,
}

impl RiotApiClient {
    /// Create a new RiotApiClient with
    /// - API token
    /// - Region
    pub fn new(api_token: String, region: Region) -> Self {
        Self {
            client: Client::new(),
            api_token,
            region: region.base_url().to_string(), // Base url
        }
    }

    /// Make a request to a specified endpoint
    /// - endpoint: The API endpoint to call
    pub async fn request<T>(&self, endpoint: Endpoint) -> Result<T, Box<dyn Error>>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        // TODO: Remove this hack
        let region = if let Endpoint::Summoner(SummonerEndpoint::ByRiotId { .. }) = &endpoint {
            // For Riot ID endpoint, use EUROPE region
            Region::EUROPE.base_url().to_string()
        } else {
            self.region.clone()
        };

        let url = format!("{}{}", region, endpoint.path());

        let response = self
            .client
            .get(&url)
            .header("X-Riot-Token", &self.api_token)
            .send()
            .await?;

        tracing::info!("Made API request to URL: {}", url);

        if !response.status().is_success() {
            return Err(format!("API request failed with status: {}", response.status()).into());
        }

        let data = response.json::<T>().await?;
        Ok(data)
    }
}

#[allow(dead_code)]
pub async fn parse_lookup_command(args: Vec<&str>, riot_client: &RiotApiClient) -> Option<String> {
    if args.is_empty() {
        return Some("Please provide a command. Usage: &lookup <summoner_name>".to_string());
    }

    let joined = args.join(" ");
    let parts = joined.split('#').map(|s| s.trim()).collect::<Vec<&str>>();

    if parts.len() != 2 {
        return Some(format!(
            "Invalid format. Please use: {}lookup <game_name>#<tag_line>",
            PREFIX
        ));
    }

    let game_name = parts[0];
    let tag_line = parts[1];

    if game_name.is_empty() || tag_line.is_empty() {
        return Some(format!(
            "Game name and tag line cannot be empty. Usage: {}lookup <game_name>#<tag_line>",
            PREFIX
        ));
    }

    if tag_line.len() > 5 {
        return Some("Tag line cannot be longer than 5 characters.".to_string());
    }

    let riot_id_endpoint = Endpoint::Summoner(SummonerEndpoint::ByRiotId {
        game_name: game_name.to_string(),
        tag_line: tag_line.to_string(),
    });
    let summoner_data: Option<SummonerData> =
        match riot_client.request::<SummonerData>(riot_id_endpoint).await {
            Ok(data) => Some(data),
            Err(err) => {
                tracing::error!("Error fetching summoner data: {:?}", err);
                return Some(
                    "Error fetching summoner data. Please check the logs for more details."
                        .to_string(),
                );
            }
        };

    // use the puuid
    let puuid = summoner_data?.puuid;
    tracing::info!(
        "Fetched summoner data for {}#{}: puuid={}",
        game_name,
        tag_line,
        puuid
    );

    let summoner_endpoint = Endpoint::Summoner(SummonerEndpoint::ByPuuid { puuid });
    let summoner_data_v4: Option<SummonerDataV4> = match riot_client
        .request::<SummonerDataV4>(summoner_endpoint)
        .await
    {
        Ok(data) => Some(data),
        Err(err) => {
            tracing::error!("Error fetching summoner data by puuid: {:?}", err);
            return Some(
                "Error fetching summoner data by puuid. Please check the logs for more details."
                    .to_string(),
            );
        }
    };

    let summoner_data_v4 = summoner_data_v4?;

    Some(format!(
        "Summoner: {}#{}\nLevel: {}\nProfile Icon ID: {}\npuuid: {}",
        game_name,
        tag_line,
        summoner_data_v4.summoner_level,
        summoner_data_v4.profile_icon_id,
        summoner_data_v4.puuid
    ))
}
