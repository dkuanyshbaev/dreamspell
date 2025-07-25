//////////////////////////////////////////
// Dreamspell server
//////////////////////////////////////////
use axum::{extract::State, http::StatusCode, routing::get, Form, Router};
use serde::Deserialize;
use std::{env, fs, sync::Arc};
use tower_http::services::ServeDir;

use templates::*;
use tzolkin::{Seals, Tzolkin};
use views::*;

pub mod tables;
pub mod templates;
pub mod tzolkin;
// pub mod auth;
// pub mod error;
// pub mod models;
pub mod views;

const SEALS: &str = "resources/seals.json";
const SEALS_EN: &str = "resources/seals_en.json";
// const DB_LOCATION: &str = "/srv/dreambase/dreambase.sqlite";
// const SEALS_LOCATION: &str = "/srv/seals";

#[derive(Debug)]
pub struct DreamspellState {
    pub secret: String,
    pub seals: Seals,
    pub seals_en: Seals,
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
        .route("/oferta_en", get(oferta_en))
        .route("/howto", get(howto))
        .route("/howto_en", get(howto_en))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state)
        .fallback(nothing);

    println!("Welcome to Dreamspell!");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8888").await.unwrap();
    axum::serve(listener, dreamspell).await.unwrap();
}
