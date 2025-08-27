//////////////////////////////////////////
// Dreamspell server
//////////////////////////////////////////
use std::{env, sync::Arc};

use axum::{routing::get, Router};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use tower_http::services::ServeDir;

use views::{health, home, home_en, howto, howto_en, nothing, oferta, oferta_en, result, result_en};

pub mod templates;
pub mod views;

const MAX_DB_CONNECTIONS: u32 = 5;
const DEFAULT_PORT: u16 = 8888;
const DEFAULT_HOST: &str = "0.0.0.0";

pub struct DreamState {
    pub secret: String,
    pub db_pool: SqlitePool,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter("dreamspell=info,tower_http=info")
        .init();
    tracing::info!("Initializing Dreamspell server");

    let secret = env::var("SECRET").expect("SECRET must be set");
    let db_location = env::var("DB_LOCATION").expect("DB_LOCATION must be set");
    let db_pool = SqlitePoolOptions::new()
        .max_connections(MAX_DB_CONNECTIONS)
        .connect(&db_location)
        .await?;
    let state = Arc::new(DreamState { secret, db_pool });

    let app = Router::new()
        .route("/", get(home).post(result))
        .route("/en", get(home_en).post(result_en))
        .route("/oferta", get(oferta))
        .route("/oferta_en", get(oferta_en))
        .route("/howto", get(howto))
        .route("/howto_en", get(howto_en))
        .route("/health", get(health))
        .nest_service("/static", ServeDir::new("apps/dreamspell/static"))
        .with_state(state)
        .fallback(nothing);

    let bind_address = format!("{}:{}", DEFAULT_HOST, DEFAULT_PORT);
    tracing::info!(address = %bind_address, "Dreamspell server started successfully");

    let listener = tokio::net::TcpListener::bind(&bind_address)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, address = %bind_address, "Failed to bind to address");
            e
        })?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Server failed to start");
            e
        })?;
    tracing::info!("Server shutdown completed gracefully");

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("Received Ctrl+C, initiating graceful shutdown");
        },
        _ = terminate => {
            tracing::info!("Received SIGTERM, initiating graceful shutdown");
        },
    }
}
