use tracing_subscriber::FmtSubscriber;

/// Initialize the logger for the application
pub fn init_logger() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global default subscriber");
}
