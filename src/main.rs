use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
};
use geek_utlshortener::{redirect, url_shortener, AppState};
use tokio::net::TcpListener;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

#[tokio::main]
async fn main() -> Result<()> {
    let tracing_layer = Layer::new().pretty().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(tracing_layer).init();

    let state = AppState::try_new().await?;
    let app = Router::new()
        .route("/shorten", post(url_shortener))
        .route("/:id", get(redirect))
        .with_state(state.clone());

    let listener = TcpListener::bind(state.addr.as_str()).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
