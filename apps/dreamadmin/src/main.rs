use axum::{
    routing::{get, Router},
};
use axum_login::{
    login_required,
    tower_sessions::{session_store::ExpiredDeletion, Expiry, SessionManagerLayer},
    AuthManagerLayerBuilder,
};
use sqlx::sqlite::SqlitePool;
use std::env;
use time::Duration;
use tokio::{signal, task::AbortHandle};
// use tower_http::{limit::RequestBodyLimitLayer, services::ServeDir};
use tower_sessions_sqlx_store::SqliteStore;

use crate::auth::Backend;

mod auth;
mod views;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenvy::dotenv().ok();
    
    let secret = env::var("SECRET").expect("SECRET must be set");
    let db_location = env::var("DB_LOCATION").expect("DB_LOCATION must be set");
    let db_pool = SqlitePool::connect(&db_location).await?;

    // Session layer.
    let session_store = SqliteStore::new(db_pool);
    session_store.migrate().await?;

    let deletion_task = tokio::task::spawn(
        session_store
            .clone()
            .continuously_delete_expired(tokio::time::Duration::from_secs(60)),
    );

    let session_layer = SessionManagerLayer::new(session_store)
        // default true - https only
        // .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::weeks(1)));

    // let session_secret = rand::thread_rng().gen::<[u8; 64]>();
    // let session_store = SessionStore::new();
    // let session_layer = SessionLayer::new(session_store, &session_secret);
    ////////////////////////////////////////////////////////

    // Auth service.
    let backend = Backend::new(secret);
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    let app = Router::new()
        .route("/admin", get(views::admin))
        .route_layer(login_required!(Backend, login_url = "/"))
        .route("/", get(views::login_get).post(views::login_post))
        .fallback(views::nothing)
        .layer(auth_layer);

    println!("Starting on 4444");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4444").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal(deletion_task.abort_handle()))
        .await?;

    // deletion_task.await??;
    deletion_task.await.unwrap().unwrap();

    Ok(())
}

async fn shutdown_signal(deletion_task_abort_handle: AbortHandle) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => { deletion_task_abort_handle.abort() },
        _ = terminate => { deletion_task_abort_handle.abort() },
    }
}
