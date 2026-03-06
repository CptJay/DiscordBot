use tracing_subscriber::FmtSubscriber;

/// Initialize the logger for the application
pub fn init_logger() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global default subscriber");
}

/// Parse "<Username#Tag>; <Username#Tag>; ..." into Vec<(Username, Tag)>
pub fn parse_riot_ids(input: &str) -> Vec<(String, String)> {
    input
        .split(';')
        .filter_map(|entry| {
            let entry = entry.trim();
            let mut parts = entry.split('#');
            let name = parts.next()?.trim().to_string();
            let tag = parts.next()?.trim().to_string();
            if !name.is_empty() && !tag.is_empty() {
                Some((name, tag))
            } else {
                None
            }
        })
        .collect()
}