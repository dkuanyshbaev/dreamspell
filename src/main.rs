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
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};
use tzolkin::{Seals, Tzolkin};
// use axum::{
//     extract::DefaultBodyLimit,
//     routing::{get, post},
//     Router,
// };
// use axum_login::{
//     axum_sessions::{async_session::CookieStore as SessionStore, SessionLayer},
//     extractors::AuthContext,
//     memory_store::MemoryStore as AuthMemoryStore,
//     AuthLayer, AuthUser, RequireAuthorizationLayer,
// };
// use rand::Rng;
// use serde::Deserialize;
// use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
// use std::{collections::HashMap, env, net::SocketAddr, process, sync::Arc};
// use tokio::sync::RwLock;
// use tower_http::{limit::RequestBodyLimitLayer, services::ServeDir};
//
// use auth::{Role, User};
// use error::HistoryError;
// use models::{book::Book, post::Post, publication::Publication, text::Text};
// use views::*;

pub mod tables;
pub mod tzolkin;
// pub mod auth;
// pub mod error;
// pub mod models;
// pub mod views;

const SEALS: &str = "resources/seals.json";
const SEALS_EN: &str = "resources/seals_en.json";
// const DB_FILE: &str = "db/history.db";
// const IMG_PATH: &str = "static/img/seals";

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
#[template(path = "site/home.html")]
struct HomeTemplate;

#[derive(Template)]
#[template(path = "site/home_en.html")]
struct HomeEnTemplate;

#[derive(Template)]
#[template(path = "site/result.html")]
struct ResultTemplate {
    result: Tzolkin,
}

#[derive(Template)]
#[template(path = "site/result_en.html")]
struct ResultEnTemplate {
    result: Tzolkin,
}

#[derive(Template)]
#[template(path = "site/oferta.html")]
struct OfaTemplate;

#[derive(Template)]
#[template(path = "site/howto.html")]
struct HowToTemplate;

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
        .route("/oferta", get(oferta))
        .route("/howto", get(howto))
        .route("/api/tzolkin", post(tzolkin))
        .route("/api/tzolkin_en", post(tzolkin_en))
        .layer(
            CorsLayer::new()
                .allow_headers([AUTHORIZATION, CONTENT_TYPE])
                .allow_methods([Method::POST])
                .allow_origin(Any),
        )
        .nest_service("/static", ServeDir::new("static"))
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

async fn oferta() -> impl IntoResponse {
    HtmlTemplate(OfaTemplate {})
}

async fn howto() -> impl IntoResponse {
    HtmlTemplate(HowToTemplate {})
}

async fn nothing() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Nothing to see here")
}

// type Auth = AuthContext<usize, User, AuthMemoryStore<usize, User>, Role>;
// type RequireAuth = RequireAuthorizationLayer<usize, User, Role>;
//
// #[derive(Deserialize, Debug)]
// pub struct LoginInput {
//     secret: String,
// }
//
// pub struct HistoryState {
//     secret: String,
//     db: SqlitePool,
// }
//
// #[tokio::main]
// async fn main() {
//     let secret = env::var("SECRET").unwrap_or_else(|_| {
//         println!("SECRET must be set");
//         process::exit(0);
//     });
//     let db = SqlitePoolOptions::new()
//         .connect(DB_FILE)
//         .await
//         .unwrap_or_else(|_| {
//             println!("Can't find db file");
//             process::exit(0);
//         });
//     let state = Arc::new(HistoryState {
//         secret: secret.clone(),
//         db,
//     });
//
//     let session_secret = rand::thread_rng().gen::<[u8; 64]>();
//     let session_store = SessionStore::new();
//     let session_layer = SessionLayer::new(session_store, &session_secret);
//
//     let store = Arc::new(RwLock::new(HashMap::default()));
//     let user = User::new(secret);
//     store.write().await.insert(user.get_id(), user);
//     let user_store = AuthMemoryStore::new(&store);
//     let auth_layer = AuthLayer::new(user_store, &session_secret);
//
//     let history = Router::new()
//         // Posts
//         .route("/posts", get(posts::all))
//         .route("/posts/create", get(posts::add).post(posts::create))
//         .route("/posts/update/:id", get(posts::edit).post(posts::update))
//         .route("/posts/delete/:id", post(posts::delete))
//         // Books
//         .route("/books", get(books::all))
//         .route("/books/create", get(books::add).post(books::create))
//         .route("/books/update/:id", get(books::edit).post(books::update))
//         .route("/books/delete/:id", post(books::delete))
//         // Publications
//         .route("/publications", get(publications::all))
//         .route(
//             "/publications/create",
//             get(publications::add).post(publications::create),
//         )
//         .route(
//             "/publications/update/:id",
//             get(publications::edit).post(publications::update),
//         )
//         .route("/publications/delete/:id", post(publications::delete))
//         // Texts
//         .route("/texts", get(texts::all))
//         .route("/texts/create", get(texts::add).post(texts::create))
//         .route("/texts/update/:id", get(texts::edit).post(texts::update))
//         .route("/texts/delete/:id", post(texts::delete))
//         // Routes above are protected
//         .route_layer(RequireAuth::login_with_role(Role::Admin..))
//         // Static
//         .nest_service("/static", ServeDir::new("static"))
//         // Admin
//         .route("/login", get(admin::form).post(admin::login))
//         .route("/logout", get(admin::logout))
//         // Handlers
//         .route("/", get(handlers::home))
//         .route("/blog", get(handlers::blog))
//         .route("/entry/:id", get(handlers::entry))
//         // System
//         .fallback(nothing)
//         // Layers
//         .layer(auth_layer)
//         .layer(session_layer)
//         .layer(DefaultBodyLimit::disable())
//         .layer(RequestBodyLimitLayer::new(8 * 1024 * 1024 #<{(| 8mb |)}>#))
//         .with_state(state);
//
//     let addr = SocketAddr::from(([127, 0, 0, 1], 8888));
//     println!("Listening on {}", addr);
//     axum::Server::bind(&addr)
//         .serve(history.into_make_service())
//         .await
//         .unwrap();
// }
//
// pub async fn nothing() -> HistoryError {
//     HistoryError::NotFound
// }
