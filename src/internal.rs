use crate::discord::client;

/// Run the bot given:
/// - `token`: The bot token to use for authentication
pub async fn run(token: String) {
    let mut client = client::create_client(token).await;

    if let Err(err) = client.start().await {
        println!("Client error: {:?}", err);
    }
}
