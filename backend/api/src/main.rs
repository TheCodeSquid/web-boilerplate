#[macro_use]
extern crate tracing;

mod config;
mod jobs;
mod routes;
mod state;

use std::{net::SocketAddr, sync::Arc, time::Duration};

use axum::routing::Router;
use service::sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, cors::CorsLayer};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use config::ApiConfig;
use state::Api;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let config = ApiConfig::from_env()?;
    init_logger(&config)?;
    let db = init_database(&config).await?;
    let state = Arc::new(Api { config, db });

    jobs::spawn_jobs(state.clone());

    let router = init_router(state.clone())?;

    let addr = SocketAddr::new(state.config.addr, state.config.port);
    info!("listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}

fn init_router(state: Arc<Api>) -> eyre::Result<Router> {
    let services = ServiceBuilder::new()
        .layer(CompressionLayer::new())
        .layer(CorsLayer::new().allow_origin(state.config.cors_origins.clone()));

    let router = Router::new()
        .merge(routes::routes())
        .layer(services)
        .with_state(state);

    Ok(router)
}

async fn init_database(config: &ApiConfig) -> eyre::Result<DatabaseConnection> {
    let mut opt = ConnectOptions::new(&config.db_url);
    opt.max_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .sqlx_logging(false);
    Ok(Database::connect(opt).await?)
}

fn init_logger(config: &ApiConfig) -> eyre::Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::builder().parse(&config.log)?)
        .init();

    debug!("logger initialized");

    Ok(())
}
