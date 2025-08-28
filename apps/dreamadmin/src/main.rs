//////////////////////////////////////////
// Dreamadmin server
//////////////////////////////////////////
use std::{env, sync::Arc};

use axum::{
    routing::{get, Router},
};
use tower_http::services::ServeDir;
use axum_login::{
    login_required,
    tower_sessions::{session_store::ExpiredDeletion, Expiry, SessionManagerLayer},
    AuthManagerLayerBuilder,
};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use time::Duration;
use tower_sessions_sqlx_store::SqliteStore;
use tracing_subscriber;

use views::{admin, login_get, login_post, logout, nothing, root_redirect};

pub mod auth;
pub mod templates;
pub mod views;

const MAX_DB_CONNECTIONS: u32 = 5;
const DEFAULT_PORT: u16 = 4444;
const DEFAULT_HOST: &str = "0.0.0.0";

pub struct AdminState {
    pub db_pool: SqlitePool,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter("dreamadmin=info,tower_http=info")
        .init();
    tracing::info!("Initializing Dreamadmin server");
    
    let secret = env::var("SECRET").expect("SECRET must be set");
    let db_location = env::var("DB_LOCATION").expect("DB_LOCATION must be set");
    tracing::info!(db_location = %db_location, "Connecting to database");
    let db_pool = SqlitePoolOptions::new()
        .max_connections(MAX_DB_CONNECTIONS)
        .connect(&db_location)
        .await?;

    // Session layer.
    let session_store = SqliteStore::new(db_pool.clone());
    session_store.migrate().await?;
    
    let state = Arc::new(AdminState { db_pool });

    let deletion_task = tokio::task::spawn(
        session_store
            .clone()
            .continuously_delete_expired(tokio::time::Duration::from_secs(60)),
    );

    let session_layer = SessionManagerLayer::new(session_store)
        .with_expiry(Expiry::OnInactivity(Duration::weeks(1)));

    // Auth service.
    let backend = auth::Backend::new(secret);
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    let app = Router::new()
        .route("/admin", get(admin))
        .route("/admin/seal/{id}", get(views::seal_detail).post(views::seal_update))
        .route("/logout", get(logout))
        .route_layer(login_required!(auth::Backend, login_url = "/login"))
        .route("/", get(root_redirect))
        .route("/login", get(login_get).post(login_post))
        .nest_service("/static", ServeDir::new("apps/dreamadmin/static"))
        .with_state(state)
        .fallback(nothing)
        .layer(auth_layer);

    let bind_address = format!("{}:{}", DEFAULT_HOST, DEFAULT_PORT);
    tracing::info!(address = %bind_address, "Dreamadmin server started successfully");
    let listener = tokio::net::TcpListener::bind(&bind_address)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, address = %bind_address, "Failed to bind to address");
            e
        })?;
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Server failed to start");
            e
        })?;
    tracing::info!("Server shutdown completed gracefully");

    deletion_task.abort();
    let _ = deletion_task.await;

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
