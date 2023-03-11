use tracing_subscriber::{prelude::*, EnvFilter};

// Setup `tracing::subscriber` to read the log level from RUST_LOG environment variable
pub fn setup_tracing() {
  let format = tracing_subscriber::fmt::layer().json();
  let filter = EnvFilter::try_from_default_env()
    .or_else(|_| EnvFilter::try_new("debug"))
    .unwrap();

  tracing_subscriber::registry()
    .with(format)
    .with(filter)
    .init();
}
