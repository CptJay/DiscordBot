use discord_bot::internal;
use discord_bot::utils;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    // Load dotenv vars
    dotenv().ok();

    // Initialize logger
    utils::init_logger();

    // Disc token
    let discord_token = env::var("DISC_TOKEN").expect("Not found: Discord Token");

    internal::run(discord_token).await;
}
