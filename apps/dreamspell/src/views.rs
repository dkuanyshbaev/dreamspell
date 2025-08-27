//////////////////////////////////////////
// Dreamspell views
//////////////////////////////////////////
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Form, Json,
};
use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::templates::{
    HomeEnTemplate, HomeTemplate, HowToEnTemplate, HowToTemplate, HtmlTemplate, OfertaEnTemplate,
    OfertaTemplate, ResultEnTemplate, ResultTemplate,
};
use tzolkin::{Language, Tzolkin};
use crate::DreamState;

#[derive(Deserialize, Debug)]
pub struct Input {
    birth_date: String,
}

fn validate_date(input: &str) -> Result<[u32; 3], String> {
    // Sanitization - only allow digits and dashes
    let sanitized = input
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == '-')
        .collect::<String>();

    if sanitized != input {
        return Err("Invalid characters in date".to_string());
    }

    // Parse with chrono
    let date = NaiveDate::parse_from_str(&sanitized, "%Y-%m-%d")
        .map_err(|_| "Invalid date format. Use YYYY-MM-DD".to_string())?;

    let year = date.year() as u32;
    if year < 1900 || year > 2100 {
        return Err("Year must be between 1900 and 2100".to_string());
    }

    Ok([year, date.month(), date.day()])
}

pub async fn home() -> impl IntoResponse {
    HtmlTemplate(HomeTemplate {})
}

pub async fn home_en() -> impl IntoResponse {
    HtmlTemplate(HomeEnTemplate {})
}

pub async fn result(
    State(state): State<Arc<DreamState>>,
    Form(input): Form<Input>,
) -> impl IntoResponse {
    tracing::info!(
        date = %input.birth_date,
        language = "ru",
        "Processing birth date calculation"
    );

    let date_parts = match validate_date(&input.birth_date) {
        Ok(parts) => parts,
        Err(e) => {
            tracing::warn!(date = %input.birth_date, error = %e, "Invalid date submitted");
            return Redirect::to("/").into_response();
        }
    };

    let result = Tzolkin::new(&state.db_pool, Language::Russian, &date_parts).await;
    HtmlTemplate(ResultTemplate { result }).into_response()
}

pub async fn result_en(
    State(state): State<Arc<DreamState>>,
    Form(input): Form<Input>,
) -> impl IntoResponse {
    tracing::info!(
        date = %input.birth_date,
        language = "en",
        "Processing birth date calculation"
    );

    let date_parts = match validate_date(&input.birth_date) {
        Ok(parts) => parts,
        Err(e) => {
            tracing::warn!(date = %input.birth_date, error = %e, "Invalid date submitted");
            return Redirect::to("/en").into_response();
        }
    };

    let result = Tzolkin::new(&state.db_pool, Language::English, &date_parts).await;
    HtmlTemplate(ResultEnTemplate { result }).into_response()
}

pub async fn oferta() -> impl IntoResponse {
    HtmlTemplate(OfertaTemplate {})
}

pub async fn oferta_en() -> impl IntoResponse {
    HtmlTemplate(OfertaEnTemplate {})
}

pub async fn howto() -> impl IntoResponse {
    HtmlTemplate(HowToTemplate {})
}

pub async fn howto_en() -> impl IntoResponse {
    HtmlTemplate(HowToEnTemplate {})
}

pub async fn nothing() -> impl IntoResponse {
    tracing::warn!("404 Not Found - unknown route requested");
    StatusCode::NOT_FOUND
}

#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
    database: String,
    timestamp: i64,
}

pub async fn health(State(state): State<Arc<DreamState>>) -> impl IntoResponse {
    // Check database connectivity
    let db_status = match sqlx::query("SELECT 1").fetch_one(&state.db_pool).await {
        Ok(_) => {
            tracing::debug!("Health check: database connection OK");
            "connected"
        }
        Err(e) => {
            tracing::error!(error = %e, "Health check: database connection failed");
            "disconnected"
        }
    };

    let response = HealthResponse {
        status: if db_status == "connected" {
            "healthy".to_string()
        } else {
            "unhealthy".to_string()
        },
        database: db_status.to_string(),
        timestamp: chrono::Utc::now().timestamp(),
    };

    let status_code = if db_status == "connected" {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    (status_code, Json(response))
}
