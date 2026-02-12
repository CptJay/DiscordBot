
use std::env;
mod internal;

#[tokio::main]
async fn main() {
    // Load dotenv vars
    dotenvy::dotenv().ok();

    // Disc token
    let discord_token = env::var("DISC_TOKEN")
        .expect("Not found: Discord Token");

    internal::run(discord_token).await;
}
