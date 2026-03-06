use super::commands::parse_message;
use crate::constants::{DEFAULT_STATUS, USER_CACHE_PATH, default_activity};
use crate::riot::cache::Cache;
use crate::riot::client::RiotApiClient;
use serenity::all::Ready;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use std::sync::{Arc, Mutex};


/// Handler for events
struct Handler {
    riot_client: RiotApiClient,
    cache: Arc<Mutex<Cache>>,
}

/// Handler for events
/// - messages being sent in channels the bot has access to
#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // Clone cache for read
        let cache_snapshot = {
            let cache = self.cache.lock().unwrap();
            cache.clone()
        };
        // Use a mutable clone for parse_message
        let mut cache_for_lookup = cache_snapshot;
        let response = parse_message(&msg, &self.riot_client, &mut cache_for_lookup).await;

        // If cache was updated (i.e., not a cache hit), write back
        {
            let mut cache = self.cache.lock().unwrap();
            *cache = cache_for_lookup;
        }

        if let Some(resp) = response
            && let Err(err) = msg.channel_id.say(&ctx.http, resp).await
        {
            tracing::error!("Error sending message: {:?}", err);
        }
    }

    async fn ready(&self, ctx: Context, data_about_bot: Ready) {
        // Set the bot's presence (status and activity)
        ctx.set_presence(default_activity(), DEFAULT_STATUS);

        tracing::info!("{} is connected!", data_about_bot.user.name);
    }
}

/// Create a new client, which will automatically prepend your bot token to requests.
pub async fn create_client(
    token: String,
    riot_client: RiotApiClient,
) -> Result<Client, Box<dyn std::error::Error>> {
    // Load cache from file
    let cache = Cache::load(USER_CACHE_PATH);
    let handler = Handler {
        riot_client,
        cache: Arc::new(Mutex::new(cache)),
    };

    // TODO: Remove the `GatewayIntents::all()` and only add the intents we need
    let client = Client::builder(&token, GatewayIntents::all())
        .event_handler(handler)
        .await?;

    Ok(client)
}
