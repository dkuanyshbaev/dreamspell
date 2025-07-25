//////////////////////////////////////////
// Dreamspell views
//////////////////////////////////////////
use axum::response::IntoResponse;
use std::sync::Arc;

use crate::*;

#[derive(Deserialize, Debug)]
pub struct Input {
    birth_date: String,
}

pub async fn home() -> impl IntoResponse {
    HtmlTemplate(HomeTemplate {})
}

pub async fn home_en() -> impl IntoResponse {
    HtmlTemplate(HomeEnTemplate {})
}

pub async fn result(
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

pub async fn result_en(
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

pub async fn oferta() -> impl IntoResponse {
    HtmlTemplate(OfaTemplate {})
}

pub async fn oferta_en() -> impl IntoResponse {
    HtmlTemplate(OfaEnTemplate {})
}

pub async fn howto() -> impl IntoResponse {
    HtmlTemplate(HowToTemplate {})
}

pub async fn howto_en() -> impl IntoResponse {
    HtmlTemplate(HowToEnTemplate {})
}

pub async fn nothing() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not found")
}
