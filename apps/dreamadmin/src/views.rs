//////////////////////////////////////////
// Dreamadmin views
//////////////////////////////////////////
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Form,
};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct SealForm {
    pub name: String,
    pub name_en: String,
    pub image: String,
    pub archetype: String,
    pub archetype_en: String,
    pub archetype_description: String,
    pub archetype_description_short: String,
    pub archetype_description_en: String,
    pub archetype_description_short_en: String,
    pub portrait_description: String,
    pub portrait_description_short: String,
    pub portrait_description_en: String,
    pub portrait_description_short_en: String,
    pub type_description: String,
    pub type_description_short: String,
    pub type_description_en: String,
    pub type_description_short_en: String,
}

use crate::auth::{AuthSession, Credentials};
use crate::templates::{HtmlTemplate, LoginTemplate, AdminTemplate, SealDetailTemplate};
use crate::AdminState;
use tzolkin::{get_all_seals, get_seal, update_seal, Seal};

pub async fn admin(State(state): State<Arc<AdminState>>) -> HtmlTemplate<AdminTemplate> {
    let seals = match get_all_seals(&state.db_pool).await {
        Ok(seals) => seals,
        Err(e) => {
            tracing::error!(error = %e, "Failed to fetch seals");
            Vec::new()
        }
    };
    
    HtmlTemplate(AdminTemplate { seals })
}

pub async fn login_get() -> HtmlTemplate<LoginTemplate> {
    HtmlTemplate(LoginTemplate { error: None })
}

pub async fn login_post(
    mut auth_session: AuthSession,
    Form(credentials): Form<Credentials>,
) -> impl IntoResponse {
    tracing::info!("Login attempt");
    
    match auth_session.authenticate(credentials.clone()).await {
        Ok(Some(user)) => {
            if auth_session.login(&user).await.is_ok() {
                tracing::info!("Successful admin login");
                Redirect::to("/admin").into_response()
            } else {
                tracing::error!("Session login failed after successful authentication");
                (StatusCode::INTERNAL_SERVER_ERROR, "Login failed").into_response()
            }
        }
        Ok(None) => {
            tracing::warn!("Failed admin login attempt: invalid password");
            let template = HtmlTemplate(LoginTemplate {
                error: Some("Invalid password".to_string()),
            });
            template.into_response()
        }
        Err(e) => {
            tracing::error!(
                error = ?e,
                "Authentication system error"
            );
            (StatusCode::INTERNAL_SERVER_ERROR, "Authentication error").into_response()
        }
    }
}

pub async fn root_redirect() -> impl IntoResponse {
    Redirect::to("/login")
}

pub async fn logout(mut auth_session: AuthSession) -> impl IntoResponse {
    tracing::info!("Admin logout");
    match auth_session.logout().await {
        Ok(_) => Redirect::to("/login").into_response(),
        Err(_) => {
            tracing::error!("Failed to logout");
            Redirect::to("/login").into_response()
        }
    }
}

pub async fn seal_detail(
    State(state): State<Arc<AdminState>>,
    Path(id): Path<u32>,
) -> impl IntoResponse {
    match get_seal(&state.db_pool, id).await {
        Ok(seal) => {
            HtmlTemplate(SealDetailTemplate { seal }).into_response()
        }
        Err(e) => {
            tracing::error!(seal_id = %id, error = %e, "Failed to fetch seal");
            (StatusCode::NOT_FOUND, "Seal not found").into_response()
        }
    }
}

pub async fn seal_update(
    State(state): State<Arc<AdminState>>,
    Path(id): Path<u32>,
    Form(form): Form<SealForm>,
) -> impl IntoResponse {
    tracing::info!(seal_id = %id, "Seal update requested");
    
    let seal = Seal {
        id: id as u8,
        name: form.name,
        name_en: form.name_en,
        image: form.image,
        archetype: form.archetype,
        archetype_en: form.archetype_en,
        archetype_description: form.archetype_description,
        archetype_description_short: form.archetype_description_short,
        archetype_description_en: form.archetype_description_en,
        archetype_description_short_en: form.archetype_description_short_en,
        portrait_description: form.portrait_description,
        portrait_description_short: form.portrait_description_short,
        portrait_description_en: form.portrait_description_en,
        portrait_description_short_en: form.portrait_description_short_en,
        type_description: form.type_description,
        type_description_short: form.type_description_short,
        type_description_en: form.type_description_en,
        type_description_short_en: form.type_description_short_en,
    };
    
    match update_seal(&state.db_pool, &seal).await {
        Ok(_) => {
            tracing::info!(seal_id = %id, "Seal updated successfully");
            Redirect::to("/admin").into_response()
        }
        Err(e) => {
            tracing::error!(seal_id = %id, error = %e, "Failed to update seal");
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update seal").into_response()
        }
    }
}



pub async fn nothing() -> impl IntoResponse {
    tracing::warn!("404 Not Found - unknown route requested");
    (StatusCode::NOT_FOUND, "Not found")
}

