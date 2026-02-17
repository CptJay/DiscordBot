use crate::discord;
use crate::riot::{client, endpoints::base::Region};

/// Run the bot given:
/// - `token`: The bot token to use for authentication
pub async fn run(disc_token: String, riot_token: String) {
    let riot_client = client::RiotApiClient::new(riot_token, Region::EUW1);

    let discord_client_res = discord::client::create_client(disc_token, riot_client).await;
    let mut discord_client = match discord_client_res {
        Ok(client) => client,
        Err(err) => {
            tracing::error!("Error creating Discord client: {:?}", err);
            return;
        }
    };

    if let Err(err) = discord_client.start().await {
        tracing::error!("Client error: {:?}", err);
    }
}
