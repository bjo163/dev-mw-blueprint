use mizan_api::{app, AppState};
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "mizan_api=info".into()))
        .init();

    let state = AppState::from_env().await?;
    let bind = std::env::var("MIZAN_BIND").unwrap_or_else(|_| "127.0.0.1:3000".into());
    let listener = TcpListener::bind(&bind).await?;
    tracing::info!(%bind, persistence = state.persistence_enabled(), "mizan api listening");
    axum::serve(listener, app(state)).await?;
    Ok(())
}
