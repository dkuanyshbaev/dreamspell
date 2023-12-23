// ---------------------------------------
// Dreamspell server
// ---------------------------------------
use askama::Template;
use axum::{
    extract::State,
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        Method, StatusCode,
    },
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Form, Json, Router,
};
use axum_auth::AuthBearer;
use serde::Deserialize;
use std::{env, fs, sync::Arc};
use tower_http::cors::{Any, CorsLayer};
use tzolkin::{Seals, Tzolkin};

pub mod tables;
pub mod tzolkin;

const SEALS: &str = "resources/seals.json";
const SEALS_EN: &str = "resources/seals_en.json";

#[derive(Deserialize, Debug)]
struct Input {
    birth_date: String,
}

#[derive(Debug)]
struct DreamspellState {
    secret: String,
    seals: Seals,
    seals_en: Seals,
}

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate;

#[derive(Template)]
#[template(path = "home_en.html")]
struct HomeEnTemplate;

#[derive(Template)]
#[template(path = "result.html")]
struct ResultTemplate {
    result: Tzolkin,
}

#[derive(Template)]
#[template(path = "result_en.html")]
struct ResultEnTemplate {
    result: Tzolkin,
}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {err}"),
            )
                .into_response(),
        }
    }
}

#[tokio::main]
async fn main() {
    let secret = env::var("SECRET").expect("SECRET must be set");
    let seals = {
        let seals = fs::read_to_string(SEALS).expect("Can't find seals file");
        serde_json::from_str::<Seals>(&seals).expect("Can't parse seals file")
    };
    let seals_en = {
        let seals = fs::read_to_string(SEALS_EN).expect("Can't find seals en file");
        serde_json::from_str::<Seals>(&seals).expect("Can't parse seals en file")
    };
    let state = Arc::new(DreamspellState {
        secret,
        seals,
        seals_en,
    });

    let dreamspell = Router::new()
        .route("/", get(home).post(result))
        .route("/en", get(home_en).post(result_en))
        .route("/api/tzolkin", post(tzolkin))
        .route("/api/tzolkin_en", post(tzolkin_en))
        .layer(
            CorsLayer::new()
                .allow_headers([AUTHORIZATION, CONTENT_TYPE])
                .allow_methods([Method::POST])
                .allow_origin(Any),
        )
        .with_state(state)
        .fallback(nothing);

    println!("Welcome to Dreamspell!");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8888").await.unwrap();
    axum::serve(listener, dreamspell).await.unwrap();
}

async fn home() -> impl IntoResponse {
    HtmlTemplate(HomeTemplate {})
}

async fn home_en() -> impl IntoResponse {
    HtmlTemplate(HomeEnTemplate {})
}

async fn result(
    State(state): State<Arc<DreamspellState>>,
    Form(input): Form<Input>,
) -> impl IntoResponse {
    let result = Tzolkin::new(
        &state.seals,
        false,
        &input
            .birth_date
            .split('-')
            .map(|s| s.parse::<u32>().unwrap_or(0))
            .collect::<Vec<u32>>()
            .try_into()
            .unwrap_or([0; 3]),
    );

    HtmlTemplate(ResultTemplate { result })
}

async fn result_en(
    State(state): State<Arc<DreamspellState>>,
    Form(input): Form<Input>,
) -> impl IntoResponse {
    let result = Tzolkin::new(
        &state.seals_en,
        true,
        &input
            .birth_date
            .split('-')
            .map(|s| s.parse::<u32>().unwrap_or(0))
            .collect::<Vec<u32>>()
            .try_into()
            .unwrap_or([0; 3]),
    );

    HtmlTemplate(ResultEnTemplate { result })
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
                false,
                &input
                    .birth_date
                    .split('-')
                    .map(|s| s.parse::<u32>().unwrap_or(0))
                    .collect::<Vec<u32>>()
                    .try_into()
                    .unwrap_or([0; 3]),
            )),
        )
    }
}

async fn tzolkin_en(
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
                &state.seals_en,
                true,
                &input
                    .birth_date
                    .split('-')
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
