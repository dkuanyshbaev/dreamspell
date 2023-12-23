// ---------------------------------------
// Dreamspell server
// ---------------------------------------
use axum::{
    extract::State,
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        Method, StatusCode,
    },
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
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

#[derive(Deserialize)]
struct Input {
    birth_date: String,
}

struct DreamspellState {
    secret: String,
    seals: Seals,
    seals_en: Seals,
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
    (
        StatusCode::OK,
        Html(
            r#"
        <!doctype html>
		<html>
        	<head><title>Dreamspell</title></head>
			<body>
			<form action="/" method="post">
				<label for="name">Введите дату рождения:
					<input type="date" name="date_of_birth">
				</label><br>
				<input type="submit" value="Узнать!">
			</form>
			</body>
		</html>"#,
        ),
    )
}

async fn home_en() -> impl IntoResponse {
    (
        StatusCode::OK,
        Html(
            r#"
        <!doctype html>
		<html>
        	<head><title>Dreamspell</title></head>
			<body>
			<form action="/en" method="post">
				<label for="name">Enter your birth date:
					<input type="date" name="date_of_birth">
				</label><br>
				<input type="submit" value="Go!">
			</form>
			</body>
		</html>"#,
        ),
    )
}

async fn result(State(state): State<Arc<DreamspellState>>) -> impl IntoResponse {
    (
        StatusCode::OK,
        Html(
            r#"
        <!doctype html>
		<html>
        	<head><title>Dreamspell</title></head>
			<body>
				Информация..
				<br>
				<a href="/">Назад</a>
			</body>
		</html>"#,
        ),
    )
}

async fn result_en(State(state): State<Arc<DreamspellState>>) -> impl IntoResponse {
    (
        StatusCode::OK,
        Html(
            r#"
        <!doctype html>
		<html>
        	<head><title>Dreamspell</title></head>
			<body>
				The info..
				<br>
				<a href="/">Back</a>
			</body>
		</html>"#,
        ),
    )
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
