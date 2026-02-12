
use std::convert::Into;
use serenity::all::{ActivityData, Ready};
use serenity::async_trait;
use serenity::model::{channel::Message, user::OnlineStatus};
use serenity::prelude::*;

/// Constant prefix for commands
static PREFIX: &str = "&&";

/// Constant for the default activity of the bot
fn default_activity() -> Option<ActivityData> {
    ActivityData::playing("with Rust").into()
}

/// Constant for the default status of the bot
fn default_status() -> Option<OnlineStatus> {
    OnlineStatus::DoNotDisturb.into()
}

/// Handler for events
struct Handler;

/// Handler for events
/// - messages being sent in channels the bot has access to
#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if let Some(resp) = parse_message(&msg).await {
            if let Err(err) = msg.channel_id.say(&ctx.http, resp).await {
                eprintln!("Error sending message: {:?}", err);
            }
        }
    }

    async fn ready(&self, ctx: Context, data_about_bot: Ready) {
        println!("{} is connected!", data_about_bot.user.name);

        // Set the bot's status
        set_status(&ctx, default_activity(), default_status()).await;
    }
}

/// Create a new client, which will automatically prepend your bot token to requests.
pub async fn create_client(token: String) -> Client {
    // TODO: Remove the `GatewayIntents::all()` and only add the intents we need
    Client::builder(&token, GatewayIntents::all())
        .event_handler(Handler)
        .await
        .expect("Error creating client")
}

/// Change the bot's status to online and set the activity to "Playing with Rust"
pub async fn set_status(ctx: &Context, activity: Option<ActivityData>, status: Option<OnlineStatus>) {
    let activity_data = activity.or_else(|| default_activity());
    let status = status.unwrap_or(OnlineStatus::DoNotDisturb);

    ctx.set_presence(activity_data, status);
}

/// Parse a message and return a response if the message is a command the bot recognizes
async fn parse_message(msg: &Message) -> Option<String> {
    if !msg.content.starts_with(PREFIX) {
        return None;
    }

    // Trim the prefix and split the message into command and args
    let content = msg.content.trim_start_matches(PREFIX);

    // Split the content into command and arguments
    let mut parts = content.split_whitespace();
    let command = parts.next()?;
    let args: Vec<&str> = parts.collect();

    // Match the command and generate a response
    match command {
        "ping" => Some("Pong!".to_string()),
        "echo" => Some(args.join(" ")),
        _ => Some(format!("Unknown command: {}", command)),
    }
}



