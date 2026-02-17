use super::super::riot::client::{RiotApiClient, parse_lookup_command};
use super::client::PREFIX;
use serenity::model::channel::Message;

/// Parse a message and return a response if the message is a command the bot recognizes
pub async fn parse_message(msg: &Message, riot_client: &RiotApiClient) -> Option<String> {
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
        "lookup" => match parse_lookup_command(args, riot_client).await {
            Some(resp) => Some(resp),
            None => Some(
                "Error processing lookup command. Please check the logs for more details."
                    .to_string(),
            ),
        },
        _ => Some(format!("Unknown command: {}", command)),
    }
}
