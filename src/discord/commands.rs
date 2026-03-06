use super::super::riot::cache::Cache;
use super::super::riot::client::RiotApiClient;
use crate::constants::PREFIX;
use crate::riot::endpoints::{Endpoint, summoner::SummonerEndpoint};
use serenity::model::channel::Message;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

// --- Type Definitions ---
type CommandHandler = Box<
    dyn Fn(
            &[&str],
            Arc<RiotApiClient>,
            &Cache,
        )
            -> Pin<Box<dyn Future<Output = (Option<String>, Vec<(String, String, String)>)> + Send>>
        + Send
        + Sync,
>;

// --- Helper Functions ---
/// Helper to parse <Username#Tag> pairs from a string
fn parse_riot_ids(input: &str) -> Vec<(String, String)> {
    input
        .split(';')
        .filter_map(|entry| {
            let entry = entry.trim();
            let mut parts = entry.split('#');
            let name = parts.next()?.trim().to_string();
            let tag = parts.next()?.trim().to_string();
            if !name.is_empty() && !tag.is_empty() {
                Some((name, tag))
            } else {
                None
            }
        })
        .collect()
}

// --- Command Map ---
fn command_map() -> HashMap<&'static str, CommandHandler> {
    let mut map: HashMap<&'static str, CommandHandler> = HashMap::new();
    map.insert(
        "lookup",
        Box::new(|args, _riot_client, _cache| {
            let args_vec: Vec<String> = args.iter().map(|s| s.to_string()).collect();
            Box::pin(async move {
                // TODO: Call the actual lookup endpoint
                (Some(format!("[lookup] args: {:?}", args_vec)), vec![])
            })
        }),
    );
    map.insert(
        "profile",
        Box::new(|args, _riot_client, _cache| {
            let args_vec: Vec<String> = args.iter().map(|s| s.to_string()).collect();
            Box::pin(async move {
                // TODO: Call the actual profile endpoint
                (Some(format!("[profile] args: {:?}", args_vec)), vec![])
            })
        }),
    );
    // Clash command handler
    map.insert(
        "clash",
        Box::new(|args, riot_client, cache| {
            let args_str = args.join(" ");
            let riot_ids = parse_riot_ids(&args_str);
            let mut final_puuids = Vec::new(); // Vec<(name, tag, puuid)>
            if riot_ids.is_empty() {
                return Box::pin(async {
                    (
                        Some("Usage: &clash <Username#Tag>; <Username#Tag>; ...".to_string()),
                        vec![],
                    )
                });
            }
            for (name, tag) in &riot_ids {
                if let Some(puuid) = cache.get_user_puuid(name, tag) {
                    final_puuids.push((name.clone(), tag.clone(), puuid.clone()));
                    tracing::info!("Cache hit for {}#{}", name, tag);
                } else {
                    final_puuids.push((name.clone(), tag.clone(), String::new()));
                }
            }
            let riot_client = Arc::clone(&riot_client);
            Box::pin(async move {
                use crate::riot::endpoints::clash::ClashEndpoint;
                use crate::riot::models::clash::ClashPlayer;
                use crate::riot::models::summoner::{SummonerData, SummonerDataV4};

                let mut resolved_puuids = Vec::new(); // Vec<(name, tag, puuid)>
                let mut to_cache_inner = Vec::new(); // Vec<(name, tag, puuid)>
                for (name, tag, puuid) in final_puuids {
                    if !puuid.is_empty() {
                        resolved_puuids.push((name, tag, puuid));
                    } else {
                        let endpoint = Endpoint::Summoner(SummonerEndpoint::ByRiotId {
                            game_name: name.clone(),
                            tag_line: tag.clone(),
                        });
                        match riot_client.request::<SummonerData>(endpoint).await {
                            Ok(summoner) => {
                                resolved_puuids.push((
                                    name.clone(),
                                    tag.clone(),
                                    summoner.puuid.clone(),
                                ));
                                to_cache_inner.push((
                                    name.clone(),
                                    tag.clone(),
                                    summoner.puuid.clone(),
                                ));
                            }
                            Err(err) => {
                                tracing::error!("Error fetching summoner by Riot ID: {:?}", err);
                                resolved_puuids.push((name.clone(), tag.clone(), String::new()));
                            }
                        }
                    }
                }

                let mut response = String::new();
                for (name, tag, puuid) in resolved_puuids.iter() {
                    if puuid.is_empty() {
                        response.push_str(&format!("Could not find summoner {}#{}\n", name, tag));
                        continue;
                    }
                    let endpoint = Endpoint::Summoner(SummonerEndpoint::ByPuuid {
                        puuid: puuid.clone(),
                    });
                    let summoner_v4: SummonerDataV4 = match riot_client.request(endpoint).await {
                        Ok(data) => data,
                        Err(err) => {
                            tracing::error!("Error fetching summoner v4 by puuid: {:?}", err);
                            response.push_str(&format!(
                                "Could not fetch Summoner V4 for {}#{}\n",
                                name, tag
                            ));
                            continue;
                        }
                    };
                    let endpoint = ClashEndpoint::PlayersByPuuid {
                        puuid: summoner_v4.puuid.clone(),
                    };
                    let endpoint = Endpoint::Clash(endpoint);
                    let clash_players: Vec<ClashPlayer> = match riot_client.request(endpoint).await
                    {
                        Ok(data) => data,
                        Err(err) => {
                            tracing::error!("Error fetching clash data: {:?}", err);
                            response.push_str(&format!(
                                "Could not fetch Clash data for {}#{}\n",
                                name, tag
                            ));
                            continue;
                        }
                    };
                    if clash_players.is_empty() {
                        response.push_str(&format!("No Clash data found for {}#{}\n", name, tag));
                        continue;
                    }
                    response.push_str(&format!("Clash data for {}#{}:\n", name, tag));
                    for player in clash_players {
                        response.push_str(&format!(
                            "- Summoner ID: {}\n  Team ID: {:?}\n  Role: {:?}\n  Position: {:?}\n",
                            player.summoner_id, player.team_id, player.role, player.position
                        ));
                    }
                }
                if response.is_empty() {
                    (
                        Some("No valid Clash data found for any user.".to_string()),
                        to_cache_inner,
                    )
                } else {
                    //TODO: "Have to connect to clash tournament endpoint to get teams"
                    (Some(response), to_cache_inner)
                }
            })
        }),
    );
    map
}

// --- Message Parsing ---
/// Parse a message and return a response if the message is a command the bot recognizes
pub async fn parse_message(
    msg: &Message,
    riot_client: Arc<RiotApiClient>,
    cache: &mut Cache,
) -> Option<String> {
    if !msg.content.starts_with(PREFIX) {
        return None;
    }

    // Trim the prefix and split the message into command and args
    let content = msg.content.trim_start_matches(PREFIX);

    // Split the content into command and arguments
    let mut parts = content.split_whitespace();
    let command = parts.next()?;
    let args: Vec<&str> = parts.collect();

    // Dispatch using the command map
    let map = command_map();
    if let Some(handler) = map.get(command) {
        let (response, to_cache) = handler(&args, Arc::clone(&riot_client), cache).await;
        // Only update cache after await, and sync to JSON
        let new_entries: Vec<(String, String, String)> = to_cache
            .into_iter()
            .filter(|(_, _, puuid)| !puuid.is_empty())
            .collect();
        if !new_entries.is_empty() {
            for (name, tag, puuid) in &new_entries {
                cache.insert_user_puuid(name, tag, puuid);
                tracing::info!("Inserted into cache: {}#{} -> {}", name, tag, puuid);
            }
            cache.sync();
        }
        response
    } else {
        Some(format!("Unknown command: {}", command))
    }
}
