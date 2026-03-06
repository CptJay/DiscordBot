use serenity::all::{ActivityData, OnlineStatus};

/*
    Bot Settings
*/

/// Command prefix for the bot
pub const PREFIX: &str = "&";

/*
    Cache Settings
*/

/// Path to the user cache file
pub const USER_CACHE_PATH: &str = "cache/user_cache.json";

/*
    Discord Profile Settings
*/

/// Returns the default activity for the bot
pub fn default_activity() -> Option<ActivityData> {
    Some(ActivityData::playing("Clash Destroyer"))
}

/// Default status for the bot
pub const DEFAULT_STATUS: OnlineStatus = OnlineStatus::DoNotDisturb;
