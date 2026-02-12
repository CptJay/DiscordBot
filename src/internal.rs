pub mod api;

/// Run the bot given:
/// - `token`: The bot token to use for authentication
pub async fn run (token: String) {

    let mut client = api::create_client(token)
        .await;

    if let Err(err) = client.start().await {
        println!("Client error: {:?}", err);
    }
}