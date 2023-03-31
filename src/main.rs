// ---------------------------------------
// Dreamspell server
// ---------------------------------------
use axum::{
    extract::State,
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        Method, StatusCode,
    },
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use axum_auth::AuthBearer;
use serde::Deserialize;
use std::{env, fs, net::SocketAddr, sync::Arc};
use tower_http::cors::{Any, CorsLayer};
use tzolkin::{Seals, Tzolkin};

pub mod tables;
pub mod tzolkin;

const SEALS: &str = "resources/seals.json";

#[derive(Deserialize)]
struct Input {
    birth_date: String,
}

struct DreamspellState {
    secret: String,
    seals: Seals,
}

#[tokio::main]
async fn main() {
    let secret = env::var("SECRET").expect("SECRET must be set");
    let seals = {
        let seals = fs::read_to_string(&SEALS).expect("Can't find seals file");
        serde_json::from_str::<Seals>(&seals).expect("Can't parse seals file")
    };
    let state = Arc::new(DreamspellState { secret, seals });

    let dreamspell = Router::new()
        .route("/", get(home))
        .route("/tzolkin", post(tzolkin))
        .layer(
            CorsLayer::new()
                .allow_headers([AUTHORIZATION, CONTENT_TYPE])
                .allow_methods([Method::POST])
                .allow_origin(Any),
        )
        .with_state(state)
        .fallback(nothing);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(dreamspell.into_make_service())
        .await
        .unwrap();
}

async fn home() -> impl IntoResponse {
    (StatusCode::OK, "Welcome to Dreamspell!")
}

async fn tzolkin(
    AuthBearer(token): AuthBearer,
    State(state): State<Arc<DreamspellState>>,
    Json(input): Json<Input>,
) -> impl IntoResponse {
    if !token.eq(&state.secret) {
        (StatusCode::UNAUTHORIZED, Json(Tzolkin::empty()))
    } else {
        (
            StatusCode::OK,
            Json(Tzolkin::new(
                &state.seals,
                &input
                    .birth_date
                    .split(".")
                    .map(|s| s.parse::<u32>().unwrap_or(0))
                    .collect::<Vec<u32>>()
                    .try_into()
                    .unwrap_or([0; 3]),
            )),
        )
    }
}

async fn nothing() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Nothing to see here")
}
