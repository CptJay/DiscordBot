use super::commands::parse_message;
use crate::riot::client::RiotApiClient;
use serenity::all::{ActivityData, Ready};
use serenity::async_trait;
use serenity::model::{channel::Message, user::OnlineStatus};
use serenity::prelude::*;
use std::convert::Into;

/// Constant prefix for commands
pub static PREFIX: &str = "&";

/// Constant for the default activity of the bot
fn default_activity() -> Option<ActivityData> {
    ActivityData::playing("with Rust").into()
}

/// Constant for the default status of the bot
fn default_status() -> Option<OnlineStatus> {
    OnlineStatus::DoNotDisturb.into()
}

/// Handler for events
struct Handler {
    riot_client: RiotApiClient,
}

/// Handler for events
/// - messages being sent in channels the bot has access to
#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if let Some(resp) = parse_message(&msg, &self.riot_client).await
            && let Err(err) = msg.channel_id.say(&ctx.http, resp).await
        {
            tracing::error!("Error sending message: {:?}", err);
        }
    }

    async fn ready(&self, ctx: Context, data_about_bot: Ready) {
        tracing::info!("{} is connected!", data_about_bot.user.name);

        // Set the bot's status
        set_status(&ctx, default_activity(), default_status()).await;
    }
}

/// Create a new client, which will automatically prepend your bot token to requests.
pub async fn create_client(
    token: String,
    riot_client: RiotApiClient,
) -> Result<Client, Box<dyn std::error::Error>> {
    // Assign the riot client to the handler
    let handler = Handler { riot_client };

    // TODO: Remove the `GatewayIntents::all()` and only add the intents we need
    let client = Client::builder(&token, GatewayIntents::all())
        .event_handler(handler)
        .await?;

    Ok(client)
}

/// Change the bot's status to online and set the activity to "Playing with Rust"
pub async fn set_status(
    ctx: &Context,
    activity: Option<ActivityData>,
    status: Option<OnlineStatus>,
) {
    let activity_data = activity.or_else(default_activity);
    let status = status.unwrap_or(OnlineStatus::DoNotDisturb);

    ctx.set_presence(activity_data, status);
}
