#![allow(dead_code, unused_variables, unused_imports)]

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

    // Riot token
    let riot_token = env::var("RIOT_TOKEN").expect("Not found: Riot Token");

    // Disc token
    let discord_token = env::var("DISC_TOKEN").expect("Not found: Discord Token");

    internal::run(discord_token, riot_token).await;
}
